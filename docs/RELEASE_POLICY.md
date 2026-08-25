# Release Policy & Signing Strategy

## Overview

This document defines the release process, code signing, notarization, update, rollback, and release-ownership policies for AlphaForge M8 Local MVP release.

---

## 1. Release Philosophy

### 1.1 Release Principles

1. **Local-First**: No forced auto-updates; users control when to update
2. **Transparent**: All releases signed and verified; release notes comprehensive
3. **Reversible**: Users can rollback to previous versions
4. **Secure**: Code signing prevents tampering; notarization ensures authenticity
5. **Minimal Friction**: Update check is manual; installation is straightforward

### 1.2 Release Cadence

| Release Type | Frequency | Example |
|--------------|-----------|---------|
| **Major** | Breaking changes, significant features | 1.0.0, 2.0.0 |
| **Minor** | New features, backward-compatible | 0.1.0, 0.2.0 |
| **Patch** | Bug fixes, security updates | 0.1.1, 0.1.2 |
| **Pre-release** | Alpha, Beta, RC | 0.1.0-alpha.1, 0.1.0-beta.2 |

### 1.3 Version Numbering

- **Semantic Versioning**: MAJOR.MINOR.PATCH
- **Pre-release suffixes**: `-alpha.N`, `-beta.N`, `-rc.N`
- **Build metadata**: Not used for releases

---

## 2. Code Signing Strategy

### 2.1 Why Code Signing?

Code signing provides:
- ✅ **Authenticity**: Users can verify the publisher
- ✅ **Integrity**: Detects if application was tampered with
- ✅ **Trust**: Reduces security warnings on macOS/Windows
- ✅ **Platform compliance**: Required for macOS notarization

### 2.2 Signing Certificates

#### macOS

**Certificate Type**: Developer ID Application Certificate

**Obtained From**: Apple Developer Program ($99/year)

**Requirements**:
- Apple Developer account
- Verified identity (legal entity or individual)
- Agreement to Apple's terms

**Validity**: 5 years (renewable)

#### Windows

**Certificate Type**: Code Signing Certificate (Standard or EV)

**Obtained From**: Trusted Certificate Authority (CA)
- DigiCert
- Sectigo
- GlobalSign
- SSL.com

**Certificate Types**:

| Type | Cost | Trust Level | Requirements |
|------|------|--------------|--------------|
| **Standard OV** | $100-300/year | Standard | Organization verification |
| **EV (Extended Validation)** | $300-600/year | Highest | Strict identity verification, hardware token |

**Recommendation for MVP**: Standard OV certificate (sufficient for open-source project)

**Validity**: 1-3 years (depending on CA)

### 2.3 Signing Process

#### macOS Signing

**Prerequisites**:
- Developer ID Application Certificate installed in Keychain
- Xcode command-line tools installed

**Signing Command**:
```bash
# Sign the application bundle
codesign --sign "Developer ID Application: Your Name (TEAM_ID)" \
  --options runtime \
  --deep \
  --force \
  --verify \
  --verbose \
  "target/release/bundle/macos/AlphaForge.app"

# Verify signature
codesign --verify --deep --strict --verbose "AlphaForge.app"
spctl --assess --verbose "AlphaForge.app"
```

**Flags Explained**:
- `--options runtime`: Enables Hardened Runtime (required for notarization)
- `--deep`: Signs all nested binaries
- `--force`: Re-signs if already signed
- `--verify`: Verifies signature after signing

#### Windows Signing

**Prerequisites**:
- Code signing certificate (PFX file)
- Private key password
- SignTool (Windows SDK) installed

**Signing Command**:
```bash
# Sign the executable
signtool sign /f "path/to/certificate.pfx" \
  /p "password" \
  /tr http://timestamp.digicert.com \
  /td SHA256 \
  /fd SHA256 \
  "target/release/alpha-forge.exe"

# Verify signature
signtool verify /pa "target/release/alpha-forge.exe"
```

**Flags Explained**:
- `/f`: Certificate file path
- `/p`: Certificate password
- `/tr`: Timestamp server URL (proves signing time)
- `/td`: Digest algorithm for timestamp
- `/fd`: File digest algorithm

**Recommended Timestamp Servers**:
- DigiCert: `http://timestamp.digicert.com`
- Sectigo: `http://timestamp.sectigo.com`
- GlobalSign: `http://timestamp.globalsign.com`

---

## 3. Notarization (macOS)

### 3.1 Why Notarization?

macOS Gatekeeper requires notarization to:
- ✅ Run without security warnings
- ✅ Pass Apple's malware scan
- ✅ Provide additional trust assurance
- ✅ Comply with macOS security requirements

