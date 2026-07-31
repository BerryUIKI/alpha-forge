# Phase 2 — Agent Runtime Architecture Review

**Review Date**: 2026-07-31
**Reviewer**: Architecture Review Agent
**Phase Status**: In Active Development

---

## Executive Summary

Phase 2 implements the Agent Runtime Foundation. This review evaluates the architectural design, identifies scalability concerns, and provides recommendations for current implementation and future evolution.

**Key Findings**:
- ✅ Clean separation between domain, runtime, and persistence layers
- ✅ Event-driven architecture enables decoupled UI updates
- ⚠️ Tool system architecture needs early definition
- ⚠️ Provider abstraction should be finalized before implementation
- ⚠️ Context management strategy requires clarification

---

## 1. Current Design Review

### 1.1 AgentTask Model

**Current Design** (from DATA_MODEL.md):

```
Agent Task
├── Attributes
│   ├── Status: queued | running | waiting_for_input | completed | failed | cancelled
│   ├── Input text
│   ├── Output JSON
│   └── Error message
├── Relationships
│   ├── Produces artifacts
│   └── Generates task events
└── Lifecycle: Queued → Running → [Waiting For Input] → Completed | Failed | Cancelled
```

**Evaluation**:

| Aspect | Assessment | Comments |
|--------|------------|----------|
| State Machine | ✅ Correct | Clear states with defined transitions |
| Concurrency Support | ✅ Designed | Queue-based execution with limits |
| Persistence | ✅ Planned | SQLite-based task history |
| Resumption | ⚠️ Needs Design | Restart recovery not fully specified |

**Strengths**:
- Clear state machine with explicit transitions
- Event-based progress streaming
- Concurrency limits prevent resource exhaustion

**Concerns**:
- **Missing**: Task priority levels (FIFO only)
- **Missing**: Task dependencies (sequential execution)
- **Missing**: Task templates for common operations
- **Unclear**: How to handle partial results on failure

**Recommendation**: Add `priority` field (low/normal/high) and support for resuming interrupted tasks after application restart.

---

### 1.2 Task Lifecycle

**Current Flow** (from AGENT_PROTOCOL.md):

```text
Created → Queued → Running → Waiting For Input → Completed
                          → Failed
                          → Cancelled
```

**Transition Rules**:
- Created → Queued: User submits task
- Queued → Running: Execution slot available
- Running → Waiting For Input: Agent needs clarification
- Waiting For Input → Running: User provides input
- Running → Completed: Success with structured output
- Running → Failed: Unrecoverable error
- Any → Cancelled: User aborts

**Evaluation**:

| Aspect | Assessment | Comments |
|--------|------------|----------|
| State Coverage | ✅ Complete | Covers all execution phases |
| Transition Guards | ⚠️ Needs Definition | Missing explicit guards per transition |
| Idempotency | ⚠️ Not Addressed | Duplicate task submission not handled |
| Timeout Handling | ✅ Designed | Configurable timeout per task |

**Strengths**:
- Covers human interaction (`Waiting For Input`)
- Supports cooperative cancellation
- Clear terminal states

**Concerns**:
- **Missing**: What happens when task times out during `Waiting For Input`?
- **Missing**: State recovery after application crash mid-execution
- **Missing**: Maximum retry count for transient failures

**Recommendation**: Define explicit timeout behavior for each state and implement task checkpointing for crash recovery.

---

### 1.3 Runtime Abstraction

**Current Architecture** (from ARCHITECTURE.md):

```text
Rust Backend
├── agent/
│   ├── runtime/          (stub)
│   ├── executor/         (stub)
│   └── scheduler/        (stub)
├── providers/
│   └── openai/           (stub)
└── database/
    └── repositories/
        └── task_repo.rs  (stub)
```

**Evaluation**:

| Aspect | Assessment | Comments |
|--------|------------|----------|
| Layer Separation | ✅ Clear | Domain / Runtime / Persistence |
| Dependency Injection | ⚠️ Needs Design | Runtime <-> Provider coupling unclear |
| Testability | ⚠️ Not Addressed | Mock providers not specified |
| Extensibility | ⚠️ Partial | Provider abstraction not finalized |

**Strengths**:
- Clean module boundaries
- Domain layer isolated from infrastructure

**Concerns**:
- **Hidden Coupling**: Direct provider calls from runtime?
- **Missing**: Provider trait definition
- **Missing**: Runtime configuration injection pattern
- **Unclear**: How to swap providers at runtime

**Recommendation**: Define a `Provider` trait that abstracts AI provider calls. Runtime should depend on `dyn Provider`, not concrete implementations.

