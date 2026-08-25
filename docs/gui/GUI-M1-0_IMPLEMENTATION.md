# GUI-M1-0: Architecture Setup - Implementation Guide

> **⚠️ SUPERSEDED (2026-08-16)** — The Codex-style GUI-M1 layout was replaced by
> the GUI redesign (PRs #105–#111, `docs/gui/GUI_MILESTONE.md`). The redesigned
> `LeftSidebar` no longer uses `WorkspaceSelector`/`ScrollableList`/`UserOperations`
> from this plan. Keep this document for historical reference only.

## Overview

**Work Package**: GUI-M1-0
**Estimated Duration**: 1-2 days
**Status**: 📋 Ready to Start
**Dependencies**: None
**Branch**: `feature/gui-layout-structure`

## Objective

Establish the component file structure and architecture foundation for the GUI layout iteration. This work package creates the skeleton for all layout components without implementing any business logic.

## Prerequisites

- Node.js 20+ and pnpm installed
- Access to AlphaForge repository
- PR #37 (GUI documentation) merged to dev
- Understanding of React, TypeScript, and Tailwind CSS

## Step-by-Step Implementation

### Step 1: Create Branch

```bash
git checkout dev
git pull origin dev
git checkout -b feature/gui-layout-structure
```

### Step 2: Create Directory Structure

Create the following component directory structure:

```
apps/desktop/src/components/layout/
├── MainLayout.tsx              # Main layout orchestrator
├── LeftSidebar/
│   ├── index.ts                # Barrel export
│   ├── LeftSidebar.tsx         # Container component
│   ├── WorkspaceSelector.tsx   # Dropdown selector
│   ├── ScrollableList.tsx      # Scrollable content area
│   └── UserOperations.tsx      # Bottom user menu
├── RightSidebar/
│   ├── index.ts
│   ├── RightSidebar.tsx        # Container component
│   └── AgentPanel.tsx          # Agent panel skeleton
├── MainContent/
│   ├── index.ts
│   ├── MainContent.tsx         # Container component
│   ├── OperationBar.tsx        # Top operation bar
│   ├── StatusBar.tsx           # Bottom status bar
│   └── WorkspaceViews/
│       ├── index.ts
│       ├── AnalyzeView.tsx
│       ├── QuantificationView.tsx
│       ├── ComprehensiveMarketView.tsx
│       ├── OptionsView.tsx
│       ├── FuturesView.tsx
│       └── OtherDerivativesView.tsx
└── types.ts                    # Shared type definitions
```

### Step 3: Define Types

Create `apps/desktop/src/components/layout/types.ts`:

```typescript
/**
 * Layout component type definitions.
 * All business logic types should be defined in their respective domains.
 */

/**
 * Supported workspace types.
 * Corresponds to the 6 workspace categories in the dropdown selector.
 */
export type WorkspaceType =
  | 'analyze'
  | 'quantification'
  | 'comprehensive-market'
  | 'options'
  | 'futures'
  | 'other-derivatives';

/**
 * Layout UI state (no business logic).
 * Only tracks UI-specific state like sidebar visibility.
 */
export interface LayoutUIState {
  /** Whether left sidebar is collapsed */
  leftSidebarCollapsed: boolean;
  /** Whether right sidebar is collapsed */
  rightSidebarCollapsed: boolean;
  /** Currently active workspace (UI only, no switching logic) */
  activeWorkspace: WorkspaceType;
}

/**
 * Sidebar resize constraints.
 */
export interface SidebarConstraints {
  minWidth: number;
  maxWidth: number;
  defaultWidth: number;
}

/**
 * Constants for layout dimensions.
 */
export const LAYOUT_CONSTANTS = {
  LEFT_SIDEBAR: {
    MIN_WIDTH: 200,
    MAX_WIDTH: 400,
    DEFAULT_WIDTH: 280,
    COLLAPSED_WIDTH: 0,
  },
  RIGHT_SIDEBAR: {
    MIN_WIDTH: 250,
    MAX_WIDTH: 500,
    DEFAULT_WIDTH: 350,
    COLLAPSED_WIDTH: 0,
  },
} as const;
```

### Step 4: Create Base Components

Each component should follow this pattern:

#### Example: `LeftSidebar.tsx`

```typescript
import { type FC } from "react";
import { WorkspaceSelector } from "./WorkspaceSelector";
import { ScrollableList } from "./ScrollableList";
import { UserOperations } from "./UserOperations";

/**
 * Left collapsible sidebar component.
 * Contains workspace selector, scrollable list, and user operations.
 * 
 * TODO: [GUI-M1-1] Implement drag-to-resize functionality
 * TODO: [GUI-M1-1] Add collapse/expand animation
 * TODO: [GUI-M1-1] Connect to global layout state
 */
export const LeftSidebar: FC = () => {
  return (
    <aside className="flex h-full flex-col border-r border-border bg-card">
      {/* Top: Workspace Selector */}
      <div className="border-b border-border p-4">
        <WorkspaceSelector />
      </div>

      {/* Middle: Scrollable List */}
      <div className="flex-1 overflow-y-auto">
        <ScrollableList />
      </div>

      {/* Bottom: User Operations */}
      <div className="border-t border-border p-4">
        <UserOperations />
      </div>
    </aside>
  );
};
```

#### Example: `WorkspaceSelector.tsx`

```typescript
import { type FC } from "react";
import { useState } from "react";

/**
 * Workspace dropdown selector component.
 * Allows selection from 6 workspace types.
 * 
 * TODO: [GUI-M1-1] Connect to workspace switching logic
 * TODO: [GUI-M1-1] Add i18n for workspace names
 */
export const WorkspaceSelector: FC = () => {
  const [isOpen, setIsOpen] = useState(false);
  const [selected, setSelected] = useState("Analyze");

  const workspaces = [
    "Analyze",
    "Quantification",
    "Comprehensive Market",
    "Options",
    "Futures",
    "Other Derivatives",
  ];

  return (
    <div className="relative">
      <button
        onClick={() => setIsOpen(!isOpen)}
        className="flex w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm"
      >
        <span>{selected}</span>
        <span className="ml-2">▼</span>
      </button>

      {isOpen && (
        <ul className="absolute left-0 right-0 top-full z-50 mt-1 rounded-md border border-border bg-card shadow-lg">
          {workspaces.map((workspace) => (
            <li key={workspace}>
              <button
                onClick={() => {
                  setSelected(workspace);
                  setIsOpen(false);
                  // TODO: Trigger workspace switching logic
                }}
                className="w-full px-3 py-2 text-left text-sm hover:bg-accent"
              >
                {workspace}
              </button>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};
```

### Step 5: Create All Workspace Views

Each view should be a simple placeholder:

```typescript
// AnalyzeView.tsx
import { type FC } from "react";

/**
 * Analyze workspace view container.
 * 
 * TODO: [Future] Implement Analyze workspace functionality
 */
export const AnalyzeView: FC = () => {
  return (
    <div className="flex h-full items-center justify-center">
      <div className="text-center">
        <h2 className="text-2xl font-semibold">Analyze Workspace</h2>
        <p className="mt-2 text-muted-foreground">
          Functionality coming soon...
        </p>
      </div>
    </div>
  );
};
```

### Step 6: Create Main Layout Orchestrator

`MainLayout.tsx` should orchestrate all modules:

```typescript
import { type FC, useState } from "react";
import { LeftSidebar } from "./LeftSidebar";
import { RightSidebar } from "./RightSidebar";
import { MainContent } from "./MainContent";
import type { WorkspaceType } from "./types";

/**
 * Main layout orchestrator component.
 * Coordinates the 4 layout modules (A/B/C/D).
 * 
 * TODO: [GUI-M1-5] Connect to global state management
 * TODO: [GUI-M1-5] Add responsive behavior
 */
export const MainLayout: FC = () => {
  const [leftCollapsed, setLeftCollapsed] = useState(false);
  const [rightCollapsed, setRightCollapsed] = useState(false);
  const [activeWorkspace, setActiveWorkspace] = useState<WorkspaceType>("analyze");

  return (
    <div className="flex h-screen">
      {/* Left Sidebar */}
      {!leftCollapsed && <LeftSidebar />}

      {/* Main Content */}
      <MainContent
        activeWorkspace={activeWorkspace}
        onToggleRightSidebar={() => setRightCollapsed(!rightCollapsed)}
      />

      {/* Right Sidebar */}
      {!rightCollapsed && <RightSidebar />}
    </div>
  );
};
```

### Step 7: Update Exports

Create barrel exports in each `index.ts`:

```typescript
// LeftSidebar/index.ts
export { LeftSidebar } from "./LeftSidebar";
export { WorkspaceSelector } from "./WorkspaceSelector";
export { ScrollableList } from "./ScrollableList";
export { UserOperations } from "./UserOperations";
```

### Step 8: Verify Structure

Run these commands to verify:

```bash
# Type check
pnpm typecheck

# Lint
pnpm lint

# Build
pnpm build

# Run tests (if any)
pnpm test
```

## Implementation Constraints

### ✅ Must Do

- Create all files with proper TypeScript types
- Use Tailwind CSS for all styling
- Add proper ARIA labels for accessibility
- Include TODO comments for future work
- Follow existing code style and conventions

### ❌ Must NOT Do

- Implement any business logic
- Connect to database or API
- Implement theme switching
- Implement actual workspace switching
- Add state management for business data
- Create complex animations (basic layout only)

### 📝 Placeholder Standard

All placeholders must use this format:

```typescript
// TODO: [GUI-M1-X] Description of what needs to be implemented
```

Example:
```typescript
// TODO: [GUI-M1-1] Implement drag-to-resize functionality
// TODO: [GUI-M1-1] Connect to workspace switching logic
// TODO: [Future] Implement actual workspace functionality
```

## Testing Checklist

After implementation, verify:

- [ ] All files created with correct structure
- [ ] TypeScript compilation succeeds (`pnpm typecheck`)
- [ ] ESLint passes (`pnpm lint`)
- [ ] Application starts (`pnpm tauri dev`)
- [ ] No runtime errors in browser console
- [ ] All components render (even if empty)
- [ ] Layout displays correctly
- [ ] No business logic implemented (visual inspection)

## Commit Guidelines

Use Conventional Commits format:

```bash
git add -A
git commit -m "feat(gui): setup layout component structure (GUI-M1-0)

- Create component directory structure
- Add type definitions for layout components
- Create skeleton components for all modules
- Add placeholder content for workspace views
- All business logic marked with TODO comments

Work package: GUI-M1-0
Status: UI structure ready for GUI-M1-1"
```

## Pull Request Template

When creating PR, use this description:

```markdown
## Summary

This PR implements GUI-M1-0: Architecture Setup for the GUI layout iteration.

### Changes

- ✅ Create component directory structure
- ✅ Add type definitions (`types.ts`)
- ✅ Create skeleton components for:
  - Left Sidebar (workspace selector, scrollable list, user operations)
  - Right Sidebar (agent panel skeleton)
  - Main Content (operation bar, status bar, 6 workspace views)
- ✅ Add MainLayout orchestrator component
- ✅ Add barrel exports for all modules

### Implementation

- All components are UI skeletons with no business logic
- All placeholders marked with `TODO: [GUI-M1-X]` comments
- Follows Tailwind CSS styling conventions
- Includes accessibility attributes

### Verification

\`\`\`bash
pnpm typecheck  # ✅ Passed
pnpm lint        # ✅ Passed
pnpm build       # ✅ Passed
\`\`\`

### Constraints Met

- ✅ No business logic implemented
- ✅ No database/API connections
- ✅ No state management for business data
- ✅ Only UI structure and layout

### Next Steps

After this PR merges:
1. GUI-M1-1: Implement left sidebar interactions
2. GUI-M1-2: Implement the frameless custom title/menu bar and window controls
3. GUI-M1-3: Implement right sidebar

## Related

- Implements GUI-M1-0 from [TASK_BREAKDOWN.md](../../docs/gui/TASK_BREAKDOWN.md)
- Follows [LAYOUT_ITERATION.md](../../docs/gui/LAYOUT_ITERATION.md)
- Depends on PR #37 (GUI documentation)
```

## Estimated Time

- **Setup**: 30 minutes
- **File creation**: 2-3 hours
- **Type definitions**: 30 minutes
- **Testing**: 30 minutes
- **Documentation**: 30 minutes

**Total**: 4-5 hours (can be done in 1 day)

## Questions or Issues?

If you encounter any issues:

1. Check [LAYOUT_ITERATION.md](../../docs/gui/LAYOUT_ITERATION.md) for requirements
2. Review [TASK_BREAKDOWN.md](../../docs/gui/TASK_BREAKDOWN.md) for context
3. Open an issue or comment on the PR

## Success Criteria

This work package is complete when:

- ✅ All files created with correct structure
- ✅ TypeScript compiles without errors
- ✅ Application starts and renders layout
- ✅ No business logic implemented
- ✅ All placeholders clearly marked
- ✅ PR created and ready for review
