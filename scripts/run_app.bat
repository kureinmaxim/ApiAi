@echo off
REM Launcher script for ApiAi

cd /d "%~dp0"

REM Check if Python is available
python --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Python not found!
    echo Please install Python 3.8+ from https://www.python.org/
    pause
    exit /b 1
)

REM Check if virtual environment exists
if not exist ".venv" (
    echo Creating virtual environment...
    python -m venv .venv
)

REM Activate virtual environment
call .venv\Scripts\activate.bat

REM Install dependencies if needed
python -c "import PySide6" 2>nul
if %errorlevel% neq 0 (
    echo Installing dependencies...
    if exist "offline_packages" (
        pip install --no-index --find-links=offline_packages -r requirements.txt
    ) else (
        pip install -r requirements.txt
    )
)

REM Run application
python main.py

pause
