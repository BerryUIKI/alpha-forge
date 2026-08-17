# rebuild-frontend-env.ps1
# 重建 AlphaForge 前端依赖并验证（修复沙盒/挂载操作导致的 node_modules 损坏）。
# 用法：右键 ->"使用 PowerShell 运行"，或管理员 PowerShell：
#   powershell -ExecutionPolicy Bypass -File scripts\rebuild-frontend-env.ps1

$ErrorActionPreference = "Stop"
Set-Location (Join-Path $PSScriptRoot "..")

if (-not (Get-Command corepack -ErrorAction SilentlyContinue)) {
    Write-Host "[!] 未找到 corepack，尝试 pnpm 全局命令作为降级路径。" -ForegroundColor Yellow
}

# 可选：先结束可能占用 node_modules 的 node 进程（建议先手动关闭 IDE/终端，避免误杀）
# Get-Process node -ErrorAction SilentlyContinue | Stop-Process -Force

Write-Host "=== [1/4] 启用 corepack ===" -ForegroundColor Cyan
try { corepack enable | Out-Null } catch { Write-Host "corepack enable 失败，继续使用现有 pnpm。" }

Write-Host "=== [2/4] 清理 node_modules ===" -ForegroundColor Cyan
Remove-Item -Recurse -Force node_modules, apps/desktop/node_modules -ErrorAction SilentlyContinue
corepack pnpm store prune

Write-Host "=== [3/4] 按 lockfile 恢复安装 ===" -ForegroundColor Cyan
corepack pnpm install --frozen-lockfile

Write-Host "=== [4/4] 验证 typecheck + test ===" -ForegroundColor Cyan
corepack pnpm typecheck
corepack pnpm test

Write-Host "=== 完成：前端依赖已重建并通过验证 ===" -ForegroundColor Green