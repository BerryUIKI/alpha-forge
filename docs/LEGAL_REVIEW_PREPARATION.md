# Legal Review Preparation

**Status**: Preparation for M8 MVP Release
**Owner**: @BerryUIKI
**Last updated**: 2026-08-03

This document prepares the legal review requirements for AlphaForge M8 Local MVP release.

---

## 1. Legal Review Scope

### 1.1 Required Reviews

| Document | Type | Owner | Status |
|----------|------|-------|--------|
| Terms of Use | Legal | @BerryUIKI | Pending |
| Privacy Policy | Legal | @BerryUIKI | Pending |
| Investment Research Disclaimer | Legal | @BerryUIKI | Pending |
| Open Source License (MIT) | Legal | @BerryUIKI | Pending |
| Third-Party Licenses | Compliance | @BerryUIKI | Pending |

### 1.2 Review Timeline

- **Target**: Before public production release
- **Lead Time**: Allow 2-4 weeks for legal review
- **Dependencies**: Product features frozen, documentation complete

---

## 2. Terms of Use

### 2.1 Required Sections

1. **Acceptance of Terms**
   - User agreement to terms
   - Modification rights

2. **Description of Service**
   - Local-first desktop application
   - No cloud services
   - Research tool, not trading platform

3. **User Responsibilities**
   - Data backup responsibility
   - No misuse of application
   - Compliance with local laws

4. **Intellectual Property**
   - Application ownership
   - User data ownership
   - License grant (MIT)

5. **Disclaimers**
   - Not financial advice
   - No warranty
   - Accuracy limitations

6. **Limitation of Liability**
   - Use at own risk
   - No liability for investment decisions
   - Maximum liability caps (if applicable)

7. **Termination**
   - License termination conditions
   - Data handling on termination

8. **Governing Law**
   - Jurisdiction
   - Dispute resolution

### 2.2 Draft Terms

**Note**: Final terms must be reviewed by qualified legal counsel.

```markdown
# AlphaForge Terms of Use

**Effective Date**: [DATE]
**Last Updated**: 2026-08-03

## 1. Acceptance of Terms

By using AlphaForge ("the Application"), you agree to these Terms of Use.

## 2. Description of Service

AlphaForge is a local-first desktop application for investment research. It does not:
- Execute trades
- Provide personalized investment advice
- Store data on remote servers
- Require an account or subscription

## 3. User Responsibilities

You are responsible for:
- Backing up your data
- Your investment decisions
- Compliance with applicable laws

## 4. Intellectual Property

AlphaForge is open-source software licensed under the MIT License. Your data remains yours.

## 5. Disclaimers

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.

AlphaForge is a research tool, not financial advice. Investment decisions are yours alone.

## 6. Limitation of Liability

IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY.

## 7. Governing Law

These terms are governed by the laws of [JURISDICTION].
```

---

## 3. Privacy Policy

### 3.1 Current Status

Privacy policy exists at [PRIVACY.md](PRIVACY.md).

### 3.2 Required Updates

- [ ] Add effective date
- [ ] Add contact email
- [ ] Add data retention policy
- [ ] Add cookie policy (N/A - local app)
- [ ] Add third-party services (GitHub Releases only)

### 3.3 Key Points

| Data Type | Collection | Storage | Sharing |
|-----------|------------|---------|---------|
| User data | No cloud | Local SQLite | None |
| Telemetry | None | N/A | N/A |
| Crash reports | None | N/A | N/A |
| Usage analytics | None | N/A | N/A |

---

## 4. Investment Research Disclaimer

### 4.1 Current Status

Disclaimer exists at [INVESTMENT_RESEARCH_DISCLAIMER.md](INVESTMENT_RESEARCH_DISCLAIMER.md).

### 4.2 Required Display Locations

- [x] Application About screen
- [x] Settings page
- [x] README
- [ ] First launch modal (recommended)
- [ ] Thesis creation flow (recommended)

### 4.3 Disclaimer Text

**Current text** (from INVESTMENT_RESEARCH_DISCLAIMER.md):

> AlphaForge is a research workspace, not an investment advisor. It does not provide personalized investment advice, recommendations, or solicit transactions. All research output is for informational purposes only.

### 4.4 Recommended Additions

```markdown
**Investment Research Disclaimer**

AlphaForge is a research tool, not a registered investment advisor.

- All content is for informational purposes only
- Past performance does not guarantee future results
- Investment decisions are yours alone
- Consult a qualified financial advisor before investing

THE AUTHORS ARE NOT FINANCIAL ADVISORS.
```

