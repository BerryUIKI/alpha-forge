use crate::error::AppError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentFormat { PlainText, Html, Pdf }

pub fn extract_text(input: &str, format: ContentFormat) -> Result<String, AppError> {
    match format {
        ContentFormat::PlainText => Ok(normalize_whitespace(input)),
        ContentFormat::Html => Ok(normalize_whitespace(&strip_html(input))),
        ContentFormat::Pdf => Err(AppError::Validation("PDF extraction is not available in this build".into())),
    }
}

fn strip_html(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_tag = false;
    for character in input.chars() {
        match character {
            '<' => { in_tag = true; result.push(' '); }
            '>' => in_tag = false,
            _ if !in_tag => result.push(character),
            _ => {}
        }
    }
    result.replace("&amp;", "&").replace("&nbsp;", " ")
}

fn normalize_whitespace(input: &str) -> String { input.split_whitespace().collect::<Vec<_>>().join(" ") }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_visible_html_text() {
        assert_eq!(extract_text("<h1>Alpha</h1><p>Beta &amp; Gamma</p>", ContentFormat::Html).unwrap(), "Alpha Beta & Gamma");
    }

    #[test]
    fn rejects_pdf_without_a_parser() {
        assert!(matches!(extract_text("bytes", ContentFormat::Pdf), Err(AppError::Validation(_))));
    }
}
