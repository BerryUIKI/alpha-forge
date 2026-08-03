use std::path::PathBuf;

use crate::error::AppError;

const MAX_PDF_BYTES: u64 = 25 * 1024 * 1024;

pub struct SelectedPdf {
    pub title: String,
    pub content: String,
}

pub async fn select_and_extract_pdf() -> Result<Option<SelectedPdf>, AppError> {
    tauri::async_runtime::spawn_blocking(|| {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("PDF", &["pdf"])
            .pick_file()
        else {
            return Ok(None);
        };
        extract_selected_pdf(path).map(Some)
    })
    .await
    .map_err(|_| AppError::Internal("PDF import task did not complete".into()))?
}

fn extract_selected_pdf(path: PathBuf) -> Result<SelectedPdf, AppError> {
    let metadata = std::fs::metadata(&path)
        .map_err(|_| AppError::Validation("Selected PDF is no longer available".into()))?;
    if !metadata.is_file() || metadata.len() > MAX_PDF_BYTES {
        return Err(AppError::Validation(
            "Selected PDF must be a file no larger than 25 MB".into(),
        ));
    }
    let bytes = std::fs::read(&path)
        .map_err(|_| AppError::Validation("Selected PDF could not be read".into()))?;
    let content = extract_pdf_text(&bytes)?;
    let title = path
        .file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("Imported PDF")
        .to_string();
    Ok(SelectedPdf { title, content })
}

pub fn extract_pdf_text(bytes: &[u8]) -> Result<String, AppError> {
    if !bytes.starts_with(b"%PDF-") {
        return Err(AppError::Validation("Selected file is not a PDF".into()));
    }
    let text = pdf_extract::extract_text_from_mem(bytes)
        .map_err(|_| AppError::Validation("PDF text could not be extracted".into()))?;
    let normalized = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if normalized.is_empty() {
        return Err(AppError::Validation(
            "PDF does not contain extractable text".into(),
        ));
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_non_pdf_input() {
        assert!(matches!(
            extract_pdf_text(b"not a pdf"),
            Err(AppError::Validation(_))
        ));
    }
}
