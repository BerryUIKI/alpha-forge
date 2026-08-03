# GUI Layout Iteration - Task Breakdown & Schedule

## Milestone Overview

**Milestone**: GUI-M1 (GUI Layout Restructuring)
**Estimated Duration**: 2-3 weeks
**Dependencies**: None (UI-only implementation)
**Parallel Work**: Can proceed alongside M8 i18n work

---

## Work Package Breakdown

### GUI-M1-0: Project Setup & Architecture Decision (Day 1-2)

**Status**: 📋 Planned

**Deliverables**:
- [ ] Architecture Decision Record (ADR) for layout framework selection
- [ ] Component hierarchy documentation
- [ ] File structure planning
- [ ] State management strategy (UI-only)

**Exit Gate**: Architecture approved, file structure defined

---

### GUI-M1-1: Module A - Left Sidebar Implementation (Day 3-7)

**Status**: 📋 Planned

**Priority**: High
**Estimated Effort**: 3-4 days

#### Tasks

1. **Sidebar Container Component**
   - [ ] Create collapsible sidebar wrapper
   - [ ] Implement drag-to-resize functionality
   - [ ] Add expand/collapse animation
   - [ ] Responsive width constraints

2. **Workspace Dropdown Selector**
   - [ ] Design dropdown component
   - [ ] Implement 6 workspace options list
   - [ ] Selection state management (UI-only)
   - [ ] Dropdown open/close interactions

3. **Middle Scrollable Area**
   - [ ] Create scroll container
   - [ ] Add placeholder content for lists
   - [ ] Implement vertical scrolling
   - [ ] Empty state placeholder

4. **Bottom User Operations**
   - [ ] User display component
   - [ ] Dropdown submenu implementation
   - [ ] Dark/light mode toggle (UI only)
   - [ ] Settings link (placeholder)

**Verification**:
```bash
pnpm test -- --run
# Component tests for sidebar interactions
# Accessibility tests for keyboard navigation
```

**Exit Gate**: Sidebar renders correctly, all interactions functional

---

### GUI-M1-2: Module B - Native Menu Bar Implementation (Day 3-5)

**Status**: 📋 Planned

**Priority**: High
**Estimated Effort**: 2-3 days
**Can Parallel with**: GUI-M1-1

#### Tasks

1. **Tauri Menu Integration**
   - [ ] Research Tauri native menu API
   - [ ] Define menu structure
   - [ ] Implement top-level menu items
   - [ ] Create submenu hierarchy

2. **Menu Items Implementation**
   - [ ] File menu (5 items)
   - [ ] Edit menu (5 items)
   - [ ] View menu (5 items)
   - [ ] Help menu (4 items)

3. **Menu Interactions**
   - [ ] Click event placeholders
   - [ ] Keyboard shortcuts display
   - [ ] Menu enable/disable states (UI only)

**Verification**:
```bash
pnpm tauri build
# Smoke test on macOS and Windows
# Verify native menu renders correctly
```

**Exit Gate**: Menu bar appears in native window chrome, all items accessible

---

### GUI-M1-3: Module C - Right Sidebar (Agent) Implementation (Day 6-8)

**Status**: 📋 Planned

**Priority**: Medium
**Estimated Effort**: 2-3 days

#### Tasks

1. **Sidebar Container Component**
   - [ ] Reuse/adapt left sidebar component
   - [ ] Implement independent state
   - [ ] Synchronized toggle with main content button

2. **Agent Panel Skeleton**
   - [ ] Create placeholder sections
   - [ ] Design future agent interface zones
   - [ ] Empty state with "Coming Soon" message

3. **Integration**
   - [ ] Wire up with main content toggle button
   - [ ] Bidirectional state sync
   - [ ] Layout adaptation

**Exit Gate**: Right sidebar renders, synchronized with main content controls

---

### GUI-M1-4: Module D - Main Content Area Implementation (Day 8-12)

**Status**: 📋 Planned

**Priority**: High
**Estimated Effort**: 4-5 days

#### Tasks

1. **Operation Bar (Top)**
   - [ ] Context title display component
   - [ ] Shortcut action buttons (placeholders)
   - [ ] Agent sidebar toggle button
   - [ ] Button group layout

2. **Workspace View Containers (Middle)**
   - [ ] Create 6 view container components
     - [ ] Analyze View
     - [ ] Quantification View
     - [ ] Comprehensive Market View
     - [ ] Options View
     - [ ] Futures View
     - [ ] Other Derivatives View
   - [ ] View switching logic (UI only)
   - [ ] Welcome/blank state page
   - [ ] Container placeholder content

3. **Status Bar (Bottom)**
   - [ ] Workspace name display
   - [ ] Running status indicator
   - [ ] Hint text component
   - [ ] Fixed positioning

4. **Layout Integration**
   - [ ] Responsive layout with sidebars
   - [ ] Width auto-adjustment
   - [ ] Proper z-index layering

**Exit Gate**: Main content area functional, all 6 views switchable

---

### GUI-M1-5: Integration & Testing (Day 13-15)

**Status**: 📋 Planned

**Priority**: High
**Estimated Effort**: 2-3 days

#### Tasks

1. **Component Integration**
   - [ ] Wire all 4 modules together
   - [ ] Global layout tests
   - [ ] Cross-module interaction tests

2. **Responsive Testing**
   - [ ] Window resize tests
   - [ ] Sidebar collapse/expand integration
   - [ ] Minimum width constraints
   - [ ] Graceful degradation

