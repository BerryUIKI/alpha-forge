#!/bin/bash
set -euo pipefail

echo "==> Building AlphaForge for release..."
pnpm tauri build
echo "==> Release build complete."
