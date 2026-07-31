# AlphaForge (Investment OS)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**投資研究のためのデスクトップファーストAIワークスペース** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## 🎯 AlphaForgeとは？

AlphaForgeは、生の情報を構造化された投資知識に変換するために設計された**AIネイティブの投資研究ワークスペース**です。

### コアプロダクトループ

```text
情報 → 知識 → 仮説 → 意思決定 → 検証 → レビュー → 改善
```

AlphaForgeは以下を支援します：
- 📊 **効率的な研究** — AI支援によるドキュメント分析と情報収集
- 💡 **仮説の構築** — 証拠と信頼度レベルを持つ投資仮説を追跡
- 📈 **十分な情報に基づいた意思決定** — チャットボットスタイルのやり取りではなく、構造化された研究ワークフロー
- ✅ **結果の検証** — 仮説のパフォーマンスを追跡し、結果から学ぶ

> **⚠️ 重要**: これは**研究ワークスペース**であり、ブローカージターミナルではありません。取引を実行したり、自律的な投資決定を行ったりすることはありません。

---

## 📋 目次

- [ステータス](#ステータス)
- [機能](#機能)
- [スクリーンショット](#スクリーンショット)
- [技術スタック](#技術スタック)
- [はじめに](#はじめに)
- [アーキテクチャ](#アーキテクチャ)
- [ドキュメント](#ドキュメント)
- [コントリビュート](#コントリビュート)
- [ロードマップ](#ロードマップ)
- [セキュリティ](#セキュリティ)
- [現在の制限](#現在の制限)
- [ライセンス](#ライセンス)

---

## 📊 ステータス

**フェーズ 1.5 — アプリケーション基盤**（進行中）

| マイルストーン | ステータス | 説明 |
|--------------|----------|------|
| M0 | ✅ 完了 | プロジェクト基盤 |
| M1 | ✅ 完了 | デスクトップランタイム基盤 |
| M1.5 | 🚧 進行中 | アプリケーション基盤 |
| M2 | 📋 計画中 | エージェントランタイム |
| M3 | 📋 計画中 | アーティファクトインテリジェンスシステム |
| M4 | 📋 計画中 | 研究ワークスペース |
| M5 | 📋 計画中 | 投資知識システム |
| M6 | 📋 計画中 | ポートフォリオインテリジェンス |
| M7 | 📋 計画中 | プラグインエコシステム |
| M8 | 📅 将来 | 本番稼働 & 商業化 |

詳細なマイルストーンについては、[MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)をご覧ください。

---

## ✨ 機能

### 現在 (M0-M1.5)
- ✅ Tauri 2 デスクトップアプリケーションシェル
- ✅ React 19 + TypeScript + Vite 基盤
- ✅ Rust バックエンドと SQLite 永続化
- ✅ IPC 通信レイヤー
- ✅ 包括的なドキュメント (17+ ドキュメント)

### 計画中 (M2+)
- 📋 AI駆動の研究支援
- 📋 投資仮説追跡
- 📋 インタラクティブアーティファクト (チャート、テーブル、ビジュアライゼーション)
- 📋 ドキュメント分析と意味検索
- 📋 ポートフォリオ-仮説アライメント
- 📋 プラグインエコシステム

---

## 🖼️ スクリーンショット

> **注意**: AlphaForgeは初期開発段階（M1.5）です。UI開発が進むにつれてスクリーンショットが追加されます。

---

## 🛠️ 技術スタック

| レイヤー | 技術 |
|---------|------|
| **デスクトップシェル** | Tauri 2 |
| **バックエンド** | Rust, Tokio, SQLx, SQLite |
| **フロントエンド** | React 19, TypeScript, Vite 6 |
| **UI** | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| **AI** | OpenAI API（計画中） |
| **品質** | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## 🚀 はじめに

### 前提条件

- Rust stable（WindowsではMSVCツールチェーン）
- Node.js 22+
- pnpm 9+

### 開発コマンド

```bash
# 依存関係のインストール
pnpm install

# フロントエンド開発
pnpm dev:web          # Vite開発サーバーを起動（フロントエンドのみ）
pnpm typecheck        # TypeScript型チェック
pnpm lint             # ESLint
pnpm test             # Vitest

# デスクトップ開発（Rustが必要）
pnpm tauri dev        # 完全なTauriデスクトップアプリを起動
pnpm tauri build      # 本番ビルド

# Rustコマンド（Rustが必要）
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 🏗️ アーキテクチャ

詳細なアーキテクチャドキュメントについては、[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)をご覧ください。

### 主な境界

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

**React**が管理：
- ページ、コンポーネント、相互作用
- フロントエンド状態
- ユーザーインターフェース

**Rust**が管理：
- エージェントランタイム
- SQLite データベース
- ファイルシステム & ネットワークアクセス
- 認証情報管理

**Tauri**が管理：
- デスクトップウィンドウ
- IPC 通信
- 権限 & セキュリティ
- OS 統合

---

## 📚 ドキュメント

### コアドキュメント

| ドキュメント | 目的 |
|------------|------|
| [AGENTS.md](AGENTS.md) | エージェントコーディング標準（**必読**） |
| [PRODUCT.md](docs/PRODUCT.md) | 製品ポジショニングとMVPスコープ |
| [VISION.md](docs/VISION.md) | 長期的方向性 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | システムアーキテクチャ |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | 製品マイルストーン |

### 技術ドキュメント

| ドキュメント | 目的 |
|------------|------|
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | エージェントタスクライフサイクル |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | アーティファクトレンダリング |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | プラグイン開発 |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | エンティティ関係 |
| [SECURITY.md](SECURITY.md) | セキュリティポリシー |

### 開発ガイド

| ドキュメント | 目的 |
|------------|------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | コントリビューションガイド |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | GitとPRワークフロー |
| [PR_BEST_PRACTICES.md](docs/PR_BEST_PRACTICES.md) | PRガイドライン |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | ローカルセットアップガイド |

---

## 🤝 コントリビュート

コントリビュートを歓迎します！

### 🔒 ブランチ保護に関するお知らせ

**mainブランチは保護されています。直接プッシュはブロックされます。**

すべての変更はPull Requestを通じて行う必要があります：
1. 機能ブランチを作成
2. 変更を行いコミット
3. Pull Requestを作成
4. 少なくとも1つの承認を得る
5. mainにマージ

詳細なワークフローについては、[CONTRIBUTING.md](CONTRIBUTING.md)をご覧ください。

### クイックスタート

1. [AGENTS.md](AGENTS.md)を読む（**必須**）
2. [CONTRIBUTING.md](CONTRIBUTING.md)を確認
3. フォーク、ブランチ作成、PR提出

すべてのコントリビュートは[行動規範](CODE_OF_CONDUCT.md)に従う必要があります。

---

## 🗺️ ロードマップ

### 開発タイムライン

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

### フェーズ概要

**基盤 (M0-M1.5)**：
- プロジェクトセットアップ
- デスクトップランタイム
- アプリケーション基盤

**インテリジェンス (M2-M3)**：
- エージェントランタイム
- AI統合
- アーティファクトシステム

**機能 (M4-M6)**：
- 研究ワークスペース
- 仮説追跡
- ポートフォリオ分析

**拡張性 (M7-M8)**：
- プラグインエコシステム
- 本番リリース

詳細については、[MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)をご覧ください。

---

## 🔐 セキュリティ

セキュリティは最優先事項です。[SECURITY.md](SECURITY.md)をご覧ください：
- 脆弱性報告プロセス
- セキュリティアーキテクチャ
- 認証情報管理
- 権限モデル

**報告**: セキュリティの問題はGitHub Securityを通じて非公開で報告してください。

---

## ⚠️ 現在の制限

1. **サンドボックス内でのRustコンパイル**: `cargo check/test/clippy`はローカルで実行する必要があります
2. **`pnpm tauri dev`**: ローカルRustコンパイルが必要
3. **アプリケーションアイコンなし**: プレースホルダーのみ
4. **実際のAI統合なし**: エージェントコマンドはスタブを返します
5. **テストがまだ書かれていない**: フレームワークは設定済み、テストファイルなし

---

## 📄 ライセンス

このプロジェクトは**GNU Affero General Public License v3.0 (AGPLv3)**の下でライセンスされています - 詳細は[LICENSE](LICENSE)ファイルをご覧ください。

### なぜAGPLv3？

AGPLv3は以下を保証します：
- ✅ すべての変更はコミュニティに共有する必要があります
- ✅ ネットワーク使用（SaaS）はコピーレフト要件をトリガーします
- ✅ ユーザーは常にソースコードにアクセスできます
- ✅ 適切なライセンスで商業利用が可能です

これはAlphaForgeのオープンソースの性質を保護しながら、持続可能な開発を可能にします。

---

## 🙏 謝辞

AlphaForgeは以下のオープンソースプロジェクトによって可能になりました：

- [Tauri](https://tauri.app) - デスクトップアプリケーションフレームワーク
- [React](https://react.dev) - UIライブラリ
- [Rust](https://www.rust-lang.org) - システムプログラミング言語
- [shadcn/ui](https://ui.shadcn.com) - UIコンポーネントライブラリ
- [Tailwind CSS](https://tailwindcss.com) - CSSフレームワーク

---

## 📞 連絡先

- **Issues**: [GitHub Issues](https://github.com/BerryUIKI/alpha-forge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions)

---

<p align="center">
  <strong>AlphaForgeチームが❤️で構築</strong>
</p>

<p align="center">
  <sub>情報を投資インテリジェンスに変換</sub>
</p>