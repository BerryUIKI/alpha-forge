# UI Guidelines

## Design Philosophy

AlphaForge is a **desktop-first professional tool**, not a consumer web app. The UI should feel dense, efficient, and calm — optimized for focused research work, not casual browsing.

### Principles

- **Information density over whitespace.** Show relevant data. Minimize clicks to reach content.
- **Consistency over novelty.** Reuse patterns. Every page should feel like the same application.
- **Keyboard-first where possible.** Power users should navigate without leaving the keyboard.
- **State visibility.** Never leave the user wondering: loading, empty, error, and offline states are always explicit.

## Design System

| Layer | Technology |
|-------|-----------|
| CSS Framework | Tailwind CSS 4 |
| Component Library | shadcn/ui (built on Radix UI) |
| Icons | Lucide React |
| Typography | Inter (sans-serif), JetBrains Mono (monospace) |
| Theme | Light/dark via CSS custom properties (HSL) |

## Navigation

### Sidebar

A fixed left sidebar (64px wide) provides primary navigation:

```text
Today       — Dashboard
Research    — Documents, tasks, notes
Journal     — Theses, evidence, review
Portfolio   — Accounts, positions, exposure
Settings    — Configuration
```

Navigation uses icon-only buttons with tooltips. The active route is highlighted.

### Command Palette

`Ctrl+K` / `Cmd+K` opens a command palette for quick navigation and actions.

### Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+K` / `Cmd+K` | Command palette |
| `Ctrl+,` / `Cmd+,` | Open Settings |
| `Escape` | Close modal, overlay, or artifact window |
| `Ctrl+Enter` | Submit current input |

## Required UI States

Every asynchronous component must handle all seven states:

| State | Behavior |
|-------|----------|
| **Initial** | First visit, no data yet. Show a helpful prompt or empty state. |
| **Loading** | Data is fetching. Show skeleton placeholders, not a blank screen. |
| **Success** | Data is rendered — the happy path. |
| **Empty** | Data loaded but nothing to show. Show "No items yet" with a create action. |
| **Error** | Something went wrong. Show the error message and a retry button. |
| **Partial** | Some data loaded, some failed. Show what we have, indicate what's missing. |
| **Offline** | No network. Show a banner. Disable actions that require connectivity. |

## Empty States

Empty states are opportunities, not dead ends. Every empty state should include:

1. A clear message: "No research documents yet."
2. A primary action: "Create your first document" button.
3. Optional: illustration or icon for visual comfort.

## Loading States

Use skeleton screens for page-level loading, spinners for button-level loading. Never show a blank page while data loads.

Skeleton patterns:
- **List items:** Rectangles matching the item height and layout.
- **Cards:** Rectangles matching card dimensions.
- **Charts:** A rectangular placeholder with a subtle pulse animation.

## Error States

Errors should be:

- **Actionable.** "Failed to load documents. [Retry]" — not "Error 500."
- **Scoped.** An error in one component should not break the entire page.
- **Recoverable.** Provide a retry mechanism. Never force a page reload.

## Desktop-Specific Behavior

- **Resizable panels.** Where content is split (e.g., document list + detail view), panels are draggable.
- **Context menus.** Right-click on items shows relevant actions (delete, rename, export).
- **Focus states.** All interactive elements show visible focus rings for keyboard navigation.
- **Platform conventions.** Use `Ctrl` on Windows/Linux, `Cmd` on macOS. Adapt shortcut labels.

## Agent Workspace UI

The agent workspace (Today page) has three zones:

1. **Agent Input.** A text area for entering research tasks. Prominent, always visible.
2. **Task Status.** Active task progress — streaming output, tool calls, completion.
3. **Task History.** Recently completed tasks with expand/collapse for details.

## Artifact Presentation

Artifact windows:

- Open as separate Tauri WebView windows (not modals or overlays).
- Render at the dimensions specified in the plugin manifest.
- Support resizing if the manifest allows.
- Show a title bar with the artifact type name.
- Have close and persist buttons.

## Data Display Conventions

- **Numbers:** Right-aligned in tables. Use locale-appropriate formatting.
- **Currency:** Default to CNY (¥). Configurable per account.
- **Percentages:** One decimal place for most metrics. Two for precision-sensitive values.
- **Dates:** ISO 8601 in data; locale-appropriate in UI.
- **Stock prices:** Increase = Red, Decrease = Green — Chinese market convention (opposite to Western markets).
- **Timestamps:** Relative ("2 hours ago") with absolute on hover.
