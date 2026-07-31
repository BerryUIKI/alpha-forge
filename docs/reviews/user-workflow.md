# AlphaForge User Workflow

**Purpose**: Define the user experience and workflows for AlphaForge as a research workspace (not a chatbot).

---

## 1. Core Philosophy

**AlphaForge is NOT a Chatbot**.

Chatbots are conversation-first interfaces. AlphaForge is a **workspace-first** interface.

**Key Differences**:

| Aspect | Chatbot | AlphaForge Workspace |
|---------|---------|----------------------|
| Primary Interface | Chat window | Workspace dashboard |
| Output Format | Chat messages | Structured artifacts |
| Persistence | Chat history | Research documents |
| Organization | Linear conversation | Projects & theses |
| Reusability | Copy-paste | Save, link, re-use |

**Design Principle**: Users work with **documents, theses, and artifacts**, not just chat messages.

---

## 2. Daily Research Workflow

### 2.1 Typical Day Workflow

```text
Open Workspace
    ↓
Review Recent Updates
    ├── New research documents
    ├── Thesis status changes
    └── Agent task completions
    ↓
Create Research Task (if needed)
    ↓
Agent Executes Research
    ├── Progress streaming
    ├── Evidence gathering
    └── Artifact generation
    ↓
Review Agent Output
    ├── Examine artifact
    ├── Validate claims
    └── Save to thesis
    ↓
Update Thesis (if applicable)
    ↓
Close or Continue
```

### 2.2 Workspace Entry

**When User Opens AlphaForge**:

1. **Show Today View**:
   - Recent tasks (completed, running)
   - Thesis updates
   - New research documents
   - Portfolio alerts (if configured)

2. **Quick Actions**:
   - "New Research Task"
   - "Continue Previous Task"
   - "Import Document"

3. **Status Indicators**:
   - Running tasks (with progress)
   - Pending approvals
   - Review-required artifacts

---

## 3. Research Workflow

### 3.1 Information → Knowledge → Thesis → Decision

```text
Information Gathering
    ├── Import documents (PDFs, web)
    ├── Agent searches sources
    └── Extract key points
    ↓
Knowledge Organization
    ├── Link to existing research
    ├── Identify themes
    └── Structure evidence
    ↓
Thesis Development
    ├── Draft thesis statement
    ├── Attach supporting evidence
    ├── Identify risks
    └── Set confidence level
    ↓
Decision & Action
    ├── Review thesis
    ├── Make investment decision
    └── Record outcome
    ↓
Validation
    ├── Track thesis over time
    ├── Record outcomes
    └── Update confidence
    ↓
Review
    ├── What worked?
    ├── What didn't?
    └── Improve process
```

### 3.2 Information Gathering

**Manual Path**:
1. User imports document (PDF, web URL)
2. System extracts text, metadata
3. Document stored in workspace
4. User can annotate, link to theses

**Agent-Assisted Path**:
1. User: "Research NVIDIA's data center revenue"
2. Agent searches configured sources
3. Agent gathers relevant documents
4. Agent summarizes findings in artifact
5. User reviews, saves relevant parts

**Workflow Diagram**:

```text
User Input: "Research [topic]"
    ↓
Agent Creates Task
    ↓
Agent Searches Sources
    ├── Web search (if configured)
    ├── Document search (local)
    └── Market data (if configured)
    ↓
Agent Generates Summary
    ├── Key findings
    ├── Evidence links
    └── Source citations
    ↓
Artifact Opens
    ├── Interactive summary
    ├── Expandable details
    └── Source links
    ↓
User Reviews
    ├── Save to research docs
    ├── Link to thesis
    └── Discard if irrelevant
```

---

## 4. Thesis Workflow

### 4.1 Thesis Lifecycle

```text
Draft (Private)
    ├── Initial idea
    ├── Rough evidence
    └── Unvalidated claims
    ↓
Active (Tracked)
    ├── Clear thesis statement
    ├── Supporting & contradicting evidence
    ├── Confidence level
    └── Validation criteria
    ↓
Validated (Confirmed)
    ├── Evidence supports thesis
    ├── Market outcome matches
    └── Confidence high
    ↓
Invalidated (Disproven)
    ├── Evidence contradicts thesis
    ├── Market outcome differs
    └── Lessons recorded
    ↓
Archived (Historical)
    ├── Complete history
    ├── Outcome recorded
    └── Lessons learned
```

### 4.2 Thesis Creation

**Manual Path**:
1. User navigates to Journal
2. User clicks "New Thesis"
3. User enters:
   - Thesis statement
   - Initial evidence
   - Confidence level
4. System creates thesis record
5. Thesis visible in workspace

**Agent-Assisted Path**:
1. User: "Create a thesis about NVIDIA"
2. Agent drafts thesis based on research
3. Agent proposes:
   - Thesis statement
   - Supporting evidence
   - Risk factors
   - Confidence estimate
4. User reviews draft
5. User accepts, modifies, or rejects
6. If accepted, thesis becomes active

