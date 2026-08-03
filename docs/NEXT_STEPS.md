# AlphaForge Next Steps

**Updated**: 2026-08-03

**Current completed milestone**: M7 Plugin Ecosystem

**Next planned milestone**: M8 Local MVP Completion & Release Readiness

Program status and milestone acceptance criteria are governed by [MILESTONE_ROADMAP.md](MILESTONE_ROADMAP.md). Implementation agents follow the [Milestone Delivery Playbook](milestones/DELIVERY_PLAYBOOK.md).

## Current status

- M0 through M7 are recorded complete in the milestone roadmap.
- M8 is in progress. No M8 implementation is marked complete by this documentation update.
- The M8 decision record still requires product-owner approval before release choices are authoritative on `dev`.
- The Option documentation is consolidated, but the Option runtime is not integrated on `dev`.
- Goose implementation is explicitly post-MVP and is not part of M8.

## Immediate actions: activate M8

1. Complete the [M8 Decision Record](M8_DECISION_RECORD.md): launch locale, platforms, privacy, export, update, signing, legal, security, and support owners.
2. Execute [i18n Implementation Plan](i18n/IMPLEMENTATION_PLAN.md), beginning with the decision and string inventory package.
3. Plan local export, installer, update, privacy, security, legal, and support work as separate vertical slices.
4. Record test, packaged-smoke, and review evidence before declaring the MVP complete.

## After MVP: M9 Option

Start with the [Option Index](option/README.md), then run the baseline audit in the [Option Integration Plan](option/INTEGRATION_PLAN.md).

The first implementation package must repair Option schema application through a new append-only migration and tests. Do not merge the historical `integration/option` branch wholesale; extract and revalidate scoped candidate code against current `dev`.

### 1. Complete M8 local-desktop release foundation
**Priority**: High
**Timeline**: To be defined

M7 is complete. The local-desktop M8 boundary is now recorded in the [M8 Decision Record](M8_DECISION_RECORD.md): no account, cloud backup, telemetry, billing, licensing, or activation in the MVP. Complete the remaining legal, security-contact, and release-owner checks before public production release.

## After MVP and M9: M10 Goose

Start with the [Goose Integration Roadmap](goose/INTEGRATION_ROADMAP.md). Reverify the current AAIF Goose source and APIs, approve an ADR and threat model, then perform a synthetic-data spike. The first shipped mode is read-only and opt-in. Goose receives no direct database, shell, unrestricted filesystem, credential, trade, or privileged Tauri capability.

## Persistent quality work

- Increase regression coverage for migrations, task cancellation/restart, Artifact isolation, and critical UI states.
- Keep error codes stable and logs redacted.
- Preserve local-first data ownership and source provenance.
- Maintain loading, success, empty, error, partial, and offline behavior for asynchronous features.
- Keep documentation aligned with integrated behavior; unmerged branch code is not completion evidence.

## Standard verification

```bash
pnpm lint
pnpm typecheck
pnpm test
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Run `pnpm test:e2e` and `pnpm tauri build` for critical flows and release-impacting work. Record commands that cannot run and the remaining risk.

### M8: Production & Commercialization (local MVP foundation)
- Manual local backup export
- Privacy notice and About surface
- Manual GitHub Release update checks
- DMG and Windows EXE packaging configuration
- Authentication, licensing, billing, and cloud backup deferred

## Next decision point

The next decision is whether M8 product and release inputs are sufficiently complete to activate the first implementation work package. M9 and M10 remain planned until their entry gates pass.

## 🔧 Technical Debt & Improvements

### High Priority
1. **Testing**
   - Add E2E testing framework
   - Increase test coverage to 80%+
   - Add performance regression tests

2. **Performance**
   - Database query optimization
   - Artifact rendering performance
   - Memory usage profiling

3. **Security**
   - Audit artifact permission model
   - Input validation hardening
   - Secure credential storage

### Medium Priority
1. **Code Quality**
   - Reduce code duplication
   - Improve error handling patterns
   - Add more inline documentation

2. **Architecture**
   - Refine service layer boundaries
   - Improve state management patterns
   - Enhance type safety

---

## 📈 Success Metrics

### M8 Decision Gate
- [x] Record a local-only identity approach
- [x] Defer licensing, entitlement, and subscription enforcement
- [x] Record the no-cloud-backup privacy model
- [x] Assign @BerryUIKI as release-signing owner and select supported platforms

### Quality Gates
- All tests passing
- Code coverage > 80%
- Zero critical bugs
- Performance benchmarks met

---

## 🎯 Recommended Next Steps

### Immediate (This Week)

1. **Maintain completed milestone documentation**
   - Keep M3–M7 status and acceptance notes aligned with delivered behavior.

2. **Complete the remaining public-release gates**
   - Publish the official privacy notice and support contact email.
   - Assign a security incident contact.
   - Run legal and release-security review before public production release.

---

## 📚 Resources Needed

### Development
- Rust PDF processing library evaluation
- Full-text search implementation research
- Document storage strategy planning

### Testing
- E2E testing framework selection
- Test data generation
- Performance testing tools

### Documentation
- User guide structure
- API documentation standards
- Tutorial creation

---

## 🎉 Conclusion

**M7 Complete**: AlphaForge supports validated internal plugins, controlled artifact rendering, and the documented official plugin set.

**Next Milestone**: M8 Production & Commercialization.

**Focus Areas**: Establish the commercial and security decisions before implementing external authentication, billing, backup, or release infrastructure.

**Timeline**: To be defined after the M8 decision gate.

---

**Status**: Ready to begin next phase ✅
