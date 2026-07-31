# Vision

## Why Investment OS

Investment research today is fragmented. Analysts juggle dozens of tools — market data terminals, news feeds, spreadsheets, note apps, and AI chat windows — none of which talk to each other. The result: context is lost, evidence goes untracked, and decisions lack an audit trail.

Investment OS is an **AI-native research workspace** that closes this gap. It gives every researcher a persistent, local environment where AI agents help collect information, structure knowledge, test theses, and preserve reasoning — all in one place.

## Why AI Changes Research

Traditional tools present data. AI helps reason about data.

A researcher no longer just pulls financials; an agent can:

- Cross-reference earnings calls against management track records.
- Compare a company's valuation to peers across multiple frameworks.
- Identify contradictions between a thesis and new evidence.
- Summarize hundreds of pages of research into testable claims.

The bottleneck shifts from _finding information_ to _forming and validating judgment_. The software should amplify that judgment, not replace it.

## Why Local-First, Not Cloud-First

Investment research involves sensitive data: portfolio positions, proprietary theses, private notes. A cloud-first architecture would require trusting a third party with that data.

Local-first means:

- **Data ownership.** All research, theses, and portfolio data live on the user's machine.
- **Privacy.** No telemetry or cloud sync unless explicitly enabled.
- **Offline capability.** Research works without an internet connection (AI features excepted).
- **Lower complexity.** No multi-tenant database, no auth system, no cloud infrastructure.

The desktop is the natural home for deep, focused research work.

## Why Agents Need Persistent Context

Chat-based AI tools reset context with every session. An investment thesis may develop over weeks or months. Without persistent context, the AI cannot:

- Remember prior research tasks and their conclusions.
- Track which evidence supported or contradicted a thesis.
- Build on previous analysis without repeating work.

Investment OS gives agents a persistent workspace: every research task, every artifact, every thesis lives in a local database. Agents can recall past work and build on it.

## Why Artifacts

A text response is a dead end. An artifact is an interactive outcome.

When an agent analyzes a company, the result should not be a wall of text — it should be a structured, interactive comparison table. When it models a portfolio, the output should be a chart you can explore, not a paragraph describing one.

Artifacts are temporary, sandboxed windows that render structured agent output. They live as long as the user needs them, then can be persisted or discarded. This is the bridge between AI reasoning and human decision-making.

## Future Ecosystem

Phase 1 delivers the foundation. Future phases unlock:

- **Plugin marketplace.** Third-party developers can create new artifact types — valuation models, industry maps, risk dashboards — that plug into the workspace.
- **Multi-agent collaboration.** Specialized agents for different research domains (fundamental analysis, technical analysis, macro) that coordinate through a shared context.
- **Team workspaces.** Optional sync for teams that want to share research while maintaining local-first privacy controls.

## Non-Goals

Investment OS will never:

- Execute securities trades.
- Make autonomous investment decisions.
- Recommend specific stocks or portfolios.
- Replace human judgment with automation.
- Become a social platform or community.
