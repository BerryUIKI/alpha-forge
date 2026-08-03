# GUI Layout Iteration - Complete Requirements

## Overview

This document outlines the GUI layout restructuring for AlphaForge desktop application, inspired by OpenAI Codex desktop client layout style.

### Core Principles

1. **UI-Only Implementation**: This iteration focuses exclusively on interface structure, basic interactions, and menu/panel UI rendering. All backend business logic, data loading, and functional implementations are reserved as placeholders marked for future development.

2. **Codex-Style Layout**: The UI interaction paradigm follows OpenAI Codex desktop client layout style.

## Window Layout Structure

```
┌─────────────────────────────────────────────────────────┐
│ B. Top Native Menu Bar: File | Edit | View | Help       │
├──────────────┬──────────────────────────┬────────────────┤
│ A. Left      │ D. Main Content Area      │ C. Right      │
│ Collapsible  │ (Core View Container)     │ Collapsible   │
│ Sidebar      │                            │ Sidebar       │
│              │                            │ (Agent)       │
└──────────────┴──────────────────────────┴────────────────┘
```

---

## Module A: Left Collapsible Sidebar

### Basic Interactions

- Support expand/collapse toggle for entire sidebar
- Support drag-to-resize panel width
- When collapsed, entire sidebar completely hidden

### Structure (Top to Bottom)

#### 1. Sidebar Top: Workspace Dropdown Selector

**Control Type**: Text label + Dropdown arrow (Dropdown Button)

**Fixed Dropdown Options**:
- Analyze
- Quantification
- Comprehensive Market
- Options
- Futures
- Other Derivatives

**Interaction**: 
- Click to expand dropdown menu
- Selection updates UI selected state only
- Workspace switching business logic remains placeholder

#### 2. Sidebar Middle: Vertical Scrollable Area

**Purpose**:承载项目、会话、知识库列表

**Features**:
- Supports vertical scrolling
- Placeholder content for projects/sessions/knowledge base lists

#### 3. Sidebar Bottom: Fixed User Operations Area

**Characteristics**: Always visible, does not scroll with list

**Content**:
- Display username
- Click username to show dropdown submenu

**Submenu Options**:
- User Profile Page
- Dark/Light Mode Toggle
- Settings

**Implementation Note**: 
- Menu clicks only establish UI response
- All internal functionality reserved for future implementation

---

## Module B: Top Native Menu Bar

### Menu Structure

**Top-level Items**: `File | Edit | View | Help`

### Implementation Scope

- Build complete menu hierarchy UI framework
- All menu item click events are placeholders
- Business functionality temporarily not developed

### Detailed Menu Structure (UI Skeleton Only)

```
File
  ├─ New Workspace
  ├─ Open Workspace
  ├─ Save
  ├─ Export
  └─ Exit

Edit
  ├─ Undo
  ├─ Redo
  ├─ Cut
  ├─ Copy
  └─ Paste

View
  ├─ Toggle Left Sidebar
  ├─ Toggle Right Sidebar
  ├─ Zoom In
  ├─ Zoom Out
  └─ Reset Zoom

Help
  ├─ Documentation
  ├─ Keyboard Shortcuts
  ├─ Report Issue
  └─ About
```

---

## Module C: Right Collapsible Sidebar (Agent)

### Basic Interactions

- Consistent with left sidebar interaction rules
- Support expand/collapse toggle
- Support drag-to-resize panel width

### Implementation Scope

- **Build Only**: Panel GUI skeleton
- **Reserved**: Agent dialogue, tool invocation, task orchestration, and all internal functionality

### Future Capabilities (Placeholder)

- Agent conversation interface
- Tool execution panel
- Task management and orchestration
- Agent status indicators

---

## Module D: Main Content Area

### Positioning

- Core view container in window center
- Auto-adaptive to fill remaining window space when sidebars expand/collapse

### Three-Layer Structure (Top to Bottom)

#### 1. Main Content Top: Operation Bar

**Components**:
- Current context title display
- Common shortcut action button group (UI placeholder)
- Quick toggle: One-click expand/collapse right Agent sidebar
  - State synchronized bidirectionally with Module C

#### 2. Main Content Middle: Canvas Area

**Workspace View Containers**:

Create 6 independent view containers, corresponding to Module A dropdown workspace types:

1. **Analyze View Container**
2. **Quantification View Container**
3. **Comprehensive Market View Container**
4. **Options View Container**
5. **Futures View Container**
6. **Other Derivatives View Container**

**Implementation Notes**:
- Workspace switching only preserves view switching entry point
- All business logic within containers remains blank
- **Fallback**: When no project/session selected, display blank welcome page (reference Codex welcome interface)

#### 3. Main Content Bottom: Status Bar

**Always Displayed Content**:
- Currently active workspace name
- Application running status
- Simple hint text

---

## Unified Development Constraints

### Must Implement (Basic UI Interactions)

1. Panel expand/collapse
2. Dropdown menu popup
3. Button click feedback
4. List scrolling
5. View container show/hide
6. Window adaptive layout
7. Drag-to-resize panels
8. Menu hierarchy navigation

### Must NOT Implement (Reserved for Future)

1. Data persistence
2. API requests
3. View business calculations
4. Theme switching logic
5. User account validation
6. Agent conversation interaction
7. Workspace actual switching logic
8. All backend integrations

### Placeholder Marking Standard

All reserved points must have clear markers for future iteration:

```typescript
// TODO: [Module-X] Implement [feature-name] functionality
// Placeholder for [specific business logic]
```

---

## Technical Implementation Notes

### State Management

- Use React local state for UI-only interactions
- No global state management for business logic (reserved)
- Sidebar states should be independent and persistent across sessions

### Responsive Design

- Main content area must auto-adjust when sidebars toggle
- Minimum width constraints for panels
- Graceful degradation on smaller screens

### Accessibility

- All interactive elements must have proper ARIA labels
- Keyboard navigation support for menus
- Focus management for sidebar toggles

---

## Success Criteria

1. ✅ All UI components render correctly
2. ✅ Basic interactions work as specified (expand/collapse, dropdowns, etc.)
3. ✅ Layout adapts responsively to window size changes
4. ✅ All placeholders are clearly marked
5. ✅ No business logic implemented (as per constraints)
6. ✅ No actual data loading or persistence
7. ✅ Menu bar native integration works
8. ✅ All 6 workspace view containers exist and are switchable

---

## References

- OpenAI Codex desktop client (UI paradigm reference)
- Current AlphaForge architecture documentation
- shadcn/ui component library (existing design system)