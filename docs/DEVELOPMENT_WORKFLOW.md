# Development Workflow Guide

## Recommended Approach: Feature-Based End-to-End Development

### Why This Approach?

**Problem with Separate Frontend/Backend Development**:
- ❌ Type mismatches between Rust and TypeScript
- ❌ Frequent coordination needed
- ❌ Slow iteration cycles
- ❌ Debugging complexity

**Benefits of End-to-End Approach**:
- ✅ Guaranteed type safety
- ✅ Faster iteration
- ✅ Better context understanding
- ✅ Easier debugging
- ✅ Consistent naming and style

---

## Workflow Pattern

### Phase 1: Plan (10%)

**Single Agent Responsibility**:
1. Read requirements carefully
2. Check existing code patterns
3. Design data flow: `Backend → Types → Frontend → UI → i18n → Tests`
4. Create implementation plan

**Output**: Clear plan with all components listed

---

### Phase 2: Backend Implementation (25%)

**Rust Development**:
```bash
# Working directory
apps/desktop/src-tauri/src/

# Files to modify/create
commands/your_feature.rs    # Command implementation
services/your_service.rs    # Business logic
error.rs                    # (if new error types)
lib.rs                      # Register command
```

**Checklist**:
- [ ] Define input/output structs
- [ ] Implement Tauri command
- [ ] Add validation logic
- [ ] Use stable error codes
- [ ] Write unit tests
- [ ] Document with examples

**Important**: Don't commit yet! Continue to frontend.

---

### Phase 3: Type Definitions (15%)

**TypeScript Development**:
```bash
# Working directory
apps/desktop/src/types/

# Files to modify/create
your_feature.ts             # Type definitions
index.ts                    # Export types
```

**Critical Rule**: **Copy Rust struct definitions EXACTLY**

```rust
// Backend (Rust)
pub struct CreateWorkspaceInput {
    pub name: String,
    pub description: Option<String>,
}
```

```typescript
// Frontend (TypeScript) - MUST MATCH
export interface CreateWorkspaceInput {
  name: string;
  description?: string;
}
```

**Checklist**:
- [ ] Types match Rust exactly
- [ ] JSDoc comments added
- [ ] Optional fields marked with `?`
- [ ] No `any` types used
- [ ] Export from index.ts

---

### Phase 4: Frontend Integration (25%)

**React Development**:
```bash
# Working directory
apps/desktop/src/

# Files to modify/create
hooks/useYourFeature.ts      # Data fetching hooks
components/yourFeature/      # UI components
lib/i18n/catalogs/           # i18n keys
```

**Implementation Steps**:

1. **Create Hook**:
```typescript
// hooks/useCreateWorkspace.ts
export const useCreateWorkspace = () => {
  return useMutation({
    mutationFn: (input: CreateWorkspaceInput) =>
      desktopApi.invoke('create_workspace', input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['workspaces'] });
    },
  });
};
```

2. **Add i18n Keys**:
```typescript
// lib/i18n/catalogs/en/workspace.ts
export default {
  createWorkspace: "Create Workspace",
  workspaceName: "Workspace Name",
  // ... other keys
};
```

3. **Create Component**:
```typescript
// components/workspace/CreateWorkspaceDialog.tsx
export function CreateWorkspaceDialog() {
  const { t } = useLocale();
  const createMutation = useCreateWorkspace();
  
  // Component implementation
}
```

**Checklist**:
- [ ] TanStack Query hooks created
- [ ] i18n keys added (en + zh-CN)
- [ ] UI components implemented
- [ ] Loading states handled
- [ ] Error handling added

---

### Phase 5: Testing (15%)

**End-to-End Testing**:

**Backend Tests**:
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_create_workspace_success() { }
    
    #[test]
    fn test_create_workspace_validation() { }
}
```

**Frontend Tests**:
```typescript
describe('CreateWorkspace', () => {
  it('should create workspace', async () => { });
  
  it('should show validation error', () => { });
});
```

**Manual Testing**:
```bash
# Run app
pnpm tauri dev

# Test flow:
# 1. Open workspace dialog
# 2. Enter workspace name
# 3. Click create
# 4. Verify workspace appears in list
```

**Checklist**:
- [ ] Backend unit tests pass
- [ ] Frontend component tests pass
- [ ] Manual end-to-end test successful
- [ ] Error cases tested
- [ ] Loading states verified

---

### Phase 6: Documentation & Submit (10%)

**Documentation**:
- [ ] Update AGENTS.md if needed
- [ ] Add JSDoc comments
- [ ] Update integration guide if needed

**Git Commit**:
```bash
git add -A
git commit -m "feat(feature): implement create workspace

Backend:
- Add create_workspace command
- Add input validation
- Add unit tests

Frontend:
- Add TypeScript types
- Add useCreateWorkspace hook
- Add CreateWorkspaceDialog component
- Add i18n keys (en, zh-CN)