```rust
// Recommended pattern
pub trait Provider {
    async fn execute(&self, request: Request) -> Result<Response, ProviderError>;
    fn name(&self) -> &str;
    fn capabilities(&self) -> ProviderCapabilities;
}

pub struct AgentRuntime<P: Provider> {
    provider: P,
    config: RuntimeConfig,
}
```

---

### 1.4 Service Boundary

**Current Design**:

```text
Commands (Tauri)
    ↓
Services
    ↓
Repositories
    ↓
SQLite
```

**Evaluation**:

| Aspect | Assessment | Comments |
|--------|------------|----------|
| Layer Separation | ✅ Correct | Commands → Services → Repositories |
| Transaction Scope | ⚠️ Not Addressed | Where do transactions begin/end? |
| Error Mapping | ⚠️ Partial | AppError exists, but mapping strategy unclear |
| Validation Location | ⚠️ Not Defined | Input validation at which layer? |

**Strengths**:
- Follows layered architecture
- Services contain business logic

**Concerns**:
- **Missing**: Transaction boundary definition
- **Missing**: Where does input validation happen? (Command? Service?)
- **Missing**: Service interface contracts (traits vs concrete types)
- **Unclear**: How are service errors mapped to user-facing messages?

**Recommendation**: Define explicit validation layers:
1. **Command Layer**: Structural validation (types, required fields)
2. **Service Layer**: Business validation (state constraints, permissions)
3. **Repository Layer**: Persistence validation (foreign keys, constraints)

---

### 1.5 Repository Boundary

**Current Design**:

```rust
// Stub from docs
pub struct TaskRepository {
    pool: SqlitePool,
}

impl TaskRepository {
    pub async fn create(&self, task: NewTask) -> Result<Task, DatabaseError>;
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<Task>, DatabaseError>;
    pub async fn update_status(&self, id: Uuid, status: TaskStatus) -> Result<(), DatabaseError>;
}
```

**Evaluation**:

| Aspect | Assessment | Comments |
|--------|------------|----------|
| CRUD Coverage | ✅ Adequate | Standard operations defined |
| Query Patterns | ⚠️ Needs Expansion | Filter/pagination not addressed |
| Transaction Support | ⚠️ Missing | No transaction abstraction |
| Domain Mapping | ⚠️ Not Addressed | Row → Domain mapping strategy unclear |

**Strengths**:
- Simple, focused repository interface
- Async-first design

**Concerns**:
- **Missing**: Complex query support (by status, date range, etc.)
- **Missing**: Batch operations (create multiple events)
- **Missing**: Repository-level transactions
- **Unclear**: How to handle optimistic locking?

**Recommendation**: Add query builders for complex queries and define a `UnitOfWork` pattern for multi-repository transactions.

---

### 1.6 Event Architecture

**Current Design** (from AGENT_PROTOCOL.md):

```text
task.started       { task_id }
task.thinking      { task_id, message }
task.tool_call     { task_id, tool_name, input }
task.tool_result   { task_id, tool_name, output }
task.streaming     { task_id, chunk }
task.progress      { task_id, percent, message }
task.completed     { task_id, output }
task.failed        { task_id, error_code, error_message }
task.cancelled     { task_id }
```

**Evaluation**:

| Aspect | Assessment | Comments |
|--------|------------|----------|
| Coverage | ✅ Complete | Covers all execution phases |
| Type Safety | ✅ Planned | Typed event payloads |
| Persistence | ✅ Designed | Append-only event log |
| UI Integration | ⚠️ Needs Detail | React subscription strategy unclear |

**Strengths**:
- Comprehensive event coverage
- Typed payloads enable safe handling
- Append-only design preserves history

**Concerns**:
- **Missing**: Event ordering guarantees
- **Missing**: Event replay strategy (for debugging)
- **Missing**: Backpressure handling (fast event production vs slow UI)
- **Unclear**: How does React subscribe to specific events?

**Recommendation**: Define event delivery guarantees (at-least-once) and implement event buffering in React for offline resilience.

---

## 2. Future Scalability Review

### 2.1 Real AI Providers

**Current Status**: Stub implementation

**Future Requirements**:
- OpenAI API integration
- Anthropic Claude integration
- Local LLM support (Ollama, LM Studio)
- Custom provider registration

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Provider Trait | ⚠️ Not Defined | Need abstraction layer |
| Provider Config | ⚠️ Not Designed | Need runtime configuration |
| Provider Failover | ❌ Not Planned | Need fallback strategy |
| Provider Selection | ⚠️ Partial | Need routing logic |

**Recommendation**: Design a `ProviderRegistry` that manages available providers and routes requests based on capability requirements.

---

### 2.2 Tool Calling

**Current Status**: Conceptual (AGENT_PROTOCOL.md mentions planned tools)

