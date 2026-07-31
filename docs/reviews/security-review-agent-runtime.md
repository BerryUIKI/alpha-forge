# Security Review — Agent Runtime

**Purpose**: Security architecture review for Phase 2 Agent Runtime implementation.

**Review Date**: 2026-07-31

---

## 1. Executive Summary

Agent Runtime introduces security considerations around autonomous execution, external API calls, and tool usage. This review identifies current security requirements and future risks.

**Key Findings**:
- ✅ Credential storage using OS keychain (not plaintext)
- ✅ Artifact window isolation defined
- ⚠️ Tool permission model not yet implemented
- ⚠️ Network request allowlist not defined
- ⚠️ Input validation strategy needs explicit definition

**Priority**: High. Security must be designed in, not retrofitted.

---

## 2. Agent Security

### 2.1 Tool Execution

**Current Status**: Tool system conceptual only

**Future Risks**:
| Tool Action | Risk | Likelihood | Impact |
|-------------|------|------------|--------|
| File read | Data leakage | High | Medium |
| File write | Data corruption | Medium | High |
| Network request | Data exfiltration | High | High |
| Command execution | System compromise | Low | Critical |

**Mitigation Requirements**:
1. **Permission Model**: Every tool must declare required permissions
2. **Allowlist Enforcement**: Network requests limited to allowlisted domains
3. **User Approval**: Destructive tools require explicit approval
4. **Audit Logging**: All tool invocations logged with input/output

**Recommended Implementation**:

```rust
pub enum ToolPermission {
    NetworkAccess { allowlist: Vec<String> },
    FileRead { allowed_paths: Vec<PathPattern> },
    FileWrite { allowed_paths: Vec<PathPattern> },
    ExecuteCommand { allowed_commands: Vec<String> },
}

pub struct PermissionGrant {
    pub tool_id: String,
    pub permissions: Vec<ToolPermission>,
    pub granted_by: UserId,
    pub granted_at: DateTime<Utc>,
}

pub trait PermissionChecker {
    fn is_allowed(&self, tool: &Tool, action: &ToolAction) -> bool;
}
```

### 2.2 File Access

**Current Status**: Not implemented

**Future Risks**:
- Unauthorized file read
- Arbitrary file write
- Directory traversal
- Path injection

**Mitigation Requirements**:
1. **Sandboxed Paths**: Agents can only access workspace-specific directories
2. **Path Validation**: Normalize and validate all paths before access
3. **No Arbitrary Paths**: Reject paths outside allowed directories
4. **Read-Only by Default**: Write access requires explicit permission

**Recommended Paths**:

```
Allowed for Agent File Access:
- ~/.alphaforge/workspaces/{workspace_id}/documents/
- ~/.alphaforge/workspaces/{workspace_id}/exports/
- ~/.alphaforge/tmp/ (temporary files, read/write)

Denied:
- ~/.alphaforge/config/ (configuration files)
- ~/.alphaforge/security/ (credentials, keys)
- Any path outside ~/.alphaforge/
```

### 2.3 Network Access

**Current Status**: Not implemented

**Future Risks**:
- Data exfiltration to malicious endpoints
- Credential leakage in headers/body
- SSRF (Server-Side Request Forgery)
- DNS rebinding attacks

**Mitigation Requirements**:
1. **Domain Allowlist**: Only approved domains accessible
2. **Protocol Restrictions**: HTTP/HTTPS only, no file://
3. **Header Inspection**: No credentials in outgoing headers
4. **Response Validation**: Validate response schemas

**Recommended Allowlist**:

```
Allowed Domains (Phase 2+):
- api.openai.com
- api.anthropic.com
- query1.finance.yahoo.com
- query2.finance.yahoo.com

Future Extensions:
- User-configurable allowlist
- Workspace-specific allowlists
- Admin approval for new domains
```

### 2.4 Credential Usage

**Current Status**: OS keychain planned, not implemented

**Security Requirements**:
1. **No Plaintext Storage**: Credentials NEVER stored in SQLite or config files
2. **OS Keychain**: Use `keyring` crate for credential storage
3. **Memory-Only Exposure**: Credentials only exposed to provider client
4. **Audit Trail**: Log every credential retrieval (without logging credential value)
5. **Encryption at Rest**: If caching needed, encrypt with OS-provided key

**Implementation Pattern**:

```rust
pub struct CredentialManager {
    keyring: Keyring,
}

impl CredentialManager {
    pub async fn get_api_key(&self, provider: &str) -> Result<String, SecurityError> {
        // Retrieve from OS keychain
        // Audit log the retrieval
        // Return for one-time use
    }
    
    pub async fn set_api_key(&self, provider: &str, key: &str) -> Result<(), SecurityError> {
        // Store in OS keychain
        // Never log the key value
    }
}
```

