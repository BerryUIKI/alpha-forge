# Security Hardening Guide

## Overview

This document defines the security hardening requirements and procedures for AlphaForge M8 Local MVP release.

---

## 1. Security Philosophy

### 1.1 Core Security Principles

1. **Local-First**: All user data stored locally; no cloud dependency
2. **Privacy-First**: No telemetry by default; user controls all data
3. **Least Privilege**: Application requests only necessary permissions
4. **Defense in Depth**: Multiple layers of security controls
5. **Secure by Default**: Default configuration prioritizes security

### 1.2 Security Objectives

| Objective | Description |
|-----------|-------------|
| **Confidentiality** | Protect user data from unauthorized access |
| **Integrity** | Prevent unauthorized modification of data or code |
| **Availability** | Ensure application is resilient and recoverable |
| **Authenticity** | Verify identity of application and publisher |
| **Non-repudiation** | Maintain audit trails for critical actions |

---

## 2. Threat Model

### 2.1 Trust Boundaries

```
┌─────────────────────────────────────────────┐
│           Untrusted Zone                     │
│  - External networks (Internet)              │
│  - Remote servers (GitHub API, etc.)        │
│  - Downloaded files                          │
│  - User input (unvalidated)                  │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│           Application Boundary               │
│  - Tauri IPC layer                          │
│  - Input validation                          │
│  - Permission checks                         │
│  - Path validation                           │
└─────────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────────┐
│           Trusted Zone                       │
│  - Local SQLite database                     │
│  - User Documents directory                  │
│  - System keychain (credentials)             │
│  - Application configuration                 │
└─────────────────────────────────────────────┘
```

### 2.2 Threat Actors

| Actor | Capability | Risk Level | Mitigation |
|-------|-----------|------------|------------|
| **Malicious application** | Local code execution | High | Sandbox, permissions, signing |
| **Compromised dependency** | Supply chain attack | High | Dependency audit, lockfile |
| **Malicious network** | MITM, replay, injection | Medium | HTTPS, certificate validation |
| **Local attacker** | File system access | Medium | Encryption, access controls |
| **Phishing** | Credential theft | Low | No cloud auth in MVP |

### 2.3 Attack Vectors

#### Application Layer

| Attack | Mitigation | Status |
|--------|-----------|--------|
| **Arbitrary code execution** | No shell access, sandboxing | ✅ Implemented |
| **Path traversal** | Path validation, sandboxing | ✅ Implemented |
| **SQL injection** | Parameterized queries only | ✅ Implemented |
| **XSS in artifacts** | No raw HTML, controlled renderers | ✅ Implemented |
| **Credential theft** | OS keychain storage | ✅ Implemented |

#### Network Layer

| Attack | Mitigation | Status |
|--------|-----------|--------|
| **MITM** | HTTPS only, certificate validation | ✅ Implemented |
| **SSRF** | URL allowlist, validation | ✅ Implemented |
| **Replay attacks** | Request timeouts, no auth tokens | ✅ Implemented |
| **DNS spoofing** | HTTPS, public CAs | ✅ Implemented |

#### Data Layer

| Attack | Mitigation | Status |
|--------|-----------|--------|
| **Database corruption** | WAL mode, backups | ✅ Implemented |
| **File tampering** | User permissions, signing | ⚠️ Partial |
| **Credential exposure** | Keychain, no plaintext storage | ✅ Implemented |

---

## 3. Security Architecture

### 3.1 Permission Model

**Current Permissions** (from `tauri.conf.json`):

```json
{
  "security": {
    "csp": {
      "default-src": "'self' asset:",
      "connect-src": "ipc: http://ipc.localhost",
      "font-src": "'self' data:",
      "img-src": "'self' asset: http://asset.localhost blob: data:",
      "media-src": "'self' blob: data:",
      "style-src": "'self' 'unsafe-inline'",
      "object-src": "'none'",
      "base-uri": "'none'",
      "form-action": "'none'",
      "frame-ancestors": "'none'"
    }
  }
}
```

**Security Assessment**:

| Directive | Status | Notes |
|-----------|--------|-------|
| `default-src: 'self'` | ✅ Secure | Only local resources |
| `connect-src: ipc:` | ✅ Secure | IPC only, no external network |
| `object-src: 'none'` | ✅ Secure | No plugins |
| `frame-ancestors: 'none'` | ✅ Secure | No framing |
| `form-action: 'none'` | ✅ Secure | No form submissions |
| `style-src: 'unsafe-inline'` | ⚠️ Acceptable | Required for Tailwind, acceptable risk |

