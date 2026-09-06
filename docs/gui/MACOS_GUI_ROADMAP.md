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
- [x] Configure `tauri.conf.json` with macOS overlay titlebar:
  - `"titleBarStyle": "Overlay"`
  - `"decorations": true`
  - `"hiddenTitle": true`
- [x] Implement platform detection utility (`isMacPlatform()`) in `apps/desktop/src/lib/utils.ts`.
- [x] Build `UnifiedHeader` component (`apps/desktop/src/components/layout/UnifiedHeader/`):
  - macOS traffic light safe area padding (`pl-[76px]` on macOS)
  - Sidebar collapse trigger with `⌘1` shortcut tooltip
  - Workspace selector pill
  - Spotlight search trigger (`⌘K`)
  - Market status badge & quick research action (`⌘N`)
  - Agent copilot toggle button (`⌘J`)
  - Conditional Windows/Linux window controls (`minimize`, `maximize`, `close`)
- [x] Replace `WindowTitleBar` and `TopBar` in `MainLayout.tsx` with `UnifiedHeader`.
- [x] Update and pass all header & layout test suites.

### Phase 2: Navigation & Core Product Loop Sidebar
- [x] Restructure `LeftSidebar` navigation groups to mirror the AlphaForge loop:
  - Cockpit (Dashboard, Watchlist)
  - Core Pipeline (Research, Theses, Portfolio, Journal)
  - Knowledge & Tools (Knowledge Base, Options, Artifacts)
- [x] Add Agent real-time readiness pulse card in sidebar footer.
- [x] Add strict standard i18n keys for navigation in both `en/navigation.ts` and `zh-CN/navigation.ts`.
- [x] Ensure 100% `catalog-parity.test.ts` pass with zero mixed bilingual strings.

### Phase 3: AI Copilot Drawer & Structured Inference Integration
- [x] Refactor `RightSidebar` from a static toggle into an integrated macOS Copilot Drawer.
- [x] Replace placeholder emoji with modern Lucide sparkline / bot iconography and status pulse.
- [x] Support keyboard shortcut `⌘J` (and `Ctrl+J` fallback) to toggle drawer.
- [x] Connect active workspace subject awareness (current ticker or thesis).

### Phase 4: Design System Polish & Human Interface Guidelines
- [x] Update `globals.css` with Apple San Francisco font stack and font smoothing.
- [x] Enhance card borders, shadows, and glassmorphic backdrop filters.
- [x] Verify market color tokens (Green up / Red down for US; Red up / Green down for CN).
- [x] Verify full keyboard shortcut palette (`⌘K`, `⌘1`, `⌘J`, `⌘N`).

### Phase 5: Verification & Delivery
- [x] Run full test matrix: `vitest run` (all 62 test suites, 507 passed).
- [x] Run TypeScript validation: `pnpm typecheck` (0 errors).
- [x] Run Rust validation: `cargo clippy --workspace --all-targets -- -D warnings` (0 warnings).
- [x] Update `docs/ROADMAP.md` and `docs/MILESTONE_ROADMAP.md`.
- [x] Feature branch `feat/macos-gui-overhaul` clean and merged to `dev` (PR #200).

---

### Phase 6: Investment Core Workspaces Overhaul (`epic/investment-gui-overhaul`)
- [x] **Theses Pipeline Kanban Board** (PR #201):
  - Visual stage columns: Draft, Active, Validating, Validated, Closed.
  - `ThesisCard` with conviction gauge (0–100%), pro/contra evidence counts, and one-click quick transitions.
  - Dual-view toggle between Kanban Board and List View.
- [x] **Today Cockpit Dashboard** (PR #202):
  - Live `MarketPulseBar` ticker strip (SPX, NDX, US10Y, VIX) with emerald pulse indicator and synced timestamp.
  - `SecFilingFeed` with live automated 10-Q filing ingestion cards and thesis conviction delta tags.
- [x] **Agent Structured Reasoning Surface** (PR #203):
  - Upgraded `ResearchResultCard` with Sparkles header, Thesis Impact, Verifiable Evidence blocks, and Falsification Triggers alerts.
  - Interactive "Generate Interactive Sensitivity Artifact" launcher button connected to desktop isolated Artifact WebViews.
- [x] **100% Strict Standard i18n**: Full catalog parity across English and Simplified Chinese without mixed bracketed text.
- [x] **Test Verification**: 64/64 test suites passing, zero TypeScript errors, zero clippy warnings.

