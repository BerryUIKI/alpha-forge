use serde_json::Value;

use crate::error::AppError;
use crate::plugins::registry::PluginManifest;

struct BundledPluginFiles {
    manifest: &'static str,
    input_schema_path: &'static str,
    input_schema: &'static str,
    entry_source: &'static str,
}

const BUNDLED_PLUGINS: [BundledPluginFiles; 7] = [
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/company-comparison/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/company-comparison/schema.json"),
        entry_source: include_str!("../../../../../plugins/company-comparison/src/index.ts"),
    },
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/valuation-model/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/valuation-model/schema.json"),
        entry_source: include_str!("../../../../../plugins/valuation-model/src/index.ts"),
    },
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/industry-map/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/industry-map/schema.json"),
        entry_source: include_str!("../../../../../plugins/industry-map/src/index.ts"),
    },
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/portfolio-risk/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/portfolio-risk/schema.json"),
        entry_source: include_str!("../../../../../plugins/portfolio-risk/src/index.ts"),
    },
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/timeline/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/timeline/schema.json"),
        entry_source: include_str!("../../../../../plugins/timeline/src/index.ts"),
    },
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/earnings-analyzer/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/earnings-analyzer/schema.json"),
        entry_source: include_str!("../../../../../plugins/earnings-analyzer/src/index.ts"),
    },
    BundledPluginFiles {
        manifest: include_str!("../../../../../plugins/macro-dashboard/manifest.json"),
        input_schema_path: "schema.json",
        input_schema: include_str!("../../../../../plugins/macro-dashboard/schema.json"),
        entry_source: include_str!("../../../../../plugins/macro-dashboard/src/index.ts"),
    },
];

pub fn load_bundled_manifests() -> Result<Vec<PluginManifest>, AppError> {
    let manifests: Vec<_> = BUNDLED_PLUGINS
        .iter()
        .map(|files| {
            let manifest = PluginManifest::parse(files.manifest)?;
            validate_bundled_files(&manifest, files)?;
            Ok(manifest)
        })
        .collect::<Result<_, AppError>>()?;
    let mut ids: Vec<_> = manifests.iter().map(|manifest| manifest.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    if ids.len() != manifests.len() { return Err(AppError::Validation("Bundled plugin IDs must be unique".to_string())); }
    Ok(manifests)
}

pub fn validate_bundled_payload(plugin_id: &str, payload: &Value) -> Result<(), AppError> {
    let files = BUNDLED_PLUGINS.iter().find(|files| {
        PluginManifest::parse(files.manifest)
            .map(|manifest| manifest.id == plugin_id)
            .unwrap_or(false)
    }).ok_or_else(|| AppError::NotFound("Internal plugin not found".to_string()))?;
    let schema: Value = serde_json::from_str(files.input_schema)
        .map_err(|_| AppError::Internal("Bundled plugin input schema is invalid".to_string()))?;
    validate_json_schema(payload, &schema)
}

fn validate_bundled_files(manifest: &PluginManifest, files: &BundledPluginFiles) -> Result<(), AppError> {
    if manifest.input_schema != files.input_schema_path || manifest.entry != "src/index.ts" || files.entry_source.trim().is_empty() {
        return Err(AppError::Validation("Bundled plugin files do not match the manifest".to_string()));
    }

    let schema: Value = serde_json::from_str(files.input_schema)
        .map_err(|_| AppError::Validation("Bundled plugin input schema is invalid JSON".to_string()))?;
    let object = schema.as_object().ok_or_else(|| AppError::Validation("Bundled plugin input schema must be an object".to_string()))?;
    if object.get("type").and_then(Value::as_str) != Some("object") || !object.get("properties").is_some_and(Value::is_object) {
        return Err(AppError::Validation("Bundled plugin input schema must define object properties".to_string()));
    }
    if let Some(required) = object.get("required").and_then(Value::as_array) {
        let properties = object
            .get("properties")
            .and_then(Value::as_object)
            .ok_or_else(|| AppError::Internal("Bundled plugin schema is missing properties".to_string()))?;
        if required.iter().any(|field| field.as_str().is_none_or(|name| !properties.contains_key(name))) {
            return Err(AppError::Validation("Bundled plugin schema required fields must be declared".to_string()));
        }
    }
    Ok(())
}

fn validate_json_schema(value: &Value, schema: &Value) -> Result<(), AppError> {
    let object = schema.as_object().ok_or_else(|| AppError::Internal("Bundled plugin schema is not an object".to_string()))?;
    match object.get("type").and_then(Value::as_str) {
        Some("object") => {
            let value = value.as_object().ok_or_else(|| AppError::Validation("Plugin payload must be an object".to_string()))?;
            let properties = object.get("properties").and_then(Value::as_object).ok_or_else(|| AppError::Internal("Bundled plugin schema is missing properties".to_string()))?;
            for field in object.get("required").and_then(Value::as_array).into_iter().flatten() {
                let field = field.as_str().ok_or_else(|| AppError::Internal("Bundled plugin schema contains an invalid required field".to_string()))?;
                if !value.contains_key(field) { return Err(AppError::Validation(format!("Plugin payload is missing required field: {field}"))); }
            }
            for (field, field_schema) in properties {
                if let Some(field_value) = value.get(field) { validate_json_schema(field_value, field_schema)?; }
            }
        }
        Some("array") => {
            let value = value.as_array().ok_or_else(|| AppError::Validation("Plugin payload contains a field with an invalid array value".to_string()))?;
            if let Some(min_items) = object.get("minItems").and_then(Value::as_u64) { if value.len() < min_items as usize { return Err(AppError::Validation("Plugin payload array has too few items".to_string())); } }
            if let Some(item_schema) = object.get("items") { for item in value { validate_json_schema(item, item_schema)?; } }
        }
        Some("string") => {
            let value = value.as_str().ok_or_else(|| AppError::Validation("Plugin payload contains a field with an invalid string value".to_string()))?;
            if let Some(min_length) = object.get("minLength").and_then(Value::as_u64) { if value.chars().count() < min_length as usize { return Err(AppError::Validation("Plugin payload contains a string that is too short".to_string())); } }
        }
        Some("number") => {
            let value = value.as_f64().ok_or_else(|| AppError::Validation("Plugin payload contains a field with an invalid number value".to_string()))?;
            if !value.is_finite() { return Err(AppError::Validation("Plugin payload contains a non-finite number".to_string())); }
            if let Some(minimum) = object.get("minimum").and_then(Value::as_f64) { if value < minimum { return Err(AppError::Validation("Plugin payload contains a number below its minimum".to_string())); } }
            if let Some(maximum) = object.get("maximum").and_then(Value::as_f64) { if value > maximum { return Err(AppError::Validation("Plugin payload contains a number above its maximum".to_string())); } }
        }
        Some(_) | None => return Err(AppError::Internal("Bundled plugin schema contains an unsupported type".to_string())),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_validated_bundled_manifests() { assert_eq!(load_bundled_manifests().unwrap().len(), 7); }

    #[test]
    fn validates_payloads_against_the_bundled_schema() {
        let valid = serde_json::json!({
            "companies": [{"ticker": "AAA"}, {"ticker": "BBB"}],
            "comparisonDimensions": ["revenue"]
        });
        assert!(validate_bundled_payload("company-comparison", &valid).is_ok());
        assert!(validate_bundled_payload("company-comparison", &serde_json::json!({"companies": []})).is_err());
    }
}
