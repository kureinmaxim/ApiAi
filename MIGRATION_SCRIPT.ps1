# PowerShell скрипт для реорганизации проекта
# Перемещает Python код в папку python/

Write-Host "Начинаем реорганизацию проекта..." -ForegroundColor Green

# Создаем папку python если её нет
if (-not (Test-Path "python")) {
    New-Item -ItemType Directory -Path "python"
    Write-Host "Создана папка python/" -ForegroundColor Yellow
}

# Список Python файлов для перемещения
$pythonFiles = @(
    "main.py",
    "config_manager.py",
    "encryption.py",
    "styles.py",
    "utils.py",
    "update_config.py",
    "config_qt.json",
    "requirements.txt",
    "run_app.bat",
    "installer_active.iss"
)

# Список папок для перемещения
$pythonDirs = @(
    "gui",
    "scripts",
    "config",
    "deployment",
    "fonts",
    "offline_package"
)

# Перемещаем файлы
foreach ($file in $pythonFiles) {
    if (Test-Path $file) {
        git mv $file "python/$file"
        Write-Host "Перемещен: $file -> python/$file" -ForegroundColor Cyan
    }
}

# Перемещаем папки
foreach ($dir in $pythonDirs) {
    if (Test-Path $dir) {
        git mv $dir "python/$dir"
        Write-Host "Перемещена: $dir -> python/$dir" -ForegroundColor Cyan
    }
}

Write-Host "`nГотово! Структура проекта реорганизована." -ForegroundColor Green
Write-Host "Теперь выполните: git status" -ForegroundColor Yellow
