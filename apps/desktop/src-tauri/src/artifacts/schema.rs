// Artifact payload validation.
//
// The model proposes an artifact with a type, title, and data object. Each
// supported type has an explicit schema; invalid drafts never reach storage.

use std::collections::HashSet;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::AppError;
use crate::text::{required_text, truncate};

pub const COMPARISON_TABLE: &str = "comparison-table";
pub const TIMELINE: &str = "timeline";

const MAX_TITLE_CHARS: usize = 200;
const MAX_DATA_CHARS: usize = 200_000;
const MAX_COLUMNS: usize = 8;
const MAX_ROWS: usize = 50;
const MAX_EVENTS: usize = 100;
const MAX_TEXT_CHARS: usize = 500;

/// An artifact as proposed by the model, before validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawArtifactDraft {
    #[serde(rename = "type")]
    pub artifact_type: String,
    pub title: String,
    pub data: Value,
}

/// A validated artifact ready for persistence.
#[derive(Debug, Clone, PartialEq)]
pub struct ArtifactDraft {
    pub artifact_type: String,
    pub title: String,
    /// Canonical JSON payload.
    pub data: String,
}

/// Validates and normalizes a proposed artifact.
pub fn validate_draft(raw: &RawArtifactDraft) -> Result<ArtifactDraft, AppError> {
    let title = required_text(&raw.title, "artifact title", MAX_TITLE_CHARS)?;

    let validated = match raw.artifact_type.as_str() {
        COMPARISON_TABLE => serde_json::to_value(validate_comparison_table(raw.data.clone())?),
        TIMELINE => serde_json::to_value(validate_timeline(raw.data.clone())?),
        other => {
            return Err(AppError::Validation(format!(
                "unsupported artifact type '{}'",
                other
            )))
        }
    }
    .map_err(|error| AppError::Internal(format!("failed to serialize artifact: {error}")))?;

    let data = serde_json::to_string(&validated)
        .map_err(|error| AppError::Internal(format!("failed to serialize artifact: {error}")))?;

    if data.chars().count() > MAX_DATA_CHARS {
        return Err(AppError::Validation(
            "artifact data is too large".to_string(),
        ));
    }

    Ok(ArtifactDraft {
        artifact_type: raw.artifact_type.clone(),
        title,
        data,
    })
}

