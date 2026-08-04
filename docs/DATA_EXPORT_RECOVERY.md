# User Data Export and Recovery Guide

**Status**: Approved for MVP
**Owner**: @BerryUIKI
**Last updated**: 2026-08-03

This guide explains how users can export and recover their AlphaForge data. AlphaForge is a local-first application — your data stays on your device, and you control all backups.

---

## Overview

AlphaForge stores all data in a local SQLite database. The application does not use cloud storage, automatic synchronization, or remote backup services. Users are responsible for their own data custody.

### What Data Is Stored

| Data Type | Description | Location |
|-----------|-------------|----------|
| Workspaces | Research containers with settings | SQLite database |
| Research Projects | Documents, sources, notes, reports | SQLite database |
| Investment Theses | Claims, evidence, confidence history | SQLite database |
| Portfolio Accounts | Holdings, transactions, allocations | SQLite database |
| Knowledge Graph | Entities and relationships | SQLite database |
| Agent Tasks | Task history and results | SQLite database |
| Artifacts | Research visualizations | SQLite database |
| Settings | User preferences, locale | SQLite database |

### What Data Is NOT Stored

- API keys or credentials (if any are added in future, they will be stored in OS credential manager)
- Cloud account information
- Telemetry or analytics data

---

## Export Methods

### Method 1: SQLite Backup Export (Recommended)

The Settings page provides a built-in backup function that exports a consistent copy of your SQLite database.

**How to use:**

1. Open Settings (`Ctrl+,` / `Cmd+,`)
2. Navigate to "Local Backup" section
3. Click "Export local backup"
4. Choose a destination file (`.db` extension recommended)
5. Wait for confirmation

**Features:**

- Creates a point-in-time snapshot
- Existing files are never overwritten
- Safe to copy to external storage
- Can be restored on the same or different machine

**Limitations:**

- Must be performed manually
- No automatic scheduling
- Requires disk space for backup file

### Method 2: Direct Database Copy

Advanced users can directly copy the SQLite database file.

**Database location:**

- **Windows**: `%APPDATA%\alpha-forge\data.db`
- **macOS**: `~/Library/Application Support/alpha-forge/data.db`

**Procedure:**

1. Close AlphaForge completely
2. Copy the `data.db` file to your backup location
3. Restart AlphaForge

**Warning:** Copying while the application is running may result in a corrupted backup.

---

## Recovery Methods

### Method 1: Restore from SQLite Backup

**How to restore:**

1. Close AlphaForge completely
2. Locate your backup `.db` file
3. Copy it to the database location (see above)
4. Rename to `data.db` if needed
5. Restart AlphaForge

**Verification:**

- Check that your workspaces appear in the workspace list
- Verify recent research projects are accessible
- Confirm thesis evidence and confidence history are intact

### Method 2: Import Data (Future Feature)

Data import from other formats (CSV, JSON) is not available in the MVP. Transaction history can be imported within the Portfolio module, but full database import is not supported.

---

## Backup Best Practices

### Frequency

| Usage Pattern | Recommended Frequency |
|---------------|----------------------|
| Heavy research (daily use) | Daily or weekly |
| Moderate use | Weekly or bi-weekly |
| Light use | Monthly |

### Storage Locations

Recommended backup destinations:

1. **External drive** — Physically separate from your main drive
2. **Cloud storage** — Google Drive, Dropbox, iCloud (manual upload)
3. **Network attached storage (NAS)** — For home/office networks

### Retention

Keep multiple backup versions:

- **Daily**: Last 7 days
- **Weekly**: Last 4 weeks
- **Monthly**: Last 12 months

---

## Recovery Scenarios

### Scenario 1: Application Reinstallation

If you reinstall AlphaForge:

1. Export backup before uninstalling
2. After reinstall, restore the backup file

### Scenario 2: Operating System Reinstall

Before OS reinstall:

1. Export backup using Settings
2. Store on external media or cloud storage
3. After OS reinstall, install AlphaForge
4. Restore backup to the database location

