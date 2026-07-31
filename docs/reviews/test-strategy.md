# AlphaForge Test Strategy

**Purpose**: Define testing philosophy and requirements for the Agent Runtime system.

---

## 1. Testing Philosophy

### 1.1 Core Principles

1. **Test Behavior, Not Implementation**: Focus on what the system does, not how
2. **Deterministic Over Probabilistic**: Mock non-deterministic systems (AI providers)
3. **Fast Feedback**: Tests should run in seconds, not minutes
4. **Isolation**: Tests must not depend on external services or state
5. **Reproducibility**: Same test, same result, every time

### 1.2 Test Pyramid

```text
        ┌─────────────┐
        │  E2E Tests  │  (Few - Critical paths only)
        └─────────────┘
      ┌───────────────────┐
      │ Integration Tests │  (Some - Component interactions)
      └───────────────────┘
    ┌─────────────────────────┐
    │    Application Tests    │  (Many - Service/command tests)
    └─────────────────────────┘
  ┌─────────────────────────────┐
  │       Unit Tests            │  (Most - Domain logic, utilities)
  └─────────────────────────────┘
```

**Target Distribution**:
- Unit Tests: 60%
- Application Tests: 25%
- Integration Tests: 10%
- E2E Tests: 5%

---

## 2. Testing Layers

### 2.1 Unit Tests

**Scope**: Single function, struct, or module in isolation

**Rust Unit Tests**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_status_transition_valid() {
        let task = Task::new(TaskStatus::Queued);
        assert!(task.transition_to(TaskStatus::Running).is_ok());
    }

    #[test]
    fn test_task_status_transition_invalid() {
        let task = Task::new(TaskStatus::Completed);
        assert!(task.transition_to(TaskStatus::Running).is_err());
    }
}
```

**What to Test**:
- Domain model business rules
- State machine transitions
- Input validation logic
- Error mapping
- Utility functions

**What NOT to Test**:
- Framework behavior (Tauri, SQLx)
- External library functionality
- Trivial getters/setters

### 2.2 Application Tests

**Scope**: Service layer with mocked repositories

**Pattern**:
```rust
#[tokio::test]
async fn test_task_service_creates_task() {
    let mock_repo = MockTaskRepository::new();
    let service = TaskService::new(mock_repo);
    
    let input = TaskInput::new("Test task");
    let result = service.create_task(input).await;
    
    assert!(result.is_ok());
    let task = result.unwrap();
    assert_eq!(task.status, TaskStatus::Queued);
}
```

**What to Test**:
- Service business logic
- Command validation
- Service-repository interactions
- Error handling at service boundary

### 2.3 Integration Tests

**Scope**: Multiple layers with real infrastructure (SQLite in-memory)

**Pattern**:
```rust
#[tokio::test]
async fn test_task_persistence_flow() {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    run_migrations(&pool).await.unwrap();
    
    let repo = TaskRepository::new(pool.clone());
    let service = TaskService::new(repo);
    
    // Create task
    let task = service.create_task(TaskInput::new("Test")).await.unwrap();
    
    // Retrieve task
    let retrieved = repo.find_by_id(task.id).await.unwrap();
    assert_eq!(retrieved.unwrap().id, task.id);
}
```

**What to Test**:
- Repository implementations
- Database queries
- Migration scripts
- Transaction boundaries

### 2.4 End-to-End Tests

**Scope**: Full application flow from UI to backend

**Pattern** (using Tauri test framework):
```rust
#[tauri::test]
async fn test_task_creation_e2e() {
    let app = launch_app().await.unwrap();
    
    // Simulate user input
    app.invoke("create_task", json!({ "input": "Test task" })).await.unwrap();
    
    // Verify task created
    let tasks: Vec<Task> = app.invoke("list_tasks", json!({})).await.unwrap();
    assert_eq!(tasks.len(), 1);
}
```

**What to Test**:
- Critical user flows
- IPC command execution
- Event streaming to UI
- UI state updates

**Priority Flows**:
1. Task creation → execution → completion
2. Task cancellation
3. Error handling and display

---

## 3. Rust Testing

### 3.1 Domain Model Tests

```rust
#[cfg(test)]
mod task_tests {
    use super::*;