### 4.3 Evidence Management

**Adding Evidence**:

```text
Source (Document / Web / Note)
    ↓
Extract Evidence Point
    ├── Claim: "NVIDIA revenue grew 50%"
    ├── Direction: Supporting
    ├── Confidence: High
    └── Source link: [document]
    ↓
Attach to Thesis
    ↓
Thesis Confidence Updates
```

**Evidence Types**:
- **Supporting**: Evidence that supports the thesis
- **Contradicting**: Evidence that contradicts the thesis
- **Neutral**: Relevant but neither supports nor contradicts

**Evidence Workflow**:

```text
Agent finds evidence
    ↓
Agent proposes attachment
    ├── Claim
    ├── Direction
    └── Confidence
    ↓
User reviews
    ↓
User approves attachment
    ↓
Evidence linked to thesis
    ↓
Thesis confidence recalculated
```

---

## 5. Agent Interaction Principles

### 5.1 Agent SHOULD

| Action | Example |
|--------|---------|
| Execute tasks | "Research NVIDIA's Q4 earnings" |
| Show progress | Live streaming of research steps |
| Provide evidence | Link claims to sources |
| Generate artifacts | Interactive comparison tables |
| Suggest actions | "Consider adding this to your thesis" |
| Track budget | "This task used 0.05 USD of API credits" |

### 5.2 Agent SHOULD NOT

| Prohibited Action | Reason |
|-------------------|--------|
| Make investment decisions | User must decide |
| Execute trades | Beyond product scope |
| Autonomous portfolio changes | User must approve |
| Share data externally | Privacy/security |
| Hide uncertainty | Honesty required |

### 5.3 Interaction Patterns

**Good Pattern - Task-Driven**:
```
User: "Research NVIDIA's competitive position in AI chips"
Agent: Creates task, executes research, produces artifact
User: Reviews artifact, saves useful parts
```

**Bad Pattern - Conversation-Driven**:
```
User: "What do you think about NVIDIA?"
Agent: "I think NVIDIA is well-positioned because..."
User: "Should I buy it?"
Agent: "I can't make investment recommendations..."
```

**Why Bad**: Encourages chatbot behavior, not research workspace behavior.

---

## 6. Task Creation Workflow

### 6.1 Simple Task

```text
User types input
    "Analyze AAPL's latest earnings call"
    ↓
System validates input
    ↓
System creates task
    ↓
Task queued for execution
    ↓
Task runs (with progress streaming)
    ↓
Task completes
    ↓
Artifact opens
    ↓
User reviews, saves, or discards
```

### 6.2 Task with Context

```text
User selects context
    ├── Research documents: [Q4 report, Analyst notes]
    ├── Thesis: "AAPL will grow services revenue"
    └── Portfolio: AAPL position
    ↓
User types input
    "How does Q4 affect my thesis?"
    ↓
Agent uses context
    ├── Reads selected documents
    ├── Understands thesis statement
    └── Knows position details
    ↓
Agent produces targeted output
    ├── Impact on thesis
    ├── Evidence update suggestions
    └── Risk assessment
```

### 6.3 Task Approval (Future)

```text
Agent prepares to execute costly action
    "Fetch real-time market data ($0.05)"
    ↓
Agent requests approval
    ├── Action: "Fetch market data"
    ├── Cost: "$0.05"
    └── Timeout: "60 seconds"
    ↓
User approves or rejects
    ↓
If approved: Agent executes
    ↓
If rejected: Task cancelled or modified
```

---

## 7. Artifact Review Workflow

### 7.1 Artifact Lifecycle

```text
Generated by Agent
    ↓
Opens in Temporary Window
    ├── Interactive content
    ├── Source links
    └── Export options
    ↓
User Reviews
    ├── Explores content
    ├── Validates claims
    └── Checks sources
    ↓
User Decision
    ├── Save to research docs
    ├── Link to thesis
    ├── Export (PDF/HTML)
    └── Discard
    ↓
Window Closes
```

### 7.2 Artifact Types

**Built-in Artifact Renderers**:
1. **Comparison Table**: Side-by-side comparison of entities
2. **Timeline**: Events over time
3. **Industry Map**: Competitive landscape
4. **Valuation Model**: Interactive valuation scenarios
5. **Risk Dashboard**: Risk summary and drill-down

**Workflow Example - Comparison Table**:

```text
User: "Compare NVIDIA vs AMD vs Intel"
    ↓
Agent researches
    ├── Gathers financial data
    ├── Collects competitive info
    └── Identifies key metrics
    ↓
Agent produces structured JSON
    ├── Companies: [NVDA, AMD, INTC]
    ├── Metrics: [Revenue, Margin, Growth]
    └── Insights: [Strengths, Weaknesses]
    ↓
Comparison Table Artifact opens
    ├── Sortable columns
    ├── Expandable details
    └── Source links
    ↓
User interacts
    ├── Sorts by metric
    ├── Expands details
    └── Clicks sources
    ↓
User saves
    ├── "Save to Research Documents"
    └── "Link to NVIDIA thesis"
```

