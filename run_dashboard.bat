@echo off
chcp 65001 >nul
echo.
echo 🏢 스마트 빌딩 KPI 대시보드
echo =========================================
echo.
echo 📦 러스트 프로젝트 빌드 중...
echo.
cargo build --release
if %errorlevel% neq 0 (
    echo.
    echo ❌ 빌드 실패했습니다!
    echo 🔧 오류를 확인하고 다시 시도하세요.
    pause
    exit /b 1
)

echo.
echo ✅ 빌드 완료!
echo 🚀 다크모드 대시보드 서버 시작 중...
echo.
echo 🌐 대시보드 주소: http://localhost:8080
echo � 실시간 KPI 모니터링 시스템
echo 🎨 다크모드 UI 적용됨
echo �🔄 실시간 데이터 새로고침 기능
echo.
echo ⏹️  서버 중지: Ctrl+C 
echo 📱 브라우저에서 위 주소로 접속하세요!
echo.

cargo run --release