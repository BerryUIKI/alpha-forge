# UI String Inventory

**Purpose**: Comprehensive inventory of all user-visible strings in AlphaForge for i18n implementation.

**Status**: Source inventory for M8-02. Strings will be externalized to locale catalogs in subsequent tasks (M8-03 through M8-09).

**Last updated**: 2026-08-03

---

## Inventory Summary

| Namespace | Owner | String Count | i18n Status |
|-----------|-------|--------------|-------------|
| navigation | Frontend team | 6 | ✅ i18n-managed (locale.ts) |
| settings | Frontend team | 26 | ✅ i18n-managed (locale.ts) |
| workspace | Frontend team | 14 | ❌ Hardcoded |
| research | Frontend team | 57 | ❌ Hardcoded |
| journal | Frontend team | 60 | ❌ Hardcoded |
| portfolio | Frontend team | 91 | ❌ Hardcoded |
| agent | Frontend team | 21 | ❌ Hardcoded |
| artifacts | Frontend team | 35 | ❌ Hardcoded |
| errors | Frontend team | 40+ | ❌ Hardcoded (distributed) |
| options | Deferred | 0 | N/A (M9) |

**Total**: ~350 user-visible strings

---

## 1. Navigation & Shell

### Namespace: navigation
**Owner**: Frontend team
**Status**: ✅ i18n-managed

| Key | EN-US | zh-CN | Context |
|-----|-------|-------|---------|
| `today` | "Today" | "今日" | Navigation label |
| `research` | "Research" | "研究" | Navigation label |
| `journal` | "Journal" | "日志" | Navigation label |
| `portfolio` | "Portfolio" | "投资组合" | Navigation label |
| `artifacts` | "Artifacts" | "成果物" | Navigation label |
| `settings` | "Settings" | "设置" | Navigation label |

**Source**: `apps/desktop/src/lib/i18n/locale.ts`

---

### Namespace: theme
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `components/theme/ThemeToggle.tsx` | "Toggle theme" | Button aria-label |

---

### Namespace: error-boundary
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `components/common/ErrorBoundary.tsx` | "Application Error" | Error boundary heading |
| `components/common/ErrorBoundary.tsx` | "An unexpected error occurred" | Fallback message |

---

## 2. Settings

### Namespace: settings
**Owner**: Frontend team
**Status**: ✅ i18n-managed

| Key | EN-US | zh-CN | Context |
|-----|-------|-------|---------|
| `settings` | "Settings" | "设置" | Page heading |
| `settings.description` | "Local-first controls..." | "本地优先控制..." | Page description |
| `language` | "Language" | "语言" | Section heading |
| `language.description` | "Chinese is the default..." | "中文为默认语言..." | Section description |
| `language.zh-CN` | "Simplified Chinese" | "简体中文" | Dropdown option |
| `language.en` | "English" | "English" | Dropdown option |
| `backup` | "Local backup" | "本地备份" | Section heading |
| `backup.description` | "Export a consistent..." | "导出一致的..." | Section description |
| `backup.export` | "Export local backup" | "导出本地备份" | Button label |
| `backup.exporting` | "Exporting..." | "正在导出..." | Loading state |
| `backup.success` | "Backup created at {path}" | "备份已创建：{path}" | Success message |
| `backup.cancelled` | "Backup export cancelled." | "已取消备份导出。" | Cancel message |
| `backup.failed` | "Backup export failed..." | "备份导出失败..." | Error message |
| `updates` | "Updates" | "更新" | Section heading |
| `updates.description` | "Checks GitHub Releases..." | "仅在你请求时..." | Section description |
| `updates.check` | "Check for updates" | "检查更新" | Button label |
| `updates.checking` | "Checking..." | "正在检查..." | Loading state |
| `updates.available` | "Version {version} is available." | "版本 {version} 可用。" | Update available |
| `updates.uptodate` | "You are up to date ({version})." | "已是最新版本（{version}）。" | Up to date |
| `updates.failed` | "Could not check GitHub Releases..." | "无法检查 GitHub Releases..." | Error message |
| `privacy` | "About and privacy" | "关于与隐私" | Section heading |
| `privacy.description` | "AlphaForge is a local-first..." | "AlphaForge 是本地优先..." | Section description |
| `privacy.notice` | "Open privacy notice" | "打开隐私声明" | Button label |
| `privacy.disclaimer` | "Open research disclaimer" | "打开研究免责声明" | Button label |

