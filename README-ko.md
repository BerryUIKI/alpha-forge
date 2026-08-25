# AlphaForge (AlphaForge)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**투자 연구를 위한 데스크톱 우선 AI 워크스페이스** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## 🎯 AlphaForge란?

AlphaForge는 원시 정보를 구조화된 투자 지식으로 변환하도록 설계된 **AI 네이티브 투자 연구 워크스페이스**입니다.

### 핵심 제품 루프

```text
정보 → 지식 → 가설 → 의사결정 → 검증 → 리뷰 → 개선
```

AlphaForge는 다음을 도와줍니다:
- 📊 **효율적인 연구** — AI 지원 문서 분석 및 정보 수집
- 💡 **가설 구축** — 증거와 신뢰도 수준을 가진 투자 가설 추적
- 📈 **정보에 입각한 의사결정** — 챗봇 스타일 상호작용이 아닌 구조화된 연구 워크플로우
- ✅ **결과 검증** — 가설 성과 추적 및 결과로부터 학습

> **⚠️ 중요**: 이것은 **연구 워크스페이스**이며, 브로커리지 터미널이 아닙니다. 거래를 실행하거나 자율적인 투자 결정을 내리지 않습니다.

---

## 📋 목차

- [상태](#상태)
- [기능](#기능)
- [스크린샷](#스크린샷)
- [기술 스택](#기술-스택)
- [시작하기](#시작하기)
- [아키텍처](#아키텍처)
- [문서](#문서)
- [기여하기](#기여하기)
- [로드맵](#로드맵)
- [보안](#보안)
- [현재 제한사항](#현재-제한사항)
- [라이선스](#라이선스)

---

## 📊 상태

**단계 1.5 — 애플리케이션 기반** (진행 중)

| 마일스톤 | 상태 | 설명 |
|---------|------|------|
| M0 | ✅ 완료 | 프로젝트 기반 |
| M1 | ✅ 완료 | 데스크톱 런타임 기반 |
| M1.5 | 🚧 진행 중 | 애플리케이션 기반 |
| M2 | 📋 계획 중 | 에이전트 런타임 |
| M3 | 📋 계획 중 | 아티팩트 인텔리전스 시스템 |
| M4 | 📋 계획 중 | 연구 워크스페이스 |
| M5 | 📋 계획 중 | 투자 지식 시스템 |
| M6 | 📋 계획 중 | 포트폴리오 인텔리전스 |
| M7 | 📋 계획 중 | 플러그인 생태계 |
| M8 | 📅 향후 | 프로덕션 & 상용화 |

상세 마일스톤은 [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)를 참조하세요.

---

## ✨ 기능

### 현재 (M0-M1.5)
- ✅ Tauri 2 데스크톱 애플리케이션 쉘
- ✅ React 19 + TypeScript + Vite 기반
- ✅ Rust 백엔드와 SQLite 지속성
- ✅ IPC 통신 레이어
- ✅ 포괄적인 문서 (17+ 문서)

### 계획 중 (M2+)
- 📋 AI 기반 연구 지원
- 📋 투자 가설 추적
- 📋 인터랙티브 아티팩트 (차트, 테이블, 시각화)
- 📋 문서 분석 및 시맨틱 검색
- 📋 포트폴리오-가설 정렬
- 📋 플러그인 생태계

---

## 🖼️ 스크린샷

> **참고**: AlphaForge는 초기 개발 단계(M1.5)입니다. UI 개발이 진행됨에 따라 스크린샷이 추가됩니다.

---

## 🛠️ 기술 스택

| 레이어 | 기술 |
|-------|------|
| **데스크톱 쉘** | Tauri 2 |
| **백엔드** | Rust, Tokio, SQLx, SQLite |
| **프론트엔드** | React 19, TypeScript, Vite 6 |
| **UI** | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| **AI** | OpenAI API (계획 중) |
| **품질** | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## 🚀 시작하기

### 필수 요구사항

- Rust stable (Windows에서는 MSVC 툴체인)
- Node.js 22+
- pnpm 9+

### 개발 명령어

```bash
# 의존성 설치
pnpm install

# 프론트엔드 개발
pnpm dev:web          # Vite 개발 서버 시작 (프론트엔드만)
pnpm typecheck        # TypeScript 타입 체크
pnpm lint             # ESLint
pnpm test             # Vitest

# 데스크톱 개발 (Rust 필요)
pnpm tauri dev        # 전체 Tauri 데스크톱 앱 시작
pnpm tauri build      # 프로덕션 빌드

# Rust 명령어 (Rust 필요)
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 🏗️ 아키텍처

전체 아키텍처 문서는 [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)를 참조하세요.

### 주요 경계

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

**React** 관리:
- 페이지, 컴포넌트, 상호작용
- 프론트엔드 상태
- 사용자 인터페이스

**Rust** 관리:
- 에이전트 런타임
- SQLite 데이터베이스
- 파일시스템 & 네트워크 액세스
- 자격증명 관리

**Tauri** 관리:
- 데스크톱 윈도우
- IPC 통신
- 권한 & 보안
- OS 통합

---

## 📚 문서

### 핵심 문서

| 문서 | 목적 |
|-----|------|
| [AGENTS.md](AGENTS.md) | 에이전트 코딩 표준 (**필독**) |
| [PRODUCT.md](docs/PRODUCT.md) | 제품 포지셔닝 및 MVP 범위 |
| [VISION.md](docs/VISION.md) | 장기 방향성 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | 시스템 아키텍처 |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | 제품 마일스톤 |

### 기술 문서

| 문서 | 목적 |
|-----|------|
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | 에이전트 태스크 수명주기 |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | 아티팩트 렌더링 |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | 플러그인 개발 |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | 엔티티 관계 |
| [SECURITY.md](SECURITY.md) | 보안 정책 |

### 개발 가이드

| 문서 | 목적 |
|-----|------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | 기여 가이드 |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Git 및 PR 워크플로우 |
| [PR_BEST_PRACTICES.md](docs/PR_BEST_PRACTICES.md) | PR 가이드라인 |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | 로컬 설정 가이드 |

---

## 🤝 기여하기

기여를 환영합니다!

### 🔒 브랜치 보호 공지

**main 브랜치는 보호됩니다. 직접 푸시는 차단됩니다.**

모든 변경은 Pull Request를 통해야 합니다:
1. 기능 브랜치 생성
2. 변경 및 커밋
3. Pull Request 생성
4. 최소 1개의 승인 획득
5. main에 병합

상세 워크플로우는 [CONTRIBUTING.md](CONTRIBUTING.md)를 참조하세요.

### 빠른 시작

1. [AGENTS.md](AGENTS.md) 읽기 (**필수**)
2. [CONTRIBUTING.md](CONTRIBUTING.md) 확인
3. 포크, 브랜치 생성, PR 제출

모든 기여는 [행동 강령](CODE_OF_CONDUCT.md)을 따라야 합니다.

---

## 🗺️ 로드맵

### 개발 타임라인

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

### 단계 개요

**기반 (M0-M1.5)**:
- 프로젝트 설정
- 데스크톱 런타임
- 애플리케이션 기반

**인텔리전스 (M2-M3)**:
- 에이전트 런타임
- AI 통합
- 아티팩트 시스템

**기능 (M4-M6)**:
- 연구 워크스페이스
- 가설 추적
- 포트폴리오 분석

**확장성 (M7-M8)**:
- 플러그인 생태계
- 프로덕션 릴리스

상세 내용은 [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)를 참조하세요.

---

## 🔐 보안

보안은 최우선 사항입니다. [SECURITY.md](SECURITY.md)를 참조하세요:
- 취약점 보고 프로세스
- 보안 아키텍처
- 자격증명 관리
- 권한 모델

**보고**: 보안 문제는 GitHub Security를 통해 비공개로 보고해 주세요.

---

## ⚠️ 현재 제한사항

1. **샌드박스 내 Rust 컴파일**: `cargo check/test/clippy`는 로컬에서 실행해야 합니다
2. **`pnpm tauri dev`**: 로컬 Rust 컴파일 필요
3. **애플리케이션 아이콘 없음**: 플레이스홀더만 존재
4. **실제 AI 통합 없음**: 에이전트 명령이 스텁을 반환합니다
5. **아직 작성된 테스트 없음**: 프레임워크는 설정됨, 테스트 파일 없음

---

## 📄 라이선스

이 프로젝트는 **GNU Affero General Public License v3.0 (AGPLv3)** 하에 라이선스됩니다 - 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

### 왜 AGPLv3?

AGPLv3은 다음을 보장합니다:
- ✅ 모든 수정 사항은 커뮤니티에 공유되어야 합니다
- ✅ 네트워크 사용(SaaS)은 카피레프트 요건을 트리거합니다
- ✅ 사용자는 항상 소스 코드에 액세스할 수 있습니다
- ✅ 적절한 라이선스로 상업적 사용이 가능합니다

이는 AlphaForge의 오픈소스 성격을 보호하면서 지속 가능한 개발을 가능하게 합니다.

---

## 🙏 감사의 말

AlphaForge는 다음 오픈소스 프로젝트 덕분에 가능했습니다:

- [Tauri](https://tauri.app) - 데스크톱 애플리케이션 프레임워크
- [React](https://react.dev) - UI 라이브러리
- [Rust](https://www.rust-lang.org) - 시스템 프로그래밍 언어
- [shadcn/ui](https://ui.shadcn.com) - UI 컴포넌트 라이브러리
- [Tailwind CSS](https://tailwindcss.com) - CSS 프레임워크

---

## 📞 연락처

- **Issues**: [GitHub Issues](https://github.com/BerryUIKI/alpha-forge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions)

---

<p align="center">
  <strong>AlphaForge 팀이 ❤️로 제작</strong>
</p>

<p align="center">
  <sub>정보를 투자 인텔리전스로 변환</sub>
</p>