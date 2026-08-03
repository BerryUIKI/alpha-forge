use std::net::IpAddr;

use reqwest::Url;

use crate::error::AppError;

const MAX_RESEARCH_URL_LENGTH: usize = 2_048;

/// Normalizes a user-provided research URL before it is persisted or displayed.
///
/// Research links are opened through the narrow opener capability, never fetched by
/// the WebView. Only public HTTPS hostnames are accepted so stored provenance cannot
/// point at local services, credentials, or non-web schemes.
pub fn normalize_research_url(input: &str) -> Result<String, AppError> {
    let value = input.trim();
    if value.is_empty() || value.len() > MAX_RESEARCH_URL_LENGTH {
        return Err(AppError::Validation(
            "Source URL must be between 1 and 2048 characters".into(),
        ));
    }

    let parsed =
        Url::parse(value).map_err(|_| AppError::Validation("Source URL is invalid".into()))?;
    if parsed.scheme() != "https" {
        return Err(AppError::Validation("Source URL must use HTTPS".into()));
    }
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return Err(AppError::Validation(
            "Source URL must not contain credentials".into(),
        ));
    }
    if parsed.port().is_some() {
        return Err(AppError::Validation(
            "Source URL must not use a custom port".into(),
        ));
    }

    let host = parsed
        .host_str()
        .ok_or_else(|| AppError::Validation("Source URL must include a hostname".into()))?;
    if host.eq_ignore_ascii_case("localhost")
        || host.ends_with(".localhost")
        || host.parse::<IpAddr>().is_ok()
    {
        return Err(AppError::Validation(
            "Source URL must use a public hostname".into(),
        ));
    }

    Ok(parsed.to_string())
}

pub fn normalize_optional_research_url(input: Option<String>) -> Result<Option<String>, AppError> {
    input
        .map(|value| normalize_research_url(&value))
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_and_normalizes_public_https_urls() {
        assert_eq!(
            normalize_research_url(" https://example.com/research?q=ai ").unwrap(),
            "https://example.com/research?q=ai"
        );
    }

    #[test]
    fn rejects_non_public_or_credentialed_destinations() {
        for value in [
            "http://example.com",
            "https://localhost:8443",
            "https://127.0.0.1",
            "https://user:secret@example.com",
        ] {
            assert!(
                normalize_research_url(value).is_err(),
                "{value} should be rejected"
            );
        }
    }
}
