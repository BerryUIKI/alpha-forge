-- Preserve one historical row per permission before enforcing the invariant.
DELETE FROM plugin_permissions
WHERE rowid NOT IN (
    SELECT MIN(rowid)
    FROM plugin_permissions
    GROUP BY plugin_id, permission
);

-- Ensure each declared permission is stored at most once for an internal plugin.
CREATE UNIQUE INDEX IF NOT EXISTS idx_plugin_permissions_plugin_permission
    ON plugin_permissions(plugin_id, permission);
