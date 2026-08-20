# AlphaForge User Documentation

Welcome to the user documentation for **AlphaForge**, a desktop-first AI workspace
for investment research.

This directory is organized by language so that new translations can be added
without restructuring the documentation set:

```text
docs/user/
├── README.md                  # This index (language switcher)
├── en/                        # English (canonical source)
│   ├── README.md              # English table of contents
│   ├── installation.md
│   ├── configuration.md
│   ├── daily-operations.md
│   ├── troubleshooting.md
│   └── faq.md
├── zh/                        # Simplified Chinese (translation pending)
│   └── README.md
└── developer/
    └── DOCUMENTATION_MAP.md   # Doc-to-module map for maintainers (dev-facing)
```

## Language

| Language | Status | Index |
|----------|--------|-------|
| English (en) | Complete (canonical) | [English documentation](en/README.md) |
| 简体中文 (zh-CN) | Structure reserved, translation pending | [中文文档](zh/README.md) |

## About AlphaForge

AlphaForge is an **AI-native investment research workspace**. It helps you gather
information, build evidence-backed investment theses, track portfolio holdings,
and validate outcomes over time.

> **Important**: AlphaForge is a **research workspace, not a brokerage terminal**.
> It does not execute trades and it does not make autonomous investment decisions.
> See the [Investment Research Disclaimer](../INVESTMENT_RESEARCH_DISCLAIMER.md).

## Documentation Set at a Glance

| Document | Audience | Purpose |
|----------|----------|---------|
| [Installation](en/installation.md) | Users | Install prerequisites, run and build the app |
| [Configuration](en/configuration.md) | Users | Language, AI provider, and Settings reference |
| [Daily Operations](en/daily-operations.md) | Users | How to use every module day-to-day |
| [Troubleshooting](en/troubleshooting.md) | Users | Common problems and their fixes |
| [FAQ](en/faq.md) | Users | Frequently asked questions |
| [Documentation Map](developer/DOCUMENTATION_MAP.md) | Developers | File-to-module mapping and maintenance rules |

## Related Project Documents

- [Privacy Notice](../PRIVACY.md)
- [Investment Research Disclaimer](../INVESTMENT_RESEARCH_DISCLAIMER.md)
- [Product Positioning](../PRODUCT.md)
- [Architecture](../ARCHITECTURE.md)