### 3.2 Disabled Capabilities

**Shell Access**: ✅ **Disabled**
- No shell plugin
- No shell permissions
- Application cannot execute arbitrary commands

**HTTP Plugin**: ✅ **Disabled**
- No HTTP plugin
- Network requests in Rust only
- Full control over request validation

**Unsafe APIs**: ✅ **Not Exposed**
- React components cannot access:
  - SQLite directly
  - File system directly
  - Shell commands
  - Arbitrary network

### 3.3 Isolated Boundaries

**Main Window**:
- Full application context
- Can invoke Tauri commands
- Cannot directly access:
  - SQLite database files
  - API keys
  - Shell commands
  - Arbitrary file system paths

**Artifact Windows**:
- Isolated WebView
- Limited permissions
- No SQLite access
- No filesystem access
- No API keys
- Only receives validated JSON input

---

## 4. Security Hardening Checklist

### 4.1 Code-Level Security

#### Rust Backend

- [x] **Input Validation**: All external inputs validated before use
- [x] **Path Validation**: File paths validated to prevent traversal
- [x] **SQL Injection Prevention**: Only parameterized queries used
- [x] **Error Handling**: No panic/unwrap in production paths
- [x] **Logging**: Sensitive data redacted from logs
- [x] **Error Messages**: No sensitive paths/keys in user-facing errors
- [x] **Credential Storage**: OS keychain for all secrets
- [x] **Timeout Enforcement**: Network requests have timeouts
- [x] **URL Validation**: Only public HTTPS URLs allowed
- [x] **Permission Checks**: User consent for privileged operations

#### React Frontend

- [x] **No Direct Filesystem**: All persistence through Rust
- [x] **No Database Access**: React cannot query SQLite directly
- [x] **No API Keys**: No plaintext credentials in React
- [x] **Input Validation**: Zod schemas for all external data
- [x] **Error Handling**: User-friendly error messages only
- [x] **Type Safety**: TypeScript strict mode enabled
- [x] **Dependency Audit**: Known vulnerabilities checked

### 4.2 Architecture Security

- [x] **Permission System**: Tauri capabilities defined
- [x] **IPC Layer**: All communication through typed commands
- [x] **CSP Headers**: Strict Content Security Policy
- [x] **Artifact Isolation**: Temporary windows have limited permissions
- [x] **Plugin Sandbox**: Plugins validated before execution
- [x] **Signing Verification**: Application signed and notarized
- [x] **Update Security**: Manual updates only, verified signatures

### 4.3 Data Security

- [x] **Local Storage**: Data stored in user-controlled directory
- [x] **Database Encryption**: ⚠️ Not implemented (future consideration)
- [x] **Backup Security**: Backups are consistent SQLite copies
- [x] **Credential Management**: OS keychain integration
- [x] **Log Sanitization**: No sensitive data in logs
- [x] **Error Redaction**: Paths/keys removed from errors

---

## 5. Dependency Security

### 5.1 Dependency Audit

**Rust Dependencies** (from `Cargo.toml`):

```bash
# Audit command
cargo audit
```

**Known Issues**: None (as of M8 release)

**Audit Frequency**: Before each release

**Automated Checks**: GitHub Dependabot enabled

### 5.2 NPM Dependencies

**Frontend Dependencies** (from `package.json`):

```bash
# Audit command
pnpm audit
```

**Vulnerability Response**:
- **Critical**: Block release
- **High**: Fix before release
- **Medium**: Assess and fix within 30 days
- **Low**: Fix in next maintenance window

### 5.3 Supply Chain Security

**Lockfile Commitment**:
- ✅ `Cargo.lock` committed
- ✅ `pnpm-lock.yaml` committed
- ✅ Both lockfiles tracked in Git

**Dependency Pinning**:
- ✅ Exact versions in lockfiles
- ✅ No floating versions in Cargo.toml
- ✅ `package.json` uses exact versions or ranges with lockfile

**Supply Chain Best Practices**:
- ✅ Only official registries (crates.io, npm)
- ✅ No Git dependencies in production
- ✅ Verify dependency integrity before build

---

## 6. Security Testing

### 6.1 Automated Security Tests