### Scenario 3: New Computer

To migrate to a new computer:

1. Export backup on old computer
2. Transfer file to new computer
3. Install AlphaForge on new computer
4. Restore backup to the database location

### Scenario 4: Data Corruption

If your database becomes corrupted:

1. Close AlphaForge immediately
2. Check if `data.db` file exists
3. If file exists but won't open, restore from most recent backup
4. If no backup exists, data may be unrecoverable

---

## Limitations and Known Issues

### Current Limitations

| Limitation | Description | Mitigation |
|------------|-------------|------------|
| No automatic backup | User must manually trigger backup | Set calendar reminder |
| No incremental backup | Each backup is a full copy | Accept disk space requirement |
| No backup verification | Application doesn't verify backup integrity | Test restore on secondary machine |
| No cross-version guarantee | Backups may not work with future versions | Keep backup with version notes |

### Known Issues

1. **Large databases**: Databases over 100 MB may take several seconds to export
2. **Network drives**: Saving directly to network locations may fail; save locally first, then copy
3. **Cloud sync conflict**: Do not store live database in cloud-synced folders (Dropbox, iCloud, etc.)

---

## Troubleshooting

### Backup Failed

**Symptoms:**
- Error message during export
- No file created at destination

**Solutions:**
1. Ensure destination is writable
2. Check available disk space
3. Try a different location
4. Close other applications that might lock the file

### Restore Failed

**Symptoms:**
- Application won't start after restore
- Data appears missing or corrupted

**Solutions:**
1. Verify backup file was copied to correct location
2. Check file permissions (readable by user)
3. Try an earlier backup version
4. Check application logs for error details

### Database Locked

**Symptoms:**
- "Database locked" error
- Cannot save changes

**Solutions:**
1. Close all AlphaForge windows
2. Check for other processes using the database
3. Restart application
4. If persists, restart computer

---

## Security Considerations

### Data Sensitivity

Your AlphaForge database may contain:

- Investment research and thesis
- Portfolio holdings and transactions
- Personal notes and analysis
- Knowledge graph with explicit relationships

Treat backups as sensitive financial documents.

### Backup Security

Recommendations:

1. **Encrypt backup storage** — Use encrypted external drives or cloud storage
2. **Limit access** — Store backups in secure locations
3. **Secure deletion** — When disposing of old backups, use secure deletion tools

### No Cloud Storage by Design

AlphaForge deliberately does not provide cloud backup because:

- Your research data is private
- No third-party can access your data
- You maintain full custody
- No account required

---

## Frequently Asked Questions

### Q: Can I schedule automatic backups?

**A:** Not in the MVP. You must manually trigger backups through Settings.

### Q: Where is my data stored?

**A:** All data is stored locally in `data.db` (SQLite). See the Database Location section above for the exact path on your OS.

### Q: Can I sync between multiple computers?

**A:** Not automatically. You can export a backup on one computer and restore it on another, but this is a manual process.

### Q: What happens if I lose my backup?

**A:** Your data cannot be recovered. AlphaForge has no cloud storage or remote backup. This is by design to keep your data private and local.

### Q: Can I export my data to another format?

**A:** Transaction history can be imported from CSV within the Portfolio module. Full database export to other formats is not supported in the MVP.

### Q: Will my backup work with future versions?

**A:** Backups are SQLite databases. While SQLite is very stable, future versions of AlphaForge may have schema changes. Always note the version when creating backups.

---

## Support

For issues with data export or recovery:

1. Check this documentation for troubleshooting steps
2. Review application logs for error details
3. Open a GitHub Issue with:
   - AlphaForge version
   - Operating system
   - Error message
   - Steps to reproduce

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-08-03 | Initial MVP documentation |

---

## See Also

- [Privacy Policy](PRIVACY.md)
- [M8 Decision Record](M8_DECISION_RECORD.md)
- [Settings Documentation](SETTINGS.md) — For backup controls
