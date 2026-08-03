# User Data Directory Specification

## Overview

This document specifies the default storage location and directory structure for user data in AlphaForge desktop application.

## Default Location

AlphaForge user data is stored in the user's Documents directory:

```
~/Documents/alpha-forge/
```

### Platform-Specific Paths

| Platform | Default Path |
|----------|--------------|
| **macOS** | `/Users/{username}/Documents/alpha-forge/` |
| **Windows** | `C:\Users\{username}\Documents\alpha-forge\` |
| **Linux** (future) | `/home/{username}/Documents/alpha-forge/` |

### Rationale

1. **Discoverability**: Users can easily find their data in a familiar location
2. **Backup-Friendly**: Documents folders are typically included in user backup strategies
3. **User Control**: Users can move or rename the directory if needed
4. **Cross-Platform Consistency**: Documents folder exists on all major platforms

## Directory Structure

```
~/Documents/alpha-forge/
├── config/                    # Application configuration
│   ├── settings.json         # User preferences (theme, locale, etc.)
│   └── profiles/             # User profile configurations (future)
│
├── workspaces/               # Workspace databases
│   ├── {workspace-id}/       # Each workspace has its own directory
│   │   ├── workspace.db      # SQLite database for the workspace
│   │   ├── workspace.db-wal  # Write-ahead log (auto-created)
│   │   ├── workspace.db-shm  # Shared memory file (auto-created)
│   │   ├── exports/          # User exports
│   │   │   └── backup-{timestamp}.db  # Manual backups
│   │   └── artifacts/        # Cached artifact data
│   │       └── {artifact-id}.json
│   └── ...
│
├── cache/                    # Temporary cache (not user data)
│   ├── pdf-text/            # Extracted PDF text cache
│   └── thumbnails/          # Image thumbnails (future)
│
└── logs/                     # Application logs
    └── alpha-forge.log      # Rolling log file