    #[test]
    fn new_task_starts_in_created_state() {
        let task = Task::new(Uuid::nil(), "Test input".to_string());
        assert_eq!(task.status, TaskStatus::Created);
    }

    #[test]
    fn task_can_transition_from_created_to_queued() {
        let mut task = Task::new(Uuid::nil(), "Test".to_string());
        assert!(task.transition(TaskStatus::Queued).is_ok());
    }

    #[test]
    fn task_cannot_transition_from_completed_to_running() {
        let mut task = Task::new(Uuid::nil(), "Test".to_string());
        task.status = TaskStatus::Completed;
        assert!(task.transition(TaskStatus::Running).is_err());
    }

    #[test]
    fn task_records_creation_timestamp() {
        let task = Task::new(Uuid::nil(), "Test".to_string());
        assert!(task.created_at <= Utc::now());
    }
}
```

### 3.2 Service Tests

```rust
#[cfg(test)]
mod task_service_tests {
    use super::*;
    use mockall::mock;

    mock! {
        pub TaskRepository {
            async fn create(&self, task: NewTask) -> Result<Task, DatabaseError>;
            async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, DatabaseError>;
            async fn update_status(&self, id: Uuid, status: TaskStatus) -> Result<(), DatabaseError>;
        }
    }

    #[tokio::test]
    async fn create_task_persists_to_repository() {
        let mut mock_repo = MockTaskRepository::new();
        mock_repo.expect_create()
            .returning(|task| Ok(Task { id: Uuid::nil(), ..Default::default() }));
        
        let service = TaskService::new(Arc::new(mock_repo));
        let result = service.create_task(TaskInput::new("Test")).await;
        
        assert!(result.is_ok());
    }
}
```

### 3.3 Repository Tests

```rust
#[cfg(test)]
mod task_repository_tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect(":memory:").await.unwrap();
        run_migrations(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn create_and_retrieve_task() {
        let pool = setup_test_db().await;
        let repo = TaskRepository::new(pool);
        
        let new_task = NewTask { input: "Test".to_string() };
        let created = repo.create(new_task).await.unwrap();
        
        let retrieved = repo.find_by_id(created.id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().input, "Test");
    }

    #[tokio::test]
    async fn update_task_status() {
        let pool = setup_test_db().await;
        let repo = TaskRepository::new(pool);
        
        let task = repo.create(NewTask { input: "Test".into() }).await.unwrap();
        
        repo.update_status(task.id, TaskStatus::Running).await.unwrap();
        
        let updated = repo.find_by_id(task.id).await.unwrap().unwrap();
        assert_eq!(updated.status, TaskStatus::Running);
    }
}
```

### 3.4 Runtime Execution Tests

```rust
#[cfg(test)]
mod runtime_tests {
    use super::*;

