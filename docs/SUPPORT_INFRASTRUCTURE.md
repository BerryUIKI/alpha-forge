# Support Infrastructure

**Status**: M8 MVP Support Setup
**Owner**: @BerryUIKI
**Last updated**: 2026-08-03

This document defines the support infrastructure for AlphaForge M8 Local MVP release.

---

## 1. Support Philosophy

### 1.1 Core Principles

1. **Self-Service First**: Comprehensive documentation for common issues
2. **Community-Powered**: GitHub Issues for bug reports and features
3. **No Paid Support**: Free and open-source project
4. **Local-First**: No account required, data privacy maintained
5. **Responsive**: Acknowledge issues within reasonable timeframe

### 1.2 Support Scope

| In Scope | Out of Scope |
|----------|--------------|
| Bug reports | Personalized investment advice |
| Feature requests | Account-related issues (no accounts) |
| Documentation improvements | Cloud data recovery (no cloud) |
| Installation issues | Real-time support |
| Data export guidance | Phone/video support |

---

## 2. Support Channels

### 2.1 Primary Channel: GitHub Issues

**Repository**: https://github.com/BerryUIKI/alpha-forge/issues

**Purpose**:
- Bug reports
- Feature requests
- Documentation issues
- General questions

**Response Time**: Best effort (no SLA for open-source project)

### 2.2 Secondary Channels

| Channel | Purpose | Status |
|---------|---------|--------|
| GitHub Discussions | Community Q&A, announcements | Future consideration |
| Discord/Slack | Real-time community | Not in MVP |
| Email | Private inquiries | TBD (@BerryUIKI) |
| Social Media | Announcements | Not in MVP |

### 2.3 MVP Support Stance

**What We Provide**:
- GitHub Issues for bug tracking
- Comprehensive documentation
- Issue templates for structured reports

**What We Don't Provide**:
- Live chat support
- Phone support
- Email support (unless explicitly added)
- Guaranteed response times
- Private consultation

---

## 3. Issue Management

### 3.1 Issue Templates

Create structured templates for common issue types:

#### Bug Report Template

```markdown
---
name: Bug Report
about: Report a bug in AlphaForge
title: '[BUG] '
labels: bug
assignees: ''
---

## Description
A clear description of the bug.

## Steps to Reproduce
1. Go to '...'
2. Click on '...'
3. See error

## Expected Behavior
What you expected to happen.

## Actual Behavior
What actually happened.

## Screenshots
If applicable, add screenshots.

## Environment
- OS: [e.g., Windows 10, macOS 14]
- AlphaForge Version: [e.g., 0.1.0]
- Installation Method: [e.g., DMG, EXE]

## Logs
If applicable, attach relevant logs.

## Additional Context
Any other context about the problem.
```

#### Feature Request Template

```markdown
---
name: Feature Request
about: Suggest an idea for AlphaForge
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

## Problem Statement
A clear description of the problem you're trying to solve.

## Proposed Solution
A clear description of what you want to happen.

## Alternatives Considered
A description of any alternative solutions you've considered.

## Additional Context
Any other context or screenshots about the feature request.
```

### 3.2 Issue Labels

| Label | Purpose |
|-------|---------|
| `bug` | Confirmed or suspected bug |
| `enhancement` | Feature request |
| `documentation` | Documentation improvements |
| `good first issue` | Good for new contributors |
| `help wanted` | Seeking community help |
| `blocked` | Blocked by external dependency |
| `wontfix` | Will not be fixed |
| `duplicate` | Duplicate of existing issue |

### 3.3 Issue Workflow

```text
New Issue
    ↓
Triage (within 7 days)
    ↓
    ├── Invalid → Close with explanation
    ├── Duplicate → Close with reference
    ├── Bug → Label, investigate
    └── Feature → Label, discuss
    ↓
Investigation
    ↓
    ├── Need more info → Comment, label "needs-info"
    ├── Confirmed bug → Label "confirmed"
    └── Cannot reproduce → Comment, close
    ↓
Fix Development
    ↓
    ├── PR created → Link issue
    └── PR merged → Close issue
```

---

## 4. Documentation

### 4.1 User Documentation

| Document | Location | Purpose |
|----------|----------|---------|
| README.md | Root | Quick start, overview |
| GETTING_STARTED | docs/ | Installation, first steps |
| DATA_EXPORT_RECOVERY.md | docs/ | Backup and restore |
| TROUBLESHOOTING | docs/ | Common issues (to be created) |

### 4.2 Developer Documentation

| Document | Location | Purpose |
|----------|----------|---------|
| ARCHITECTURE.md | docs/ | System design |
| DEVELOPMENT.md | docs/ | Dev setup |
| CONTRIBUTING.md | Root | Contribution guide |
| AGENTS.md | Root | AI agent instructions |

### 4.3 Troubleshooting Guide

