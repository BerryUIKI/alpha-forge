# Backend Agent - Frontend Integration Task

## Your Task

You are implementing backend features for Investment OS. Before writing any code, you MUST follow the frontend-backend integration standards to ensure smooth collaboration.

---

## 📖 MUST READ First

**Integration Guide**: `docs/FRONTEND_BACKEND_INTEGRATION.md`

This guide is **MANDATORY** reading. It contains all standards for:
- Tauri command structure
- TypeScript type definitions
- Error handling patterns
- Input validation
- i18n integration
- Testing requirements

---

## ✅ Before Implementing Any Feature

### Step 1: Check Existing Structure

**Backend Files**:
- `src-tauri/src/commands/` - Command implementations
- `src-tauri/src/services/` - Business logic
- `src-tauri/src/error.rs` - Error definitions

**Frontend Files**:
- `apps/desktop/src/types/` - TypeScript types
- `apps/desktop/src/hooks/` - Data fetching hooks
- `apps/desktop/src/lib/i18n/catalogs/` - i18n keys

### Step 2: Define Your Feature

**What are you implementing?**
- Feature name: ___________
- New command or modify existing: ___________
- Affected components: ___________

### Step 3: Follow This Checklist

**Backend Implementation**:
- [ ] Define input/output types (structs)
- [ ] Implement Tauri command with `#[tauri::command]`
- [ ] Add input validation
- [ ] Use stable error codes (INTERNAL, NOT_FOUND, VALIDATION, PERMISSION_DENIED, TIMEOUT)
- [ ] Add unit tests
- [ ] Document the command

**Frontend Types**:
- [ ] Create TypeScript interface matching Rust struct exactly
- [ ] Add JSDoc comments with backend file reference
- [ ] Mark optional fields with `?`
- [ ] Use proper TypeScript types (no `any`)

**i18n Keys**:
- [ ] Add keys to `catalogs/en/*.ts`
- [ ] Add keys to `catalogs/zh-CN/*.ts`
- [ ] Use descriptive key names
- [ ] Include error messages

**Documentation**:
- [ ] Provide example input/output
- [ ] Document error cases
- [ ] Update integration guide if needed

---

## 🔧 Implementation Template

### Backend (Rust)

```rust
// File: src-tauri/src/commands/YOUR_MODULE.rs

use serde::{Deserialize, Serialize};
use tauri::State;
use crate::app::state::AppState;
use crate::error::AppError;

/// Input for YOUR_FEATURE
#[derive(Debug, Deserialize)]
pub struct YourFeatureInput {
    /// Field description
    pub field_name: String,
    
    /// Optional field
    pub optional_field: Option<String>,
}

/// Output from YOUR_FEATURE
#[derive(Debug, Serialize)]
pub struct YourFeatureOutput {
    /// Result field
    pub result_field: String,
}

/// YOUR_FEATURE command
/// 
/// # Errors
/// Returns `AppError` on failure
#[tauri::command]
pub async fn your_feature(
    input: YourFeatureInput,
    state: State<'_, AppState>,
) -> Result<YourFeatureOutput, AppError> {
    // 1. Validate input
    if input.field_name.is_empty() {
        return Err(AppError::validation("Field name is required"));
    }
    
    // 2. Call service layer
    let result = state.your_service.process(input).await?;
    
    // 3. Return output
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_validation() {
        // Add tests
    }
}
```

### Frontend Types (TypeScript)

```typescript
// File: apps/desktop/src/types/yourModule.ts

/**
 * Input for YOUR_FEATURE
 * 
 * Backend: src-tauri/src/commands/your_module.rs
 * Command: your_feature
 */
export interface YourFeatureInput {
  /** Field description (required, max length: 100) */
  fieldName: string;
  
  /** Optional field */
  optionalField?: string;
}

/**
 * Output from YOUR_FEATURE
 */
export interface YourFeatureOutput {
  /** Result field */
  resultField: string;
}
```

### i18n Keys (TypeScript)

```typescript
// File: apps/desktop/src/lib/i18n/catalogs/en/yourModule.ts

export default {
  yourFeature: "Your Feature",
  fieldName: "Field Name",
  fieldNamePlaceholder: "Enter field name",
  fieldNameRequired: "Field name is required",
  failedToProcess: "Failed to process your feature",
};

// File: apps/desktop/src/lib/i18n/catalogs/zh-CN/yourModule.ts

export default {
  yourFeature: "您的功能",
  fieldName: "字段名称",
  fieldNamePlaceholder: "输入字段名称",
  fieldNameRequired: "字段名称为必填项",
  failedToProcess: "处理您的功能失败",
};
```

---

## ⚠️ Critical Requirements

### Type Matching

**Rust and TypeScript types MUST match exactly**:

| Rust Type | TypeScript Type |
|-----------|-----------------|
| `String` | `string` |
| `i32`, `i64` | `number` |
| `bool` | `boolean` |
| `Option<T>` | `T \| undefined` or `T?` |
| `Vec<T>` | `T[]` |
| `HashMap<K, V>` | `Record<K, V>` |

### Error Codes

