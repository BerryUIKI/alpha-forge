#!/bin/bash
set -euo pipefail

echo "==> Running typecheck..."
pnpm typecheck

echo "==> Running lint..."
pnpm lint

echo "==> Running Rust checks..."
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings

echo "==> All checks passed."