**Forbidden Patterns**:
```rust
// ❌ NEVER DO THIS
let api_key = "sk-...";  // Hardcoded credential
config.api_key = "sk-...";  // Stored in config
sqlx::query("INSERT INTO keys VALUES (?)").bind(key);  // Stored in DB
println!("Key: {}", key);  // Logged to console
```

---

## 3. Permission Model

### 3.1 Tauri Capabilities

**Current Status**: Basic capability files exist

**Main Window Capabilities** (`capabilities/main-window.json`):
- `fs`: Limited filesystem access
- `dialog`: File open/save dialogs
- `shell`: Open external URLs (with allowlist)
- `notification`: Desktop notifications
- `event`: Emit/listen to events

**Artifact Window Capabilities** (`capabilities/artifact-window.json`):
- Minimal permissions
- No filesystem access
- No network access
- Event listen only

**Verification Required**:
1. Review all capability files
2. Apply principle of least privilege
3. Test artifact window isolation

### 3.2 Artifact Isolation

**Current Status**: Defined in ARCHITECTURE.md

**Isolation Requirements**:
| Capability | Main Window | Artifact Window |
|------------|-------------|-----------------|
| SQLite Access | ✅ Via Rust | ❌ None |
| Filesystem | ✅ Limited | ❌ None |
| Network | ✅ Allowlist | ❌ None |
| Shell | ❌ None | ❌ None |
| Credentials | ✅ Via keychain | ❌ None |

**Implementation Notes**:
- Artifact WebView runs in separate process (Tauri feature)
- Content Security Policy restricts scripts/styles
- No access to main window cookies or storage
- Communication only via explicit message protocol

### 3.3 Plugin Permissions

**Current Status**: PLUGIN_SPEC.md defines, not implemented

**Permission Requirements**:
```rust
pub struct PluginManifest {
    pub id: String,
    pub version: String,
    pub permissions: Vec<PluginPermission>,
}

pub enum PluginPermission {
    Network { domains: Vec<String> },
    Storage { max_size_mb: u32 },
    FileSystem { paths: Vec<PathPattern> },
}

pub fn validate_plugin(manifest: PluginManifest) -> Result<(), ValidationError> {
    // Verify no dangerous permissions
    // Verify no path traversal in allowed paths
    // Verify no wildcard domains
}
```

**Forbidden Plugin Permissions**:
- Shell execution
- Arbitrary code execution
- Access to ~/.alphaforge/security/
- Access to main window data

---

## 4. Data Security

### 4.1 Local Database

**Current Status**: SQLite with SQLx

**Security Requirements**:
1. **File Permissions**: Database file readable only by user
2. **Encryption**: Sensitive fields encrypted (future)
3. **Backup Security**: Backups inherit file permissions
4. **Migration Safety**: Migrations never expose sensitive data in logs

**Database File Security**:
```bash
# Database location
~/.alphaforge/data/alphaforge.db

# Required permissions
chmod 600 alphaforge.db  # User read/write only
```

**Future: Field-Level Encryption**:
```rust
// For sensitive fields (e.g., notes about positions)
pub struct EncryptedField<T> {
    ciphertext: Vec<u8>,
    nonce: [u8; 12],
}

impl<T: Serialize> EncryptedField<T> {
    pub fn encrypt(value: &T, key: &[u8]) -> Result<Self, CryptoError>;
    pub fn decrypt(&self, key: &[u8]) -> Result<T, CryptoError>;
}
```

### 4.2 User Research Data

**Security Classification**:
| Data Type | Classification | Storage |
|-----------|----------------|---------|
| Research notes | Confidential | SQLite (encrypted future) |
| Thesis statements | Confidential | SQLite |
| Portfolio positions | Confidential | SQLite |
| Agent task history | Internal | SQLite (append-only) |
| Source URLs | Internal | SQLite |
| User preferences | Internal | SQLite |

**Retention Policy**:
- User controls retention (delete workspace → delete all data)
- No cloud sync by default
- Export capability for user backup

### 4.3 API Keys

**Security Requirements**:
- Storage: OS keychain ONLY
- Exposure: Memory-only, single-use
- Logging: Never log key values
- Transmission: HTTPS only to provider APIs

**Implementation Checklist**:
- [ ] Implement `CredentialManager` with OS keychain
- [ ] Add audit logging for key retrieval
- [ ] Verify no hardcoded keys in code/tests
- [ ] Add pre-commit hook to scan for key patterns

### 4.4 Sensitive Information

**Categories of Sensitive Data**:
1. **API Keys**: AI provider keys
2. **Portfolio Data**: Holdings, transactions
3. **Research Insights**: Unpublished thesis, notes
4. **Personal Information**: User preferences, patterns

