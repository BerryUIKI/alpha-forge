# Agent Protocol v2 — Future Design

**Purpose**: Design the future agent protocol while maintaining compatibility with current Phase 2 implementation.

**Status**: Design Document (Not for Implementation Yet)

---

## 1. Agent Lifecycle

**Current Phase 2**:
```text
Created → Queued → Running → [Waiting For Input] → Completed | Failed | Cancelled
```

**Future Extension**:
```text
Created
    ↓
Planning          (Agent decomposes task into steps)
    ↓
Executing         (Agent runs steps)
    ↓
WaitingForTool    (Agent awaits tool result)
    ↓
WaitingForUser    (Agent needs user input/approval)
    ↓
Reviewing         (Agent reviews own output)
    ↓
Completed
    ↓
Reviewed          (User reviews and accepts/rejects)
```

**State Additions**:
- `Planning`: Task decomposition phase
- `WaitingForTool`: Tool execution pending
- `Reviewing`: Self-evaluation phase
- `Reviewed`: Post-completion user validation

**Implementation Note**: Phase 2 should add `agent_id` field to prepare for these states.

---

## 2. Task Model

### 2.1 Concepts

**Task**:
- Top-level unit of work
- Has defined input and output schema
- Tracked from creation to completion
- Persists in SQLite with all events

**Step** (Future):
- Sub-unit within a task
- Has defined dependency on previous steps
- Can be retried independently
- Enables multi-step reasoning

**Action** (Future):
- Atomic operation within a step
- Maps to a single tool call or AI generation
- Has explicit inputs and outputs

**Tool Call**:
- Invocation of a registered tool
- Logged with input, output, duration
- Subject to permission checks

**Result**:
- Structured output from task
- Validated against schema
- Linked to artifacts if applicable

**Artifact**:
- Interactive visualization of result
- Rendered by plugin system
- Can be persisted or discarded

### 2.2 Data Model

```rust
// Phase 2 Foundation
pub struct Task {
    pub id: Uuid,
    pub status: TaskStatus,
    pub input: TaskInput,
    pub output: Option<TaskOutput>,
    pub error: Option<TaskError>,
}

// Future Extension
pub struct TaskStep {
    pub id: Uuid,
    pub task_id: Uuid,
    pub step_number: u32,
    pub status: StepStatus,
    pub depends_on: Vec<Uuid>,  // Previous step IDs
    pub action: StepAction,
}

pub enum StepAction {
    Generate { prompt: String },
    ToolCall { tool_id: String, input: JsonValue },
    Review { target_step: Uuid },
}
```

---

## 3. Context Model

**Purpose**: Provide agent with relevant information to execute tasks intelligently.

### 3.1 Context Types

**Workspace Context**:
- Current workspace settings
- User preferences
- Available documents
- Active theses

**User Context**:
- User expertise level
- Communication preferences
- Approval thresholds
- Budget limits

**Research Context**:
- Selected documents
- Relevant sources
- Prior research on topic
- Related theses

**Task Context**:
- Task-specific inputs
- Prior task outcomes
- Task templates

**Memory Context** (Future):
- Conversation history
- Learned preferences
- Frequently used patterns

### 3.2 Context Assembly

```rust
pub struct TaskContext {
    pub workspace: WorkspaceContext,
    pub user: UserContext,
    pub research: ResearchContext,
    pub task: TaskInput,
    // Future
    pub memory: Option<MemoryContext>,
}

pub trait ContextAssembler {
    async fn assemble(&self, task_input: &TaskInput) -> TaskContext;
}
```

**Implementation Note**: Phase 2 should implement basic workspace and research context. Memory context can be deferred.

---

## 4. Tool Interface

### 4.1 Tool Definition

```rust
pub trait Tool: Send + Sync {
    // Identity
    fn id(&self) -> &str;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    
    // Schema
    fn input_schema(&self) -> JsonSchema;
    fn output_schema(&self) -> JsonSchema;
    
    // Capabilities
    fn capabilities(&self) -> Vec<ToolCapability>;
    fn permissions_required(&self) -> Vec<Permission>;
    
    // Execution
    async fn execute(&self, input: JsonValue) -> Result<JsonValue, ToolError>;
}

pub enum ToolCapability {
    ReadOnly,      // Does not modify state
    Write,         // Modifies state
    NetworkAccess, // Makes external requests
    FileAccess,    // Reads/writes files
    Destructive,   // Cannot be undone
}

pub enum Permission {
    NetworkRequest { allowlist: Vec<String> },
    FileRead { paths: Vec<PathBuf> },
    FileWrite { paths: Vec<PathBuf> },
    ExecuteCommand { commands: Vec<String> },
}
```

### 4.2 Tool Registry

```rust
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl ToolRegistry {
    pub fn register(&mut self, tool: Arc<dyn Tool>);
    pub fn get(&self, id: &str) -> Option<Arc<dyn Tool>>;
    pub fn list_by_capability(&self, cap: ToolCapability) -> Vec<&Arc<dyn Tool>>;
    pub fn check_permissions(&self, tool_id: &str, grants: &PermissionGrants) -> bool;
}
```

**Implementation Note**: Phase 2 should define the trait and implement 2-3 stub tools for validation.

---

## 5. Human Control

### 5.1 Approval Points

**Automatic Approval** (No user interaction):
- Read-only operations
- Information retrieval
- Document search

**Notification Only** (User informed, no action needed):
- Low-cost API calls
- Non-destructive writes

**Explicit Approval Required**:
- External network requests
- File modifications
- Destructive operations
- High-cost operations (> $0.10)

### 5.2 Approval Workflow

```rust
pub struct ApprovalRequest {
    pub id: Uuid,
    pub task_id: Uuid,
    pub action: String,
    pub risk_level: RiskLevel,
    pub timeout: Duration,
    pub default_on_timeout: DefaultAction,
}

pub enum RiskLevel {
    Low,      // Minimal impact
    Medium,   // Reversible changes
    High,     // Irreversible or costly
}

pub enum DefaultAction {
    Approve,
    Reject,
    AskLater,
}

pub enum ApprovalResponse {
    Approved { notes: Option<String> },
    Rejected { reason: String },
    Modified { alternative_input: JsonValue },
}
```

### 5.3 Interruption

**User can interrupt at any point**:
1. **Pause**: Temporarily stop task execution
2. **Modify**: Change task parameters mid-execution
3. **Cancel**: Abort task completely
4. **Override**: Force a specific action

**Implementation**: Cooperative cancellation via `CancellationToken` checked between steps.

---

## 6. Compatibility with Phase 2

### 6.1 What Phase 2 Must Include

**Required for Future Compatibility**:
- `agent_id` field in Task model
- Structured event types (not just strings)
- Repository pattern for tasks
- Event persistence

**Not Required Yet**:
- Step model
- Context assembler
- Tool registry
- Approval workflow

### 6.2 Migration Path

**Phase 2 → Phase 3**:
1. Add Step model (new table)
2. Implement ToolRegistry
3. Add ContextAssembler
4. Extend state machine

**Phase 3 → Phase 4**:
1. Add ApprovalRequest model
2. Implement approval UI
3. Add MemoryContext

---

## 7. Design Principles

1. **Simplicity First**: Each feature solves a real problem
2. **Backward Compatible**: New states don't break old tasks
3. **Explicit Over Implicit**: Clear contracts, no hidden behavior
4. **Fail-Safe**: Default to safe actions on uncertainty
5. **Observable**: All actions logged, all decisions traceable

---

**Document Version**: 1.0
**Last Updated**: 2026-07-31
**Status**: Design Proposal (Not for Implementation)