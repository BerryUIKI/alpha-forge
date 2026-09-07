// MarketDataService — Orchestrates asset quote resolution, provider fetching,
// and quote persistence into SQLite QuoteRepository.
//
// Complies with AlphaForge principles:
// - Explicit typed AppError (no unwrap / expect panics)
// - Thin command layer delegation
// - Fallback to FixtureProvider in tests/offline mode

use std::sync::Arc;

use chrono::Utc;
use domain::financial::{Asset, InstrumentType, Quote as DomainQuote, QuoteMode, UpsertQuoteInput};
use market_data::{
    FixtureProvider, InstrumentId, ProviderRegistry, QuoteContext, QuoteIdentifiers, ResolverChain,
    YahooProvider,
};
use std::borrow::Cow;

use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
use crate::error::AppError;

pub struct MarketDataService {
    asset_repo: Arc<AssetRepository>,
    quote_repo: Arc<QuoteRepository>,
    registry: Arc<ProviderRegistry>,
}

impl MarketDataService {
    pub fn new(asset_repo: Arc<AssetRepository>, quote_repo: Arc<QuoteRepository>) -> Self {
        let resolver = Arc::new(ResolverChain::new());
        let mut registry = ProviderRegistry::new(resolver);

        // Register default providers:
        // Priority 1: FixtureProvider (deterministic offline / fallback quotes)
        // Priority 5: YahooProvider
        registry.register(Arc::new(FixtureProvider::new_for_provider("FIXTURE")));
        registry.register(Arc::new(YahooProvider::new()));

        Self {
            asset_repo,
            quote_repo,
            registry: Arc::new(registry),
        }
    }

    /// Construct with a custom ProviderRegistry (e.g. for testing)
    pub fn with_registry(
        asset_repo: Arc<AssetRepository>,
        quote_repo: Arc<QuoteRepository>,
        registry: Arc<ProviderRegistry>,
    ) -> Self {
        Self {
            asset_repo,
            quote_repo,
            registry,
        }
    }

    /// Map a domain Asset into a market-data InstrumentId
    pub fn map_asset_to_instrument(asset: &Asset) -> Result<InstrumentId, AppError> {
        let symbol = asset
            .instrument_symbol
            .as_deref()
            .or(asset.display_code.as_deref())
            .ok_or_else(|| {
                AppError::Validation(format!(
                    "asset {} has no symbol or display_code for quote lookup",
                    asset.id
                ))
            })?;

        let inst_type = asset.instrument_type.unwrap_or(InstrumentType::Equity);
        let mic = asset
            .instrument_exchange_mic
            .as_ref()
            .map(|m| Cow::Owned(m.clone()));

        match inst_type {
            InstrumentType::Equity => Ok(InstrumentId::Equity {
                ticker: Arc::from(symbol),
                mic,
            }),
            InstrumentType::Crypto => {
                let quote_ccy = if asset.quote_ccy.is_empty() {
                    "USD"
                } else {
                    &asset.quote_ccy
                };
                Ok(InstrumentId::Crypto {
                    base: Arc::from(symbol),
                    quote: Cow::Owned(quote_ccy.to_string()),
                })
            }
            InstrumentType::Fx => {
                let quote_ccy = if asset.quote_ccy.is_empty() {
                    "USD"
                } else {
                    &asset.quote_ccy
                };
                Ok(InstrumentId::Fx {
                    base: Cow::Owned(symbol.to_string()),
                    quote: Cow::Owned(quote_ccy.to_string()),
                })
            }
            InstrumentType::Metal => {
                let quote_ccy = if asset.quote_ccy.is_empty() {
                    "USD"
                } else {
                    &asset.quote_ccy
                };
                Ok(InstrumentId::Metal {
                    code: Arc::from(symbol),
                    quote: Cow::Owned(quote_ccy.to_string()),
                })
            }
            InstrumentType::Option => Ok(InstrumentId::Option {
                occ_symbol: Arc::from(symbol),
            }),
        }
    }

    /// Refresh and persist latest quote for a single asset.
    pub async fn refresh_quote_for_asset(&self, asset_id: &str) -> Result<DomainQuote, AppError> {
        let asset = self
            .asset_repo
            .get(asset_id)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("asset {asset_id} not found")))?;

        if asset.quote_mode == QuoteMode::Manual {
            return Err(AppError::Validation(format!(
                "asset {asset_id} is in MANUAL quote mode and cannot be auto-refreshed"
            )));
        }

        let instrument = Self::map_asset_to_instrument(&asset)?;
        let currency_hint = if asset.quote_ccy.is_empty() {
            None
        } else {
            Some(Cow::Owned(asset.quote_ccy.clone()))
        };

        let context = QuoteContext {
            instrument,
            identifiers: QuoteIdentifiers::default(),
            overrides: None,
            currency_hint,
            preferred_provider: None,
            bond_metadata: None,
            custom_provider_code: None,
        };

        let fetched_quote = self
            .registry
            .get_latest_quote(&context)
            .await
            .map_err(|e| {
                AppError::Internal(format!("failed to fetch quote for asset {asset_id}: {e}"))
            })?;

        let today = Utc::now().date_naive();
        let upsert_input = UpsertQuoteInput {
            asset_id: asset.id.clone(),
            day: today,
            source: fetched_quote.source.clone(),
            open: fetched_quote.open,
            high: fetched_quote.high,
            low: fetched_quote.low,
            close: fetched_quote.close,
            adjclose: None,
            volume: fetched_quote.volume,
            currency: if fetched_quote.currency.is_empty() {
                asset.quote_ccy.clone()
            } else {
                fetched_quote.currency
            },
            notes: Some(format!("Auto-refreshed via {}", fetched_quote.source)),
        };

        self.quote_repo.upsert(upsert_input).await
    }

    /// Refresh latest quotes for all active market-mode assets.
    pub async fn refresh_all_active_quotes(&self) -> Result<Vec<DomainQuote>, AppError> {
        let active_assets = self.asset_repo.list_active().await?;
        let mut refreshed = Vec::new();

        for asset in active_assets {
            if asset.quote_mode == QuoteMode::Market {
                match self.refresh_quote_for_asset(&asset.id).await {
                    Ok(q) => refreshed.push(q),
                    Err(err) => {
                        tracing::warn!(
                            asset_id = %asset.id,
                            error = %err,
                            "Failed to refresh quote for active asset, skipping"
                        );
                    }
                }
            }
        }

        Ok(refreshed)
    }

    /// List historical quotes for an asset
    pub async fn list_quotes_for_asset(
        &self,
        asset_id: &str,
    ) -> Result<Vec<DomainQuote>, AppError> {
        self.quote_repo.list_for_asset(asset_id).await
    }
}