Testing:
- Backend unit tests: 2/2 passing
- Frontend tests: 3/3 passing
- Manual E2E: Verified

Refs: [Feature ID]"
```

**Create PR**:
```bash
gh pr create --title "feat(feature): implement create workspace" --base dev
```

---

## Example: Complete Feature Flow

### Feature: Export Workspace to PDF

**Step 1: Backend (Rust)**
```rust
// commands/workspace.rs

#[derive(Deserialize)]
pub struct ExportWorkspaceInput {
    pub workspace_id: String,
    pub format: ExportFormat,
}

#[derive(Serialize)]
pub struct ExportWorkspaceOutput {
    pub file_path: String,
}

#[tauri::command]
pub async fn export_workspace(
    input: ExportWorkspaceInput,
    state: State<'_, AppState>,
) -> Result<ExportWorkspaceOutput, AppError> {
    // Implementation
}
```

**Step 2: Types (TypeScript)**
```typescript
// types/workspace.ts

export interface ExportWorkspaceInput {
  workspaceId: string;
  format: ExportFormat;
}

export type ExportFormat = 'pdf' | 'csv' | 'json';

export interface ExportWorkspaceOutput {
  filePath: string;
}
```

**Step 3: Hook**
```typescript
// hooks/useWorkspace.ts

export const useExportWorkspace = () => {
  const { t } = useLocale();
  
  return useMutation({
    mutationFn: (input: ExportWorkspaceInput) =>
      desktopApi.invoke('export_workspace', input),
    onSuccess: (data) => {
      showToast({ title: t('exportSuccess') });
    },
  });
};
```

**Step 4: UI Component**
```typescript
// components/workspace/ExportDialog.tsx

export function ExportDialog({ workspaceId }: Props) {
  const exportMutation = useExportWorkspace();
  
  return (
    <Dialog>
      {/* UI implementation */}
    </Dialog>
  );
}
```

**Step 5: i18n**
```typescript
// catalogs/en/workspace.ts
export default {
  exportWorkspace: "Export Workspace",
  exportFormat: "Export Format",
  exportSuccess: "Workspace exported successfully",
};

// catalogs/zh-CN/workspace.ts
export default {
  exportWorkspace: "导出工作空间",
  exportFormat: "导出格式",
  exportSuccess: "工作空间导出成功",
};
```

**Step 6: Tests**
```rust
// Backend test
#[test]
fn test_export_workspace() { }
```

```typescript
// Frontend test
it('should export workspace', async () => { });
```

**Step 7: Commit & PR**
```bash
git commit -m "feat(workspace): add export workspace feature"
gh pr create --title "feat(workspace): add export workspace feature"
```

---

## When to Use Separate Agents

### Use Separate Agents When:

1. **Pure Frontend Work**:
   - UI-only changes (no backend changes)
   - Styling updates
   - Layout adjustments

2. **Pure Backend Work**:
   - Performance optimizations
   - Database migrations
   - Internal refactoring

3. **Large Features**:
   - Break into smaller sub-features
   - Different agents for different sub-features
   - Coordinate through documentation

---

## Best Practices

### 1. Start Small
- One feature at a time
- Complete end-to-end before moving to next
- Don't batch multiple features

### 2. Type-First Development
- Define types before implementation
- Ensure Rust and TypeScript types match
- Use types as contract

### 3. Test Continuously
- Write tests as you code
- Run tests after each phase
- Manual test before committing

### 4. Document Clearly
- JSDoc for all public functions
- Update AGENTS.md if needed
- Clear commit messages

### 5. Communicate Early
- If blocked, ask immediately
- If design decision needed, clarify first
- Don't guess - confirm

---

## Checklist Template

**Before Starting**:
- [ ] Requirements clear
- [ ] Existing code reviewed
- [ ] Implementation plan created

**Backend**:
- [ ] Structs defined
- [ ] Command implemented
- [ ] Validation added
- [ ] Tests written
- [ ] Command registered

**Types**:
- [ ] TypeScript interfaces match Rust
- [ ] JSDoc added
- [ ] Exported from index

**Frontend**:
- [ ] Hook created
- [ ] i18n added (en + zh-CN)
- [ ] Component implemented
- [ ] Error handling added
- [ ] Loading states added

**Testing**:
- [ ] Backend tests pass
- [ ] Frontend tests pass
- [ ] Manual test successful

**Submit**:
- [ ] Code reviewed
- [ ] Committed with clear message
- [ ] PR created with description

---

## Summary

**Recommended Approach**: Feature-Based End-to-End

**Key Principle**: One Agent owns the entire feature from backend to frontend

**Benefits**:
- ✅ Type safety guaranteed
- ✅ Faster iteration
- ✅ Better quality
- ✅ Less coordination needed
- ✅ Easier to maintain

**When to Split**: Only for pure frontend/backend work or very large features

---

*Version: 1.0*
*Last Updated: 2026-08-03*