// Tests for MarketDataService.
//
// Covers asset-to-instrument mapping, quote fetching, upserting quotes,
// skipping manual quote mode, and refreshing all active assets.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use chrono::Utc;
    use domain::financial::{AssetKind, CreateAssetInput, InstrumentType, QuoteMode};
    use market_data::{FixtureProvider, ProviderRegistry, ResolverChain};

    use crate::database::repositories::asset_repository::{AssetRepository, QuoteRepository};
    use crate::database::repositories::test_support::setup_test_db;
    use crate::services::market_data_service::MarketDataService;

    #[tokio::test]
    async fn test_market_data_service_refresh_quote() {
        let pool = setup_test_db().await;
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let quote_repo = Arc::new(QuoteRepository::new(pool.clone()));

        // Setup service with fixture provider
        let resolver = Arc::new(ResolverChain::new());
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(Arc::new(FixtureProvider::new_for_provider("FIXTURE")));
        let service = MarketDataService::with_registry(
            asset_repo.clone(),
            quote_repo.clone(),
            Arc::new(registry),
        );

        // Create an active equity asset
        let asset = asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some("Apple Inc".to_string()),
                display_code: Some("AAPL".to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some("AAPL".to_string()),
                instrument_exchange_mic: Some("XNAS".to_string()),
                provider_config: None,
            })
            .await
            .expect("create asset");

        // Refresh quote
        let quote = service
            .refresh_quote_for_asset(&asset.id)
            .await
            .expect("refresh quote");

        assert_eq!(quote.asset_id, asset.id);
        assert_eq!(quote.source, "FIXTURE");
        assert_eq!(quote.currency, "USD");
        assert!(quote.close > rust_decimal::Decimal::ZERO);

        // Verify quote is in the repository
        let today = Utc::now().date_naive();
        let fetched = quote_repo
            .get_for_day(&asset.id, &today, "FIXTURE")
            .await
            .expect("query quote")
            .expect("found quote");

        assert_eq!(fetched.close, quote.close);
    }

    #[tokio::test]
    async fn test_market_data_service_manual_asset_fails() {
        let pool = setup_test_db().await;
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let quote_repo = Arc::new(QuoteRepository::new(pool.clone()));

        let service = MarketDataService::new(asset_repo.clone(), quote_repo.clone());

        // Create a manual quote mode asset
        let asset = asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Property,
                name: Some("Downtown Condo".to_string()),
                display_code: Some("CONDO".to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Manual,
                quote_ccy: "USD".to_string(),
                instrument_type: None,
                instrument_symbol: None,
                instrument_exchange_mic: None,
                provider_config: None,
            })
            .await
            .expect("create asset");

        let res = service.refresh_quote_for_asset(&asset.id).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn test_market_data_service_refresh_all_active() {
        let pool = setup_test_db().await;
        let asset_repo = Arc::new(AssetRepository::new(pool.clone()));
        let quote_repo = Arc::new(QuoteRepository::new(pool.clone()));

        let resolver = Arc::new(ResolverChain::new());
        let mut registry = ProviderRegistry::new(resolver);
        registry.register(Arc::new(FixtureProvider::new_for_provider("FIXTURE")));
        let service = MarketDataService::with_registry(
            asset_repo.clone(),
            quote_repo.clone(),
            Arc::new(registry),
        );

        // Create 2 market assets and 1 inactive asset
        asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some("Microsoft".to_string()),
                display_code: Some("MSFT".to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some("MSFT".to_string()),
                instrument_exchange_mic: Some("XNAS".to_string()),
                provider_config: None,
            })
            .await
            .expect("create msft");

        asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some("Bitcoin".to_string()),
                display_code: Some("BTC".to_string()),
                notes: None,
                is_active: true,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Crypto),
                instrument_symbol: Some("BTC".to_string()),
                instrument_exchange_mic: None,
                provider_config: None,
            })
            .await
            .expect("create btc");

        asset_repo
            .create(CreateAssetInput {
                kind: AssetKind::Investment,
                name: Some("Inactive Inc".to_string()),
                display_code: Some("INACT".to_string()),
                notes: None,
                is_active: false,
                quote_mode: QuoteMode::Market,
                quote_ccy: "USD".to_string(),
                instrument_type: Some(InstrumentType::Equity),
                instrument_symbol: Some("INACT".to_string()),
                instrument_exchange_mic: None,
                provider_config: None,
            })
            .await
            .expect("create inactive");

        let refreshed = service
            .refresh_all_active_quotes()
            .await
            .expect("refresh all active");

        // Should have refreshed the 2 active assets
        assert_eq!(refreshed.len(), 2);
    }
}
