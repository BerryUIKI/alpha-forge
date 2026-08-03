# Frontend-Backend Integration Guide

## Purpose

This document provides standards and guidelines for integrating frontend components with backend services. It ensures smooth collaboration between frontend and backend development, making it easier for AI agents to implement features correctly.

---

## Quick Reference for Backend Developers

### When Adding New Features

**Must Provide**:
1. ✅ Tauri Command Definition (Rust)
2. ✅ TypeScript Type Definitions
3. ✅ Input Validation Schema
4. ✅ Error Code Mapping
5. ✅ Example Usage

**Update These Files**:
- `src-tauri/src/commands/*.rs` - Command implementations
- `src/lib/i18n/catalogs/*/` - i18n keys for new features
- `src/components/*/types.ts` - TypeScript types (if needed)

---

## Integration Standards

### 1. Tauri Command Structure

**Backend (Rust)**:
```rust
#[tauri::command]
pub async fn create_workspace(
    input: CreateWorkspaceInput,
    state: State<'_, AppState>,
) -> Result<CreateWorkspaceOutput, AppError> {
    // 1. Validate input
    validate_input(&input)?;
    
    // 2. Call service layer
    let result = state.workspace_service.create(input).await?;
    
    // 3. Return typed output
    Ok(result)
}
```

**Frontend (TypeScript)**:
```typescript
// 1. Import desktop API
import { desktopApi } from '@/lib/desktop-api';

// 2. Define types (should match Rust types)
interface CreateWorkspaceInput {
  name: string;
  description?: string;
}

interface CreateWorkspaceOutput {
  id: string;
  name: string;
  created_at: string;
}

// 3. Call command
const createWorkspace = async (input: CreateWorkspaceInput) => {
  return await desktopApi.invoke<CreateWorkspaceOutput>(
    'create_workspace',
    input
  );
};
```

---

### 2. Type Definition Standards

**Backend Must Provide**:

```typescript
// File: src/types/workspace.ts

/**
 * Input for creating a workspace
 * 
 * Backend: src-tauri/src/commands/workspace.rs
 * Command: create_workspace
 */
export interface CreateWorkspaceInput {
  /** Workspace name (required, max 200 chars) */
  name: string;
  
  /** Optional description */
  description?: string;
}

/**
 * Output from creating a workspace
 */
export interface CreateWorkspaceOutput {
  /** Unique workspace ID */
  id: string;
  
  /** Workspace name */
  name: string;
  
  /** ISO 8601 timestamp */
  created_at: string;
}
```

**Requirements**:
- ✅ Match Rust types exactly
- ✅ Include JSDoc comments
- ✅ Reference backend file location
- ✅ Mark optional fields with `?`
- ✅ Use proper TypeScript types (not `any`)

---

### 3. Error Handling Standards

**Backend Error Codes** (Stable):

```rust
pub enum ErrorCode {
    INTERNAL,        // Internal application error
    NOT_FOUND,       // Resource not found
    VALIDATION,      // Input validation failed
    PERMISSION_DENIED, // Permission denied
    TIMEOUT,         // Operation timeout
}
```

**Frontend Error Handling**:

```typescript
import { processErrorResponse } from '@/lib/i18n/errorMessages';

try {
  const result = await desktopApi.invoke('create_workspace', input);
  // Handle success
} catch (error) {
  const errorResponse = error as ErrorResponse;
  
  // Get localized error messages
  const localized = processErrorResponse(locale, errorResponse);
  
  // Display to user
  showToast({
    title: localized.title,
    description: localized.description,
    variant: 'error',
  });
}
```

**Error Response Structure**:
```typescript
interface ErrorResponse {
  code: string;        // Stable error code
  message: string;     // Technical message (for logs)
  recoverable: boolean; // Can user retry?
}
```

---

### 4. Input Validation

