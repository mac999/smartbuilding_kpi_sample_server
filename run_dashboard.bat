@echo off
chcp 65001 >nul
echo.
echo smart building KPI dashboard
echo =========================================
echo.
echo build rust project ...
echo.
cargo build --release
if %errorlevel% neq 0 (
    echo.
    echo fail!
    pause
    exit /b 1
)

echo.
echo build done!
echo start dark mode dashboard server...
echo.
echo dashboard address: http://localhost:8080
echo.
echo stop server: Ctrl+C 
echo.

cargo run --release