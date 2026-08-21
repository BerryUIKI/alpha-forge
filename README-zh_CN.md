# AlphaForge (Investment OS)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**面向投资研究的桌面优先 AI 工作空间** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## 🎯 什么是 AlphaForge？

AlphaForge 是一个**AI 原生投资研究工作空间**，旨在将原始信息转化为结构化的投资知识。

### 核心产品循环

```text
信息 → 知识 → 论点 → 决策 → 验证 → 复盘 → 改进
```

AlphaForge 帮助您：
- 📊 **高效研究** — AI 辅助文档分析和信息收集
- 💡 **构建论点** — 跟踪投资论点及其证据和置信度
- 📈 **做出明智决策** — 结构化的研究工作流，而非聊天机器人式交互
- ✅ **验证结果** — 跟踪论点表现并从结果中学习

> **⚠️ 重要说明**：这是一个**研究工作空间**，而非交易终端。它不执行交易，也不做出自主的投资决策。

---

## 📋 目录

- [项目状态](#项目状态)
- [功能特性](#功能特性)
- [截图](#截图)
- [技术栈](#技术栈)
- [快速开始](#快速开始)
- [架构](#架构)
- [文档](#文档)
- [贡献指南](#贡献指南)
- [路线图](#路线图)
- [安全](#安全)
- [当前限制](#当前限制)
- [许可证](#许可证)

---

## 📊 项目状态

**阶段 1.5 — 应用基础**（进行中）

| 里程碑 | 状态 | 描述 |
|--------|------|------|
| M0 | ✅ 完成 | 项目基础架构 |
| M1 | ✅ 完成 | 桌面运行时基础 |
| M1.5 | ✅ 完成 | 应用基础（工作区、设置） |
| M2 | ✅ 已稳定 (S1) | Agent 运行时与研究任务生命周期 |
| M3 | ⚠️ 稳定化中 | Artifact 智能系统与独立窗口 |
| M4 | ✅ 完成 | 研究工作空间 |
| M5 | ✅ 完成 | 投资知识图谱系统 |
| M6 | ✅ 完成 | 投资组合智能 |
| M7 | ⚠️ 部分完成 | 内部插件生态 |
| M8 | 🚧 重新基准化 | 本地 MVP 完善与发布就绪 |
| M9 | 🚧 重新基准化 | 期权分析模块集成 |
| M10 | 📋 计划中 | Goose Agent 深度集成 |

详细里程碑请参阅 [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) 与 [STABILIZATION_ROADMAP.md](docs/STABILIZATION_ROADMAP.md)。

---

## ✨ 功能特性

### 已实现基础
- ✅ Tauri 2 桌面应用外壳
- ✅ React 19 + TypeScript + Vite 现代化前端
- ✅ Rust 原生后端与 SQLite (SQLx) 健壮持久化
- ✅ IPC 严格校验通信层（176 组命令 100% 静态注册对齐，严格 Zod 解析）
- ✅ Agent 研究任务闭环：任务创建、后台执行、实时事件流推送、失败原因提示与结构化成果渲染
- ✅ 完善的工程与架构文档规范（30+ 文档）

---

## 🖼️ 截图

> **注意**：AlphaForge 处于早期开发阶段（M1.5）。随着 UI 开发的进展，将添加截图。

---

## 🛠️ 技术栈

| 层级 | 技术 |
|------|------|
| **桌面外壳** | Tauri 2 |
| **后端** | Rust, Tokio, SQLx, SQLite |
| **前端** | React 19, TypeScript, Vite 6 |
| **UI** | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| **AI** | OpenAI API（计划中） |
| **质量** | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## 🚀 快速开始

### 环境要求

- Rust stable（Windows 上使用 MSVC 工具链）
- Node.js 22+
- pnpm 9+

### 开发命令

```bash
# 安装依赖
pnpm install

# 前端开发
pnpm dev:web          # 启动 Vite 开发服务器（仅前端）
pnpm typecheck        # TypeScript 类型检查
pnpm lint             # ESLint
pnpm test             # Vitest

# 桌面开发（需要 Rust）
pnpm tauri dev        # 启动完整 Tauri 桌面应用
pnpm tauri build      # 生产构建

# Rust 命令（需要 Rust）
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 🏗️ 架构

详细架构文档请参阅 [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)。

### 关键边界

```
┌─────────────────────────────────────────┐
│            Tauri 2 Desktop              │
│  ┌────────────┐       ┌──────────────┐  │
│  │   React    │◄─────►│    Rust      │  │
│  │ Frontend   │  IPC  │   Backend    │  │
│  └────────────┘       └──────────────┘  │
│                            │             │
│                        ┌───┴───┐         │
│                        │SQLite │         │
│                        └───────┘         │
└─────────────────────────────────────────┘
```

**React** 负责：
- 页面、组件、交互
- 前端状态
- 用户界面

**Rust** 负责：
- Agent 运行时
- SQLite 数据库
- 文件系统与网络访问
- 凭证管理

**Tauri** 负责：
- 桌面窗口
- IPC 通信
- 权限与安全
- 操作系统集成

---

## 📚 文档

### 核心文档

| 文档 | 用途 |
|------|------|
| [AGENTS.md](AGENTS.md) | Agent 编码标准（**必读**） |
| [PRODUCT.md](docs/PRODUCT.md) | 产品定位和 MVP 范围 |
| [VISION.md](docs/VISION.md) | 长期方向 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | 系统架构 |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | 产品里程碑 |

### 技术文档

| 文档 | 用途 |
|------|------|
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | Agent 任务生命周期 |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | Artifact 渲染 |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | 插件开发 |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | 实体关系 |
| [SECURITY.md](SECURITY.md) | 安全策略 |

### 开发指南

| 文档 | 用途 |
|------|------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | 贡献指南 |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Git 和 PR 工作流 |
| [PR_BEST_PRACTICES.md](docs/PR_BEST_PRACTICES.md) | PR 指南 |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | 本地设置指南 |

---

## 🤝 贡献指南

我们欢迎各种形式的贡献！

### 🔒 分支保护说明

**主分支受保护。直接推送被阻止。**

所有变更必须通过 Pull Request：
1. 创建功能分支
2. 进行变更并提交
3. 创建 Pull Request
4. 获得至少 1 个批准
5. 合并到 main

详细工作流程请参阅 [CONTRIBUTING.md](CONTRIBUTING.md)。

### 快速开始

1. 阅读 [AGENTS.md](AGENTS.md)（**所有贡献者必读**）
2. 查看 [CONTRIBUTING.md](CONTRIBUTING.md) 了解工作流程
3. Fork、创建分支、提交 PR

所有贡献必须遵守我们的[行为准则](CODE_OF_CONDUCT.md)。

---

## 🗺️ 路线图

### 开发时间线

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

### 阶段概览

**基础 (M0-M1.5)**：
- 项目设置
- 桌面运行时
- 应用基础

**智能 (M2-M3)**：
- Agent 运行时
- AI 集成
- Artifact 系统

**功能 (M4-M6)**：
- 研究工作空间
- 论点跟踪
- 投资组合分析

**扩展性 (M7-M8)**：
- 插件生态
- 生产发布

详情请参阅 [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)。

---

## 🔐 安全

安全是重中之重。请参阅 [SECURITY.md](SECURITY.md) 了解：
- 漏洞报告流程
- 安全架构
- 凭证管理
- 权限模型

**报告**：请通过 GitHub Security 私下报告安全问题。

---

## ⚠️ 当前限制

1. **沙盒中的 Rust 编译**：`cargo check/test/clippy` 必须在本地运行
2. **`pnpm tauri dev`**：需要本地 Rust 编译
3. **无应用图标**：目前只有占位符
4. **无真实 AI 集成**：Agent 命令返回占位符
5. **尚未编写测试**：框架已配置，但没有测试文件

---

## 📄 许可证

本项目采用 **GNU Affero General Public License v3.0 (AGPLv3)** 许可证 - 详情请查看 [LICENSE](LICENSE) 文件。

### 为什么选择 AGPLv3？

AGPLv3 确保：
- ✅ 所有修改必须回馈社区
- ✅ 网络使用（SaaS）触发 copyleft 要求
- ✅ 用户始终可以访问源代码
- ✅ 允许在适当许可下进行商业使用

这保护了 AlphaForge 的开源性质，同时允许可持续开发。

---

## 🙏 致谢

AlphaForge 的实现离不开以下开源项目：

- [Tauri](https://tauri.app) - 桌面应用框架
- [React](https://react.dev) - UI 库
- [Rust](https://www.rust-lang.org) - 系统编程语言
- [shadcn/ui](https://ui.shadcn.com) - UI 组件库
- [Tailwind CSS](https://tailwindcss.com) - CSS 框架

---

## 📞 联系方式

- **Issues**: [GitHub Issues](https://github.com/BerryUIKI/alpha-forge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions)

---

<p align="center">
  <strong>AlphaForge 团队用 ❤️ 构建</strong>
</p>

<p align="center">
  <sub>将信息转化为投资智能</sub>
</p>