---

## 5. Open Source License

### 5.1 Current License

**License**: AGPL v3 (Affero GNU General Public License v3.0)

**File**: [LICENSE](../LICENSE)

### 5.2 License Compliance

| Requirement | Status |
|-------------|--------|
| License file in repository | ✅ Present |
| License header in source files | ⚠️ Not required for AGPL |
| Copy of license in distributions | ✅ Included |
| Source code availability | ✅ GitHub repository |

### 5.3 Third-Party Licenses

**Dependencies**:
- Rust crates (Cargo.toml)
- npm packages (package.json)

**Action Required**:
- [ ] Run `cargo license` to audit Rust dependencies
- [ ] Run `pnpm licenses` to audit npm dependencies
- [ ] Check for license compatibility with AGPL v3
- [ ] Create THIRD_PARTY_LICENSES.md if required

---

## 6. Third-Party License Audit

### 6.1 Rust Dependencies

**Command**:
```bash
cargo license --all-features --avoid-dev-deps
```

**Expected Licenses**: MIT, Apache-2.0, BSD-3-Clause, ISC (compatible with AGPL)

**Review Required For**:
- Any GPL-2.0 or GPL-3.0 dependencies
- Any proprietary licenses
- Any copyleft licenses that may conflict

### 6.2 npm Dependencies

**Command**:
```bash
pnpm licenses list
```

**Expected Licenses**: MIT, Apache-2.0, BSD-3-Clause, ISC

**Review Required For**:
- Any GPL dependencies
- Any proprietary licenses
- Any packages with unusual license terms

---

## 7. Jurisdiction and Applicable Law

### 7.1 Recommended Jurisdiction

| Factor | Recommendation |
|--------|----------------|
| **Project Owner Location** | Mainland China |
| **Target Markets** | China primary, English-speaking markets secondary |
| **Governing Law** | Laws of the People's Republic of China |
| **Dispute Resolution** | Negotiation, then arbitration in [CITY] |

**Note**: Final jurisdiction must be determined by legal counsel.

### 7.2 International Considerations

| Market | Consideration |
|--------|---------------|
| China | ICP filing (if website), cybersecurity law compliance |
| United States | No SEC/FINRA registration required (not a broker-dealer) |
| EU | GDPR compliance (N/A - no cloud data) |
| Other | General disclaimer applicability |

---

## 8. Release Checklist

### 8.1 Legal Review Checklist

- [ ] Terms of Use drafted and reviewed
- [ ] Privacy Policy updated with contact and dates
- [ ] Investment Disclaimer reviewed and approved
- [ ] Third-party license audit completed
- [ ] License compatibility verified
- [ ] Jurisdiction determined
- [ ] Governing law clause drafted
- [ ] Limitation of liability reviewed
- [ ] All legal documents dated and signed off

### 8.2 Distribution Checklist

- [ ] LICENSE file included in distribution
- [ ] Third-party licenses included (if required)
- [ ] Terms accessible from application
- [ ] Disclaimer displayed appropriately
- [ ] Privacy policy accessible from application

---

## 9. Legal Contacts

### 9.1 Internal

| Role | Contact |
|------|---------|
| Product Owner | @BerryUIKI |
| Legal Review | TBD |

### 9.2 External (If Needed)

| Service | Purpose |
|---------|---------|
| Legal Counsel | Terms, privacy, compliance |
| IP Attorney | Trademark, copyright |

---

## 10. Timeline

### 10.1 Pre-Review Preparation

| Task | Owner | Status |
|------|-------|--------|
| Draft Terms of Use | @BerryUIKI | Pending |
| Update Privacy Policy | @BerryUIKI | Pending |
| Audit third-party licenses | Developer | Pending |
| Prepare jurisdiction info | @BerryUIKI | Pending |

### 10.2 Legal Review

| Phase | Duration | Status |
|-------|----------|--------|
| Document preparation | 1 week | Pending |
| Legal review | 2-4 weeks | Pending |
| Revisions | 1 week | Pending |
| Final approval | 1 week | Pending |

**Total Estimated Time**: 5-7 weeks

---

## 11. Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-08-03 | Initial preparation document |

---

## See Also

- [Privacy Policy](PRIVACY.md)
- [Investment Research Disclaimer](INVESTMENT_RESEARCH_DISCLAIMER.md)
- [M8 Decision Record](M8_DECISION_RECORD.md)
- [LICENSE](../LICENSE)