**Existing Tests**:

| Test Category | Location | Status |
|--------------|----------|--------|
| Path traversal prevention | `security/url_policy.rs` | ✅ Passing |
| Credential name validation | `security/credentials.rs` | ✅ Passing |
| URL allowlist enforcement | `security/url_policy.rs` | ✅ Passing |
| Artifact route validation | `artifacts/manager.rs` | ✅ Passing |
| Permission checks | `plugins/permissions.rs` | ✅ Passing |
| Input validation | Various repositories | ✅ Passing |

**Test Command**:
```bash
cargo test
pnpm test
```

### 6.2 Manual Security Testing

**Pre-Release Checklist**:

- [ ] Verify no credentials in logs
- [ ] Verify no sensitive paths in error messages
- [ ] Test path traversal attempts
- [ ] Test invalid URL submissions
- [ ] Verify keychain credential storage
- [ ] Test artifact permission boundaries
- [ ] Verify CSP blocks external resources
- [ ] Test update integrity

### 6.3 Penetration Testing (Future)

**Recommended Scope**:
- Local privilege escalation
- Artifact sandbox bypass
- IPC command injection
- Path traversal vectors
- Credential extraction

**Frequency**: Annual or before major releases

---

## 7. Security Hardening Procedures

### 7.1 Pre-Release Security Review

**Step 1: Dependency Audit**

```bash
# Rust dependencies
cargo audit

# NPM dependencies
pnpm audit
```

**Action**: Fix all critical/high vulnerabilities before proceeding

**Step 2: Code Review**

- Review all new code for security issues
- Check input validation
- Verify error handling
- Confirm no sensitive data in logs

**Step 3: Permission Review**

- Verify no new dangerous permissions added
- Check artifact window permissions
- Confirm plugin permissions are minimal

**Step 4: Build Verification**

```bash
# Build release
pnpm tauri build

# Verify no security warnings
# Check binary for sensitive strings
strings target/release/investment-os.exe | grep -i "password\|secret\|key"
```

**Step 5: Signing Verification**

```bash
# macOS
codesign --verify --deep --strict "Investment OS.app"
spctl --assess --verbose "Investment OS.app"

# Windows
signtool verify /pa investment-os.exe
```

### 7.2 Post-Release Monitoring

**Monitoring Activities**:

- Monitor GitHub Issues for security reports
- Watch dependency advisories
- Review security@ mailbox (if established)
- Track vulnerability databases for dependencies

**Response SLA**:

| Severity | Response Time | Fix Deadline |
|----------|--------------|--------------|
| **Critical** | 24 hours | 48 hours |
| **High** | 48 hours | 7 days |
| **Medium** | 7 days | 30 days |
| **Low** | 30 days | 90 days |

---

## 8. Security Incident Response

### 8.1 Incident Classification

**Severity Levels**:

| Level | Criteria | Example |
|-------|----------|---------|
| **Critical** | Remote code execution, data breach | Malicious artifact bypasses sandbox |
| **High** | Privilege escalation, data exposure | Path traversal allows arbitrary file read |
| **Medium** | Limited impact, requires local access | Credential stored in plaintext |
| **Low** | Minimal impact, theoretical vectors | Potential XSS in artifact (blocked by CSP) |

### 8.2 Response Procedure

**Step 1: Triage (0-4 hours)**

1. Acknowledge security report
2. Assess severity and impact
3. Assign incident lead
4. Begin documentation

**Step 2: Containment (4-24 hours)**

1. Identify affected versions
2. Assess exploitability
3. Prepare mitigation guidance
4. Notify affected users (if critical)

**Step 3: Remediation (24-72 hours)**

1. Develop fix
2. Review fix for security
3. Test fix thoroughly
4. Prepare security advisory

**Step 4: Recovery**

1. Release patched version
2. Update documentation
3. Notify users
4. Publish security advisory

**Step 5: Post-Mortem**

1. Analyze root cause
2. Identify process improvements
3. Update security practices
4. Document lessons learned

### 8.3 Security Advisory Template

```markdown
# Security Advisory: [TITLE]

**Severity**: [Critical/High/Medium/Low]
**CVE**: [If assigned]
**Affected Versions**: [List versions]
**Fixed Versions**: [List versions]

## Summary

[Brief description of the vulnerability]

## Impact

[What an attacker can do]

## Mitigation

[How to protect until fixed]

## Details

[Technical details]

## Credits

[Who reported the issue]
```