**Backend Validation** (Rust):
```rust
fn validate_input(input: &CreateWorkspaceInput) -> Result<(), AppError> {
    if input.name.is_empty() {
        return Err(AppError::validation("Workspace name is required"));
    }
    if input.name.len() > 200 {
        return Err(AppError::validation("Workspace name too long"));
    }
    Ok(())
}
```

**Frontend Validation** (TypeScript):
```typescript
import { z } from 'zod';

const CreateWorkspaceSchema = z.object({
  name: z.string()
    .min(1, 'Workspace name is required')
    .max(200, 'Workspace name too long'),
  description: z.string().optional(),
});

// Validate before calling backend
const validateInput = (input: unknown) => {
  return CreateWorkspaceSchema.safeParse(input);
};
```

**Requirements**:
- ✅ Validate on frontend first (better UX)
- ✅ Backend must always validate (security)
- ✅ Use same validation rules
- ✅ Provide clear error messages

---

### 5. i18n Integration

**When Adding New Features**:

**Step 1**: Add i18n keys
```typescript
// File: src/lib/i18n/catalogs/en/workspace.ts
export default {
  createWorkspace: "Create Workspace",
  workspaceName: "Workspace Name",
  workspaceNamePlaceholder: "My Research",
  workspaceNameRequired: "Workspace name is required",
  failedToCreateWorkspace: "Failed to create workspace",
};
```

**Step 2**: Use in components
```typescript
import { useLocale } from '@/lib/i18n/useLocale';

const { t } = useLocale();

// In JSX
<h1>{t('createWorkspace')}</h1>
<input placeholder={t('workspaceNamePlaceholder')} />
```

**Requirements**:
- ✅ Add keys for all UI strings
- ✅ Support both `en` and `zh-CN`
- ✅ Use descriptive key names
- ✅ Keep keys organized by feature

---

### 6. State Management Standards

**Using TanStack Query**:
```typescript
import { useQuery, useMutation } from '@tanstack/react-query';

// Query
const useWorkspace = (id: string) => {
  return useQuery({
    queryKey: ['workspace', id],
    queryFn: () => desktopApi.invoke('get_workspace', { id }),
    staleTime: 5000,
  });
};

// Mutation
const useCreateWorkspace = () => {
  return useMutation({
    mutationFn: (input: CreateWorkspaceInput) =>
      desktopApi.invoke('create_workspace', input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['workspaces'] });
    },
  });
};
```

**Requirements**:
- ✅ Use TanStack Query for async state
- ✅ Invalidate queries on mutations
- ✅ Handle loading/error states
- ✅ Implement proper caching

---

### 7. Event Streaming Standards

**Backend Events** (Rust):
```rust
// Emit event from backend
app_handle.emit("task_progress", &ProgressEvent {
    task_id: task_id.to_string(),
    progress: 50,
    message: "Processing...".to_string(),
})?;
```

**Frontend Listener** (TypeScript):
```typescript
import { listen } from '@tauri-apps/api/event';

// Setup listener
const unlisten = await listen<ProgressEvent>('task_progress', (event) => {
  console.log('Progress:', event.payload);
  updateProgressUI(event.payload);
});

// Cleanup on unmount
onUnmounted(() => {
  unlisten();
});
```

**Event Structure**:
```typescript
interface ProgressEvent {
  task_id: string;
  progress: number;    // 0-100
  message: string;
}
```

---

### 8. Testing Standards

**Backend Must Provide**:
- ✅ Example inputs/outputs
- ✅ Error case examples
- ✅ Edge case documentation

**Frontend Tests**:
```typescript
import { describe, it, expect } from 'vitest';

describe('createWorkspace', () => {
  it('should create workspace with valid input', async () => {
    const input = { name: 'Test Workspace' };
    const result = await createWorkspace(input);
    
    expect(result.id).toBeDefined();
    expect(result.name).toBe(input.name);
  });
  
  it('should fail with empty name', async () => {
    const input = { name: '' };
    
    await expect(createWorkspace(input)).rejects.toThrow();
  });
});
```

---

## Integration Checklist

### For Backend Developers