**Handling Rules**:
- Never log sensitive values
- Never include in error messages
- Never transmit over unencrypted channels
- Never store in temporary files without encryption

---

## 5. Current Security Requirements (Phase 2)

### 5.1 Must Implement

| Requirement | Priority | Effort |
|-------------|----------|--------|
| OS keychain integration | High | 2 days |
| Input validation (Zod + Rust) | High | 1 day |
| Credential audit logging | High | 1 day |
| File permission verification | Medium | 1 day |

### 5.2 Should Implement

| Requirement | Priority | Effort |
|-------------|----------|--------|
| Tool permission model | Medium | 3 days |
| Network allowlist | Medium | 2 days |
| Database file permissions check | Low | 0.5 day |

### 5.3 Can Defer

| Requirement | Phase | Reason |
|-------------|-------|--------|
| Field-level encryption | Phase 4+ | No encryption needed in MVP |
| Plugin sandbox | Phase 7 | External plugins not in MVP |
| Backup encryption | Phase 6+ | Backup not in MVP |

---

## 6. Future Security Requirements

### 6.1 Phase 3-4 (Agent Intelligence)

| Requirement | Risk Addressed |
|-------------|----------------|
| Tool execution sandbox | Arbitrary code execution |
| Approval workflow | Destructive actions |
| Context isolation | Data leakage across tasks |

### 6.2 Phase 5-6 (Knowledge System)

| Requirement | Risk Addressed |
|-------------|----------------|
| Thesis encryption | Confidential research |
| Audit trail integrity | Tampering with history |
| Export encryption | Data leakage via export |

### 6.3 Phase 7-8 (Plugins & Production)

| Requirement | Risk Addressed |
|-------------|----------------|
| Plugin sandboxing | Malicious plugins |
| Permission prompts | User awareness |
| Security audits | Unknown vulnerabilities |

---

## 7. Security Testing

### 7.1 Required Security Tests

```rust
#[test]
fn test_credentials_not_logged() {
    let manager = CredentialManager::new();
    manager.set_api_key("test_provider", "sk-test-key").unwrap();
    
    let logs = capture_logs(|| {
        manager.get_api_key("test_provider").unwrap();
    });
    
    assert!(!logs.contains("sk-test-key"));
}

#[test]
fn test_path_traversal_blocked() {
    let validator = PathValidator::new();
    
    assert!(validator.is_safe("../secrets/keys").is_err());
    assert!(validator.is_safe("~/.ssh/id_rsa").is_err());
    assert!(validator.is_safe("/etc/passwd").is_err());
}

#[test]
fn test_artifact_window_has_no_db_access() {
    let caps = load_capabilities("artifact-window");
    assert!(!caps.has_permission("sqlite"));
    assert!(!caps.has_permission("fs"));
}
```

### 7.2 Security Audit Checklist

**Pre-Release Security Audit**:
- [ ] No hardcoded credentials in source
- [ ] All API calls use HTTPS
- [ ] OS keychain used for credential storage
- [ ] Artifact windows have minimal capabilities
- [ ] Input validation on all IPC boundaries
- [ ] No sensitive data in logs
- [ ] Database file has correct permissions
- [ ] No arbitrary filesystem access
- [ ] Tool execution requires permissions

---

## 8. Recommendations

### 8.1 Immediate Actions (Phase 2)

1. **Implement CredentialManager** with OS keychain
2. **Add input validation** at Command and Service layers
3. **Verify database file permissions** on creation
4. **Add audit logging** for credential access

### 8.2 Design Decisions

1. **Tool Permission Model**: Define early, implement permissions before tools
2. **Network Allowlist**: Start with minimal set (OpenAI, Anthropic)
3. **File Access**: Sandbox to workspace directory only
4. **Encryption**: Defer to Phase 4+ unless required earlier

### 8.3 Security Monitoring

**Log These Events**:
- Credential retrieval (who, when, which provider)
- Tool invocation (which tool, input hash, output hash)
- Permission denial (which action, why denied)
- Artifact creation (which plugin, input hash)

**Never Log**:
- Credential values
- API keys
- Passwords
- Sensitive user data

---

## 9. Conclusion

Agent Runtime introduces security risks that must be addressed proactively.

**Critical Actions**:
1. Implement OS keychain for credentials
2. Define tool permission model before tool system
3. Sandbox file and network access
4. Validate all inputs at boundaries

**Do NOT**:
- Store credentials in plaintext
- Allow arbitrary file/network access
- Skip input validation
- Log sensitive values

**Overall Assessment**: Security architecture is defined but implementation is incomplete. Priority work needed before Phase 2 completion.

---

**Document Version**: 1.0
**Last Updated**: 2026-07-31
**Next Review**: After Phase 2 implementation