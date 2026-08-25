use std::net::IpAddr;
use std::time::Duration;

use reqwest::header::{CONTENT_TYPE, LOCATION};
use reqwest::{redirect, Client, Url};

use crate::documents::parser::{extract_text, ContentFormat};
use crate::error::AppError;
use crate::security::url_policy::normalize_research_url;

const MAX_WEB_BYTES: usize = 5 * 1024 * 1024;
const MAX_REDIRECTS: usize = 3;

pub struct ImportedWebPage {
    pub url: String,
    pub title: String,
    pub content: String,
}

pub async fn fetch_web_page(input: &str) -> Result<ImportedWebPage, AppError> {
    let mut url = normalize_research_url(input)?;
    let client = Client::builder()
        .redirect(redirect::Policy::none())
        .timeout(Duration::from_secs(15))
        .user_agent("AlphaForge/0.1 research importer")
        .build()
        .map_err(|_| AppError::Internal("Web importer could not be configured".into()))?;
    for _ in 0..=MAX_REDIRECTS {
        validate_fetch_target(&url).await?;
        let response = client
            .get(&url)
            .header("Accept", "text/html, text/plain;q=0.9")
            .send()
            .await
            .map_err(|_| AppError::Validation("Web page could not be retrieved".into()))?;
        if response.status().is_redirection() {
            let location = response
                .headers()
                .get(LOCATION)
                .and_then(|value| value.to_str().ok())
                .ok_or_else(|| AppError::Validation("Web page redirect is invalid".into()))?;
            url = Url::parse(&url)
                .and_then(|base| base.join(location))
                .map_err(|_| AppError::Validation("Web page redirect is invalid".into()))?
                .to_string();
            continue;
        }
        if !response.status().is_success() {
            return Err(AppError::Validation(
                "Web page returned an unsuccessful status".into(),
            ));
        }
        let content_type = response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        let format = if content_type.starts_with("text/html") {
            ContentFormat::Html
        } else if content_type.starts_with("text/plain") {
            ContentFormat::PlainText
        } else {
            return Err(AppError::Validation(
                "Web page must return HTML or plain text".into(),
            ));
        };
        if response
            .content_length()
            .is_some_and(|length| length > MAX_WEB_BYTES as u64)
        {
            return Err(AppError::Validation("Web page is larger than 5 MB".into()));
        }
        let mut bytes = Vec::new();
        let mut response = response;
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| AppError::Validation("Web page could not be read".into()))?
        {
            if bytes.len() + chunk.len() > MAX_WEB_BYTES {
                return Err(AppError::Validation("Web page is larger than 5 MB".into()));
            }
            bytes.extend_from_slice(&chunk);
        }
        let raw = String::from_utf8(bytes)
            .map_err(|_| AppError::Validation("Web page must use UTF-8 text".into()))?;
        let content = extract_text(&raw, format)?;
        if content.is_empty() {
            return Err(AppError::Validation(
                "Web page does not contain extractable text".into(),
            ));
        }
        let title = extract_title(&raw).unwrap_or_else(|| {
            Url::parse(&url)
                .ok()
                .and_then(|parsed| parsed.host_str().map(str::to_string))
                .unwrap_or_else(|| "Imported web page".into())
        });
        return Ok(ImportedWebPage {
            url,
            title,
            content,
        });
    }
    Err(AppError::Validation(
        "Web page exceeded the redirect limit".into(),
    ))
}

async fn validate_fetch_target(input: &str) -> Result<(), AppError> {
    let normalized = normalize_research_url(input)?;
    let parsed = Url::parse(&normalized)
        .map_err(|_| AppError::Validation("Web page URL is invalid".into()))?;
    let host = parsed
        .host_str()
        .ok_or_else(|| AppError::Validation("Web page URL must include a hostname".into()))?;
    let addresses = tokio::net::lookup_host((host, 443))
        .await
        .map_err(|_| AppError::Validation("Web page hostname could not be resolved".into()))?;
    if addresses
        .into_iter()
        .any(|address| !is_public_ip(address.ip()))
    {
        return Err(AppError::Validation(
            "Web page hostname resolves to a restricted address".into(),
        ));
    }
    Ok(())
}

fn is_public_ip(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(value) => {
            let octets = value.octets();
            !(octets[0] == 0
                || octets[0] == 10
                || octets[0] == 127
                || (octets[0] == 100 && (64..=127).contains(&octets[1]))
                || (octets[0] == 169 && octets[1] == 254)
                || (octets[0] == 172 && (16..=31).contains(&octets[1]))
                || (octets[0] == 192 && octets[1] == 168)
                || octets[0] >= 224
                || (octets[0] == 192 && octets[1] == 0 && octets[2] == 2)
                || (octets[0] == 198 && octets[1] == 51 && octets[2] == 100)
                || (octets[0] == 203 && octets[1] == 0 && octets[2] == 113))
        }
        IpAddr::V6(value) => {
            let segments = value.segments();
            !value.is_loopback()
                && !value.is_unspecified()
                && !value.is_multicast()
                && (segments[0] & 0xfe00) != 0xfc00
                && (segments[0] & 0xffc0) != 0xfe80
                && !(segments[0] == 0x2001 && segments[1] == 0x0db8)
        }
    }
}

fn extract_title(html: &str) -> Option<String> {
    let lower = html.to_ascii_lowercase();
    let start = lower.find("<title")?;
    let content_start = lower[start..].find('>')? + start + 1;
    let end = lower[content_start..].find("</title>")? + content_start;
    let title = extract_text(&html[content_start..end], ContentFormat::Html).ok()?;
    (!title.is_empty()).then_some(title)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::{Ipv4Addr, Ipv6Addr};

    #[test]
    fn rejects_non_public_network_ranges() {
        for address in [
            IpAddr::V4(Ipv4Addr::LOCALHOST),
            IpAddr::V4(Ipv4Addr::new(10, 0, 0, 1)),
            IpAddr::V6(Ipv6Addr::LOCALHOST),
        ] {
            assert!(!is_public_ip(address));
        }
        assert!(is_public_ip(IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8))));
    }

    #[test]
    fn extracts_an_html_title() {
        assert_eq!(
            extract_title("<title>Research &amp; Data</title>"),
            Some("Research & Data".into())
        );
    }
}
