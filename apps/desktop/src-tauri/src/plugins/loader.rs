use crate::error::AppError;
use crate::plugins::registry::PluginManifest;

const BUNDLED_MANIFESTS: [&str; 5] = [
    include_str!("../../../../../plugins/company-comparison/manifest.json"),
    include_str!("../../../../../plugins/valuation-model/manifest.json"),
    include_str!("../../../../../plugins/industry-map/manifest.json"),
    include_str!("../../../../../plugins/portfolio-risk/manifest.json"),
    include_str!("../../../../../plugins/timeline/manifest.json"),
];

pub fn load_bundled_manifests() -> Result<Vec<PluginManifest>, AppError> {
    let manifests: Vec<_> = BUNDLED_MANIFESTS.iter().map(|json| PluginManifest::parse(json)).collect::<Result<_, _>>()?;
    let mut ids: Vec<_> = manifests.iter().map(|manifest| manifest.id.as_str()).collect();
    ids.sort_unstable();
    ids.dedup();
    if ids.len() != manifests.len() { return Err(AppError::Validation("Bundled plugin IDs must be unique".to_string())); }
    Ok(manifests)
}

#[cfg(test)]
mod tests { use super::*; #[test] fn loads_validated_bundled_manifests() { assert_eq!(load_bundled_manifests().unwrap().len(), 5); } }