**Source**: `apps/desktop/src/lib/i18n/locale.ts`

---

## 3. Workspace

### Namespace: workspace
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Create Workspace" | Dialog heading |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Workspace Name" | Form label |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "My Research" | Input placeholder |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Workspace name is required" | Validation error |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Workspace name must be 200 characters or less" | Validation error |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Failed to create workspace" | Error message |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Cancel" | Button label |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Create" | Button label |
| `features/workspace/components/CreateWorkspaceDialog.tsx` | "Creating..." | Loading state |
| `features/workspace/components/WorkspaceList.tsx` | "No workspaces yet" | Empty state title |
| `features/workspace/components/WorkspaceList.tsx` | "Create your first workspace to start organizing your research." | Empty state description |
| `features/workspace/components/WorkspaceList.tsx` | "Create Workspace" | Empty state button |
| `features/workspace/components/WorkspaceList.tsx` | "Failed to load workspaces" | Error message |
| `features/workspace/components/WorkspaceList.tsx` | "Created" | Label prefix |
| `pages/today/TodayPage.tsx` | "Today" | Page heading |
| `pages/today/TodayPage.tsx` | "Your investment research dashboard" | Page description |
| `pages/today/TodayPage.tsx` | "Select a Workspace" | Section heading |
| `pages/today/TodayPage.tsx` | "Choose a workspace to view and manage your research..." | Section description |
| `pages/today/TodayPage.tsx` | "Workspace selected. Research features coming in Phase 2+." | Status message |
| `pages/today/TodayPage.tsx` | "Change workspace" | Action link |

---

## 4. Research

### Namespace: research
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `pages/research/ResearchPage.tsx` | "Research" | Page heading |
| `pages/research/ResearchPage.tsx` | "Capture projects, source provenance, and document annotations." | Page description |
| `pages/research/ResearchPage.tsx` | "Unable to save the research item..." | Validation error |
| `pages/research/ResearchPage.tsx` | "Workspace" | Form label |
| `pages/research/ResearchPage.tsx` | "Select a workspace" | Dropdown placeholder |
| `pages/research/ResearchPage.tsx` | "Projects" | Section heading |
| `pages/research/ResearchPage.tsx` | "Project title" | Input placeholder |
| `pages/research/ResearchPage.tsx` | "Create" | Button label |
| `pages/research/ResearchPage.tsx` | "Documents" | Section heading |
| `pages/research/ResearchPage.tsx` | "Document title" | Input placeholder |
| `pages/research/ResearchPage.tsx` | "Add" | Button label |
| `pages/research/ResearchPage.tsx` | "Importing PDF..." | Loading state |
| `pages/research/ResearchPage.tsx` | "Import PDF" | Button label |
| `pages/research/ResearchPage.tsx` | "https://example.com/research" | URL placeholder |
| `pages/research/ResearchPage.tsx` | "Importing..." | Loading state |
| `pages/research/ResearchPage.tsx` | "Import web page" | Button label |
| `pages/research/ResearchPage.tsx` | "PDFs use the native picker..." | Help text |
| `pages/research/ResearchPage.tsx` | "Reports" | Section heading |
| `pages/research/ResearchPage.tsx` | "Report title" | Input placeholder |
| `pages/research/ResearchPage.tsx` | "Report findings" | Textarea placeholder |
| `pages/research/ResearchPage.tsx` | "Analysis" / "Summary" / "Thesis" / "Recommendation" | Report type options |
| `pages/research/ResearchPage.tsx` | "Save report" | Button label |
| `pages/research/ResearchPage.tsx` | "Notes" | Section heading |
| `pages/research/ResearchPage.tsx` | "Add note" | Button label |
| `pages/research/ResearchPage.tsx` | "Sources" | Section heading |
| `pages/research/ResearchPage.tsx` | "https://example.com" | URL placeholder |
| `pages/research/ResearchPage.tsx` | "Source title (optional)" | Input placeholder |
| `pages/research/ResearchPage.tsx` | "Add source" | Button label |
| `pages/research/ResearchPage.tsx` | "Sources must use public HTTPS hostnames." | Help text |
| `pages/research/ResearchPage.tsx` | "Untitled source" | Fallback text |
| `pages/research/ResearchPage.tsx` | "Search this document" | Section heading |
| `pages/research/ResearchPage.tsx` | "Find terms in the saved document text" | Input placeholder |
| `pages/research/ResearchPage.tsx` | "Exact terms" / "Related terms" | Search mode options |
| `pages/research/ResearchPage.tsx` | "Search" | Button label |
| `pages/research/ResearchPage.tsx` | "Related terms use a local, explainable investment vocabulary..." | Help text |
| `pages/research/ResearchPage.tsx` | "Score" | Result label |

