# macOS-First Desktop GUI Redesign Roadmap

**Milestone Identifier**: `GUI-M4-MACOS`  
**Base Branch**: `dev`  
**Feature Branch**: `feat/macos-gui-overhaul`  
**Status**: In Progress  

---

## 1. Execution Phases & Work Packages

```
Phase 1: Shell & Architecture (Tauri 2 macOS Window + Unified Header)
   ↓
Phase 2: Navigation & Core Product Loop Sidebar
   ↓
Phase 3: AI Copilot Drawer & Structured Inference Integration
   ↓
Phase 4: Design System Polish (Typography, Glassmorphism, Keyboard Shortcuts)
   ↓
Phase 5: Verification, Parity Tests & Packaged Build
```

---

### Phase 1: Shell & Architecture (Tauri 2 macOS Window + Unified Header)
- [x] Create and archive high-fidelity interactive prototype ([`docs/prototypes/macos-gui-prototype.html`](../prototypes/macos-gui-prototype.html)).
- [ ] Configure `tauri.conf.json` with macOS overlay titlebar:
  - `"titleBarStyle": "Overlay"`
  - `"decorations": true`
  - `"hiddenTitle": true`
- [ ] Implement platform detection utility (`isMacPlatform()`) in `apps/desktop/src/lib/utils/platform.ts`.
- [ ] Build `UnifiedHeader` component (`apps/desktop/src/components/layout/UnifiedHeader/`):
  - macOS traffic light safe area padding (`pl-[76px]` on macOS)
  - Sidebar collapse trigger with `⌘1` shortcut tooltip
  - Workspace selector pill
  - Spotlight search trigger (`⌘K`)
  - Market status badge & quick research action (`⌘N`)
  - Agent copilot toggle button (`⌘J`)
  - Conditional Windows/Linux window controls (`minimize`, `maximize`, `close`)
- [ ] Replace `WindowTitleBar` and `TopBar` in `MainLayout.tsx` with `UnifiedHeader`.
- [ ] Update and pass all header & layout test suites.

### Phase 2: Navigation & Core Product Loop Sidebar
- [ ] Restructure `LeftSidebar` navigation groups to mirror the AlphaForge loop:
  - Cockpit (Dashboard, Watchlist)
  - Core Pipeline (Research, Theses, Portfolio, Journal)
  - Knowledge & Tools (Knowledge Base, Options, Artifacts)
- [ ] Add Agent real-time readiness pulse card in sidebar footer.
- [ ] Add strict standard i18n keys for navigation in both `en/navigation.ts` and `zh-CN/navigation.ts`.
- [ ] Ensure 100% `catalog-parity.test.ts` pass with zero mixed bilingual strings.

### Phase 3: AI Copilot Drawer & Structured Inference Integration
- [ ] Refactor `RightSidebar` from a static toggle into an integrated macOS Copilot Drawer.
- [ ] Replace placeholder emoji with modern Lucide sparkline / bot iconography and status pulse.
- [ ] Support keyboard shortcut `⌘J` (and `Ctrl+J` fallback) to toggle drawer.
- [ ] Connect active workspace subject awareness (current ticker or thesis).

### Phase 4: Design System Polish & Human Interface Guidelines
- [ ] Update `globals.css` with Apple San Francisco font stack and font smoothing.
- [ ] Enhance card borders, shadows, and glassmorphic backdrop filters.
- [ ] Verify market color tokens (Green up / Red down for US; Red up / Green down for CN).
- [ ] Verify full keyboard shortcut palette (`⌘K`, `⌘1`, `⌘J`, `⌘N`).

### Phase 5: Verification & Delivery
- [ ] Run full test matrix: `vitest run` (all 61+ test suites).
- [ ] Run TypeScript validation: `pnpm typecheck`.
- [ ] Run Rust validation: `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Update `docs/ROADMAP.md` and `docs/MILESTONE_ROADMAP.md`.
- [ ] Prepare clean pull request to `dev`.