---

## 8. Portfolio Integration Workflow

### 8.1 Portfolio Overview

```text
User navigates to Portfolio
    ↓
Dashboard shows
    ├── Accounts
    ├── Positions
    ├── Allocation
    └── Thesis links
    ↓
User reviews
    ├── Concentration risks
    ├── Thesis alignment
    └── Performance
```

### 8.2 Thesis-Position Linking

```text
User has thesis: "NVDA will dominate AI chips"
    ↓
User links thesis to NVDA position
    ↓
Dashboard shows
    ├── Position: NVDA
    ├── Thesis: "NVDA will dominate..."
    ├── Confidence: 75%
    └── Status: Active
    ↓
Agent monitors
    ├── New evidence related to thesis
    ├── Price movements
    └── Thesis updates
    ↓
Agent alerts if
    ├── Contradicting evidence found
    ├── Thesis invalidated
    └── Significant price movement
```

---

## 9. Search & Discovery Workflow

### 9.1 Quick Search

```text
User types in search bar
    "NVIDIA"
    ↓
System searches
    ├── Research documents
    ├── Theses
    ├── Agent tasks
    └── Portfolio positions
    ↓
Results grouped by type
    ├── Documents (5)
    ├── Theses (2)
    ├── Tasks (3)
    └── Positions (1)
    ↓
User clicks result
    ↓
Opens relevant view
```

### 9.2 Advanced Search

```text
User opens advanced search
    ↓
User specifies filters
    ├── Type: Research documents
    ├── Date range: Last 30 days
    ├── Keywords: "earnings", "revenue"
    └── Companies: NVDA, AMD
    ↓
System executes query
    ↓
Results displayed
    ↓
User can
    ├── Open document
    ├── Create task from results
    └── Export results
```

---

## 10. User Experience Principles

### 10.1 Workspace, Not Chat

**Design Rule**: Every feature should reinforce "workspace" metaphor, not "chat" metaphor.

**Examples**:
- ✅ "Create research document" (workspace)
- ❌ "Chat about research" (chatbot)

- ✅ "Link evidence to thesis" (structured)
- ❌ "Tell me about the evidence" (conversational)

- ✅ "Save artifact to workspace" (persistence)
- ❌ "Copy response" (temporary)

### 10.2 Persistence Over Ephemeral

**Design Rule**: Everything should be savable, linkable, reusable.

**Examples**:
- Research documents persist
- Theses evolve over time
- Artifacts can be saved
- Evidence links to sources
- Task history is permanent

### 10.3 Structure Over Free-Form

**Design Rule**: Prefer structured data over free-form text.

**Examples**:
- Thesis has defined fields (statement, evidence, confidence)
- Evidence has direction (supporting/contradicting)
- Artifacts have schemas (not raw markdown)
- Tasks have outputs (validated JSON)

### 10.4 User Control Over Automation

**Design Rule**: Agent assists, user decides.

**Examples**:
- Agent suggests thesis → User accepts/modifies/rejects
- Agent proposes evidence → User approves attachment
- Agent finds risks → User decides action
- Agent executes research → User interprets results

**Never**:
- Agent autonomously modifies portfolio
- Agent makes investment decisions
- Agent takes actions without user visibility

---

## 11. Workflow Summary

### 11.1 Primary Workflows

1. **Research**:
   - Gather information → Organize knowledge → Generate artifacts

2. **Thesis Development**:
   - Draft thesis → Add evidence → Track confidence → Validate/Invalidate

3. **Portfolio Alignment**:
   - Link theses to positions → Monitor alignment → Review risks

4. **Daily Review**:
   - Open workspace → Check updates → Review artifacts → Update theses

### 11.2 Agent's Role

**Agent is a research assistant**:
- Gathers information
- Structures data
- Identifies patterns
- Surfaces contradictions
- Generates visualizations

**Agent is NOT a decision-maker**:
- Does NOT make investment decisions
- Does NOT execute trades
- Does NOT autonomously modify data
- Does NOT replace user judgment

---

## 12. Implementation Notes

### 12.1 UI Priorities

**Phase 2 (Agent Runtime)**:
- Task creation input
- Task status display
- Progress streaming
- Basic artifact rendering

**Phase 3 (Artifacts)**:
- Interactive artifacts
- Save/link artifacts
- Export functionality

**Phase 4 (Research Workspace)**:
- Document management
- Search functionality
- Thesis UI

### 12.2 Interaction Patterns

**Use These Patterns**:
- Task creation form
- Progress indicators
- Structured forms for thesis/evidence
- Artifact viewers
- Dashboard layouts

**Avoid These Patterns**:
- Continuous chat streams
- Infinite scroll conversations
- Unstructured note-taking
- Ephemeral responses

---

**Document Version**: 1.0
**Last Updated**: 2026-07-31