    #[tokio::test]
    async fn runtime_executes_task_async() {
        let provider = MockProvider::new();
        let runtime = AgentRuntime::new(provider);
        
        let handle = runtime.spawn(TaskInput::new("Test")).await.unwrap();
        
        // Should return immediately
        assert!(handle.is_active());
        
        // Wait for completion
        let result = handle.await_completion().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn runtime_cancels_task_on_request() {
        let provider = MockProvider::new_with_delay(Duration::from_secs(10));
        let runtime = AgentRuntime::new(provider);
        
        let handle = runtime.spawn(TaskInput::new("Test")).await.unwrap();
        
        // Cancel immediately
        handle.cancel();
        
        let result = handle.await_completion().await;
        assert!(matches!(result, Err(AgentError::Cancelled)));
    }
}
```

---

## 4. Frontend Testing

### 4.1 Component Tests

```typescript
import { render, screen } from '@testing-library/react';
import { TaskStatus } from './TaskStatus';

describe('TaskStatus', () => {
  it('displays queued status', () => {
    render(<TaskStatus status="queued" />);
    expect(screen.getByText('Queued')).toBeInTheDocument();
  });

  it('shows progress bar when running', () => {
    render(<TaskStatus status="running" progress={50} />);
    expect(screen.getByRole('progressbar')).toHaveValue(50);
  });

  it('displays error message when failed', () => {
    render(<TaskStatus status="failed" error="API error" />);
    expect(screen.getByText('API error')).toBeInTheDocument();
  });
});
```

### 4.2 Hook Tests

```typescript
import { renderHook, waitFor } from '@testing-library/react';
import { useTask } from './useTask';
import { desktopApi } from '../lib/desktop-api';

jest.mock('../lib/desktop-api');

describe('useTask', () => {
  it('creates task and returns ID', async () => {
    const mockCreateTask = jest.fn().mockResolvedValue('task-123');
    desktopApi.agent.createTask = mockCreateTask;
    
    const { result } = renderHook(() => useTask());
    
    await result.current.createTask('Test input');
    
    expect(mockCreateTask).toHaveBeenCalledWith({ input: 'Test input' });
    expect(result.current.taskId).toBe('task-123');
  });
});
```

### 4.3 API Layer Tests

```typescript
import { desktopApi } from '../lib/desktop-api';
import { invoke } from '@tauri-apps/api/tauri';

jest.mock('@tauri-apps/api/tauri');

describe('desktopApi.agent', () => {
  it('calls create_task command', async () => {
    (invoke as jest.Mock).mockResolvedValue({ id: 'task-123', status: 'queued' });
    
    const result = await desktopApi.agent.createTask({ input: 'Test' });
    
    expect(invoke).toHaveBeenCalledWith('create_task', { input: 'Test' });
    expect(result.id).toBe('task-123');
  });
});
```

### 4.4 User Flow Tests

```typescript
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { App } from '../App';
import { desktopApi } from '../lib/desktop-api';

jest.mock('../lib/desktop-api');

describe('Task creation flow', () => {
  it('allows user to create and monitor task', async () => {
    // Setup mocks
    desktopApi.agent.createTask = jest.fn().mockResolvedValue({ id: 'task-1' });
    desktopApi.agent.listTasks = jest.fn().mockResolvedValue([]);
    
    render(<App />);
    
    // Enter task input
    const input = screen.getByPlaceholderText('Enter research task');
    fireEvent.change(input, { target: { value: 'Analyze NVIDIA' } });
    
    // Submit task
    fireEvent.click(screen.getByText('Create Task'));
    
    // Verify API called
    await waitFor(() => {
      expect(desktopApi.agent.createTask).toHaveBeenCalledWith({
        input: 'Analyze NVIDIA'
      });
    });
  });
});
```

---

## 5. Agent Testing

### 5.1 Task Lifecycle Tests

```rust
#[cfg(test)]
mod task_lifecycle_tests {
    use super::*;

    #[test]
    fn task_transitions_through_all_states() {
        let mut task = Task::new(Uuid::nil(), "Test".into());
        
        // Created -> Queued
        assert!(task.transition(TaskStatus::Queued).is_ok());
        assert_eq!(task.status, TaskStatus::Queued);
        
        // Queued -> Running
        assert!(task.transition(TaskStatus::Running).is_ok());
        assert_eq!(task.status, TaskStatus::Running);
        
        // Running -> Completed
        assert!(task.transition(TaskStatus::Completed).is_ok());
        assert_eq!(task.status, TaskStatus::Completed);
    }