**Use ONLY these stable error codes**:
- `INTERNAL` - Internal application error
- `NOT_FOUND` - Resource not found
- `VALIDATION` - Input validation failed
- `PERMISSION_DENIED` - Permission denied
- `TIMEOUT` - Operation timeout

```rust
// Example
return Err(AppError::not_found("Workspace not found"));
return Err(AppError::validation("Invalid input"));
return Err(AppError::internal("Database error"));
```

### Input Validation

**Backend MUST validate all inputs**:

```rust
fn validate_input(input: &YourFeatureInput) -> Result<(), AppError> {
    if input.field_name.is_empty() {
        return Err(AppError::validation("Field name is required"));
    }
    if input.field_name.len() > 100 {
        return Err(AppError::validation("Field name too long"));
    }
    Ok(())
}
```

**Frontend should also validate** (for better UX):

```typescript
import { z } from 'zod';

const YourFeatureSchema = z.object({
  fieldName: z.string()
    .min(1, 'Field name is required')
    .max(100, 'Field name too long'),
  optionalField: z.string().optional(),
});
```

---

## 📝 Documentation Requirements

### For Each Command, Provide:

1. **Purpose**: What does this command do?
2. **Input Schema**: All fields with descriptions
3. **Output Schema**: All fields with descriptions
4. **Error Cases**: What errors can occur?
5. **Example Usage**: How to call it?

### Example Documentation:

```markdown
## create_workspace

**Purpose**: Create a new workspace for organizing research.

**Input**:
- `name` (string, required): Workspace name, max 200 characters
- `description` (string, optional): Workspace description

**Output**:
- `id` (string): Unique workspace ID
- `name` (string): Workspace name
- `created_at` (string): ISO 8601 timestamp

**Errors**:
- `VALIDATION`: Name is empty or too long
- `INTERNAL`: Database error

**Example**:
```typescript
const result = await desktopApi.invoke('create_workspace', {
  name: 'My Research',
  description: 'Tesla Q4 analysis',
});
// Result: { id: 'ws_abc123', name: 'My Research', created_at: '2026-08-03T...' }
```
```

---

## 🧪 Testing Requirements

### Backend Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_workspace_success() {
        // Test successful creation
    }
    
    #[test]
    fn test_create_workspace_empty_name() {
        // Test validation error
    }
    
    #[test]
    fn test_create_workspace_long_name() {
        // Test max length validation
    }
}
```

### Frontend Tests

```typescript
describe('createWorkspace', () => {
  it('should create workspace with valid input', async () => {
    const input = { name: 'Test' };
    const result = await createWorkspace(input);
    expect(result.id).toBeDefined();
  });
  
  it('should fail with empty name', async () => {
    const input = { name: '' };
    await expect(createWorkspace(input)).rejects.toThrow();
  });
});
```

---

## 🚀 After Implementation

### Update These Files:

1. **Command Registration**:
   - `src-tauri/src/lib.rs` - Add to `invoke_handler`

2. **Type Exports**:
   - `apps/desktop/src/types/index.ts` - Export new types

3. **i18n Index**:
   - `apps/desktop/src/lib/i18n/catalogs/en/index.ts` - Import new keys
   - `apps/desktop/src/lib/i18n/catalogs/zh-CN/index.ts` - Import new keys

4. **Hooks** (if needed):
   - `apps/desktop/src/hooks/useYourModule.ts` - Create data hooks

### Notify Frontend Team:

**Create a summary including**:
1. New command names
2. Input/output types
3. Error cases
4. Example usage
5. Any breaking changes

---

## ✅ Final Checklist

Before submitting your code, verify:

**Backend**:
- [ ] Command implemented with `#[tauri::command]`
- [ ] Input/output types defined
- [ ] Validation logic added
- [ ] Error handling with stable codes
- [ ] Unit tests written
- [ ] Command registered in `lib.rs`

**Frontend Types**:
- [ ] TypeScript interfaces match Rust types
- [ ] JSDoc comments added
- [ ] Types exported from index

**i18n**:
- [ ] English keys added
- [ ] Chinese keys added
- [ ] Keys imported in index files

**Documentation**:
- [ ] Example usage provided
- [ ] Error cases documented
- [ ] Integration guide updated (if needed)

---

## 📚 References

- **Integration Guide**: `docs/FRONTEND_BACKEND_INTEGRATION.md`
- **Existing Commands**: `src-tauri/src/commands/`
- **Error Definitions**: `src-tauri/src/error.rs`
- **Type Examples**: `apps/desktop/src/types/`
- **i18n Examples**: `apps/desktop/src/lib/i18n/catalogs/`

---

## 💡 Tips

1. **Start Small**: Implement one command at a time
2. **Test Early**: Write tests as you code
3. **Validate Everything**: Never trust input data
4. **Document Clearly**: Future you will thank you
5. **Follow Patterns**: Look at existing commands for reference

---

## 🆘 Need Help?

If you're unsure about anything:
1. Read the integration guide again
2. Check existing command implementations
3. Ask in team chat
4. Create an issue for discussion

---

**Remember**: Good integration starts with clear standards. Follow this guide carefully to ensure smooth collaboration!

*Last Updated: 2026-08-03*
*Version: 1.0*