### 3.2 Notarization Requirements

1. **Developer ID Application Certificate** (signed with Hardened Runtime)
2. **App-specific password** for Apple ID
3. **Xcode 13+** or later
4. **Application signed** with runtime option

### 3.3 Notarization Process

**Step 1: Create App-Specific Password**

1. Go to appleid.apple.com
2. Sign in with Apple ID
3. Generate app-specific password for notarization
4. Store securely (one-time use)

**Step 2: Store Credentials**

```bash
# Store Apple ID credentials in Keychain
xcrun notarytool store-credentials \
  --apple-id "your@email.com" \
  --team-id "TEAM_ID" \
  --password "xxxx-xxxx-xxxx-xxxx" \
  "alpha-forge-notary"
```

**Step 3: Submit for Notarization**

```bash
# Create ZIP archive
ditto -c -k --keepParent "AlphaForge.app" "AlphaForge.zip"

# Submit to Apple
xcrun notarytool submit "AlphaForge.zip" \
  --keychain-profile "alpha-forge-notary" \
  --wait

# Expected output:
# Successfully received submission ID: <submission-id>
# Waiting for processing to complete
# Status: in progress...
# Status: accepted
```

**Step 4: Staple Ticket**

```bash
# Staple notarization ticket to app bundle
xcrun stapler staple "AlphaForge.app"

# Verify stapling
spctl --assess --verbose --type execute "AlphaForge.app"
```

### 3.4 Notarization Timeline

| Stage | Duration | Description |
|-------|----------|-------------|
| **Upload** | 1-2 minutes | ZIP upload to Apple |
| **Processing** | 1-15 minutes | Apple scans for malware |
| **Review** | 1-30 minutes | Manual review (if flagged) |
| **Approval** | Immediate | Ticket generated |

**Typical Total Time**: 5-20 minutes

### 3.5 Notarization Failure Handling

If notarization fails:

1. **Check logs**:
   ```bash
   xcrun notarytool log <submission-id> \
     --keychain-profile "alpha-forge-notary"
   ```

2. **Common failure reasons**:
   - Hardened Runtime not enabled
   - Unsigned nested binaries
   - Usage of deprecated APIs
   - Entitlements issues

3. **Fix and resubmit**:
   - Fix identified issues
   - Re-sign application
   - Resubmit to Apple

---

## 4. Update Strategy

### 4.1 Update Philosophy

- **User-Controlled**: No automatic updates; user initiates check
- **GitHub Releases**: Distribution via GitHub Releases page
- **Manual Installation**: User downloads and installs manually
- **Version Check**: Application checks current vs. latest version

### 4.2 Update Check Implementation

**Backend API**: `checkForUpdate` command (already implemented)

```rust
#[tauri::command]
pub async fn check_for_update() -> Result<UpdateCheckResult, AppError> {
    let current_version = env!("CARGO_PKG_VERSION");
    let client = reqwest::Client::new();
    
    let response = client
        .get("https://api.github.com/repos/BerryUIKI/alpha-forge/releases/latest")
        .header("User-Agent", "AlphaForge")
        .send()
        .await?;
    
    let release: GitHubRelease = response.json().await?;
    
    Ok(UpdateCheckResult {
        update_available: release.tag_name != format!("v{}", current_version),
        current_version: current_version.to_string(),
        latest_version: release.tag_name,
        release_url: release.html_url,
    })
}
```

**Frontend Integration**:

```typescript
const checkForUpdate = async () => {
  const release = await desktopApi.system.checkForUpdate();
  if (release.updateAvailable) {
    // Show notification
    showToast({
      title: formatMessage(t("updateAvailable"), { version: release.latestVersion }),
      action: {
        label: t("download"),
        onClick: () => openUrl(release.releaseUrl)
      }
    });
  } else {
    showToast({
      title: formatMessage(t("upToDate"), { version: release.currentVersion })
    });
  }
};
```

### 4.3 Update Flow

```text
User clicks "Check for Updates"
    ↓
Application queries GitHub API
    ↓
Compare versions
    ↓
If update available:
    - Show update notification
    - Provide download link
    - User downloads manually
    ↓
User installs new version
    ↓
User reopens application
```

### 4.4 Update Frequency

- **Check manually**: Users decide when to check
- **No background checks**: No automatic update checks
- **No notifications**: Application doesn't notify about updates

---

## 5. Rollback Strategy

### 5.1 Rollback Philosophy

Users should be able to revert to previous versions if:
- New version has critical bugs
- Performance regression
- Compatibility issues
- User preference

### 5.2 Rollback Support

