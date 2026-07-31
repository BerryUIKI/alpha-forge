# Investment OS

面向投资研究的桌面优先 AI 工作空间。

```text
信息 → 知识 → 论点 → 决策 → 验证 → 复盘 → 改进
```

Investment OS 将原始信息转化为结构化的投资知识，将研究转化为可验证的投资论点和明智的决策。

**这是一个研究工作空间，而非交易终端。** 它不执行交易，也不做出自主的投资决策。

## 项目状态

**阶段 1 — 技术基础**（进行中）

- [x] 仓库结构
- [x] pnpm workspace（包含 13 个包）
- [x] Tauri 2 + React + TypeScript + Vite 基础
- [x] TypeScript 严格模式 — 通过 `pnpm typecheck`
- [x] ESLint + Prettier 配置 — 通过检查
- [x] Vitest 测试框架配置完成
- [x] Rust 模块结构（包含 `AppError`、`AppState`、IPC 命令）
- [x] SQLite 迁移系统（SQLx）
- [ ] Rust 编译（`cargo check`）— 受沙盒限制（见限制说明）
- [ ] `pnpm tauri dev` — 需要本地 Rust 编译

## 技术栈

| 层级 | 技术 |
|------|------|
| 桌面外壳 | Tauri 2 |
| 后端 | Rust, Tokio, SQLx, SQLite |
| 前端 | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| AI | OpenAI API（未来） |
| 质量 | ESLint, Prettier, Vitest, Rustfmt, Clippy |

## 快速开始

### 环境要求

- Rust stable（Windows 上使用 MSVC 工具链）
- Node.js 22+
- pnpm 9+

### 开发命令

```bash
pnpm install          # 安装所有依赖
pnpm dev:web          # 启动 Vite 开发服务器（仅前端）
pnpm typecheck        # TypeScript 类型检查（所有包）
pnpm lint             # ESLint 检查
pnpm format:check     # Prettier 格式检查
pnpm format           # Prettier 自动修复
pnpm test             # Vitest 测试
```

### Tauri 开发（需要本地 Rust 环境）

```bash
pnpm tauri dev        # 启动完整的 Tauri 桌面应用
pnpm tauri build      # 生产构建
```

### Rust 命令（需要本地 Rust 环境）

```bash
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

## 架构

详细架构文档请参阅 [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)。

关键边界：

- **React** 负责页面、组件、交互、前端状态。
- **Rust** 负责 Agent 运行时、SQLite、文件系统、网络、凭证。
- **Tauri** 负责窗口、IPC、权限、操作系统集成。

## 文档

| 文档 | 用途 |
|------|------|
| [AGENTS.md](AGENTS.md) | Agent 编码标准和规则（最高优先级） |
| [PRODUCT.md](docs/PRODUCT.md) | 产品定位、目标用户、MVP 范围 |
| [VISION.md](docs/VISION.md) | 长期方向和设计理念 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | 系统边界、组件职责、IPC 流程 |
| [SYSTEM_DESIGN.md](docs/SYSTEM_DESIGN.md) | 九大子系统 — 目的、输入、输出、依赖关系 |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | 概念实体、关系、生命周期 |
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | Agent 任务生命周期、工具使用、结构化输出、事件 |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | Artifact 概念、渲染模型、权限模型 |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | 插件清单、版本控制、权限、生命周期 |
| [SECURITY.md](docs/SECURITY.md) | 凭证存储、窗口权限、输入验证 |
| [UI_GUIDELINES.md](docs/UI_GUIDELINES.md) | 设计系统、必需的 UI 状态、导航模式 |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | 本地设置、开发命令、Agent 工作流、故障排除 |
| [ROADMAP.md](docs/ROADMAP.md) | 12 阶段开发路线图（技术阶段） |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | 产品里程碑、交付物和验收标准 |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | 分支策略、提交约定、PR 流程 |
| [PROJECT_BOOTSTRAP.md](docs/PROJECT_BOOTSTRAP.md) | 完整初始化计划（10 个实施阶段） |
| [DECISIONS/](docs/DECISIONS/) | 架构决策记录（3 个 ADR） |

## 当前限制

1. **沙盒中的 Rust 编译**：WorkBuddy 沙盒阻止原生二进制执行。`cargo check`、`cargo test`、`cargo clippy` 必须在本地运行。
2. **`pnpm tauri dev`**：依赖 Rust 编译。必须在本地运行。
3. **无应用图标**：目前只有占位符目录。发布构建前需要图标。
4. **无真实 AI 集成**：Agent 命令返回占位符。真实集成在第 7 阶段。
5. **尚未编写测试**：Vitest 框架已配置但尚无测试文件。

## 许可证

MIT