3. **Accessibility Audit**
   - [ ] Keyboard navigation
   - [ ] Screen reader compatibility
   - [ ] ARIA labels verification
   - [ ] Focus management

4. **Cross-Platform Testing**
   - [ ] macOS smoke test
   - [ ] Windows smoke test
   - [ ] Native menu verification

**Verification**:
```bash
pnpm test
pnpm tauri build
# Manual smoke tests on both platforms
```

**Exit Gate**: All success criteria met, packaged builds working

---

### GUI-M1-6: Documentation & Code Review (Day 15-16)

**Status**: 📋 Planned

**Priority**: Medium
**Estimated Effort**: 1-2 days

#### Tasks

1. **Code Documentation**
   - [ ] Component API documentation
   - [ ] Inline code comments
   - [ ] README updates

2. **Architecture Documentation**
   - [ ] Update ARCHITECTURE.md
   - [ ] Document state management approach
   - [ ] Document component hierarchy

3. **Developer Guide**
   - [ ] How to add new workspace views
   - [ ] How to extend sidebar components
   - [ ] How to add menu items

**Exit Gate**: All documentation reviewed and approved

---

## Task Sequence Diagram

```
Day 1-2:   [GUI-M1-0] Setup & Architecture
Day 3-7:   [GUI-M1-1] Left Sidebar ────┐
Day 3-5:   [GUI-M1-2] Menu Bar ────────┼─> Can parallel
Day 6-8:   [GUI-M1-3] Right Sidebar ───┘
Day 8-12:  [GUI-M1-4] Main Content Area
Day 13-15: [GUI-M1-5] Integration & Testing
Day 15-16: [GUI-M1-6] Documentation
```

---

## Parallel Work Opportunities

### Can Work in Parallel with M8 i18n

- **GUI-M1-0 to GUI-M1-3**: No conflict with i18n work
- **GUI-M1-4**: May need i18n keys for UI strings
- **Recommendation**: Complete i18n M8-10 first, then proceed with GUI-M1-4+

### Independent Work Streams

```
Stream 1 (i18n):     M8-10 → I18N-4 → I18N-5
Stream 2 (GUI):      GUI-M1-0 → GUI-M1-1 → GUI-M1-2 → ...
```

---

## Risk Assessment

| Risk | Impact | Mitigation |
|------|--------|------------|
| Tauri native menu API limitations | Medium | Research early in GUI-M1-2, fallback to custom menu if needed |
| Sidebar drag-to-resize complexity | Low | Use existing resize libraries, test early |
| State synchronization between modules | Medium | Clear state management plan in GUI-M1-0 |
| Cross-platform menu differences | Medium | Test on both platforms early, document differences |
| Layout performance with complex views | Low | Performance testing in GUI-M1-5 |

---

## Success Metrics

1. **Functionality**
   - All UI interactions work as specified
   - No crash or error states
   - Smooth animations (60fps)

2. **Quality**
   - Test coverage > 80% for UI components
   - Zero accessibility violations
   - Lighthouse accessibility score > 90

3. **Performance**
   - Initial render < 500ms
   - Sidebar toggle animation < 300ms
   - Menu response < 100ms

4. **Developer Experience**
   - Clear component API
   - Easy to extend
   - Well documented

---

## Definition of Done

- [ ] All work packages completed
- [ ] All tests passing
- [ ] Code reviewed and approved
- [ ] Documentation complete
- [ ] Smoke tests passed on macOS and Windows
- [ ] No business logic implemented (constraint met)
- [ ] All placeholders clearly marked
- [ ] PR merged to dev branch

---

## Post-Milestone Considerations

### Future Work (Not in This Milestone)

1. **Business Logic Implementation**
   - Workspace actual switching logic
   - Data loading and persistence
   - Agent conversation functionality
   - User authentication

2. **Enhanced Features**
   - Drag-and-drop workspace reordering
   - Custom workspace creation
   - Workspace-specific settings
   - Agent tool panels

3. **Integration**
   - Backend API integration
   - Database integration
   - Real-time updates
   - Agent runtime integration

---

## Appendix: Component File Structure

```
apps/desktop/src/
├── components/
│   ├── layout/
│   │   ├── MainLayout.tsx           # Main layout orchestrator
│   │   ├── LeftSidebar/
│   │   │   ├── LeftSidebar.tsx
│   │   │   ├── WorkspaceSelector.tsx
│   │   │   ├── UserOperations.tsx
│   │   │   └── ScrollableList.tsx
│   │   ├── RightSidebar/
│   │   │   ├── RightSidebar.tsx
│   │   │   └── AgentPanel.tsx
│   │   ├── MainContent/
│   │   │   ├── MainContent.tsx
│   │   │   ├── OperationBar.tsx
│   │   │   ├── StatusBar.tsx
│   │   │   └── WorkspaceViews/
│   │   │       ├── AnalyzeView.tsx
│   │   │       ├── QuantificationView.tsx
│   │   │       ├── ComprehensiveMarketView.tsx
│   │   │       ├── OptionsView.tsx
│   │   │       ├── FuturesView.tsx
│   │   │       └── OtherDerivativesView.tsx
│   │   └── MenuBar/
│   │       └── (Tauri native menu config)
│   └── ui/                           # Existing shadcn/ui components
├── hooks/
│   └── layout/
│       ├── useSidebarState.ts
│       ├── useWorkspaceSwitch.ts
│       └── useMenuActions.ts
└── types/
    └── layout.ts
```