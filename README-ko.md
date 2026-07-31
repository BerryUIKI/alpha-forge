# AlphaForge (Investment OS)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**투자 연구를 위한 데스크톱 우선 AI 워크스페이스** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md)

---

## 핵심 제품 루프

```text
정보 → 지식 → 가설 → 의사결정 → 검증 → 리뷰 → 개선
```

AlphaForge는 원시 정보를 구조화된 투자 지식으로 변환하고, 연구를 검증 가능한 투자 가설과 정보에 입각한 의사결정으로 전환합니다.

> **⚠️ 중요**: 이것은 **연구 워크스페이스**이며, 브로커리지 터미널이 아닙니다. 거래를 실행하거나 자율적인 투자 결정을 내리지 않습니다.

---

## 목차

- [상태](#상태)
- [스크린샷](#스크린샷)
- [기술 스택](#기술-스택)
- [시작하기](#시작하기)
- [아키텍처](#아키텍처)
- [문서](#문서)
- [기여하기](#기여하기)
- [로드맵](#로드맵)
- [현재 제한사항](#현재-제한사항)
- [라이선스](#라이선스)

---

## 상태

**단계 1.5 — 애플리케이션 기반** (진행 중)

| 마일스톤 | 상태 | 설명 |
|---------|------|------|
| M0 | ✅ 완료 | 프로젝트 기반 |
| M1 | ✅ 완료 | 데스크톱 런타임 기반 |
| M1.5 | 🚧 진행 중 | 애플리케이션 기반 |
| M2-M7 | 📋 계획 중 | 에이전트 런타임 → 플러그인 생태계 |
| M8 | 📅 향후 | 프로덕션 & 상용화 |

상세 마일스톤은 [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)를 참조하세요.

---

## 스크린샷

> **참고**: AlphaForge는 초기 개발 단계(M1.5)입니다. UI 개발이 진행됨에 따라 스크린샷이 추가됩니다.

---

## 기술 스택

| 레이어 | 기술 |
|-------|------|
| 데스크톱 쉘 | Tauri 2 |
| 백엔드 | Rust, Tokio, SQLx, SQLite |
| 프론트엔드 | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| AI | OpenAI API (향후) |
| 품질 | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## 시작하기

### 필수 요구사항

- Rust stable (Windows에서는 MSVC 툴체인)
- Node.js 22+
- pnpm 9+

### 개발 명령어

```bash
pnpm install          # 모든 의존성 설치
pnpm dev:web          # Vite 개발 서버 시작 (프론트엔드만)
pnpm typecheck        # TypeScript 타입 체크 (모든 패키지)
pnpm lint             # ESLint
pnpm format:check     # Prettier 포맷 체크
pnpm format           # Prettier 자동 수정
pnpm test             # Vitest
```

### Tauri 개발 (로컬 Rust 환경 필요)

```bash
pnpm tauri dev        # 전체 Tauri 데스크톱 앱 시작
pnpm tauri build      # 프로덕션 빌드
```

### Rust 명령어 (로컬 Rust 환경 필요)

```bash
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 아키텍처

전체 아키텍처 문서는 [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)를 참조하세요.

주요 경계:

- **React** - 페이지, 컴포넌트, 상호작용, 프론트엔드 상태 관리
- **Rust** - 에이전트 런타임, SQLite, 파일시스템, 네트워크, 자격증명 관리
- **Tauri** - 윈도우, IPC, 권한, OS 통합 관리

---

## 문서

| 문서 | 목적 |
|-----|------|
| [AGENTS.md](AGENTS.md) | 에이전트 코딩 표준 및 규칙 (최우선 순위) |
| [PRODUCT.md](docs/PRODUCT.md) | 제품 포지셔닝, 타겟 사용자, MVP 범위 |
| [VISION.md](docs/VISION.md) | 장기 방향성 및 설계 철학 |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | 시스템 경계, 컴포넌트 책임, IPC 흐름 |
| [SYSTEM_DESIGN.md](docs/SYSTEM_DESIGN.md) | 9개 하위 시스템 — 목적, 입력, 출력, 의존성 |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | 개념적 엔티티, 관계, 수명주기 |
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | 에이전트 태스크 수명주기, 도구 사용, 구조화된 출력, 이벤트 |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | 아티팩트 개념, 렌더링 모델, 권한 모델 |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | 플러그인 매니페스트, 버전 관리, 권한, 수명주기 |
| [SECURITY.md](SECURITY.md) | 보안 정책, 자격증명 저장소, 취약점 보고 |
| [UI_GUIDELINES.md](docs/UI_GUIDELINES.md) | 디자인 시스템, 필수 UI 상태, 내비게이션 패턴 |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | 로컬 설정, 개발 명령어, 에이전트 워크플로우, 문제해결 |
| [ROADMAP.md](docs/ROADMAP.md) | 12단계 개발 로드맵 (기술 단계) |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | 제품 마일스톤, 산출물, 수락 기준 |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | 브랜치 전략, 커밋 규칙, PR 프로세스 |
| [PROJECT_BOOTSTRAP.md](docs/PROJECT_BOOTSTRAP.md) | 전체 초기화 계획 (10개 구현 단계) |
| [DECISIONS/](docs/DECISIONS/) | 아키텍처 결정 기록 (3개 ADR) |

---

## 기여하기

기여에 관심을 가져주셔서 감사합니다!

### 빠른 시작

1. 저장소 클론
2. [AGENTS.md](AGENTS.md) 읽기 (**필수**)
3. [CONTRIBUTING.md](CONTRIBUTING.md) 확인
4. 기능 브랜치 생성 후 PR 제출

모든 기여는 [행동 강령](CODE_OF_CONDUCT.md)을 따라야 합니다.

---

## 로드맵

AlphaForge는 9개 마일스톤으로 개발되고 있습니다:

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

- **M0-M1**: 기반 (완료)
- **M1.5**: 애플리케이션 기반 (현재)
- **M2-M7**: 인텔리전스 & 기능 (계획 중)
- **M8**: 프로덕션 & 상용화 (향후)

상세 마일스톤은 [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md)를 참조하세요.

---

## 현재 제한사항

1. **샌드박스 내 Rust 컴파일**: WorkBuddy 샌드박스가 네이티브 바이너리 실행을 방지합니다. `cargo check`, `cargo test`, `cargo clippy`는 로컬에서 실행해야 합니다.
2. **`pnpm tauri dev`**: Rust 컴파일에 의존. 로컬에서 실행해야 합니다.
3. **애플리케이션 아이콘 없음**: 플레이스홀더 디렉토리만 존재. 릴리스 빌드 전 아이콘이 필요합니다.
4. **실제 AI 통합 없음**: 에이전트 명령이 스텁을 반환합니다. 실제 통합은 단계 7에서.
5. **아직 작성된 테스트 없음**: Vitest 프레임워크는 설정되었지만 테스트 파일이 없습니다.

---

## 라이선스

이 프로젝트는 MIT 라이선스에 따라 라이선스가 부여됩니다 - 자세한 내용은 [LICENSE](LICENSE) 파일을 참조하세요.

---

## 감사의 말

AlphaForge는 다음 오픈소스 프로젝트 덕분에 가능했습니다:

- [Tauri](https://tauri.app) - 데스크톱 애플리케이션 프레임워크
- [React](https://react.dev) - UI 라이브러리
- [Rust](https://www.rust-lang.org) - 시스템 프로그래밍 언어
- [shadcn/ui](https://ui.shadcn.com) - UI 컴포넌트 라이브러리
- [Tailwind CSS](https://tailwindcss.com) - CSS 프레임워크

---

<p align="center">
  AlphaForge 팀이 ❤️로 제작
</p>