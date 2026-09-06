# macOS-First Desktop GUI Redesign Specification

**Status**: Active Architecture Plan  
**Scope**: Desktop Shell, Window Management, macOS HIG Alignment, Unified Navigation, Investment Research Cockpit, AI Copilot Drawer, and Standard Pure i18n.  
**Audience**: Frontend Engineers, Desktop Core Developers, Product Designers.

---

## 1. Executive Summary & Problem Diagnosis

The current AlphaForge desktop frontend suffers from layout fragmentation, excessive vertical chrome, and an unidiomatic desktop experience—particularly on macOS:

1. **Redundant Stacked Topbars**: `MainLayout.tsx` renders both `WindowTitleBar` (36px) and `TopBar` (52px), consuming 88px of vertical screen height with disconnected controls, split breadcrumbs, and redundant search triggers.
2. **Windows 95-Style Controls on macOS**: The current titlebar embeds Windows-style minimize/maximize/close icons (`Minus`, `Square`, `X`) on the top right, while in-window dropdown menus (`File | Edit | View | Help`) contradict macOS Human Interface Guidelines (HIG), which prescribe system menubars at the top of the display and native traffic lights on the top-left.
3. **Weak Investment Loop Hierarchy**: The navigation model lacks alignment with the core product thesis:
   $$\text{Information} \to \text{Knowledge} \to \text{Thesis} \to \text{Decision} \to \text{Validation} \to \text{Review}$$
   The AI agent experience is relegated to a static icon without rich live context or structured hypothesis testing.
4. **Bilingual Clutter vs. Standard i18n**: Mixed language strings (e.g., `"今日总览 (Today)"`) violate professional production standards. The interface must use strict standard i18n with clean parity between `en-US` and `zh-CN`.

---

## 2. Architecture & Design Principles

### 2.1 Single Unified macOS Header (Height: 46px)
- **Eliminate Dual Topbars**: Consolidate `WindowTitleBar` and `TopBar` into a single, high-efficiency `UnifiedHeader`.
- **macOS Window Integration**:
  - Configure Tauri 2 with `"titleBarStyle": "Overlay"`, `"decorations": true`, and `"hiddenTitle": true`.
  - Reserve 76px left-padding (`pl-[76px]`) on macOS to accommodate native red/yellow/green traffic lights without DOM collision.
  - Enable native macOS window dragging via `data-tauri-drag-region` across neutral header zones.
  - Render Windows-style minimize/maximize/close buttons only when running on Windows/Linux environments.
- **Header Layout Composition**:
  - **Left**: Traffic light safety zone + Sidebar toggle button (`⌘1`) + Workspace Switcher Pill.
  - **Center**: Global Spotlight Search Pill (`⌘K Search tickers, theses, filings, or ask Agent...`).
  - **Right**: Market Session Badge (e.g., `US Market Open 14:32`), New Research quick action (`⌘N`), and AI Copilot drawer toggle (`⌘J`).

### 2.2 Re-architected Navigation Sidebar
Align navigation categories directly with the investment research pipeline:
- **Cockpit**: Today / Dashboard, Watchlist Movers.
- **Core Pipeline**: Research Lab, Theses (Active, Under Review, Validated), Portfolio & Holdings, Decision Journal.
- **Knowledge & Tools**: Knowledge Base (SEC 10-K, transcripts, filings), Options Flow, Generated Artifacts.
- **Footer**: Real-time Agent status pulse (e.g., `Claude 3.7 Sonnet · Standby`) and User / Settings menu.
- **Ergonomics**: Smooth spring-width collapse (240px ↔ 64px) with persistent state in `localStorage` and keyboard navigation (`⌘1`).

### 2.3 Apple San Francisco Typography & Glassmorphism
- Update font stack in `globals.css`:
  `-apple-system, BlinkMacSystemFont, "SF Pro Text", "SF Pro Display", "PingFang SC", "Helvetica Neue", sans-serif`
  with subpixel antialiasing (`-webkit-font-smoothing: antialiased`).
- Adopt dark graphite (`#16181f`) and frosted glass surfaces with high-contrast semantic borders (`border-white/[0.08]` in dark mode, `border-black/[0.08]` in light mode).

### 2.4 AI Investment Copilot Drawer
- Right-hand collapsible drawer (`w-[340px]`, `⌘J` toggle).
- Integrated with current active workspace context (active ticker, thesis, or filing).
- Structured schema reasoning viewer:
  - Thesis Impact summary
  - Verifiable evidence claims with filing citations (`[SEC 10-Q]`, `[Earnings Call]`)
  - Falsification triggers and contra-evidence alerts
  - Action trigger to spawn interactive temporary WebView Artifacts.

### 2.5 Strict Standard i18n
- Maintain strict catalog parity between `catalogs/en/` and `catalogs/zh-CN/`.
- No bilingual bracket strings. All UI copy must be clean, natural, and idiomatic in its respective language.

---

## 3. High-Fidelity Prototype Reference

An interactive HTML prototype validating these architectural specifications is permanently archived at:
- **Repository Location**: [`docs/prototypes/macos-gui-prototype.html`](../prototypes/macos-gui-prototype.html)
- **Archive Documentation**: [`docs/prototypes/README.md`](../prototypes/README.md)
- **Features Tested**: Unified titlebar, traffic light padding, ⌘K spotlight, expandable sidebar, agent copilot drawer, dark/light themes, and live i18n switching.
