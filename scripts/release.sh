#!/bin/bash
set -euo pipefail

echo "==> Building Investment OS for release..."
pnpm tauri build
echo "==> Release build complete."
