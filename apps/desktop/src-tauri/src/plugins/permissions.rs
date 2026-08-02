use crate::plugins::registry::{PluginManifest, PluginPermission};

pub fn permits(manifest: &PluginManifest, permission: PluginPermission) -> bool {
    manifest.permissions.contains(&permission)
}

pub fn permits_network(manifest: &PluginManifest) -> bool {
    permits(manifest, PluginPermission::Network)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugins::registry::{PluginManifest, PluginWindow};
    #[test] fn denies_undeclared_network_access() { let manifest = PluginManifest { id: "test-plugin".into(), name: "Test".into(), version: "1".into(), entry: "index.html".into(), input_schema: "schema.json".into(), permissions: vec![], window: PluginWindow { width: 400, height: 300, resizable: true } }; assert!(!permits_network(&manifest)); }
}
