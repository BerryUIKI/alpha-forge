// Tests for the provider configuration service.

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::error::AppError;
    use crate::security::credentials::{CredentialStore, MemoryCredentialStore, OPENAI_API_KEY};
    use crate::services::provider_service::ProviderService;

    fn setup() -> (ProviderService, Arc<MemoryCredentialStore>) {
        let store = Arc::new(MemoryCredentialStore::new());
        (ProviderService::new(store.clone(), "gpt-4o-mini"), store)
    }

    #[test]
    fn test_status_starts_unconfigured() {
        let (service, _) = setup();

        let status = service.status().expect("status should load");

        assert_eq!(status.provider, "OpenAI");
        assert!(!status.configured);
        assert_eq!(status.model, "gpt-4o-mini");
    }

    #[test]
    fn test_set_api_key_marks_provider_configured() {
        let (service, store) = setup();

        service.set_api_key("sk-test").expect("key should save");

        assert!(service.status().expect("status should load").configured);
        assert_eq!(
            store.get(OPENAI_API_KEY).expect("key should load"),
            Some("sk-test".to_string())
        );
    }

    #[test]
    fn test_set_api_key_trims_whitespace() {
        let (service, store) = setup();

        service.set_api_key("  sk-test  ").expect("key should save");

        assert_eq!(
            store.get(OPENAI_API_KEY).expect("key should load"),
            Some("sk-test".to_string())
        );
    }

    #[test]
    fn test_empty_api_key_is_rejected() {
        let (service, store) = setup();

        let result = service.set_api_key("   ");

        assert!(matches!(result, Err(AppError::Validation(_))));
        assert!(store
            .get(OPENAI_API_KEY)
            .expect("key should load")
            .is_none());
    }

    #[test]
    fn test_clear_api_key_removes_configuration() {
        let (service, store) = setup();
        service.set_api_key("sk-test").expect("key should save");

        service.clear_api_key().expect("key should clear");

        assert!(!service.status().expect("status should load").configured);
        assert!(store
            .get(OPENAI_API_KEY)
            .expect("key should load")
            .is_none());
    }
}