When adding new features, complete this checklist:

**Command Implementation**:
- [ ] Define Rust command with `#[tauri::command]`
- [ ] Create input/output types
- [ ] Implement validation logic
- [ ] Add error handling with stable codes
- [ ] Write unit tests

**Documentation**:
- [ ] Add TypeScript type definitions
- [ ] Document input/output structure
- [ ] Provide example usage
- [ ] Add i18n keys for error messages
- [ ] Update this integration guide

**Testing**:
- [ ] Test with valid inputs
- [ ] Test with invalid inputs
- [ ] Test error scenarios
- [ ] Verify i18n error messages

### For Frontend Developers (AI Agent)

When integrating new features:

**Preparation**:
- [ ] Read backend command documentation
- [ ] Copy TypeScript types exactly
- [ ] Add i18n keys
- [ ] Setup TanStack Query hooks

**Implementation**:
- [ ] Add input validation (Zod)
- [ ] Implement error handling
- [ ] Add loading/error states
- [ ] Test with example data

**Quality**:
- [ ] Use proper TypeScript types
- [ ] Handle all error cases
- [ ] Add proper loading states
- [ ] Write component tests

---

## Common Patterns

### Pattern 1: CRUD Operations

**Create**:
```typescript
const useCreateResource = () => {
  return useMutation({
    mutationFn: (input: CreateInput) =>
      desktopApi.invoke('create_resource', input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['resources'] });
      showToast({ title: t('createdSuccessfully'), variant: 'success' });
    },
    onError: (error) => {
      const localized = processErrorResponse(locale, error);
      showToast({ title: localized.title, variant: 'error' });
    },
  });
};
```

**Read**:
```typescript
const useResource = (id: string) => {
  return useQuery({
    queryKey: ['resource', id],
    queryFn: () => desktopApi.invoke('get_resource', { id }),
    enabled: !!id,
  });
};
```

**Update**:
```typescript
const useUpdateResource = () => {
  return useMutation({
    mutationFn: (input: UpdateInput) =>
      desktopApi.invoke('update_resource', input),
    onSuccess: (data) => {
      queryClient.setQueryData(['resource', data.id], data);
      showToast({ title: t('updatedSuccessfully'), variant: 'success' });
    },
  });
};
```

**Delete**:
```typescript
const useDeleteResource = () => {
  return useMutation({
    mutationFn: (id: string) =>
      desktopApi.invoke('delete_resource', { id }),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['resources'] });
      showToast({ title: t('deletedSuccessfully'), variant: 'success' });
    },
  });
};
```

### Pattern 2: List with Pagination

```typescript
const useResources = (page: number, pageSize: number) => {
  return useQuery({
    queryKey: ['resources', page, pageSize],
    queryFn: () => desktopApi.invoke('list_resources', { 
      offset: page * pageSize,
      limit: pageSize,
    }),
  });
};
```

### Pattern 3: Search/Filter

```typescript
const useSearchResources = (query: string) => {
  return useQuery({
    queryKey: ['resources', 'search', query],
    queryFn: () => desktopApi.invoke('search_resources', { query }),
    enabled: query.length > 0,
    debounce: 300,
  });
};
```

---

## File Organization

### Backend Structure
```
src-tauri/src/
├── commands/
│   ├── workspace.rs      # Workspace commands
│   ├── research.rs       # Research commands
│   └── agent.rs          # Agent commands
├── services/
│   ├── workspace.rs      # Business logic
│   └── research.rs
└── error.rs              # Error definitions
```

### Frontend Structure
```
src/
├── types/
│   ├── workspace.ts      # TypeScript types
│   └── research.ts
├── hooks/
│   ├── useWorkspace.ts   # Data fetching hooks
│   └── useResearch.ts
├── components/
│   ├── workspace/        # UI components
│   └── research/
└── lib/i18n/
    └── catalogs/         # i18n translations
```

---

## Communication Protocol

### When Backend Changes