**Total**: ~57 strings

---

## 5. Journal & Thesis

### Namespace: journal
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `pages/journal/JournalPage.tsx` | "Journal" | Page heading |
| `features/thesis/components/CreateThesisForm.tsx` | "New investment thesis" | Form heading |
| `features/thesis/components/CreateThesisForm.tsx` | "Capture a testable claim before you start tracking evidence." | Form description |
| `features/thesis/components/CreateThesisForm.tsx` | "Title" | Form label |
| `features/thesis/components/CreateThesisForm.tsx` | "AI infrastructure demand remains durable" | Placeholder |
| `features/thesis/components/CreateThesisForm.tsx` | "Thesis statement" | Form label |
| `features/thesis/components/CreateThesisForm.tsx` | "State the claim, why it may be true, and what could disprove it." | Placeholder |
| `features/thesis/components/CreateThesisForm.tsx` | "Initial confidence" | Form label |
| `features/thesis/components/CreateThesisForm.tsx` | "A title and thesis statement are required." | Validation error |
| `features/thesis/components/CreateThesisForm.tsx` | "Unable to create the thesis." | Error message |
| `features/thesis/components/CreateThesisForm.tsx` | "Creating..." | Loading state |
| `features/thesis/components/CreateThesisForm.tsx` | "Create thesis" | Button label |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Knowledge graph" | Section heading |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Connect companies, industries, technologies, and macro themes." | Description |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Company" / "Industry" / "Technology" / "Macro theme" | Entity type options |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Entity name" | Input placeholder |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Add entity" | Button label |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Source" / "Target" | Select placeholders |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Add relationship" | Button label |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Entity name is required." | Validation error |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Unable to create the knowledge entity." | Error message |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Source, target, and relationship type are required." | Validation error |
| `features/thesis/components/KnowledgeGraphPanel.tsx` | "Unable to create the relationship." | Error message |
| `features/thesis/components/ThesisDashboard.tsx` | "Failed to load workspaces." | Error message |
| `features/thesis/components/ThesisDashboard.tsx` | "Create a workspace first" | Empty state title |
| `features/thesis/components/ThesisDashboard.tsx` | "Theses are stored in a workspace so their evidence remains organized." | Empty state description |
| `features/thesis/components/ThesisDashboard.tsx` | "Investment theses" | Section heading |
| `features/thesis/components/ThesisDashboard.tsx` | "Make your reasoning explicit, track evidence, and validate outcomes." | Description |
| `features/thesis/components/ThesisDashboard.tsx` | "Workspace" | Form label |
| `features/thesis/components/ThesisDashboard.tsx` | "Select a thesis" | Empty state title |
| `features/thesis/components/ThesisDashboard.tsx` | "Choose a thesis to review its confidence, lifecycle, and evidence." | Empty state description |
| `features/thesis/components/ThesisDetail.tsx` | "Delete thesis" | aria-label |
| `features/thesis/components/ThesisDetail.tsx` | "Confidence" | Form label |
| `features/thesis/components/ThesisDetail.tsx` | "Save confidence" | Button label |
| `features/thesis/components/ThesisDetail.tsx` | "Activate thesis" / "Start validation" / "Close thesis" | Action buttons |
| `features/thesis/components/ThesisDetail.tsx` | "Confidence history" | Section heading |
| `features/thesis/components/ThesisDetail.tsx` | "Failed to load confidence history." | Error message |
| `features/thesis/components/ThesisDetail.tsx` | "Knowledge links" | Section heading |
| `features/thesis/components/ThesisDetail.tsx` | "Link an entity..." | Select placeholder |
| `features/thesis/components/ThesisDetail.tsx` | "Link" | Button label |
| `features/thesis/components/ThesisDetail.tsx` | "Select an entity to link." | Validation error |
| `features/thesis/components/ThesisDetail.tsx` | "Validation outcome" | Form label |
| `features/thesis/components/ThesisDetail.tsx` | "Thesis was validated" | Checkbox label |
| `features/thesis/components/ThesisDetail.tsx` | "Record an outcome before completing validation." | Validation error |
| `features/thesis/components/ThesisDetail.tsx` | "Complete validation" | Button label |
| `features/thesis/components/ThesisDetail.tsx` | "Outcome: " | Label prefix |
| `features/thesis/components/ThesisDetail.tsx` | "Evidence" | Section heading |
| `features/thesis/components/ThesisDetail.tsx` | "Supporting" / "Contradicting" | Direction options |
| `features/thesis/components/ThesisDetail.tsx` | "Source ID (optional)" | Input placeholder |
| `features/thesis/components/ThesisDetail.tsx` | "Add a fact, data point, or argument..." | Textarea placeholder |
| `features/thesis/components/ThesisDetail.tsx` | "Add evidence" | Button label |
| `features/thesis/components/ThesisDetail.tsx` | "Failed to load evidence." | Error message |
| `features/thesis/components/ThesisDetail.tsx` | "Delete evidence" | aria-label |
| `features/thesis/components/ThesisDetail.tsx` | "Source: " | Label prefix |
| `features/thesis/components/ThesisDetail.tsx` | "Evidence text is required." | Validation error |
| `features/thesis/components/ThesisDetail.tsx` | "The thesis could not be updated." | Error message |
| `features/thesis/components/ThesisList.tsx` | "Failed to load theses." | Error message |
| `features/thesis/components/ThesisList.tsx` | "No theses yet" | Empty state title |
| `features/thesis/components/ThesisList.tsx` | "Create a thesis to begin preserving your investment reasoning." | Empty state description |
| `features/thesis/components/ThesisList.tsx` | "confidence" | Status label suffix |

