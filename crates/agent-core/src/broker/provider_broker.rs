use crate::error::{SupervisorError, SupervisorResult};
use agent_protocol::messages::{ProviderRequest, ProviderResponse, ProviderUsage};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;
use tracing::{debug, warn};

/// Trait for backend model completion adapters used by the host-side ProviderBroker.
#[async_trait]
pub trait ModelAdapter: Send + Sync {
    async fn complete(
        &self,
        request: &ProviderRequest,
    ) -> Result<(String, Option<ProviderUsage>), String>;
}

/// Mock/deterministic model adapter for testing and offline runs.
pub struct FixtureModelAdapter {
    canned_response: String,
}

impl FixtureModelAdapter {
    pub fn new(canned_response: impl Into<String>) -> Self {
        Self {
            canned_response: canned_response.into(),
        }
    }
}

#[async_trait]
impl ModelAdapter for FixtureModelAdapter {
    async fn complete(
        &self,
        request: &ProviderRequest,
    ) -> Result<(String, Option<ProviderUsage>), String> {
        let usage = ProviderUsage {
            prompt_tokens: Some(request.prompt.len() as u64 / 4),
            completion_tokens: Some(self.canned_response.len() as u64 / 4),
            total_tokens: Some((request.prompt.len() + self.canned_response.len()) as u64 / 4),
            reasoning_tokens: Some(0),
            estimated_cost_usd: Some(0.001),
        };
        Ok((self.canned_response.clone(), Some(usage)))
    }
}

/// Host-owned provider broker that manages credential security, rate limits, and provider execution.
pub struct ProviderBroker {
    allowed_providers: HashSet<String>,
    allowed_models: HashSet<String>,
    adapters: HashMap<String, Arc<dyn ModelAdapter>>,
    default_timeout: Duration,
    total_tokens_used: std::sync::atomic::AtomicU64,
}

impl ProviderBroker {
    pub fn new() -> Self {
        let mut allowed_providers = HashSet::new();
        allowed_providers.insert("openai".into());
        allowed_providers.insert("deepseek".into());
        allowed_providers.insert("fixture".into());

        let mut allowed_models = HashSet::new();
        allowed_models.insert("gpt-4o".into());
        allowed_models.insert("gpt-4o-mini".into());
        allowed_models.insert("deepseek-chat".into());
        allowed_models.insert("deepseek-reasoner".into());
        allowed_models.insert("fixture-model".into());

        let mut adapters: HashMap<String, Arc<dyn ModelAdapter>> = HashMap::new();
        adapters.insert(
            "fixture".into(),
            Arc::new(FixtureModelAdapter::new(
                "Synthesized research analysis findings.",
            )),
        );

        Self {
            allowed_providers,
            allowed_models,
            adapters,
            default_timeout: Duration::from_secs(60),
            total_tokens_used: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub fn register_adapter(
        &mut self,
        provider_name: impl Into<String>,
        adapter: Arc<dyn ModelAdapter>,
    ) {
        let name = provider_name.into();
        self.allowed_providers.insert(name.clone());
        self.adapters.insert(name, adapter);
    }

    pub fn total_tokens_used(&self) -> u64 {
        self.total_tokens_used
            .load(std::sync::atomic::Ordering::Relaxed)
    }

    /// Handles a brokered provider request from an untrusted worker.
    pub async fn handle_request(
        &self,
        req: &ProviderRequest,
    ) -> SupervisorResult<ProviderResponse> {
        // 1. Validate allowlists
        if !self.allowed_providers.contains(&req.provider) {
            warn!(provider = %req.provider, "Unauthorized provider requested by worker");
            return Err(SupervisorError::Internal(format!(
                "Provider '{}' is not allowlisted",
                req.provider
            )));
        }

        if !self.allowed_models.contains(&req.model) {
            warn!(model = %req.model, "Unauthorized model requested by worker");
            return Err(SupervisorError::Internal(format!(
                "Model '{}' is not allowlisted",
                req.model
            )));
        }

        // 2. Resolve adapter
        let adapter = self.adapters.get(&req.provider).ok_or_else(|| {
            SupervisorError::Internal(format!(
                "No adapter registered for provider '{}'",
                req.provider
            ))
        })?;

        // 3. Execute with timeout
        debug!(
            request_id = %req.request_id,
            provider = %req.provider,
            model = %req.model,
            "Dispatching brokered model completion"
        );

        let completion_future = adapter.complete(req);
        let (content, usage) =
            match tokio::time::timeout(self.default_timeout, completion_future).await {
                Ok(Ok(res)) => res,
                Ok(Err(err)) => {
                    return Err(SupervisorError::Internal(format!(
                        "Model provider failed: {}",
                        err
                    )));
                }
                Err(_) => {
                    return Err(SupervisorError::Internal(
                        "Model provider request timed out".into(),
                    ));
                }
            };

        if let Some(ref u) = usage {
            if let Some(tokens) = u.total_tokens {
                self.total_tokens_used
                    .fetch_add(tokens, std::sync::atomic::Ordering::Relaxed);
            }
        }

        Ok(ProviderResponse {
            request_id: req.request_id.clone(),
            content,
            usage,
        })
    }
}

impl Default for ProviderBroker {
    fn default() -> Self {
        Self::new()
    }
}