Create `docs/TROUBLESHOOTING.md` covering:

- Installation failures
- Startup issues
- Database errors
- Export/import problems
- Performance issues
- Update check failures

---

## 5. Release Management

### 5.1 Release Notes

**Location**: GitHub Releases page

**Format**:
```markdown
# AlphaForge vX.Y.Z

## Highlights
- Key feature 1
- Key feature 2

## New Features
- Feature description

## Bug Fixes
- Fix description

## Breaking Changes
- Breaking change description (if any)

## Known Issues
- Known issue description

## Downloads
- [Windows EXE](link)
- [macOS DMG](link)
- [Checksums](link)

## Upgrade Instructions
How to upgrade from previous version.
```

### 5.2 Announcement Channels

| Channel | Purpose | Priority |
|---------|---------|----------|
| GitHub Releases | Primary announcement | Required |
| GitHub Discussions | Community discussion | Optional |
| README Status | Version badge | Required |

---

## 6. Community Management

### 6.1 Contribution Guidelines

**File**: `CONTRIBUTING.md` (existing)

**Key Points**:
- Fork and PR workflow
- Code style requirements
- Commit message format
- Testing requirements
- Documentation requirements

### 6.2 Code of Conduct

**File**: `CODE_OF_CONDUCT.md` (to be created)

**Key Points**:
- Be respectful
- Be inclusive
- Be constructive
- Zero tolerance for harassment

### 6.3 Maintainer Responsibilities

| Responsibility | Owner |
|----------------|-------|
| Issue triage | @BerryUIKI |
| PR review | @BerryUIKI |
| Release creation | @BerryUIKI |
| Documentation updates | Contributors + @BerryUIKI |
| Security response | @BerryUIKI |

---

## 7. Security Incident Response

### 7.1 Security Contact

**Primary**: @BerryUIKI (via GitHub Security Advisories)

**Process**:
1. Report via GitHub Security Advisories (private)
2. Acknowledge within 48 hours
3. Investigate and fix
4. Publish security advisory
5. Release patched version

### 7.2 Supported Versions

| Version | Support Status |
|---------|----------------|
| Latest release | Active support |
| Previous release | Security fixes only |
| Older releases | No support |

---

## 8. Metrics and Monitoring

### 8.1 Support Metrics (Manual Tracking)

| Metric | Frequency |
|--------|-----------|
| Open issues count | Weekly |
| Average issue age | Weekly |
| Issues closed per week | Weekly |
| PR merge rate | Weekly |

### 8.2 Health Indicators

| Indicator | Good | Warning | Critical |
|-----------|------|---------|----------|
| Open issues | < 50 | 50-100 | > 100 |
| Oldest open issue | < 30 days | 30-90 days | > 90 days |
| Unanswered issues | < 10 | 10-30 | > 30 |

---

## 9. Support Cost

### 9.1 Time Investment

| Activity | Estimated Time/Week |
|----------|---------------------|
| Issue triage | 2-4 hours |
| PR reviews | 2-4 hours |
| Documentation | 1-2 hours |
| Community interaction | 1-2 hours |

**Total**: ~6-12 hours/week

### 9.2 Tooling Cost

| Tool | Cost |
|------|------|
| GitHub (public repo) | Free |
| GitHub Issues | Free |
| GitHub Releases | Free |
| GitHub Actions | Free (public) |

**Total Infrastructure Cost**: $0

---

## 10. Support Improvement Roadmap

### 10.1 MVP (Current)

- [x] GitHub Issues enabled
- [ ] Issue templates created
- [ ] CONTRIBUTING.md updated
- [ ] CODE_OF_CONDUCT.md created
- [ ] TROUBLESHOOTING.md created

### 10.2 Post-MVP

- [ ] GitHub Discussions enabled
- [ ] Automated issue labeling
- [ ] FAQ documentation
- [ ] Video tutorials
- [ ] Community chat (if demand exists)

---

## 11. Contact Information

### 11.1 Public Contact

| Purpose | Channel |
|---------|---------|
| Bug reports | GitHub Issues |
| Feature requests | GitHub Issues |
| Security issues | GitHub Security Advisories |
| General questions | GitHub Issues |

### 11.2 Private Contact (If Needed)

**Email**: TBD by @BerryUIKI

---

## 12. Success Criteria

- [ ] Issue templates created and tested
- [ ] CONTRIBUTING.md reviewed and updated
- [ ] CODE_OF_CONDUCT.md created
- [ ] TROUBLESHOOTING.md created
- [ ] First issues triaged within 7 days
- [ ] Response time tracked
- [ ] Security contact published

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-08-03 | Initial support infrastructure plan |

---

## See Also

- [CONTRIBUTING.md](../CONTRIBUTING.md)
- [PRIVACY.md](PRIVACY.md)
- [M8 Decision Record](M8_DECISION_RECORD.md)