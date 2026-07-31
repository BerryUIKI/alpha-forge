# PR-Agent Quick Setup Card

## 🚀 Quick Start (For Maintainers)

### 1. Configure GitHub Secrets

Go to: **Settings → Secrets and variables → Actions → New repository secret**

Add these two Secrets:

| Name | Value |
|------|-------|
| `OPENAI_KEY` | `Contact Berry for the key` |
| `OPENAI_API_BASE` | `https://api.sfkey.cn/v1` |

### 2. Verify Files Exist

Ensure these files exist:
- ✅ `.pr_agent.toml`
- ✅ `.github/workflows/pr-agent.yml`

### 3. Enable GitHub Actions

**Settings → Actions → General**
- ✅ Allow all actions and reusable workflows

### 4. Test

Create a test PR, PR-Agent will automatically review it.

---

## 📋 Available Commands

Type in PR comments:

| Command | Function |
|---------|----------|
| `/review` | Code review |
| `/describe` | Generate description |
| `/improve` | Improvement suggestions |
| `/ask "question"` | Ask questions |
| `/test` | Test suggestions |

---

## ⚠️ Security Reminder

- API key has been shared via secure channel
- **NEVER** commit the key to the repository
- If leaked, notify Berry immediately to revoke the key

---

## 📞 Contact

- GitHub: @BerryUIKI
- Issues: https://github.com/BerryUIKI/alpha-forge/issues