#### macOS

**Method**: Keep previous `.app` bundles

**Process**:
1. Rename current version: `AlphaForge.app` → `AlphaForge-0.2.0.app`
2. Download previous version from GitHub Releases
3. Install previous version
4. User opens previous version

**Recommendation**: Users should backup old versions before updating

#### Windows

**Method**: Keep previous installers

**Process**:
1. Download previous `.exe` installer from GitHub Releases
2. Uninstall current version (optional)
3. Run previous version installer
4. User opens previous version

**Note**: NSIS installer prompts to uninstall old version first

### 5.3 Data Compatibility

**Database Migration**: 
- SQLite database is workspace-specific
- Application version stored in database metadata
- Backward compatible for minor/patch versions
- Major versions may require migration

**Recommendation**:
- Users backup data before major version updates
- Application warns before irreversible migrations

### 5.4 Rollback Documentation

User-facing rollback guide should include:
1. How to download previous version from GitHub
2. How to preserve data during rollback
3. How to verify rollback successful
4. When to consider rollback (bug reports, performance issues)

---

## 6. Release Ownership

### 6.1 Release Authority

**Release Owner**: Project maintainer (currently: @BerryUIKI)

**Responsibilities**:
- Approve release timing
- Verify build integrity
- Sign and notarize binaries
- Publish GitHub Release
- Update documentation
- Announce release

### 6.2 Release Approval Criteria

Before a release is approved:

**Code Quality**:
- [ ] All tests passing (Rust + TypeScript)
- [ ] No compiler warnings
- [ ] No Clippy warnings
- [ ] Type check passing
- [ ] Lint passing

**Functionality**:
- [ ] Critical paths manually tested
- [ ] New features documented
- [ ] Breaking changes documented
- [ ] Migration guide written (if needed)

**Security**:
- [ ] No known vulnerabilities
- [ ] Security review completed (for major releases)
- [ ] Dependencies audited

**Platform Testing**:
- [ ] macOS build tested
- [ ] Windows build tested
- [ ] Installer tested on clean system
- [ ] Update flow tested

**Documentation**:
- [ ] Release notes written
- [ ] CHANGELOG.md updated
- [ ] README.md updated (if needed)
- [ ] Migration guide (if needed)

### 6.3 Release Process Checklist

**Pre-Release**:
- [ ] Bump version number
- [ ] Update CHANGELOG.md
- [ ] Run full test suite
- [ ] Build release binaries
- [ ] Sign binaries (macOS + Windows)
- [ ] Notarize macOS binary
- [ ] Test signed binaries

**Release**:
- [ ] Create Git tag (e.g., `v0.1.0`)
- [ ] Push tag to GitHub
- [ ] Create GitHub Release
- [ ] Upload signed binaries
- [ ] Upload checksums
- [ ] Publish release notes

**Post-Release**:
- [ ] Announce release (GitHub Discussions, etc.)
- [ ] Monitor for issues
- [ ] Prepare hotfix plan (if needed)

---

## 7. CI/CD Pipeline (Future)

### 7.1 Recommended Pipeline

**Platform**: GitHub Actions

**Triggers**:
- Push to `main` or `release/*` branches
- Git tags (`v*`)

