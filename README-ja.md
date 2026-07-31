# AlphaForge (Investment OS)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**投資研究のためのデスクトップファーストAIワークスペース** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## コアプロダクトループ

```text
情報 → 知識 → 仮説 → 意思決定 → 検証 → レビュー → 改善
```

AlphaForgeは生の情報を構造化された投資知識に変換し、研究を検証可能な投資仮説と十分な情報に基づいた意思決定に変換します。

> **⚠️ 重要**: これは**研究ワークスペース**であり、ブローカージターミナルではありません。取引を実行したり、自律的な投資決定を行ったりすることはありません。

---

## 目次

- [ステータス](#ステータス)
- [スクリーンショット](#スクリーンショット)
- [技術スタック](#技術スタック)
- [はじめに](#はじめに)
- [アーキテクチャ](#アーキテクチャ)
- [ドキュメント](#ドキュメント)
- [コントリビュート](#コントリビュート)
- [ロードマップ](#ロードマップ)
- [現在の制限](#現在の制限)
- [ライセンス](#ライセンス)

---

## ステータス

**フェーズ 1.5 — アプリケーション基盤**（進行中）

| マイルストーン | ステータス | 説明 |
|--------------|----------|------|
| M0 | ✅ 完了 | プロジェクト基盤 |
| M1 | ✅ 完了 | デスクトップランタイム基盤 |
| M1.5 | 🚧 進行中 | アプリケーション基盤 |
| M2-M7 | 📋 計画中 | エージェントランタイム → プラグインエコシステム |
| M8 | 📅 将来 | 本番稼働 & 商業化 |

詳細なマイルストーンについては、[MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)をご覧ください。

---

## スクリーンショット

> **注意**: AlphaForgeは初期開発段階（M1.5）です。UI開発が進むにつれてスクリーンショットが追加されます。

---

## 技術スタック

| レイヤー | 技術 |
|---------|------|
| デスクトップシェル | Tauri 2 |
| バックエンド | Rust, Tokio, SQLx, SQLite |
| フロントエンド | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| AI | OpenAI API（将来） |
| 品質 | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## はじめに

### 前提条件

- Rust stable（WindowsではMSVCツールチェーン）
- Node.js 22+
- pnpm 9+

### 開発コマンド

```bash
pnpm install          # すべての依存関係をインストール
pnpm dev:web          # Vite開発サーバーを起動（フロントエンドのみ）
pnpm typecheck        # TypeScript型チェック（すべてのパッケージ）
pnpm lint             # ESLint
pnpm format:check     # Prettierフォーマットチェック
pnpm format           # Prettier自動修正
pnpm test             # Vitest
```

### Tauri開発（ローカルRust環境が必要）

```bash
pnpm tauri dev        # 完全なTauriデスクトップアプリを起動
pnpm tauri build      # 本番ビルド
```

### Rustコマンド（ローカルRust環境が必要）

```bash
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## アーキテクチャ

詳細なアーキテクチャドキュメントについては、[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)をご覧ください。

主な境界：

- **React** - ページ、コンポーネント、相互作用、フロントエンド状態を管理
- **Rust** - エージェントランタイム、SQLite、ファイルシステム、ネットワーク、認証情報を管理
- **Tauri** - ウィンドウ、IPC、権限、OS統合を管理

---

## ドキュメント

| ドキュメント | 目的 |
|------------|------|
| [AGENTS.md](AGENTS.md) | エージェントコーディング標準とルール（最高優先度） |
| [PRODUCT.md](docs/PRODUCT.md) | 製品ポジショニング、ターゲットユーザー、MVPスコープ |
| [VISION.md](docs/VISION.md) | 長期的方向性と設計哲学 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | システム境界、コンポーネント責任、IPCフロー |
| [SYSTEM_DESIGN.md](docs/SYSTEM_DESIGN.md) | 9つのサブシステム — 目的、入力、出力、依存関係 |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | 概念エンティティ、関係、ライフサイクル |
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | エージェントタスクライフサイクル、ツール使用、構造化出力、イベント |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | アーティファクト概念、レンダリングモデル、権限モデル |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | プラグインマニフェスト、バージョニング、権限、ライフサイクル |
| [SECURITY.md](SECURITY.md) | 認証情報ストレージ、ウィンドウ権限、入力検証 |
| [UI_GUIDELINES.md](docs/UI_GUIDELINES.md) | デザインシステム、必要なUI状態、ナビゲーションパターン |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | ローカルセットアップ、開発コマンド、エージェントワークフロー、トラブルシューティング |
| [ROADMAP.md](docs/ROADMAP.md) | 12フェーズ開発ロードマップ（技術フェーズ） |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | 製品マイルストーン、成果物、受け入れ基準 |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | ブランチ戦略、コミット規約、PRプロセス |
| [PROJECT_BOOTSTRAP.md](docs/PROJECT_BOOTSTRAP.md) | 完全な初期化計画（10の実装フェーズ） |
| [DECISIONS/](docs/DECISIONS/) | アーキテクチャ決定記録（3つのADR） |

---

## コントリビュート

コントリビュートに興味をお持ちいただきありがとうございます！

### クイックスタート

1. リポジトリをクローン
2. [AGENTS.md](AGENTS.md)を読む（**必須**）
3. [CONTRIBUTING.md](CONTRIBUTING.md)を確認
4. 機能ブランチを作成してPRを提出

### 開発ワークフロー

- `main`から機能ブランチを作成
- `AGENTS.md`のワークフローに従う
- すべてのチェックに合格することを確認
- PRを提出

詳細については、[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。

---

## ロードマップ

AlphaForgeは8つのマイルストーンで開発されています：

1. ✅ **M0** — プロジェクト基盤
2. ✅ **M1** — デスクトップランタイム基盤
3. 🚧 **M1.5** — アプリケーション基盤（現在）
4. 📋 **M2** — エージェントランタイム
5. 📋 **M3** — アーティファクトインテリジェンスシステム
6. 📋 **M4** — 研究ワークスペース
7. 📋 **M5** — 投資知識システム
8. 📋 **M6** — ポートフォリオインテリジェンス
9. 📋 **M7** — プラグインエコシステム
10. 📅 **M8** — 本番稼働 & 商業化

完全なロードマップについては、[MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)をご覧ください。

---

## 現在の制限

1. **サンドボックス内でのRustコンパイル**: WorkBuddyサンドボックスはネイティブバイナリの実行を防止します。`cargo check`、`cargo test`、`cargo clippy`はローカルで実行する必要があります。
2. **`pnpm tauri dev`**: Rustコンパイルに依存。ローカルで実行する必要があります。
3. **アプリケーションアイコンなし**: プレースホルダーディレクトリのみ。リリースビルド前にアイコンが必要です。
4. **実際のAI統合なし**: エージェントコマンドはスタブを返します。実際の統合はフェーズ7で。
5. **テストがまだ書かれていない**: Vitestフレームワークは設定済みですが、テストファイルはありません。

---

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルをご覧ください。

---

## 謝辞

AlphaForgeは以下のオープンソースプロジェクトによって可能になりました：

- [Tauri](https://tauri.app) - デスクトップアプリケーションフレームワーク
- [React](https://react.dev) - UIライブラリ
- [Rust](https://www.rust-lang.org) - システムプログラミング言語
- [shadcn/ui](https://ui.shadcn.com) - UIコンポーネントライブラリ

---

<p align="center">
  Built with ❤️ by the AlphaForge team
</p>