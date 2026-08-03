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

## Data Recovery Guide

### Recovery Scenarios

This section guides users through recovering their AlphaForge data in various scenarios.

#### Scenario 1: Restore from Manual Backup

If you have a backup file (exported via Settings > Export Local Backup):

**macOS/Windows:**

1. Open AlphaForge
2. Go to **Settings > Local Backup**
3. Click **Import Backup**
4. Select the backup file (e.g., `backup-2026-08-03.db`)
5. Confirm the import
6. Restart the application

**Note:** Importing a backup will replace the current workspace data.

#### Scenario 2: Restore from File System Backup

If you backed up the entire `~/Documents/alpha-forge/` directory:

**macOS:**

```bash
# 1. Close AlphaForge completely

# 2. Backup current data (if exists)
mv ~/Documents/alpha-forge ~/Documents/alpha-forge.backup

# 3. Restore from backup location
cp -R /path/to/backup/alpha-forge ~/Documents/

# 4. Verify permissions
chmod -R 700 ~/Documents/alpha-forge
```

**Windows (PowerShell):**

```powershell
# 1. Close AlphaForge completely

# 2. Backup current data (if exists)
Move-Item "$env:USERPROFILE\Documents\alpha-forge" "$env:USERPROFILE\Documents\alpha-forge.backup"

# 3. Restore from backup location
Copy-Item -Path "D:\backups\alpha-forge" -Destination "$env:USERPROFILE\Documents\" -Recurse
```

#### Scenario 3: Recover Specific Workspace

To restore only one workspace without affecting others:

**Steps:**

1. Close AlphaForge
2. Navigate to `~/Documents/alpha-forge/workspaces/`
3. Identify the workspace directory by ID or creation date
4. Replace the specific workspace folder:
   - **macOS**: `cp -R /backup/alpha-forge/workspaces/{workspace-id} ~/Documents/alpha-forge/workspaces/`
   - **Windows**: `Copy-Item -Path "D:\backup\workspaces\{workspace-id}" -Destination "$env:USERPROFILE\Documents\alpha-forge\workspaces\" -Recurse`
5. Restart AlphaForge

#### Scenario 4: Recover from Corrupted Database

If a workspace database is corrupted:

**Symptoms:**
- Application crashes when opening specific workspace
- Error message: "Failed to load workspace" or "Database disk image is malformed"

**Recovery Steps:**

1. **Attempt Automatic Repair** (if SQLite WAL files exist):
   ```bash
   # The application attempts automatic recovery on startup
   # If .db-wal and .db-shm files exist, SQLite may recover
   ```

2. **Restore from Backup**:
   - Check `~/Documents/alpha-forge/workspaces/{id}/exports/` for manual backups
   - If no backup, data may be unrecoverable

3. **Create New Workspace** (if recovery fails):
   - Application will prompt to create a new workspace
   - Old corrupted database remains in place (not deleted automatically)

#### Scenario 5: Recover from Application Crash

If AlphaForge crashed and data appears missing:

1. **Check for WAL Recovery**:
   - SQLite Write-Ahead Log (WAL) files may contain uncommitted changes
   - Restart the application - SQLite will attempt automatic WAL recovery

2. **Check Workspace List**:
   - Workspaces are stored in `~/Documents/alpha-forge/workspaces/`
   - Each workspace has a unique ID directory
   - If directory exists, workspace can be reopened

3. **Check Application Logs**:
   - macOS: `~/Documents/alpha-forge/logs/alpha-forge.log`
   - Windows: `%USERPROFILE%\Documents\alpha-forge\logs\alpha-forge.log`
   - Logs may indicate what went wrong

#### Scenario 6: Recover After OS Reinstall

If you reinstalled your operating system:

