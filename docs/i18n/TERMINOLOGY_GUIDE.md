# English Terminology Guide

**Purpose**: Define canonical source terms for AlphaForge's investment research domain. This guide ensures consistent terminology across English (`en`) and Simplified Chinese (`zh-CN`) locales.

**Audience**: Developers, translators, and bilingual reviewers.

**Translation policy**: Translated application values belong in locale catalog data (`apps/desktop/src/lib/i18n/catalogs/`), not in this documentation. This guide defines **source terms only**.

---

## Core Investment Research Terms

### research

**Definition**: The process of gathering, analyzing, and organizing investment information.

**Context**: Research projects contain documents, sources, notes, and reports.

**Translation notes**:
- EN: "Research" (noun and verb)
- zh-CN: "研究" (noun) / "研究" (verb)
- Compound forms: "Research project" → "研究项目"

---

### thesis

**Definition**: A testable investment claim with supporting/contradicting evidence and confidence tracking.

**Context**: Investment theses are the core unit of reasoning in AlphaForge.

**Translation notes**:
- EN: "Thesis" (singular), "Theses" (plural)
- zh-CN: "投资论点" (singular and plural)
- Status transitions: draft → active → validating → closed/completed

**Related terms**:
- Thesis statement: The claim itself
- Confidence: Numerical conviction level (0–100%)
- Validation outcome: Whether the thesis was validated

---

### evidence

**Definition**: A fact, data point, or argument that supports or contradicts a thesis.

**Context**: Evidence items link to sources and have a direction (supporting/contradicting).

**Translation notes**:
- EN: "Evidence" (collective noun)
- zh-CN: "证据"
- Direction: "Supporting" → "支持", "Contradicting" → "反驳"

---

### source

**Definition**: An external reference with provenance (URL, document, or citation).

**Context**: Sources provide the origin for evidence and imported content.

**Translation notes**:
- EN: "Source" (singular), "Sources" (plural)
- zh-CN: "来源"
- Provenance fields: source_id, source_title, source_url, retrieved_at, published_at

---

### provenance

**Definition**: The recorded origin and timestamp chain for imported or analyzed data.

**Context**: Provenance ensures research is traceable and auditable.

**Translation notes**:
- EN: "Provenance"
- zh-CN: "溯源" or "来源追溯"
- Not typically shown directly to users; appears in technical contexts

---

## Portfolio Terms

### portfolio

**Definition**: A collection of investment accounts and their holdings for tracking and analysis.

**Context**: AlphaForge tracks portfolios for research alignment and risk review—not for trade execution.

**Translation notes**:
- EN: "Portfolio"
- zh-CN: "投资组合"
- **Critical**: AlphaForge never executes trades. Portfolio features are "tracking only."

---

### account

**Definition**: A named container for holdings (e.g., "Primary brokerage", "Retirement").

**Context**: Accounts group positions within a workspace.

**Translation notes**:
- EN: "Account"
- zh-CN: "账户"
- Types: Brokerage, Retirement, Cash, Other

---

### holding / position

**Definition**: A recorded security ownership (symbol, quantity, cost basis).

**Context**: Holdings are manually entered or imported via CSV.

**Translation notes**:
- EN: "Holding" or "Position" (used interchangeably in UI)
- zh-CN: "持仓"
- Fields: Symbol, Quantity, Cost basis

---

### cost basis

**Definition**: The original purchase price used for allocation and exposure calculations.

**Context**: AlphaForge uses cost basis (not live market prices) for risk analysis.

**Translation notes**:
- EN: "Cost basis"
- zh-CN: "成本基础"
- Important: Positions without cost basis contribute zero to allocation

---

### allocation

**Definition**: The distribution of recorded cost across symbols.

**Context**: Allocation is calculated from cost basis and shows concentration.

**Translation notes**:
- EN: "Allocation"
- zh-CN: "配置"
- Compound: "Cost-basis allocation" → "成本基础配置"

---

### concentration

**Definition**: A rules-based signal when a position exceeds cost-basis thresholds (10% moderate, 25% high).

**Context**: Concentration is informational—not investment advice.

**Translation notes**:
- EN: "Concentration"
- zh-CN: "集中度"
- **Critical**: Always accompany with disclaimer: "It is not investment advice."

---

### exposure

**Definition**: The measured risk or theme exposure based on recorded holdings.

**Context**: Exposure uses cost basis and explicit theme links.

**Translation notes**:
- EN: "Exposure"
- zh-CN: "敞口"
- Compound: "Theme exposure" → "主题敞口"

---

### transaction

**Definition**: A recorded buy/sell event in account history.

**Context**: Transactions are imported via CSV; they never trigger trades.

**Translation notes**:
- EN: "Transaction"
- zh-CN: "交易"
- Import format: symbol, transaction_type, quantity, price, executed_at

---

## Risk Terms

### risk