```

## Configuration Files

### settings.json

```json
{
  "version": 1,
  "locale": "zh-CN",
  "theme": "system",
  "workspace": {
    "defaultId": "uuid-here",
    "recentIds": ["uuid-1", "uuid-2"]
  },
  "updates": {
    "checkAutomatically": false,
    "lastChecked": "2026-08-03T12:00:00Z"
  },
  "features": {
    "experimental": []
  }
}
```

### Notes

- **Version**: Schema version for future migrations
- **Locale**: User's selected language
- **Theme**: `light`, `dark`, or `system`
- **Workspace**: References to workspace IDs (actual data in workspaces/)
- **Updates**: Manual update check preferences
- **Features**: Feature flags for experimental features

## Data Migration Strategy

### Versioning

1. Each config file includes a `version` field
2. When schema changes, increment version number
3. On startup, if version < current, run migration
4. Backup old file before migration

### Migration Example

```rust
fn migrate_settings(settings: &mut Settings) -> Result<()> {
    match settings.version {
        1 => {
            // Version 1 is current, no migration needed
            Ok(())
        }
        0 => {
            // Legacy format (pre-versioning)
            settings.version = 1;
            // Add new fields with defaults
            if settings.features.is_none() {
                settings.features = Features::default();
            }
            Ok(())
        }
        _ => Err(AppError::Validation(format!(
            "Unsupported settings version: {}",
            settings.version
        )))
    }
}
```

## Workspace Isolation

Each workspace is completely self-contained:

1. **Own SQLite Database**: No cross-workspace data sharing
2. **Own Exports**: Backups and exports are workspace-specific
3. **Own Artifacts**: Cached artifact data is workspace-specific

### Benefits

- **Portability**: Users can copy/move individual workspace directories
- **Backup Granularity**: Users can backup specific workspaces
- **Isolation**: Workspace corruption doesn't affect other workspaces
- **Multi-Device**: Sync individual workspaces across devices (future)

## Security Considerations

### File Permissions

- Application creates directories with user-only permissions (0700 on Unix)
- Database files are readable/writable only by the user
- No elevated privileges required

### Sensitive Data

- **No plaintext secrets**: API keys stored in system keychain
- **Database encryption**: Future consideration for sensitive data
- **Log sanitization**: Logs do not contain sensitive data

### Path Validation

- All file operations validate paths are within the data directory
- Prevent directory traversal attacks
- Reject symlinks pointing outside data directory

## Implementation Requirements

### Rust Backend

1. **Path Resolution**
   ```rust
   fn get_data_directory() -> Result<PathBuf> {
       let docs_dir = dirs::document_dir()
           .ok_or(AppError::Internal("Cannot find Documents directory".into()))?;
       Ok(docs_dir.join("alpha-forge"))
   }
   ```

2. **Directory Creation**
   - Create data directory on first startup
   - Create subdirectories lazily when needed
   - Set appropriate permissions

3. **Configuration Persistence**
   - Load settings from `config/settings.json`
   - Save settings atomically (write to temp, then rename)
   - Handle missing/corrupt files gracefully

### Frontend

- **No direct filesystem access**: All persistence through Rust backend
- **Settings UI**: Display current data directory location
- **Migration UI**: Show migration progress if needed

## User Control

### Settings UI

Users can view (but not change) the data directory in Settings:

```
About & Privacy
├── Data Directory: ~/Documents/alpha-forge/
├── Storage Used: 45 MB (calculated)
└── Open in Explorer/Finder [button]
```

### Future Features

- **Custom Location**: Allow users to change data directory (requires migration)
- **Export/Import**: Workspace export and import functionality
- **Sync**: Cloud sync integration (future)

## Backup Recommendations

### Automatic (Future)

- Weekly automatic backup of each workspace database
- Keep last 4 weekly backups + last 12 monthly backups
- Backup location: `~/Documents/alpha-forge/workspaces/{id}/backups/`

### Manual

- User-triggered backup creates timestamped copy
- User can export workspace to any location
- Export includes full database + metadata

## Cross-Platform Notes

### macOS

- Sandbox-friendly: Documents directory is within sandbox
- iCloud: Users can enable iCloud Drive for Documents folder
- Spotlight: Excludes cache and logs from indexing

### Windows

- Documents library is a known folder
- OneDrive: Users can sync Documents to OneDrive
- Windows Search: Excludes cache and logs from indexing

### Linux (Future)

- Follow XDG Base Directory Specification as fallback
- Prefer Documents directory if available
- Default to `~/.local/share/alpha-forge/` if no Documents

## Testing Requirements

1. **Directory Creation**: Verify directories created with correct permissions
2. **Path Validation**: Test directory traversal prevention
3. **Migration**: Test version upgrade and downgrade scenarios
4. **Cross-Platform**: Verify paths on macOS and Windows
5. **Missing Directory**: Handle case where user deletes data directory
6. **Corrupt Config**: Graceful recovery from corrupt settings file

## Documentation Updates

Update the following documents:

1. **ARCHITECTURE.md**: Add data directory section
2. **SECURITY.md**: Document path validation and permissions
3. **M8 Decision Record**: Record data directory decision
4. **User Guide**: Explain data directory location and backup

## Success Criteria

- [ ] Data directory created in `~/Documents/alpha-forge/`
- [ ] Subdirectories created with correct structure
- [ ] Configuration files persisted correctly
- [ ] Settings UI shows data directory location
- [ ] Path validation prevents directory traversal
- [ ] Migration handles version upgrades
- [ ] Tests cover all critical scenarios
- [ ] Documentation updated

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| User deletes data directory | Detect and recreate on startup with defaults |
| User moves data directory | Future: Support custom directory selection |
| Disk full during write | Atomic writes with rollback |
| Permission denied | Clear error message with path shown |
| Cross-device sync conflicts | Future: Add conflict resolution |

## Timeline

- **MVP**: Basic directory structure and settings persistence
- **Post-MVP**: Custom directory location, auto-backup, cloud sync

---

## References

- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- [Apple App Sandbox](https://developer.apple.com/documentation/security/app_sandbox)
- [Windows Known Folders](https://docs.microsoft.com/en-us/windows/win32/shell/known-folders)