**Total**: ~60 strings

---

## 6. Portfolio

### Namespace: portfolio
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `pages/portfolio/PortfolioPage.tsx` | "Portfolio" | Page heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Track holdings for review and risk analysis..." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Workspace" | Form label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to load workspaces." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Create a workspace first" | Empty state title |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Portfolio accounts are organized within a workspace." | Empty state description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Select an account" | Empty state title |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Choose an account to review and record its holdings." | Empty state description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "New account" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Add a source of holdings for tracking only." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Account name" | aria-label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Primary brokerage" | Input placeholder |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Account type" | aria-label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Brokerage" / "Retirement" / "Cash" / "Other" | Account type options |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Account currency" | aria-label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Account name is required." | Validation error |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Unable to create the account." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Creating..." | Loading state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Add account" | Button label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Accounts" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to load portfolio accounts." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "No accounts yet" | Empty state title |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Add an account to start tracking holdings." | Empty state description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Symbol" / "Quantity" / "Cost basis" | Table headers & inputs |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Enter a symbol, a non-zero quantity..." | Validation error |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Unable to add the holding." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Adding..." | Loading state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Add holding" | Button label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to load holdings." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "No holdings yet" | Empty state title |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Record a holding manually, or import your transaction history below." | Empty state description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Import transaction history" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Paste CSV with exactly: symbol, transaction_type..." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Transaction CSV" | aria-label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "symbol,transaction_type,quantity,price,executed_at\nMSFT,buy,2,420,2026-08-01T00:00:00Z" | CSV placeholder |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Unable to import transactions." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Importing..." | Loading state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Import transactions" | Button label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Imported transactions" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to load transactions." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "No transaction history has been imported." | Empty state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Cost-basis allocation" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Exposure uses recorded cost basis..." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to calculate allocation." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Add holdings with a cost basis to see allocation..." | Empty state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Recorded cost:" | Label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Concentration review" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "A rules-based signal: moderate at 10% and high at 25%..." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to analyze concentration." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "No positions currently exceed the review thresholds." | Empty state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Theme exposure" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Link a held symbol to an existing knowledge entity..." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Theme symbol" / "Knowledge entity" | aria-labels |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Knowledge entity..." | Select placeholder |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Link theme" | Button label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Unable to link theme." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to load theme exposure." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "No theme links yet." | Empty state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Thesis alignment and review" | Section heading |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Matches symbols only when their ticker appears..." | Description |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Failed to check thesis alignment." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "No held symbols currently match a workspace thesis." | Empty state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Reviewing..." | Loading state |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Generate portfolio review" | Button label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Unable to generate review." | Error message |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Review generated" | Label |
| `features/portfolio/components/PortfolioDashboard.tsx` | "Unaligned symbols:" / "Concentration signals:" | Labels |
| `features/portfolio/components/PortfolioDashboard.tsx` | "None" | Empty value |