**Definition**: Potential adverse outcomes for a portfolio or thesis.

**Context**: Risk labels (concentration, unaligned symbols) are informational.

**Translation notes**:
- EN: "Risk"
- zh-CN: "风险"
- **Critical**: Risk signals are not recommendations

---

### concentration risk

**Definition**: A position exceeding cost-basis thresholds (10% moderate, 25% high).

**Translation notes**:
- EN: "Concentration risk"
- zh-CN: "集中度风险"
- Thresholds must be explicitly shown in UI

---

## Agent Terms

### agent

**Definition**: A background AI task that performs research operations.

**Context**: Agents run asynchronously with streaming progress events.

**Translation notes**:
- EN: "Agent"
- zh-CN: "智能体" or "代理" (prefer "智能体" in AlphaForge context)
- Task states: created, queued, running, waiting_for_input, completed, failed, cancelled

---

### task

**Definition**: A user-initiated agent operation with lifecycle and persistence.

**Context**: Tasks run in Rust backend; progress streams to React UI.

**Translation notes**:
- EN: "Task"
- zh-CN: "任务"
- Compound: "Agent task" → "智能体任务"

---

### artifact

**Definition**: An interactive visualization or report produced by an agent.

**Context**: Artifacts render in isolated windows with controlled permissions.

**Translation notes**:
- EN: "Artifact"
- zh-CN: "成果物" or "制品"
- Types: Comparison Table, Timeline, Industry Map, Valuation Model, Risk Dashboard, Earnings Analyzer, Macro Dashboard

---

## Knowledge Graph Terms

### knowledge graph

**Definition**: A network of companies, industries, technologies, and macro themes linked to theses.

**Context**: Knowledge entities provide structured context for research.

**Translation notes**:
- EN: "Knowledge graph"
- zh-CN: "知识图谱"
- Entity types: Company, Industry, Technology, Macro theme

---

### entity

**Definition**: A node in the knowledge graph (company, industry, technology, or macro theme).

**Translation notes**:
- EN: "Entity"
- zh-CN: "实体"
- Compound: "Knowledge entity" → "知识实体"

---

### relationship

**Definition**: A typed connection between knowledge entities.

**Translation notes**:
- EN: "Relationship"
- zh-CN: "关系"
- Example: NVIDIA → (produces) → CUDA Platform

---

## Options Terms (Deferred)

### option

**Definition**: A financial derivative contract.

**Context**: Option analysis is planned for M9 but not implemented in M8.

**Translation notes**:
- EN: "Option"
- zh-CN: "期权"
- Related: Option chain, Option strategy, Greeks, Implied volatility

**Status**: Option UI is deferred. These terms are defined for future reference but should not appear in M8 UI.

---

## UI State Terms

### loading

**Definition**: Asynchronous operation in progress.

**Translation notes**:
- EN: "Loading..." / "Creating..." / "Importing..." / "Checking..."
- zh-CN: "加载中..." / "创建中..." / "导入中..." / "检查中..."

---

### empty state

**Definition**: No data available for a list or container.

**Translation notes**:
- EN: "No X yet" pattern
- zh-CN: "暂无X" pattern
- Example: "No workspaces yet" → "暂无工作区"

---

### error

**Definition**: An operation failed or data could not be loaded.

**Translation notes**:
- EN: "Failed to..." / "Unable to..." / "Something went wrong"
- zh-CN: "无法..." / "出错了"
- Errors must use stable error codes; no paths or secrets in messages

---

## Forbidden Translations

The following must **not** be translated:

1. **User content**: Imported research, agent output, evidence quotations, thesis statements
2. **Source URLs**: Links to external references
3. **Ticker symbols**: AAPL, MSFT, etc.
4. **Currency codes**: USD, CNY, etc.
5. **Timestamps**: ISO 8601 format remains unchanged
6. **Error codes**: Stable application error codes (e.g., `VALIDATION_ERROR`)
7. **API identifiers**: Workspace IDs, thesis IDs, entity IDs

---

## Consistency Rules

1. **One term, one translation**: Use the same zh-CN term for each EN term consistently
2. **Compound terms**: Build from canonical parts (e.g., "Investment thesis" = "投资" + "论点")
3. **Verb-noun consistency**: Use the same character for noun and verb forms unless context demands otherwise
4. **Financial precision**: Maintain regulatory accuracy for disclaimers and risk labels
5. **No creative translation**: Follow this guide; do not introduce new terms without approval

---

## Review Process

1. Bilingual reviewer (@BerryUIKI) approves terminology before implementation
2. Translated values are recorded in locale catalog files, not documentation
3. This guide is updated only through the M8 decision process

---

## References

- [M8 Decision Record](../M8_DECISION_RECORD.md)
- [i18n Implementation Plan](./IMPLEMENTATION_PLAN.md)
- [i18n Architecture](./ARCHITECTURE.md)