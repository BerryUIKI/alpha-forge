# AlphaForge (Investment OS)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**Espacio de trabajo AI primero para escritorio para investigación de inversiones** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## Ciclo de Producto Principal

```text
Información → Conocimiento → Tesis → Decisión → Validación → Revisión → Mejora
```

AlphaForge transforma información cruda en conocimiento de inversión estructurado, convirtiendo la investigación en tesis comprobables y decisiones informadas.

> **⚠️ Importante**: Este es un **espacio de trabajo de investigación**, no un terminal de corretaje. No ejecuta operaciones ni toma decisiones de inversión autónomas.

---

## Tabla de Contenidos

- [Estado](#estado)
- [Capturas de Pantalla](#capturas-de-pantalla)
- [Stack Tecnológico](#stack-tecnológico)
- [Inicio Rápido](#inicio-rápido)
- [Arquitectura](#arquitectura)
- [Documentación](#documentación)
- [Contribuir](#contribuir)
- [Hoja de Ruta](#hoja-de-ruta)
- [Limitaciones Actuales](#limitaciones-actuales)
- [Licencia](#licencia)

---

## Estado

**Fase 1.5 — Fundación de Aplicación** (en progreso)

| Hito | Estado | Descripción |
|------|--------|-------------|
| M0 | ✅ Completado | Fundación del Proyecto |
| M1 | ✅ Completado | Fundación de Runtime de Escritorio |
| M1.5 | 🚧 En Progreso | Fundación de Aplicación |
| M2-M7 | 📋 Planificado | Runtime de Agente → Ecosistema de Plugins |
| M8 | 📅 Futuro | Producción & Comercialización |

Ver [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) para hitos detallados.

---

## Capturas de Pantalla

> **Nota**: AlphaForge está en desarrollo temprano (M1.5). Las capturas de pantalla se agregarán a medida que progrese el desarrollo de UI.

---

## Stack Tecnológico

| Capa | Tecnología |
|------|-----------|
| Shell de Escritorio | Tauri 2 |
| Backend | Rust, Tokio, SQLx, SQLite |
| Frontend | React 19, TypeScript, Vite 6 |
| UI | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| AI | OpenAI API (futuro) |
| Calidad | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## Inicio Rápido

### Requisitos Previos

- Rust stable (toolchain MSVC en Windows)
- Node.js 22+
- pnpm 9+

### Comandos de Desarrollo

```bash
pnpm install          # Instalar todas las dependencias
pnpm dev:web          # Iniciar servidor de desarrollo Vite (solo frontend)
pnpm typecheck        # Verificación de tipos TypeScript (todos los paquetes)
pnpm lint             # ESLint
pnpm format:check     # Verificación de formato Prettier
pnpm format           # Auto-corrección de Prettier
pnpm test             # Vitest
```

### Desarrollo Tauri (requiere Rust local)

```bash
pnpm tauri dev        # Iniciar app de escritorio Tauri completa
pnpm tauri build      # Build de producción
```

### Comandos Rust (requiere Rust local)

```bash
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## Arquitectura

Ver [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) para el documento de arquitectura completo.

Límites clave:

- **React** gestiona páginas, componentes, interacción, estado del frontend.
- **Rust** gestiona runtime de agente, SQLite, sistema de archivos, red, credenciales.
- **Tauri** gestiona ventanas, IPC, permisos, integración con OS.

---

## Documentación

| Documento | Propósito |
|-----------|-----------|
| [AGENTS.md](AGENTS.md) | Estándares y reglas de codificación de agentes (máxima prioridad) |
| [PRODUCT.md](docs/PRODUCT.md) | Posicionamiento del producto, usuarios objetivo, alcance MVP |
| [VISION.md](docs/VISION.md) | Dirección a largo plazo y filosofía de diseño |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Límites del sistema, responsabilidades de componentes, flujo IPC |
| [SYSTEM_DESIGN.md](docs/SYSTEM_DESIGN.md) | Nueve subsistemas — propósito, entradas, salidas, dependencias |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | Entidades conceptuales, relaciones, ciclos de vida |
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | Ciclo de vida de tareas de agente, uso de herramientas, salida estructurada, eventos |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | Concepto de artefacto, modelo de renderizado, modelo de permisos |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | Manifiesto de plugin, versionado, permisos, ciclo de vida |
| [SECURITY.md](SECURITY.md) | Política de seguridad, almacenamiento de credenciales, reporte de vulnerabilidades |
| [UI_GUIDELINES.md](docs/UI_GUIDELINES.md) | Sistema de diseño, estados de UI requeridos, patrones de navegación |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | Configuración local, comandos de desarrollo, flujo de trabajo de agente, solución de problemas |
| [ROADMAP.md](docs/ROADMAP.md) | Hoja de ruta de desarrollo de 12 fases (fases técnicas) |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | Hitos del producto, entregables y criterios de aceptación |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Estrategia de branches, convenciones de commit, proceso de PR |
| [PROJECT_BOOTSTRAP.md](docs/PROJECT_BOOTSTRAP.md) | Plan de inicialización completo (10 fases de implementación) |
| [DECISIONS/](docs/DECISIONS/) | Registros de Decisiones de Arquitectura (3 ADRs) |

---

## Contribuir

¡Gracias por tu interés en contribuir!

### Inicio Rápido

1. Clonar el repositorio
2. Leer [AGENTS.md](AGENTS.md) (**requerido**)
3. Revisar [CONTRIBUTING.md](CONTRIBUTING.md)
4. Crear branch de feature y enviar PR

Todas las contribuciones deben seguir nuestro [Código de Conducta](CODE_OF_CONDUCT.md).

---

## Hoja de Ruta

AlphaForge se está desarrollando en 9 hitos:

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

- **M0-M1**: Fundación (Completado)
- **M1.5**: Fundación de Aplicación (Actual)
- **M2-M7**: Inteligencia & Funciones (Planificado)
- **M8**: Producción & Comercialización (Futuro)

Ver [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) para hitos detallados.

---

## Limitaciones Actuales

1. **Compilación de Rust en sandbox**: El sandbox de WorkBuddy impide la ejecución de binarios nativos. `cargo check`, `cargo test`, `cargo clippy` deben ejecutarse localmente.
2. **`pnpm tauri dev`**: Depende de la compilación de Rust. Debe ejecutarse localmente.
3. **Sin iconos de aplicación**: Solo directorios de marcador de posición. Se necesitan iconos antes del build de release.
4. **Sin integración de AI real**: Los comandos de agente devuelven stubs. Integración real en la Fase 7.
5. **Sin pruebas escritas aún**: Framework Vitest configurado pero sin archivos de prueba.

---

## Licencia

Este proyecto está licenciado bajo la Licencia MIT - ver el archivo [LICENSE](LICENSE) para detalles.

---

## Agradecimientos

AlphaForge es posible gracias a estos proyectos de código abierto:

- [Tauri](https://tauri.app) - Framework de aplicación de escritorio
- [React](https://react.dev) - Biblioteca UI
- [Rust](https://www.rust-lang.org) - Lenguaje de programación de sistemas
- [shadcn/ui](https://ui.shadcn.com) - Biblioteca de componentes UI
- [Tailwind CSS](https://tailwindcss.com) - Framework CSS

---

<p align="center">
  Construido con ❤️ por el equipo de AlphaForge
</p>