#[derive(Debug, Deserialize, Serialize)]
struct ComparisonTableInput {
    columns: Vec<ComparisonColumn>,
    rows: Vec<ComparisonRow>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComparisonColumn {
    key: String,
    label: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ComparisonRow {
    label: String,
    #[serde(default)]
    values: serde_json::Map<String, Value>,
}

fn validate_comparison_table(data: Value) -> Result<ComparisonTableInput, AppError> {
    let mut input: ComparisonTableInput = serde_json::from_value(data)
        .map_err(|error| AppError::Validation(format!("invalid comparison-table data: {error}")))?;

    if input.columns.is_empty() || input.columns.len() > MAX_COLUMNS {
        return Err(AppError::Validation(format!(
            "comparison-table requires between 1 and {} columns",
            MAX_COLUMNS
        )));
    }

    let mut keys = HashSet::new();
    for column in &mut input.columns {
        column.key = required_text(&column.key, "column key", 100)?;
        column.label = required_text(&column.label, "column label", 100)?;
        if !keys.insert(column.key.clone()) {
            return Err(AppError::Validation(format!(
                "duplicate comparison column key '{}'",
                column.key
            )));
        }
    }

    if input.rows.is_empty() || input.rows.len() > MAX_ROWS {
        return Err(AppError::Validation(format!(
            "comparison-table requires between 1 and {} rows",
            MAX_ROWS
        )));
    }

    for row in &mut input.rows {
        row.label = required_text(&row.label, "row label", MAX_TEXT_CHARS)?;
    }

    Ok(input)
}

#[derive(Debug, Deserialize, Serialize)]
struct TimelineInput {
    events: Vec<TimelineEvent>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TimelineEvent {
    date: String,
    title: String,
    #[serde(default)]
    description: Option<String>,
}

fn validate_timeline(data: Value) -> Result<TimelineInput, AppError> {
    let mut input: TimelineInput = serde_json::from_value(data)
        .map_err(|error| AppError::Validation(format!("invalid timeline data: {error}")))?;

    if input.events.is_empty() || input.events.len() > MAX_EVENTS {
        return Err(AppError::Validation(format!(
            "timeline requires between 1 and {} events",
            MAX_EVENTS
        )));
    }

    for event in &mut input.events {
        event.date = required_text(&event.date, "event date", 100)?;
        event.title = required_text(&event.title, "event title", MAX_TITLE_CHARS)?;
        event.description = event
            .description
            .as_deref()
            .map(str::trim)
            .filter(|description| !description.is_empty())
            .map(|description| truncate(description, MAX_TEXT_CHARS));
    }

    Ok(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn draft(artifact_type: &str, data: Value) -> RawArtifactDraft {
        RawArtifactDraft {
            artifact_type: artifact_type.to_string(),
            title: "Tesla vs Nvidia".to_string(),
            data,
        }
    }

    #[test]
    fn test_valid_comparison_table_is_normalized() {
        let raw = draft(
            COMPARISON_TABLE,
            json!({
                "columns": [
                    {"key": "pe", "label": "P/E"},
                    {"key": "growth", "label": "Growth"}
                ],
                "rows": [
                    {"label": " Tesla ", "values": {"pe": 28.5, "growth": "5%"}},
                    {"label": "Nvidia", "values": {"pe": 40.0}}
                ]
            }),
        );

        let validated = validate_draft(&raw).expect("artifact should validate");
        let data: Value = serde_json::from_str(&validated.data).expect("data should be JSON");

        assert_eq!(validated.artifact_type, COMPARISON_TABLE);
        assert_eq!(validated.title, "Tesla vs Nvidia");
        assert_eq!(data["rows"][0]["label"], "Tesla");
    }

    #[test]
    fn test_comparison_table_rejects_duplicate_column_keys() {
        let raw = draft(
            COMPARISON_TABLE,
            json!({
                "columns": [
                    {"key": "pe", "label": "P/E"},
                    {"key": "pe", "label": "P/E again"}
                ],
                "rows": [{"label": "Tesla", "values": {}}]
            }),
        );

        assert!(matches!(validate_draft(&raw), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_comparison_table_requires_rows_and_columns() {
        let empty_columns = draft(
            COMPARISON_TABLE,
            json!({"columns": [], "rows": [{"label": "Tesla"}]}),
        );
        let empty_rows = draft(
            COMPARISON_TABLE,
            json!({"columns": [{"key": "pe", "label": "P/E"}], "rows": []}),
        );

        assert!(matches!(
            validate_draft(&empty_columns),
            Err(AppError::Validation(_))
        ));
        assert!(matches!(
            validate_draft(&empty_rows),
            Err(AppError::Validation(_))
        ));
    }

    #[test]
    fn test_valid_timeline_normalizes_descriptions() {
        let raw = draft(
            TIMELINE,
            json!({
                "events": [
                    {"date": "2026-01-15", "title": "Q4 earnings", "description": "  Beat estimates  "},
                    {"date": "2026-02-01", "title": "Thesis review", "description": "   "}
                ]
            }),
        );

        let validated = validate_draft(&raw).expect("artifact should validate");
        let data: Value = serde_json::from_str(&validated.data).expect("data should be JSON");

        assert_eq!(data["events"][0]["description"], "Beat estimates");
        assert!(data["events"][1]["description"].is_null());
    }

    #[test]
    fn test_timeline_requires_events() {
        let raw = draft(TIMELINE, json!({"events": []}));

        assert!(matches!(validate_draft(&raw), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_rejects_unsupported_artifact_type() {
        let raw = draft("pie-chart", json!({}));

        assert!(matches!(validate_draft(&raw), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_rejects_empty_title() {
        let mut raw = draft(TIMELINE, json!({"events": [{"date": "x", "title": "y"}]}));
        raw.title = "   ".to_string();

        assert!(matches!(validate_draft(&raw), Err(AppError::Validation(_))));
    }

    #[test]
    fn test_raw_draft_deserializes_from_model_output() {
        let raw: RawArtifactDraft = serde_json::from_value(json!({
            "type": "timeline",
            "title": "Timeline",
            "data": {"events": [{"date": "2026", "title": "Event"}]}
        }))
        .expect("draft should deserialize");

        assert_eq!(raw.artifact_type, TIMELINE);
    }
}
