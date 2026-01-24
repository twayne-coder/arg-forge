# 遇到错误立即停止
$ErrorActionPreference = "Stop"

Write-Host "--- 1. 开始前端检查 (pnpm build) ---" -ForegroundColor Cyan
pnpm install
pnpm build

Write-Host "`n--- 2. 开始 Rust 格式检查 (fmt) ---" -ForegroundColor Cyan
cd src-tauri
cargo fmt

Write-Host "`n--- 3. 开始 Rust 静态分析 (clippy) ---" -ForegroundColor Cyan
# -D warnings 表示把警告当成错误处理
cargo clippy -- -D warnings

Write-Host "`n--- 4. 开始运行 Rust 单元测试 (test) ---" -ForegroundColor Cyan
cargo test --verbose

cd ..
Write-Host "`n✅ 所有检查已通过！" -ForegroundColor Green