1. **Restore from External Backup**:
   - Copy entire `alpha-forge` directory from external backup
   - Place in `~/Documents/` (macOS) or `%USERPROFILE%\Documents\` (Windows)

2. **Verify Application Version**:
   - Ensure you're using the same or newer version of AlphaForge
   - Older versions may not read newer database formats

3. **Check Workspace Directories**:
   - Each workspace in `workspaces/` directory should be intact
   - Workspace database files: `workspace.db`, `workspace.db-wal`, `workspace.db-shm`

### Recovery Best Practices

1. **Regular Backups**:
   - Use Settings > Export Local Backup weekly
   - Store backups in a different location (external drive, cloud storage)

2. **Test Backups**:
   - Periodically test backup restoration on a test workspace
   - Verify backup files are not corrupted

3. **Multiple Backup Copies**:
   - Keep multiple backup versions (e.g., last 4 weekly backups)
   - Use timestamped filenames: `backup-2026-08-03.db`

4. **Document Workspace IDs**:
   - Note workspace names and IDs for easier recovery
   - Workspace IDs are visible in Settings > Workspace

---

## Data Migration Guide

### Migration Scenarios

This section guides users through migrating AlphaForge data between devices or versions.

#### Scenario 1: Migrate to New Computer

**Prerequisites:**
- Both computers have AlphaForge installed
- Sufficient disk space on new computer
- External storage or network transfer method

**Steps:**

1. **On Old Computer**:
   - Close AlphaForge completely
   - Navigate to data directory:
     - macOS: `/Users/{username}/Documents/alpha-forge/`
     - Windows: `C:\Users\{username}\Documents\alpha-forge\`
   - Copy entire `alpha-forge` directory to external storage or network location

2. **Transfer Data**:
   - Use external drive, network transfer, or cloud storage
   - Ensure transfer is complete and verified

3. **On New Computer**:
   - Install AlphaForge (same version or newer)
   - Close AlphaForge if it auto-launched
   - Copy `alpha-forge` directory to Documents folder:
     - macOS: `~/Documents/alpha-forge/`
     - Windows: `%USERPROFILE%\Documents\alpha-forge\`

4. **Verify Migration**:
   - Open AlphaForge
   - Check Settings > Workspace to verify workspaces appear
   - Open each workspace to verify data integrity

**Troubleshooting:**
- If workspaces don't appear, check directory structure matches specification
- If database errors occur, ensure SQLite version compatibility
- If settings are missing, check `config/settings.json` exists

#### Scenario 2: Migrate Between macOS and Windows

**Cross-Platform Considerations:**

1. **Path Differences**:
   - macOS: `/Users/{username}/Documents/alpha-forge/`
   - Windows: `C:\Users\{username}\Documents\alpha-forge\`

2. **File System Differences**:
   - macOS uses POSIX paths (forward slashes)
   - Windows uses backslashes (handled automatically)
   - No conversion needed - paths are relative

3. **Line Endings**:
   - JSON files use UTF-8 encoding on both platforms
   - No line ending conversion required

**Migration Steps:**

1. Export data on source platform using Settings > Export Local Backup
2. Transfer backup file to target platform
3. Import backup using Settings > Import Backup

**Note:** Direct directory copy also works - AlphaForge is cross-platform compatible.

#### Scenario 3: Upgrade to New AlphaForge Version

**Before Upgrade:**

1. **Create Backup**:
   - Use Settings > Export Local Backup
   - Save backup to external location

2. **Check Release Notes**:
   - Review breaking changes or migration requirements
   - Note any database schema changes

**After Upgrade:**

1. **Automatic Migration**:
   - AlphaForge automatically migrates data on first run
   - Migration logs appear in `logs/alpha-forge.log`

2. **Verify Data**:
   - Check all workspaces appear correctly
   - Verify recent research and theses are intact
   - Check settings have been migrated

3. **Handle Migration Failures**:
   - If migration fails, application will show error message
   - Restore from backup created before upgrade
   - Report migration issue with log file attached

**Version Compatibility:**
- AlphaForge uses semantic versioning (MAJOR.MINOR.PATCH)
- MINOR and PATCH upgrades are backward-compatible
- MAJOR upgrades may require migration (documented in release notes)

#### Scenario 4: Migrate Workspace Between Users

To transfer a workspace to another user account (same computer):

**Steps:**

1. **Export Workspace** (as original user):
   - Open workspace in AlphaForge
   - Use Settings > Export Local Backup
   - Save to shared location (e.g., `/tmp/` or `C:\Temp\`)

2. **Import Workspace** (as target user):
   - Log in as target user
   - Open AlphaForge
   - Use Settings > Import Backup
   - Select the backup file

**Alternative Method:**

1. Copy workspace directory:
   - macOS: `cp -R ~/Documents/alpha-forge/workspaces/{workspace-id} /tmp/workspace-transfer`
   - Windows: `Copy-Item -Path "$env:USERPROFILE\Documents\alpha-forge\workspaces\{workspace-id}" -Destination "C:\Temp\workspace-transfer" -Recurse`

2. Change ownership:
   - macOS: `sudo chown -R targetuser:targetuser /tmp/workspace-transfer`
   - Windows: Use File Explorer > Properties > Security

3. Move to target user's directory:
   - Log in as target user
   - Move to `~/Documents/alpha-forge/workspaces/` or `%USERPROFILE%\Documents\alpha-forge\workspaces\`

#### Scenario 5: Migrate from Legacy Version

If upgrading from a version before M8 (data directory structure changed):

**Automatic Migration:**

AlphaForge M8+ automatically detects legacy data locations:
- Old location: Application-specific data directory
- New location: `~/Documents/alpha-forge/`

**Manual Migration (if automatic fails):**

1. Locate legacy data directory:
   - macOS: `~/Library/Application Support/alpha-forge/`
   - Windows: `%APPDATA%\alpha-forge\`

2. Copy to new location:
   ```bash
   # macOS
   mkdir -p ~/Documents/alpha-forge
   cp -R ~/Library/Application\ Support/alpha-forge/* ~/Documents/alpha-forge/

   # Windows
   mkdir "%USERPROFILE%\Documents\alpha-forge"
   xcopy "%APPDATA%\alpha-forge" "%USERPROFILE%\Documents\alpha-forge" /E /I
   ```

3. Verify migration successful, then delete legacy directory

### Migration Checklist

Before starting any migration:

- [ ] Close AlphaForge completely on source device
- [ ] Create backup of entire data directory
- [ ] Verify backup integrity (can be opened)
- [ ] Ensure target device has sufficient disk space
- [ ] Install same or newer AlphaForge version on target device
- [ ] Document current workspace names and IDs

After migration:

- [ ] Verify all workspaces appear in workspace list
- [ ] Open each workspace to verify data integrity
- [ ] Check settings (theme, locale, preferences)
- [ ] Test basic operations (create note, run task)
- [ ] Check application logs for errors
- [ ] Delete backup only after verified everything works

---

## References

- [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html)
- [Apple App Sandbox](https://developer.apple.com/documentation/security/app_sandbox)
- [Windows Known Folders](https://docs.microsoft.com/en-us/windows/win32/shell/known-folders)