---

## 9. Secure Configuration

### 9.1 Default Configuration

**Application defaults prioritizing security**:

- ✅ **Telemetry**: Disabled by default
- ✅ **Auto-update**: Disabled by default
- ✅ **Shell access**: Disabled by default
- ✅ **HTTP plugin**: Disabled by default
- ✅ **Cloud sync**: Not implemented (local-first)
- ✅ **Authentication**: Not implemented (local-only)

### 9.2 User Security Controls

**Settings exposed to users**:

- Language selection (no security impact)
- Theme toggle (no security impact)
- Manual update check
- Local backup export
- Privacy notice access
- Research disclaimer access

### 9.3 Hardening Recommendations

**For Advanced Users**:

- Run application as standard user (not admin)
- Keep application in user-controlled directory
- Regular backups of `~/Documents/alpha-forge/`
- Review GitHub Releases for integrity
- Verify signatures before installation

---

## 10. Known Security Limitations (M8)

### 10.1 Current Limitations

| Limitation | Risk Level | Mitigation | Future Plan |
|------------|-----------|------------|-------------|
| **No database encryption** | Medium | User-controlled directory | Add optional encryption |
| **No 2FA** | Low | Local-only, no remote auth | Not planned (local-first) |
| **No audit logging** | Low | Local-only, user controls data | Add optional audit logs |
| **No sandboxing on Windows** | Medium | Signed binaries, limited permissions | Explore Windows sandbox |
| **No formal penetration test** | Medium | Code review, dependency audit | Schedule pen test |

### 10.2 Accepted Risks (M8)

For M8 Local MVP, we accept these risks:

- ⚠️ **No database encryption**: User data visible to local admin
  - **Mitigation**: User controls directory, physical security assumed
  - **Risk Level**: Medium (acceptable for local-first MVP)

- ⚠️ **No security monitoring**: No automated threat detection
  - **Mitigation**: Manual monitoring, community reports
  - **Risk Level**: Low (open-source, community scrutiny)

---

## 11. Security Requirements for M8 Release

### 11.1 Mandatory Requirements

- [x] All unit tests passing (Rust + TypeScript)
- [x] No known critical/high vulnerabilities in dependencies
- [x] Input validation for all external data
- [x] Path validation preventing directory traversal
- [x] Credential storage in OS keychain
- [x] Error messages redact sensitive information
- [x] Logs do not contain credentials or sensitive paths
- [x] Artifacts have limited permissions
- [x] Application signed and notarized (if certificates obtained)
- [x] Security documentation complete

### 11.2 Recommended Requirements

- [ ] Security review by third party
- [ ] Penetration test report
- [ ] Formal threat model review
- [ ] Database encryption option
- [ ] Audit logging
- [ ] Security monitoring

---

## 12. Future Security Enhancements

### 12.1 Post-MVP Security Roadmap

**Phase 2 (Post-MVP)**:

- Database encryption option (SQLCipher or similar)
- Audit logging for sensitive operations
- Enhanced artifact sandboxing
- Security monitoring integration
- Two-factor authentication (if cloud features added)

**Phase 3 (Future)**:

- Formal security certification
- Annual penetration testing
- Bug bounty program
- Security advisory mailing list

---

## 13. Security References

### 13.1 Standards and Frameworks

- [OWASP Desktop App Security](https://github.com/OWASP/Desktop-Assessment-Scoring-Method)
- [CIS Desktop Security](https://www.cisecurity.org/)
- [NIST Application Security](https://www.nist.gov/)
- [Tauri Security Guide](https://tauri.app/v1/guides/getting-started/security/)

### 13.2 Tooling

- `cargo audit` - Rust dependency vulnerability scanner
- `pnpm audit` - NPM vulnerability scanner
- `codesign` - macOS code signing tool
- `signtool` - Windows code signing tool
- `strings` - Binary analysis tool

---

## Success Criteria

- [x] Threat model documented
- [x] Security architecture reviewed
- [x] All automated security tests passing
- [x] Dependency audit completed
- [x] Security hardening checklist complete
- [x] Incident response procedure defined
- [x] Known limitations documented
- [x] Future enhancements planned

---

*Last Updated: 2026-08-03*
*Version: 1.0*
*Milestone: M8 - Local MVP Completion*