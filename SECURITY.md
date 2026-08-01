# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |
| < 0.1   | :x:                |

## Security Architecture

AlphaForge is a desktop-first AI workspace for investment research. Our security model is built on the following principles:

### 1. Credential Protection
- API keys and credentials are stored in the OS keychain (not in plaintext)
- Secrets are never exposed to the React frontend
- Credential access requires explicit user permission
- The Rust credential adapter validates credential names, bounds values, and redacts platform-specific keychain errors
- Startup and task-event diagnostics use stable error codes and contextual messages rather than raw paths or underlying error strings

### 2. Data Privacy
- All data is stored locally in SQLite
- No data is sent to external servers without explicit user action
- Network requests are validated and logged

### 3. Plugin Security
- Plugins run in isolated environments
- Permission model restricts plugin capabilities
- Manifest validation prevents unauthorized access

### 4. Artifact Isolation
- Agent-generated content is rendered in isolated WebViews
- Artifacts cannot access main application privileges
- Input validation prevents injection attacks
- Artifact routes require a UUID identifier, a safe type segment, and bounded window dimensions before a WebView is created

### 5. Input Validation
- All IPC inputs are validated with Zod (TypeScript) and Serde (Rust)
- File paths are normalized and checked for directory traversal
- URLs are validated against an allowlist

## Reporting a Vulnerability

**Do NOT open a GitHub issue for security vulnerabilities.**

Instead, please report security issues privately:

1. **Email**: Send details to [security@alphaforge.dev] (replace with actual email)
2. **Include**:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Fix Timeline**: Depends on severity
  - Critical: 7 days
  - High: 14 days
  - Medium: 30 days
  - Low: Next release

### Disclosure Policy

We follow **coordinated disclosure**:

1. We acknowledge and validate the report
2. We develop and test a fix
3. We release the fix
4. We publicly disclose the vulnerability (after giving users time to update)

We credit reporters who follow this process (unless they prefer to remain anonymous).

## Security Best Practices for Contributors

### For Developers

1. **Never commit secrets**
   - Use `.env` files (already in `.gitignore`)
   - Use OS keychain for production credentials

2. **Validate all inputs**
   - TypeScript: Use Zod schemas
   - Rust: Use Serde + explicit validation

3. **Handle errors properly**
   - Never expose stack traces to users
   - Log errors with context but redact secrets

4. **Follow least privilege**
   - Request minimal permissions
   - Avoid shell access unless necessary
   - Validate file paths before filesystem access

The shell plugin is not registered, and no application window receives shell permissions. External links use the narrower opener capability.
The frontend HTTP plugin is also not registered. Network requests remain owned by Rust services, where providers and request policies can be validated centrally.

5. **Secure dependencies**
   - Run `npm audit` regularly
   - Run `cargo audit` regularly
   - Review dependency updates carefully

### For Agents (AI Assistants)

Agents working on this codebase must:

1. **Never generate code that:**
   - Executes arbitrary shell commands
   - Reads plaintext credentials
   - Bypasses Tauri permission system
   - Injects unvalidated HTML into main window

2. **Always:**
   - Validate external inputs
   - Use typed IPC commands
   - Follow architecture boundaries
   - Document new security requirements

See [AGENTS.md](AGENTS.md) for complete agent security rules.

## Security-Related Configuration

### Environment Variables

```bash
# .env.example
OPENAI_API_KEY=      # Stored in OS keychain, not here
DATABASE_URL=        # Local SQLite path
LOG_LEVEL=info       # Logging level
```

### Tauri Security

```json
// src-tauri/tauri.conf.json
{
  "security": {
    "csp": "default-src 'self'; script-src 'self'",
    "dangerousDisableAssetCspModification": false
  }
}
```

### Content Security Policy

- Scripts: Only from same origin
- Styles: Inline styles allowed (Tailwind)
- Images: Data URIs and same origin
- Connect: Restricted to allowed domains

## Known Security Considerations

### Current Limitations

1. **Development Mode**
   - CSP is relaxed in development
   - Debug features are enabled
   - Not for production use

2. **Plugin System**
   - Currently internal plugins only
   - External plugins will require sandboxing
   - Permission model in progress

3. **Artifact Windows**
   - Isolation implemented but not battle-tested
   - Need penetration testing before production
   - Route validation prevents path traversal in artifact window URLs; it does not replace a full production security audit

### Future Enhancements

- [ ] Penetration testing audit
- [ ] External plugin sandboxing
- [ ] Audit logging for all privileged operations
- [ ] Biometric unlock for sensitive operations
- [ ] Encrypted database option

## Security Checklist for Releases

Before each release, verify:

- [ ] No secrets in git history (`git log --all --full-history -- "*.env"`)
- [ ] All dependencies audited (`npm audit`, `cargo audit`)
- [ ] CSP headers validated
- [ ] Permission boundaries tested
- [ ] Error messages don't leak paths
- [ ] Logs don't contain credentials
- [ ] Update process is secured (signature verification)

## Contact

- **Security Email**: [security@alphaforge.dev]
- **Project Maintainer**: Berry Wahlberg
- **Response Time**: 48 hours (acknowledgment)

---

Last Updated: 2026-07-31

For complete security architecture, see [docs/SECURITY.md](docs/SECURITY.md).