**Future Requirements**:
- Tool registration system
- Tool schema validation
- Tool execution sandbox
- Tool result handling

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Tool Registry | ❌ Not Designed | Need central registration |
| Tool Interface | ⚠️ Conceptual | Need trait definition |
| Permission Model | ⚠️ Mentioned | Need enforcement layer |
| Tool Discovery | ❌ Not Addressed | Need agent tool awareness |

**Recommendation**: Design tool system early. Tools should:
1. Declare capabilities via schema
2. Require explicit permission grants
3. Return structured results (not raw text)
4. Be independently testable

---

### 2.3 Streaming Responses

**Current Status**: Planned (`task.streaming` event)

**Future Requirements**:
- Real-time token streaming
- Partial result display
- Stream interruption handling
- UI update throttling

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Event Streaming | ✅ Designed | `task.streaming` event exists |
| Backpressure | ❌ Not Addressed | Need buffer/throttle strategy |
| UI Throttling | ❌ Not Designed | Need React update optimization |
| Cancellation During Stream | ⚠️ Partial | Need stream abort logic |

**Recommendation**: Implement event throttling in the runtime (e.g., max 10 UI updates per second) and use React's `useDeferredValue` for rendering optimization.

---

### 2.4 Human Approval

**Current Status**: Mentioned in AGENT_PROTOCOL.md

**Future Requirements**:
- Approval request events
- User approval UI
- Approval timeout handling
- Approval history logging

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Approval Events | ❌ Not Designed | Need new event type |
| Approval UI | ❌ Not Designed | Need React component |
| Timeout Handling | ❌ Not Addressed | Need auto-reject/approve |
| Audit Trail | ✅ Partial | Event log covers history |

**Recommendation**: Add `task.approval_required` event with:
- Action description
- Risk level
- Timeout
- Default action on timeout

---

### 2.5 Multi-Step Reasoning

**Current Status**: Not addressed (MVP scope)

**Future Requirements**:
- Step-by-step task execution
- Step dependencies
- Intermediate state persistence
- Step rollback on failure

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Step Model | ❌ Not Designed | Need step abstraction |
| Dependency Graph | ❌ Not Addressed | Need DAG execution |
| State Management | ⚠️ Partial | Task state exists, step state needed |
| Rollback | ❌ Not Designed | Need compensating actions |

**Recommendation**: For MVP, defer multi-step reasoning. Design single-step tasks first, then extend to multi-step in Phase 5+.

---

### 2.6 Multiple Agents

**Current Status**: Not in MVP scope

**Future Requirements**:
- Agent identity
- Agent specialization
- Inter-agent communication
- Agent coordination

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Agent Identity | ❌ Not Needed Yet | Single agent in MVP |
| Agent Registry | ❌ Not Designed | Future requirement |
| Communication Protocol | ❌ Not Designed | Future requirement |
| Task Routing | ❌ Not Designed | Future requirement |

**Recommendation**: Design for single agent first. Add `agent_id` field to tasks early to prepare for multi-agent support.

---

### 2.7 Long-Running Tasks

**Current Status**: Timeout designed (5 min default)

**Future Requirements**:
- Tasks running hours/days
- Progress persistence
- Checkpoint/restart
- Background execution (app closed)

**Scalability Assessment**:

| Requirement | Current Readiness | Gap |
|-------------|-------------------|-----|
| Timeout Handling | ✅ Designed | Configurable timeout |
| Progress Persistence | ⚠️ Partial | Events logged, but not checkpoints |
| Checkpoint/Restart | ❌ Not Designed | Need serialization strategy |
| Background Execution | ❌ Not Possible | Tauri requires app running |

**Recommendation**: For MVP, accept that tasks must complete within app session. Add checkpoint system in Phase 4+ for long-running operations.

---

## 3. Risks

### 3.1 Technical Debt Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| **Provider Coupling** | High | High | Define `Provider` trait early |
| **Missing Abstractions** | Medium | Medium | Document deferred features clearly |
| **Hardcoded Configs** | Low | Medium | Use configuration structs |
| **Incomplete Error Handling** | High | Medium | Define error taxonomy upfront |

**Critical Risk**: Provider coupling. If runtime directly uses OpenAI SDK, swapping providers becomes expensive refactoring.

---

### 3.2 Over-Engineering Risks

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| **Premature Multi-Agent** | High | Low | Stick to MVP scope |
| **Complex Tool System** | Medium | Medium | Start with 3-5 built-in tools |
| **Enterprise Event Sourcing** | Medium | Low | Simple event log sufficient for MVP |
| **Distributed Task Queue** | High | Low | In-process Tokio sufficient |

**Recommendation**: Resist enterprise patterns. AlphaForge is a desktop app, not a distributed system.

---

### 3.3 Missing Abstractions