**Backend Developer Must**:
1. Update command implementation
2. Update TypeScript types in docs
3. Update i18n keys if needed
4. Notify frontend team (update `CHANGELOG.md`)
5. Provide migration guide for breaking changes

**Frontend Agent Must**:
1. Review TypeScript type changes
2. Update components to match new types
3. Update i18n keys
4. Test integration
5. Update tests

### Breaking Changes Policy

**Avoid Breaking Changes**:
- Keep command signatures stable
- Add new optional fields instead of changing existing ones
- Deprecate old commands gradually
- Maintain backward compatibility

**If Breaking Change Required**:
1. Create new command version (e.g., `create_workspace_v2`)
2. Keep old command working
3. Document migration path
4. Set deprecation timeline
5. Update all frontend code

---

## Example: Adding a New Feature

### Scenario: Add "Export Workspace" Feature

**Step 1: Backend Implementation**
```rust
// File: src-tauri/src/commands/workspace.rs

#[derive(Serialize, Deserialize)]
pub struct ExportWorkspaceInput {
    pub workspace_id: String,
    pub format: ExportFormat,
}

#[derive(Serialize, Deserialize)]
pub enum ExportFormat {
    Json,
    Csv,
    Pdf,
}

#[derive(Serialize, Deserialize)]
pub struct ExportWorkspaceOutput {
    pub file_path: String,
    pub created_at: String,
}

#[tauri::command]
pub async fn export_workspace(
    input: ExportWorkspaceInput,
    state: State<'_, AppState>,
) -> Result<ExportWorkspaceOutput, AppError> {
    // Implementation
}
```

**Step 2: Frontend Types**
```typescript
// File: src/types/workspace.ts

export interface ExportWorkspaceInput {
  workspace_id: string;
  format: ExportFormat;
}

export type ExportFormat = 'Json' | 'Csv' | 'Pdf';

export interface ExportWorkspaceOutput {
  file_path: string;
  created_at: string;
}
```

**Step 3: i18n Keys**
```typescript
// File: src/lib/i18n/catalogs/en/workspace.ts

export default {
  // ... existing keys
  exportWorkspace: "Export Workspace",
  exportFormat: "Export Format",
  exporting: "Exporting...",
  exportSuccessful: "Workspace exported successfully",
  exportFailed: "Failed to export workspace",
};
```

**Step 4: Frontend Hook**
```typescript
// File: src/hooks/useWorkspace.ts

export const useExportWorkspace = () => {
  const { t } = useLocale();
  
  return useMutation({
    mutationFn: (input: ExportWorkspaceInput) =>
      desktopApi.invoke<ExportWorkspaceOutput>('export_workspace', input),
    onSuccess: (data) => {
      showToast({
        title: t('exportSuccessful'),
        description: `Saved to: ${data.file_path}`,
        variant: 'success',
      });
    },
    onError: (error) => {
      const localized = processErrorResponse(locale, error);
      showToast({
        title: t('exportFailed'),
        description: localized.description,
        variant: 'error',
      });
    },
  });
};
```

**Step 5: Component**
```typescript
// File: src/components/workspace/ExportDialog.tsx

export function ExportDialog({ workspaceId }: { workspaceId: string }) {
  const { t } = useLocale();
  const exportMutation = useExportWorkspace();
  const [format, setFormat] = useState<ExportFormat>('Json');
  
  const handleExport = () => {
    exportMutation.mutate({
      workspace_id: workspaceId,
      format,
    });
  };
  
  // ... JSX implementation
}
```

---

## Summary

This guide ensures:
- ✅ Consistent integration patterns
- ✅ Clear communication between teams
- ✅ Type safety throughout
- ✅ Proper error handling
- ✅ i18n support
- ✅ Testable code

**When in doubt, follow these principles**:
1. Match backend types exactly
2. Validate inputs early
3. Handle all error cases
4. Use i18n for all UI strings
5. Test thoroughly

---

*Last Updated: 2026-08-03*
*Version: 1.0*
*Status: Active*