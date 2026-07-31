#!/bin/bash
set -euo pipefail

echo "==> Bootstrapping Investment OS development environment..."

echo "==> Installing pnpm dependencies..."
pnpm install

echo "==> Bootstrap complete."
echo "Run 'pnpm dev' to start the desktop application."
