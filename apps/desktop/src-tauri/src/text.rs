// Shared text validation and truncation helpers.

use crate::error::AppError;

/// Trims `value` and rejects empty input.
pub fn non_empty_text(value: &str, field: &str) -> Result<String, AppError> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(AppError::Validation(format!("{} cannot be empty", field)));
    }

    Ok(trimmed.to_string())
}

/// Trims `value`, rejects empty input, and enforces a character limit.
pub fn required_text(value: &str, field: &str, max_chars: usize) -> Result<String, AppError> {
    let trimmed = non_empty_text(value, field)?;

    if trimmed.chars().count() > max_chars {
        return Err(AppError::Validation(format!(
            "{} cannot exceed {} characters",
            field, max_chars
        )));
    }

    Ok(trimmed)
}

/// Truncates to `max_chars` and marks the cut with an ellipsis.
pub fn truncate(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let mut truncated: String = value.chars().take(max_chars.saturating_sub(1)).collect();
    truncated.push('…');
    truncated
}

/// Truncates to `max_chars` and appends a visible truncation notice.
pub fn truncate_with_notice(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let truncated: String = value.chars().take(max_chars).collect();
    format!("{truncated}… (truncated)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_non_empty_text_trims() {
        assert_eq!(non_empty_text("  note  ", "title").unwrap(), "note");
        assert!(matches!(
            non_empty_text("   ", "title"),
            Err(AppError::Validation(_))
        ));
    }

    #[test]
    fn test_required_text_enforces_limit() {
        assert_eq!(required_text("abc", "title", 3).unwrap(), "abc");
        assert!(matches!(
            required_text("abcd", "title", 3),
            Err(AppError::Validation(_))
        ));
    }

    #[test]
    fn test_truncate_uses_ellipsis() {
        assert_eq!(truncate("short", 10), "short");
        assert_eq!(truncate("abcdef", 4), "abc…");
    }

    #[test]
    fn test_truncate_with_notice_marks_cut() {
        assert_eq!(truncate_with_notice("short", 10), "short");
        assert_eq!(truncate_with_notice("abcdef", 4), "abcd… (truncated)");
    }
}