**Stages**:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        include:
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: macos-latest
            target: x86_64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    
    runs-on: ${{ matrix.os }}
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 22
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      
      - name: Install dependencies
        run: pnpm install
      
      - name: Build
        run: pnpm tauri build --target ${{ matrix.target }}
      
      - name: Sign (macOS)
        if: matrix.os == 'macos-latest'
        run: |
          codesign --sign "$DEVELOPER_ID" \
            --options runtime \
            --deep --force --verify \
            "target/release/bundle/macos/AlphaForge.app"
        env:
          DEVELOPER_ID: ${{ secrets.DEVELOPER_ID }}
      
      - name: Notarize (macOS)
        if: matrix.os == 'macos-latest'
        run: |
          xcrun notarytool submit app.zip \
            --apple-id "$APPLE_ID" \
            --password "$APPLE_PASSWORD" \
            --team-id "$TEAM_ID" \
            --wait
        env:
          APPLE_ID: ${{ secrets.APPLE_ID }}
          APPLE_PASSWORD: ${{ secrets.APPLE_PASSWORD }}
          TEAM_ID: ${{ secrets.TEAM_ID }}
      
      - name: Sign (Windows)
        if: matrix.os == 'windows-latest'
        run: |
          signtool sign /f certificate.pfx /p "$CERT_PASSWORD" \
            /tr http://timestamp.digicert.com \
            /td SHA256 /fd SHA256 \
            alpha-forge.exe
        env:
          CERT_PASSWORD: ${{ secrets.WINDOWS_CERT_PASSWORD }}
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.target }}
          path: target/release/bundle/*
```

### 7.2 Secrets Management

**Required Secrets**:

| Secret | Description | Used For |
|--------|-------------|-----------|
| `DEVELOPER_ID` | macOS signing identity | macOS code signing |
| `APPLE_ID` | Apple ID email | Notarization |
| `APPLE_PASSWORD` | App-specific password | Notarization |
| `TEAM_ID` | Apple Developer Team ID | Notarization |
| `WINDOWS_CERT_BASE64` | Windows certificate (base64) | Windows signing |
| `WINDOWS_CERT_PASSWORD` | Certificate password | Windows signing |

**Security**:
- Store secrets in GitHub Actions secrets
- Use repository secrets (not environment secrets)
- Rotate secrets annually
- Never commit secrets to repository

---

## 8. Security Considerations

### 8.1 Signing Key Security

**Certificate Storage**:
- Store in secure location (Keychain, hardware token)
- Never commit to repository
- Use CI/CD secrets for automated signing

**Certificate Rotation**:
- Renew before expiration
- Plan rotation 30 days in advance
- Test with new certificate before expiration

### 8.2 Build Integrity

**Verify reproducibility**:
- Use deterministic builds
- Pin dependency versions
- Document build environment

**Build provenance**:
- Track build machine
- Store build logs
- Verify artifacts match source

### 8.3 Distribution Security

**HTTPS only**: All downloads via HTTPS
**Checksums**: Provide SHA256 checksums for verification
**Signatures**: All binaries signed
**No third-party mirrors**: Distribute only via GitHub Releases

---

## 9. M8 MVP Constraints

### 9.1 What We Will NOT Do (M8)

For M8 Local MVP, we explicitly defer:

- ❌ Automatic updates
- ❌ Background update checks
- ❌ Update notifications
- ❌ Delta updates
- ❌ Auto-rollback
- ❌ Central update server
- ❌ Commercial code signing certificate (if cost prohibitive)

### 9.2 What We WILL Do (M8)

- ✅ Manual update checks via Settings
- ✅ Downloads from GitHub Releases
- ✅ Manual installation
- ✅ Self-signed or community certificate (if available)
- ✅ Clear rollback instructions in documentation
- ✅ Version comparison in UI

---

## 10. Cost Estimates

### 10.1 Signing Costs

| Platform | Certificate Type | Cost | Frequency |
|----------|-----------------|------|-----------|
| **macOS** | Developer ID Application | $99/year | Annual |
| **Windows** | Standard OV | $100-300/year | Annual |
| **Windows** | EV (optional) | $300-600/year | Annual |

**Total Annual Cost**: $199-999/year (depending on certificate choices)

### 10.2 Infrastructure Costs

| Service | Cost | Notes |
|---------|------|-------|
| GitHub Actions | Free (public repo) | For CI/CD |
| GitHub Releases | Free | Distribution |
| Timestamp server | Free | Included with certificate |
| Notarization | Free | Apple provides free |

**Total Infrastructure**: $0 (for open-source project)

---

## 11. Timeline

### 11.1 Implementation Phases

**Phase 1: Manual Process (M8 MVP)**
- Manual build signing
- Manual notarization
- Manual release creation
- Manual distribution via GitHub Releases

**Phase 2: Automated Pipeline (Post-MVP)**
- GitHub Actions CI/CD
- Automated signing
- Automated notarization
- Automated release creation

**Phase 3: Enhanced Features (Future)**
- Delta updates
- Auto-update (optional)
- Central update server (optional)

---

## 12. Success Criteria

- [ ] Code signing implemented for macOS and Windows
- [ ] Notarization working for macOS
- [ ] Manual release process documented
- [ ] Update check functionality working
- [ ] Rollback guide documented
- [ ] Release ownership defined
- [ ] CI/CD pipeline designed (even if not implemented)
- [ ] Cost estimates approved

---

## 13. References

- [Apple Code Signing Guide](https://developer.apple.com/support/code-signing/)
- [Apple Notarization Guide](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)
- [Windows Code Signing](https://docs.microsoft.com/en-us/windows/win32/seccrypto/cryptography-tools)
- [GitHub Releases API](https://docs.github.com/en/rest/reference/releases)
- [Tauri Signing Guide](https://tauri.app/v1/guides/distribute/sign/)
- [Semantic Versioning](https://semver.org/)

---

*Last Updated: 2026-08-03*
*Version: 1.0*
*Milestone: M8 - Local MVP Completion*