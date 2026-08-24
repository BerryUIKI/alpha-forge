# Goose Credential and Provider Policy (M10-G5)

## Overview

This document specifies the security controls, credential management mechanisms, and provider allowlisting policies governing the Goose integration in Investment OS (AlphaForge).

---

## 1. Credential Model

### 1.1 OS Keyring Vault
- **Service Identifier**: `alphaforge-goose`
- **Supported Platforms**:
  - **Windows**: Windows Credential Manager (`wincred`)
  - **macOS**: Apple Keychain Services (`security`)
  - **Linux**: Freedesktop Secret Service API / `keyutils`
- **Zero Plaintext Storage**: API keys are never stored in SQLite, plaintext `.json` / `.yaml` / `.env` files, or frontend memory state.

### 1.2 Plaintext Fallback Ban
- File-based secret fallback (e.g. `~/.config/goose/secrets.yaml`) is disabled for production releases.
- Attempting to pass API keys in CLI arguments or environment variable dumps is blocked.

---

## 2. Sensitive Redaction Rules

Rust process supervisor (`apps/desktop/src-tauri/src/goose/adapter.rs`) applies real-time regex sanitization on all output streams (stdout, stderr, crash logs):

1. **Bearer Tokens**: `Bearer [a-zA-Z0-9_\-\.]{16,}` -> `Bearer [REDACTED]`
2. **OpenAI Keys**: `sk-[a-zA-Z0-9_\-]{20,}` -> `sk-[REDACTED]`
3. **Anthropic Keys**: `sk-ant-[a-zA-Z0-9_\-]{20,}` -> `sk-ant-[REDACTED]`
4. **Generic Secret Tokens**: `(api[_-]?key|secret|password|token)\s*[:=]\s*["']?[^"'\s]{8,}["']?` -> `[REDACTED_SECRET]`

---

## 3. Provider & Model Allowlist

Only validated providers and models may be invoked:

| Provider | Supported Models | Description |
| :--- | :--- | :--- |
| **openai** | `gpt-4o`, `gpt-4o-mini`, `o1`, `o3-mini` | OpenAI official API endpoints |
| **anthropic** | `claude-3-5-sonnet-20241022`, `claude-3-5-haiku-20241022` | Anthropic official API endpoints |
| **ollama** | `llama3.2`, `deepseek-r1`, `qwen2.5` | Fully local, privacy-preserving LLMs |
| **demo** | `synthetic-v1` | Offline fixture provider for testing |

Any request specifying an unrecognized provider is rejected before process launch.

---

## 4. Execution Budgets & Ceilings

- **Timeout**: Maximum duration of 300 seconds (5 minutes) per task run.
- **Token Limit**: 100,000 tokens ceiling per execution.
- **Cost Budget**: Maximum \$1.00 USD per execution.
- **Concurrency Limit**: Maximum 1 concurrent Goose sidecar process.
