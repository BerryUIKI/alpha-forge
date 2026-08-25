# Product

## Positioning

AlphaForge is an **AI-native investment research workspace** — a desktop application that helps investors and analysts transform raw information into structured knowledge, testable theses, and documented decisions.

It is a **research tool**, not a trading tool.

## Target Users

- Independent investors who do their own research.
- Professional analysts who manage multiple theses across sectors.
- Portfolio managers who need to track evidence for investment decisions.
- Anyone who wants a structured, persistent system for investment thinking — not just another chat window.

## Core Problems Solved

1. **Fragmented workflow.** Research happens across market data terminals, news apps, spreadsheets, note-taking tools, and AI chats. Nothing connects. AlphaForge unifies these into one workspace.

2. **Lost context.** AI chat sessions reset. Investment theses take weeks to form. Without persistent context, every session starts from zero.

3. **Weak evidence tracking.** Most investors cannot trace why they made a decision six months ago. AlphaForge links every thesis to its supporting and contradicting evidence.

4. **Unstructured AI output.** Raw text from AI is hard to act on. AlphaForge renders agent output as interactive artifacts — comparison tables, risk dashboards, valuation models — not walls of text.

## Main Workflows

### Research Flow

```text
Collect information (documents, news, data)
  → Agent processes and structures
    → Generate research notes
      → Form investment thesis
        → Collect supporting/contradicting evidence
          → Validate thesis over time
            → Review outcome
```

### Daily Workflow

```text
Open Today dashboard
  → See active tasks, recent artifacts, portfolio snapshot
    → Enter a research task for the agent
      → Agent collects, analyzes, produces artifact
        → Review artifact, save to journal
          → Link to thesis or portfolio
```

## MVP Scope

### In Scope

- Desktop application shell with sidebar navigation.
- Agent workspace: submit tasks, view status and history.
- Research page: documents, tasks, sources, notes.
- Journal: investment theses with evidence tracking.
- Portfolio overview: accounts, positions, exposure.
- Settings: AI provider configuration, data providers, local storage management.
- Agent runtime: single-agent, async task execution, structured output.
- Artifact system: temporary interactive windows for agent output.
- Seven internal plugins: company comparison, valuation model, portfolio risk, industry map, research timeline, earnings analyzer, and macro dashboard.
- Local SQLite persistence with migration support.

### Explicitly Out of Scope

- Securities order execution or brokerage integration.
- Automated stock recommendations.
- Real-time market data terminal.
- Social or community features.
- Third-party plugin marketplace (MVP is internal plugins only).
- Cloud sync or team collaboration.
- Mobile applications.
- Multi-agent orchestration (single-agent only for MVP).
