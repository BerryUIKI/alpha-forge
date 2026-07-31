# Phase 1.5 Implementation Plan

## Current State Analysis

**Backend (Rust)**:
- ✅ Database connection and migrations working
- ✅ AppState with db_pool
- ✅ AppError handling framework
- ❌ Commands contain inline SQL (settings.rs)
- ❌ No service layer
- ❌ Repository layer is empty placeholder
- ✅ Domain models exist but unused

**Frontend (React)**:
- ✅ Router and layout working
- ✅ desktopApi abstraction layer
- ✅ TanStack Query configured
- ❌ All pages are placeholders
- ❌ No workspace concept
- ❌ No loading/empty/error states

---

## Implementation Strategy

### 1. Rust Service Layer

**Create**: `src-tauri/src/services/`

**Files**:
```
services/
├── mod.rs              # Module exports
├── settings_service.rs # Settings business logic
├── system_service.rs   # System/app operations
└── workspace_service.rs # Workspace operations
```

**Design Principles**:
- Services own business logic and validation
- Services call repositories, never SQL directly
- Services return domain models, not database rows
- Services handle error mapping

**Example Structure**:
```rust
pub struct WorkspaceService {
    repo: WorkspaceRepository,
}

impl WorkspaceService {
    pub async fn create(&self, name: String) -> Result<Workspace, AppError> {
        // Validation
        if name.trim().is_empty() {
            return Err(AppError::Validation("Workspace name cannot be empty".into()));
        }
        
        // Call repository
        let workspace = self.repo.create(name).await?;
        
        Ok(workspace)
    }
}
```

---

### 2. Repository Layer

**Create**: `src-tauri/src/database/repositories/`

**Files**:
```
repositories/
├── mod.rs              # Module exports
├── settings_repository.rs
└── workspace_repository.rs
```

**Design Principles**:
- Repositories own all SQL queries
- Convert between database rows and domain models
- Handle SQL errors, map to AppError
- One repository per domain entity

**Example Structure**:
```rust
pub struct WorkspaceRepository {
    pool: SqlitePool,
}

impl WorkspaceRepository {
    pub async fn create(&self, name: String) -> Result<Workspace, AppError> {
        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now();
        
        sqlx::query(
            "INSERT INTO workspaces (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)"
        )
        .bind(&id)
        .bind(&name)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create workspace: {}", e)))?;
        
        Ok(Workspace { id, name, created_at: now, updated_at: now })
    }
}
```

---

### 3. Workspace Domain Entity

**Add**: `crates/domain/src/workspace.rs`

**Model**:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workspace {
    pub id: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspaceInput {
    pub name: String,
}
```

**Update**: `crates/domain/src/lib.rs` to export workspace module

**Note**: Workspace table already exists in `0001_initial.sql` migration

---

### 4. Refactor Commands

**Pattern**:
```rust
// Old (settings.rs):
#[tauri::command]
pub async fn get_setting(key: String, state: State<'_, AppState>) -> Result<Option<String>, AppError> {
    let row = sqlx::query_scalar(...).bind(&key).fetch_optional(&state.db_pool).await?;
    Ok(row)
}

// New:
#[tauri::command]
pub async fn get_setting(key: String, state: State<'_, AppState>) -> Result<Option<String>, AppError> {
    state.settings_service.get(&key).await
}
```

**Commands to refactor**:
- `settings.rs`: get_setting, set_setting
- Add new `workspace.rs` commands: create_workspace, list_workspaces, get_workspace

---

### 5. Frontend Workspace API

**Create**: `src/lib/desktop-api/workspace.ts`

**API**:
```typescript
export interface Workspace {
  id: string;
  name: string;
  createdAt: string;
  updatedAt: string;
}

export async function createWorkspace(name: string): Promise<Workspace>
export async function listWorkspaces(): Promise<Workspace[]>
export async function getWorkspace(id: string): Promise<Workspace | null>
```

**Update**: `src/lib/desktop-api/index.ts` to export workspace API

---

### 6. Frontend Workspace UI

**Create**: 
- `src/features/workspace/components/WorkspaceList.tsx`
- `src/features/workspace/components/CreateWorkspaceDialog.tsx`
- `src/features/workspace/hooks/useWorkspaces.ts`

**Update Pages**:
- `src/pages/today/TodayPage.tsx`: Add workspace selector + welcome state
- `src/pages/settings/SettingsPage.tsx`: Add workspace management section

**States to implement**:
- Loading state (skeleton/spinner)
- Empty state (no workspaces)
- Error state (API failure)
- Success state (workspace list)

---

### 7. Application Shell Improvements

**Add**:
- Global error boundary in `src/app/providers.tsx`
- Toast notifications setup
- Loading fallback for route transitions

**Create**:
- `src/components/common/LoadingSpinner.tsx`
- `src/components/common/EmptyState.tsx`
- `src/components/common/ErrorState.tsx`

---

### 8. Tests

**Rust Tests**:
- `src-tauri/src/database/migrations.rs`: Test migration execution
- `src-tauri/src/database/repositories/workspace_repository.rs`: Test CRUD operations
- `src-tauri/src/services/workspace_service.rs`: Test validation logic

**Frontend Tests**:
- `src/app/router.test.tsx`: Test route rendering
- `src/features/workspace/WorkspaceList.test.tsx`: Test component rendering
- `src/lib/desktop-api/workspace.test.ts`: Test API mocking

---

## Implementation Order

1. **Backend Foundation** (Day 1)
   - Add workspace domain model
   - Create repository layer
   - Create service layer
   - Refactor settings commands
   - Add workspace commands

2. **Frontend Foundation** (Day 1-2)
   - Add workspace API layer
   - Create common UI components (Loading, Empty, Error)
   - Add global error boundary and toasts
   - Implement workspace hooks

3. **Workspace Feature** (Day 2)
   - Create workspace components
   - Update Today page with workspace selector
   - Add workspace creation flow
   - Add workspace listing

4. **Testing & Verification** (Day 2-3)
   - Write Rust tests
   - Write frontend tests
   - Run full verification
   - Commit changes

---

## Architecture Flow (After Implementation)

```
React Component
  ↓
useWorkspaces hook (TanStack Query)
  ↓
desktopApi.workspace.listWorkspaces()
  ↓
invoke("list_workspaces")
  ↓
Tauri Command (commands/workspace.rs)
  ↓
WorkspaceService.list()
  ↓
WorkspaceRepository.list()
  ↓
SQLx query → SQLite
  ↓
Domain Model (Workspace)
  ↓
JSON response → React
```

---

## Success Criteria

1. ✅ User can create a workspace
2. ✅ Workspace persists after app restart
3. ✅ User can see workspace list
4. ✅ Commands contain no SQL
5. ✅ Services handle business logic
6. ✅ Repositories own database access
7. ✅ All tests pass
8. ✅ Lint, typecheck, clippy pass
9. ✅ Application launches successfully

---

## Restrictions Maintained

- ❌ No AI provider integration
- ❌ No agent runtime
- ❌ No research engine
- ❌ No artifact system
- ❌ No plugin system
- ❌ No market data
- ❌ No portfolio analysis

Only foundation layer completed.