| Abstraction | Current Status | Impact if Missing |
|-------------|----------------|-------------------|
| **Provider Trait** | Not defined | High - Locks to single provider |
| **Tool Interface** | Conceptual | High - Limits extensibility |
| **Context Model** | Not defined | Medium - Limits task intelligence |
| **Approval Workflow** | Mentioned | Medium - Limits safety controls |

**Priority**: Define Provider and Tool abstractions in Phase 2. Context and Approval can be Phase 3-4.

---

### 3.4 Security Concerns

| Concern | Severity | Likelihood | Mitigation |
|---------|----------|------------|------------|
| **Unrestricted Tool Execution** | High | High | Permission model + allowlist |
| **Credential Leakage** | High | Medium | OS keychain, no plaintext storage |
| **Unvalidated AI Output** | Medium | High | Schema validation, sanitization |
| **Artifact Injection** | High | Medium | Content Security Policy, input sanitization |

**Critical Concern**: Tool execution must have permission boundaries. Agents should NOT have unrestricted filesystem/network access.

---

## 4. Recommendations

### 4.1 Short-Term (Phase 2)

**Must Have**:
1. Define `Provider` trait before implementing OpenAI integration
2. Define `Tool` trait for future tool system
3. Implement structured error taxonomy (`AgentError` enum)
4. Add `agent_id` field to tasks (prepare for multi-agent)
5. Document transaction boundaries for service methods

**Should Have**:
1. Provider configuration system (enable/disable, credentials)
2. Basic tool registry (even if only stub tools)
3. Event ordering guarantees (sequential event processing)
4. Task templates for common operations

**Can Defer**:
1. Multi-step reasoning
2. Agent-to-agent communication
3. Long-running task checkpointing

---

### 4.2 Medium-Term (Phase 3-4)

**Must Have**:
1. Tool permission model and enforcement
2. Human approval workflow
3. Provider failover logic
4. Context management system

**Should Have**:
1. Multi-provider support (OpenAI + Anthropic)
2. Task dependency graph (sequential execution)
3. Enhanced query capabilities in repositories
4. Event replay for debugging

---

### 4.3 Intentionally Deferred

**Not in Current Roadmap**:
1. Distributed task execution (requires backend infrastructure)
2. Custom agent programming language
3. Plugin marketplace
4. Real-time collaboration
5. Mobile support

**Reason**: AlphaForge is a **single-user desktop application**. Enterprise features add complexity without proportional value for the target use case.

---

## 5. Architecture Decision Records

### ADR-0004: Provider Abstraction

**Status**: Proposed

**Context**: Phase 2 needs AI provider integration. Direct coupling to OpenAI SDK creates maintenance burden and limits future provider options.

**Decision**: Introduce `Provider` trait that abstracts:
- Request execution
- Streaming support
- Capability declaration
- Error handling

**Consequences**:
- ✅ Enables provider swapping
- ✅ Simplifies testing (mock providers)
- ⚠️ Adds one layer of abstraction
- ⚠️ Requires careful trait design upfront

---

### ADR-0005: Tool System Scope

**Status**: Proposed

**Context**: Agents need tools to interact with system. Full tool system is complex. MVP needs minimal tool support.

**Decision**: Defer full tool system to Phase 3. In Phase 2, implement:
- Tool trait definition
- 2-3 built-in tools (document search, note creation)
- Simple permission model (allowlist)

**Consequences**:
- ✅ Reduces Phase 2 scope
- ✅ Validates tool architecture early
- ⚠️ Limited agent capabilities in MVP
- ⚠️ External tools not supported yet

---

### ADR-0006: Event Delivery Guarantees

**Status**: Proposed

**Context**: Agent tasks emit events. UI must receive them reliably. Lost events mean lost progress visibility.

**Decision**: Implement **at-least-once** delivery:
- Events are persisted before emission
- UI maintains sequence numbers
- Missing events trigger reconnection/fetch

**Consequences**:
- ✅ No lost events
- ✅ Supports offline resilience
- ⚠️ Duplicate events possible (UI must dedupe)
- ⚠️ Additional persistence overhead

---

## 6. Conclusion

Phase 2 design demonstrates **solid fundamentals**:
- Clear state machine for tasks
- Event-driven architecture
- Layered separation of concerns

**Critical Actions**:
1. Define Provider trait **before** OpenAI integration
2. Define Tool trait **before** implementing tools
3. Document transaction boundaries
4. Implement error taxonomy

**Deferred Features**: Justified deferral of multi-agent, long-running tasks, and complex tool systems. MVP scope is appropriate.

**Overall Assessment**: **Ready for implementation** with minor design clarifications needed.

---

**Next Steps**:
1. Implement Provider trait
2. Implement Tool trait
3. Create ADR-0004, ADR-0005, ADR-0006
4. Begin Phase 2 implementation

---

**Document Version**: 1.0
**Last Updated**: 2026-07-31