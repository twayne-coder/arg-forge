@echo off
echo 正在清理 Tauri 构建缓存...

cd /d "%~dp0"

REM 清理前端缓存
echo 清理 node_modules...
if exist node_modules rmdir /s /q node_modules

REM 清理 Tauri 构建缓存
echo 清理 Tauri target...
if exist src-tauri\target rmdir /s /q src-tauri\target

REM 清理 Tauri 生成的配置
if exist src-tauri\gen rmdir /s /q src-tauri\gen

echo.
echo ========================================
echo 缓存清理完成！
echo.
echo 请运行以下命令重新安装依赖并启动：
echo   pnpm install
echo   pnpm tauri dev
echo ========================================
echo.
pause
