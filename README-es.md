# AlphaForge (Investment OS)

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)
[![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)](CHANGELOG.md)
[![Made with Tauri](https://img.shields.io/badge/Made%20with-Tauri-24C8DB.svg)](https://tauri.app)

**Espacio de trabajo AI primero para escritorio para investigación de inversiones** 🚀

[English](README.md) | [简体中文](README-zh_CN.md) | [日本語](README-ja.md) | [한국어](README-ko.md) | [Español](README-es.md)

---

## 🎯 ¿Qué es AlphaForge?

AlphaForge es un **espacio de trabajo de investigación de inversión nativo de AI** diseñado para transformar información cruda en conocimiento de inversión estructurado.

### Ciclo de Producto Principal

```text
Información → Conocimiento → Tesis → Decisión → Validación → Revisión → Mejora
```

AlphaForge te ayuda a:
- 📊 **Investigar eficientemente** — Análisis de documentos asistido por AI y recopilación de información
- 💡 **Construir tesis** — Seguir tesis de inversión con evidencia y niveles de confianza
- 📈 **Tomar decisiones informadas** — Flujo de trabajo de investigación estructurado, no interacción estilo chatbot
- ✅ **Validar resultados** — Seguir el rendimiento de tesis y aprender de los resultados

> **⚠️ Importante**: Este es un **espacio de trabajo de investigación**, no un terminal de corretaje. No ejecuta operaciones ni toma decisiones de inversión autónomas.

---

## 📋 Tabla de Contenidos

- [Estado](#estado)
- [Características](#características)
- [Capturas de Pantalla](#capturas-de-pantalla)
- [Stack Tecnológico](#stack-tecnológico)
- [Inicio Rápido](#inicio-rápido)
- [Arquitectura](#arquitectura)
- [Documentación](#documentación)
- [Contribuir](#contribuir)
- [Hoja de Ruta](#hoja-de-ruta)
- [Seguridad](#seguridad)
- [Limitaciones Actuales](#limitaciones-actuales)
- [Licencia](#licencia)

---

## 📊 Estado

**Fase 1.5 — Fundación de Aplicación** (en progreso)

| Hito | Estado | Descripción |
|------|--------|-------------|
| M0 | ✅ Completado | Fundación del Proyecto |
| M1 | ✅ Completado | Fundación de Runtime de Escritorio |
| M1.5 | 🚧 En Progreso | Fundación de Aplicación |
| M2 | 📋 Planificado | Runtime de Agente |
| M3 | 📋 Planificado | Sistema de Inteligencia de Artefactos |
| M4 | 📋 Planificado | Espacio de Trabajo de Investigación |
| M5 | 📋 Planificado | Sistema de Conocimiento de Inversión |
| M6 | 📋 Planificado | Inteligencia de Portafolio |
| M7 | 📋 Planificado | Ecosistema de Plugins |
| M8 | 📅 Futuro | Producción & Comercialización |

Ver [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) para hitos detallados.

---

## ✨ Características

### Actuales (M0-M1.5)
- ✅ Shell de aplicación de escritorio Tauri 2
- ✅ Fundación React 19 + TypeScript + Vite
- ✅ Backend Rust con persistencia SQLite
- ✅ Capa de comunicación IPC
- ✅ Documentación comprensiva (17+ documentos)

### Planificadas (M2+)
- 📋 Asistencia de investigación impulsada por AI
- 📋 Seguimiento de tesis de inversión
- 📋 Artefactos interactivos (gráficos, tablas, visualizaciones)
- 📋 Análisis de documentos y búsqueda semántica
- 📋 Alineación portafolio-tesis
- 📋 Ecosistema de plugins

---

## 🖼️ Capturas de Pantalla

> **Nota**: AlphaForge está en desarrollo temprano (M1.5). Las capturas de pantalla se agregarán a medida que progrese el desarrollo de UI.

---

## 🛠️ Stack Tecnológico

| Capa | Tecnología |
|------|-----------|
| **Shell de Escritorio** | Tauri 2 |
| **Backend** | Rust, Tokio, SQLx, SQLite |
| **Frontend** | React 19, TypeScript, Vite 6 |
| **UI** | Tailwind CSS 4, shadcn/ui, Radix UI, Lucide |
| **AI** | OpenAI API (planificado) |
| **Calidad** | ESLint, Prettier, Vitest, Rustfmt, Clippy |

---

## 🚀 Inicio Rápido

### Requisitos Previos

- Rust stable (toolchain MSVC en Windows)
- Node.js 22+
- pnpm 9+

### Comandos de Desarrollo

```bash
# Instalar dependencias
pnpm install

# Desarrollo frontend
pnpm dev:web          # Iniciar servidor de desarrollo Vite (solo frontend)
pnpm typecheck        # Verificación de tipos TypeScript
pnpm lint             # ESLint
pnpm test             # Vitest

# Desarrollo de escritorio (requiere Rust)
pnpm tauri dev        # Iniciar app de escritorio Tauri completa
pnpm tauri build      # Build de producción

# Comandos Rust (requiere Rust)
cargo check --workspace
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --workspace
```

---

## 🏗️ Arquitectura

Ver [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) para el documento de arquitectura completo.

### Límites Clave

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

**React** gestiona:
- Páginas, componentes, interacción
- Estado del frontend
- Interfaz de usuario

**Rust** gestiona:
- Runtime de agente
- Base de datos SQLite
- Acceso a sistema de archivos y red
- Gestión de credenciales

**Tauri** gestiona:
- Ventanas de escritorio
- Comunicación IPC
- Permisos y seguridad
- Integración con OS

---

## 📚 Documentación

### Documentos Principales

| Documento | Propósito |
|-----------|-----------|
| [AGENTS.md](AGENTS.md) | Estándares de codificación de agentes (**lectura obligatoria**) |
| [PRODUCT.md](docs/PRODUCT.md) | Posicionamiento del producto y alcance MVP |
| [VISION.md](docs/VISION.md) | Dirección a largo plazo |
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Arquitectura del sistema |
| [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) | Hitos del producto |

### Documentación Técnica

| Documento | Propósito |
|-----------|-----------|
| [AGENT_PROTOCOL.md](docs/AGENT_PROTOCOL.md) | Ciclo de vida de tareas de agente |
| [ARTIFACT_SYSTEM.md](docs/ARTIFACT_SYSTEM.md) | Renderizado de artefactos |
| [PLUGIN_SPEC.md](docs/PLUGIN_SPEC.md) | Desarrollo de plugins |
| [DATA_MODEL.md](docs/DATA_MODEL.md) | Relaciones de entidades |
| [SECURITY.md](SECURITY.md) | Política de seguridad |

### Guías de Desarrollo

| Documento | Propósito |
|-----------|-----------|
| [CONTRIBUTING.md](CONTRIBUTING.md) | Guía de contribución |
| [GIT_WORKFLOW.md](docs/GIT_WORKFLOW.md) | Flujo de trabajo Git y PR |
| [PR_BEST_PRACTICES.md](docs/PR_BEST_PRACTICES.md) | Directrices de PR |
| [DEVELOPMENT.md](docs/DEVELOPMENT.md) | Guía de configuración local |

---

## 🤝 Contribuir

¡Bienvenidas las contribuciones!

### 🔒 Aviso de Protección de Branch

**La branch main está protegida. Los push directos están BLOQUEADOS.**

Todos los cambios deben pasar por Pull Request:
1. Crear branch de feature
2. Hacer cambios y commit
3. Crear Pull Request
4. Obtener al menos 1 aprobación
5. Hacer merge a main

Ver [CONTRIBUTING.md](CONTRIBUTING.md) para el flujo de trabajo detallado.

### Inicio Rápido

1. Leer [AGENTS.md](AGENTS.md) (**requerido**)
2. Revisar [CONTRIBUTING.md](CONTRIBUTING.md)
3. Hacer fork, crear branch, enviar PR

Todas las contribuciones deben seguir nuestro [Código de Conducta](CODE_OF_CONDUCT.md).

---

## 🗺️ Hoja de Ruta

### Timeline de Desarrollo

```
M0 → M1 → M1.5 → M2 → M3 → M4 → M5 → M6 → M7 → M8
✅    ✅    🚧     📋    📋    📋    📋    📋    📋    📅
```

### Resumen de Fases

**Fundación (M0-M1.5)**:
- Configuración del proyecto
- Runtime de escritorio
- Fundación de aplicación

**Inteligencia (M2-M3)**:
- Runtime de agente
- Integración AI
- Sistema de artefactos

**Características (M4-M6)**:
- Espacio de trabajo de investigación
- Seguimiento de tesis
- Análisis de portafolio

**Extensibilidad (M7-M8)**:
- Ecosistema de plugins
- Release de producción

Ver [MILESTONE_ROADMAP.md](docs/MILESTONE_ROADMAP.md) para detalles.

---

## 🔐 Seguridad

La seguridad es una prioridad máxima. Ver [SECURITY.md](SECURITY.md) para:
- Proceso de reporte de vulnerabilidades
- Arquitectura de seguridad
- Gestión de credenciales
- Modelo de permisos

**Reportes**: Por favor reporta problemas de seguridad de forma privada vía GitHub Security.

---

## ⚠️ Limitaciones Actuales

1. **Compilación de Rust en sandbox**: `cargo check/test/clippy` debe ejecutarse localmente
2. **`pnpm tauri dev`**: Requiere compilación local de Rust
3. **Sin iconos de aplicación**: Solo placeholders
4. **Sin integración de AI real**: Los comandos de agente devuelven stubs
5. **Sin pruebas escritas aún**: Framework configurado, sin archivos de prueba

---

## 📄 Licencia

Este proyecto está licenciado bajo la **GNU Affero General Public License v3.0 (AGPLv3)** - ver el archivo [LICENSE](LICENSE) para detalles.

### ¿Por qué AGPLv3?

AGPLv3 asegura que:
- ✅ Todas las modificaciones deben ser compartidas con la comunidad
- ✅ El uso en red (SaaS) activa los requisitos de copyleft
- ✅ Los usuarios siempre tienen acceso al código fuente
- ✅ Se permite el uso comercial con licencia adecuada

Esto protege la naturaleza de código abierto de AlphaForge mientras permite desarrollo sostenible.

---

## 🙏 Agradecimientos

AlphaForge es posible gracias a estos proyectos de código abierto:

- [Tauri](https://tauri.app) - Framework de aplicación de escritorio
- [React](https://react.dev) - Biblioteca UI
- [Rust](https://www.rust-lang.org) - Lenguaje de programación de sistemas
- [shadcn/ui](https://ui.shadcn.com) - Biblioteca de componentes UI
- [Tailwind CSS](https://tailwindcss.com) - Framework CSS

---

## 📞 Contacto

- **Issues**: [GitHub Issues](https://github.com/BerryUIKI/alpha-forge/issues)
- **Discussions**: [GitHub Discussions](https://github.com/BerryUIKI/alpha-forge/discussions)

---

<p align="center">
  <strong>Construido con ❤️ por el equipo de AlphaForge</strong>
</p>

<p align="center">
  <sub>Transformando información en inteligencia de inversión</sub>
</p>