**Total**: ~91 strings

---

## 7. Agent Tasks

### Namespace: agent
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `features/agent/components/TaskStatusBadge.tsx` | "Created" / "Queued" / "Running" / "Waiting" / "Completed" / "Failed" / "Cancelled" | Status badges |
| `features/agent/components/CreateAgentTask.tsx` | "New Task" | Button label |
| `features/agent/components/CreateAgentTask.tsx` | "Create Agent Task" | Form heading |
| `features/agent/components/CreateAgentTask.tsx` | "Title" | Form label |
| `features/agent/components/CreateAgentTask.tsx` | "Research Tesla's Q4 earnings" | Placeholder |
| `features/agent/components/CreateAgentTask.tsx` | "Description (optional)" | Form label |
| `features/agent/components/CreateAgentTask.tsx` | "Provide more details about what you want to research..." | Placeholder |
| `features/agent/components/CreateAgentTask.tsx` | "Task title is required" | Validation error |
| `features/agent/components/CreateAgentTask.tsx` | "Failed to create task" | Error message |
| `features/agent/components/CreateAgentTask.tsx` | "Cancel" | Button label |
| `features/agent/components/CreateAgentTask.tsx` | "Creating..." | Loading state |
| `features/agent/components/CreateAgentTask.tsx` | "Create Task" | Button label |
| `features/agent/components/AgentTaskList.tsx` | "Failed to load agent tasks" | Error message |
| `features/agent/components/AgentTaskList.tsx` | "No tasks yet" | Empty state title |
| `features/agent/components/AgentTaskList.tsx` | "Create your first agent task to start researching." | Empty state description |

**Total**: ~21 strings

---

## 8. Artifacts

### Namespace: artifacts
**Owner**: Frontend team
**Status**: ❌ Hardcoded