    #[test]
    fn task_can_be_cancelled_from_any_non_terminal_state() {
        for status in &[TaskStatus::Created, TaskStatus::Queued, TaskStatus::Running] {
            let mut task = Task::new(Uuid::nil(), "Test".into());
            task.status = status.clone();
            assert!(task.transition(TaskStatus::Cancelled).is_ok());
        }
    }
}
```

### 5.2 Event Sequence Tests

```rust
#[tokio::test]
async fn task_emits_events_in_correct_order() {
    let runtime = AgentRuntime::new(MockProvider::new());
    let events = Arc::new(Mutex::new(Vec::new()));
    
    // Subscribe to events
    let events_clone = events.clone();
    runtime.on_event(move |event| {
        events_clone.lock().unwrap().push(event);
    });
    
    // Execute task
    let handle = runtime.spawn(TaskInput::new("Test")).await.unwrap();
    handle.await_completion().await.unwrap();
    
    // Verify event order
    let events = events.lock().unwrap();
    assert!(events.len() > 0);
    assert_eq!(events[0].event_type, "task.started");
    assert!(events.iter().any(|e| e.event_type == "task.completed"));
}
```

### 5.3 Failure Recovery Tests

```rust
#[tokio::test]
async fn task_retries_on_transient_failure() {
    let provider = MockProvider::new()
        .fail_times(2)
        .then_succeed();
    
    let runtime = AgentRuntime::new(provider);
    runtime.config.max_retries = 3;
    
    let result = runtime.execute(TaskInput::new("Test")).await;
    
    assert!(result.is_ok()); // Should succeed after retries
}

#[tokio::test]
async fn task_fails_after_max_retries() {
    let provider = MockProvider::new()
        .always_fail();
    
    let runtime = AgentRuntime::new(provider);
    runtime.config.max_retries = 2;
    
    let result = runtime.execute(TaskInput::new("Test")).await;
    
    assert!(result.is_err());
}
```

### 5.4 Deterministic Runtime Tests

```rust
#[tokio::test]
async fn deterministic_produces_same_output_for_same_input() {
    let provider = DeterministicMockProvider::new();
    let runtime = AgentRuntime::new(provider);
    
    let input = TaskInput::new("Analyze AAPL");
    
    let result1 = runtime.execute(input.clone()).await.unwrap();
    let result2 = runtime.execute(input).await.unwrap();
    
    assert_eq!(result1, result2);
}
```

---

## 6. What NOT to Test

### 6.1 External AI Model Behavior

**Why**: Non-deterministic, provider-specific, not your code

**What to Test Instead**:
- Provider interface contract
- Input validation before provider call
- Output validation after provider call

### 6.2 Market Predictions

**Why**: Financial markets are inherently unpredictable

**What to Test Instead**:
- Data transformation logic
- Risk calculation algorithms
- Thesis state transitions

### 6.3 Investment Outcomes

**Why**: Dependent on external factors beyond control

**What to Test Instead**:
- Portfolio calculation accuracy
- Position tracking correctness
- Thesis evidence linking

---

## 7. Test Infrastructure

### 7.1 Test Utilities

```rust
pub mod test_utils {
    use super::*;
    
    pub fn mock_provider() -> MockProvider {
        MockProvider::new().with_default_response()
    }
    
    pub fn in_memory_db() -> SqlitePool {
        // Setup in-memory SQLite
    }
    
    pub fn sample_task() -> Task {
        Task {
            id: Uuid::nil(),
            status: TaskStatus::Queued,
            input: "Test task".into(),
            ..Default::default()
        }
    }
}
```

### 7.2 Mock Implementations

```rust
pub struct MockProvider {
    responses: VecDeque<Result<ProviderResponse, ProviderError>>,
}

impl MockProvider {
    pub fn new() -> Self {
        Self { responses: VecDeque::new() }
    }
    
    pub fn with_response(mut self, response: ProviderResponse) -> Self {
        self.responses.push_back(Ok(response));
        self
    }
    
    pub fn with_error(mut self, error: ProviderError) -> Self {
        self.responses.push_back(Err(error));
        self
    }
}

impl Provider for MockProvider {
    async fn execute(&mut self, _request: Request) -> Result<Response, ProviderError> {
        self.responses.pop_front().unwrap()
    }
}
```

---

## 8. Continuous Integration

### 8.1 Required Checks

Every PR must pass:
- `cargo test --workspace`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo fmt --check`
- `pnpm test`
- `pnpm typecheck`
- `pnpm lint`

### 8.2 Coverage Target

**Minimum**: 70% line coverage
**Target**: 80% line coverage

**Excluded from Coverage**:
- Test utilities
- Mock implementations
- Tauri glue code

---

**Document Version**: 1.0
**Last Updated**: 2026-07-31