| File | String | Context |
|------|--------|---------|
| `features/artifacts/components/ArtifactViewer.tsx` | "Loading artifact..." | Loading state |
| `features/artifacts/components/ArtifactViewer.tsx` | "Error loading artifact:" | Error prefix |
| `features/artifacts/components/ArtifactViewer.tsx` | "Artifact not found" | Empty state |
| `features/artifacts/components/ArtifactViewer.tsx` | "No renderer available for artifact type:" | Error message |
| `features/artifacts/components/ArtifactViewer.tsx` | "Status:" / "Created:" | Labels |
| `features/artifacts/renderers/ComparisonTableRenderer.tsx` | "Invalid comparison table data" | Error message |
| `features/artifacts/renderers/ComparisonTableRenderer.tsx` | "No companies to compare" | Empty state |
| `features/artifacts/renderers/ComparisonTableRenderer.tsx` | "Company" | Table header |
| `features/artifacts/renderers/EarningsAnalyzerRenderer.tsx` | "No earnings analysis to display" | Empty state |
| `features/artifacts/renderers/EarningsAnalyzerRenderer.tsx` | "Earnings period:" | Label |
| `features/artifacts/renderers/IndustryMapRenderer.tsx` | "No industry data to display" | Empty state |
| `features/artifacts/renderers/IndustryMapRenderer.tsx` | "Industry Map" | Heading |
| `features/artifacts/renderers/IndustryMapRenderer.tsx` | "Market Share:" | Label |
| `features/artifacts/renderers/MacroDashboardRenderer.tsx` | "No macro indicators to display" | Empty state |
| `features/artifacts/renderers/MacroDashboardRenderer.tsx` | "Macro Dashboard" | Heading |
| `features/artifacts/renderers/MacroDashboardRenderer.tsx` | "As of" / "Change:" | Labels |
| `features/artifacts/renderers/RiskDashboardRenderer.tsx` | "No risk data to display" | Empty state |
| `features/artifacts/renderers/RiskDashboardRenderer.tsx` | "Risk Dashboard" | Heading |
| `features/artifacts/renderers/RiskDashboardRenderer.tsx` | "Overall Risk Score:" / "Mitigation:" | Labels |
| `features/artifacts/renderers/TimelineRenderer.tsx` | "No events to display" | Empty state |
| `features/artifacts/renderers/ValuationModelRenderer.tsx` | "No valuation data to display" | Empty state |
| `features/artifacts/renderers/ValuationModelRenderer.tsx` | "Valuation" | Heading |
| `features/artifacts/renderers/ValuationModelRenderer.tsx` | "Methodology:" / "Current Price:" / "upside" | Labels |
| `features/artifacts/renderers/index.ts` | "Comparison Table" / "Timeline" / "Industry Map" / "Valuation Model" / "Risk Dashboard" / "Earnings Analyzer" / "Macro Dashboard" | Artifact type names |
| `features/artifacts/renderers/index.ts` | "Compare multiple companies across dimensions" / "Display chronological events..." / etc. | Artifact type descriptions |
| `pages/artifacts/ArtifactsPage.tsx` | "Artifacts" | Page heading |

**Total**: ~35 strings

---

## 9. Error States

### Namespace: errors
**Owner**: Frontend team
**Status**: ❌ Hardcoded (distributed across components)

**Common error strings** (used in multiple locations):

| String | Context |
|--------|---------|
| "Something went wrong" | Default error state title |
| "Try Again" | Retry button label |
| "Application Error" | Error boundary title |
| "An unexpected error occurred" | Error boundary fallback |
| "Failed to load..." | Pattern for data loading errors |
| "Unable to..." | Pattern for operation errors |
| "...is required" | Pattern for validation errors |

**Total**: ~40+ unique error strings (see agent scan results for full list)

---

## 10. Options (Deferred)

### Namespace: options
**Owner**: N/A (M9)
**Status**: Deferred

**No Option UI exists in M8.** Type definitions are prepared in `types/option.ts` but no user-facing strings are implemented.

---

## Next Steps

1. **M8-03**: Implement typed i18n runtime foundation
2. **M8-04**: Localize shell and common states
3. **M8-05 through M8-09**: Localize each feature area

Each implementation task will externalize hardcoded strings from this inventory into locale catalog files.

---

## References

- [Terminology Guide](./TERMINOLOGY_GUIDE.md)
- [i18n Implementation Plan](./IMPLEMENTATION_PLAN.md)
- [i18n Architecture](./ARCHITECTURE.md)