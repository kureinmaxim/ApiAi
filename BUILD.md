# 🔨 ApiAi - Build & Run Guide

Руководство по сборке, запуску и отладке всех версий ApiAi.

---

## 🌐 Tauri Version (Recommended)

### Требования
- Node.js v16+ ([nodejs.org](https://nodejs.org))
- Rust (latest stable) ([rustup.rs](https://rustup.rs))
- npm или yarn

### Первоначальная настройка

```bash
cd tauri-app
npm install
```

### Запуск в режиме разработки

```bash
cd tauri-app
npm run tauri dev
```

**Что происходит:**
- Frontend запускается с hot-reload
- Rust backend компилируется автоматически
- Открывается окно приложения
- Изменения в HTML/CSS/JS применяются моментально

### Сборка релиза

```bash
cd tauri-app
npm run tauri build
```

**Результат:**
- **macOS**: `src-tauri/target/release/bundle/dmg/ApiAi_*.dmg`
- **Windows**: `src-tauri/target/release/bundle/msi/ApiAi_*.msi`
- **Linux**: `src-tauri/target/release/bundle/deb/apiai_*.deb`

### Запуск собранного релиза

**macOS:**
```bash
# Установить DMG или запустить напрямую
./tauri-app/src-tauri/target/release/tauri-app
```

**Windows:**
```bash
# Установить MSI или запустить напрямую
.\tauri-app\src-tauri\target\release\tauri-app.exe
```

**Linux:**
```bash
# Установить DEB или запустить напрямую
./tauri-app/src-tauri/target/release/tauri-app
```

### Логи

**Режим разработки:**
- Логи Rust отображаются в терминале, где запущен `npm run tauri dev`
- JavaScript логи в DevTools (открыть правой кнопкой → Inspect Element)

**Релиз:**
- **macOS**: `~/Library/Logs/com.apiai.app/`
- **Windows**: `%APPDATA%\com.apiai.app\logs\`
- **Linux**: `~/.local/share/com.apiai.app/logs/`

---

## 🐍 Python Version

### Требования
- Python 3.8+ ([python.org](https://python.org))
- pip (устанавливается с Python)

### Установка зависимостей

```bash
cd python
python3 -m pip install -r requirements.txt
```

**Основные зависимости:**
- PySide6 (GUI)
- requests (HTTP)
- cryptography (шифрование)

### Запуск приложения

```bash
cd python
python3 main.py
```

**Windows:**
```bash
cd python
python main.py
```

### Сборка инсталлятора

```bash
cd python
python3 scripts/build.py
```

**Требования для сборки:**
- PyInstaller
- Inno Setup (Windows) или py2app (macOS)

**Результат:**
- **Windows**: `dist/ApiAi-Setup.exe`
- **macOS**: `dist/ApiAi.dmg`

### Запуск собранного приложения

**macOS:**
```bash
open dist/ApiAi.app
```

**Windows:**
```bash
.\dist\ApiAi.exe
```

### Логи

**Расположение логов:**
- **macOS**: `~/Library/Application Support/ApiAi/logs/`
- **Windows**: `%APPDATA%\ApiAi\logs\`
- **Linux**: `~/.local/share/ApiAi/logs/`

**Файлы логов:**
- `app.log` - общие логи приложения
- `api.log` - логи API запросов
- `error.log` - ошибки

**Просмотр логов:**
```bash
# macOS/Linux
tail -f ~/Library/Application\ Support/ApiAi/logs/app.log

# Windows
type %APPDATA%\ApiAi\logs\app.log
```

---

## 🦀 Rust CLI Version

### Требования
- Rust (latest stable) ([rustup.rs](https://rustup.rs))

### Установка Rust (если не установлен)

**macOS/Linux:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**
Скачать и установить с [rustup.rs](https://rustup.rs)

### Запуск в режиме разработки

```bash
cd rust
cargo run
```

**С аргументами:**
```bash
cargo run -- --provider telegram --query "Hello"
```

### Сборка релиза

**Debug сборка (быстрая компиляция):**
```bash
cd rust
cargo build
```
Результат: `target/debug/apiai`

**Release сборка (оптимизированная):**
```bash
cd rust
cargo build --release
```
Результат: `target/release/apiai`

### Запуск собранного релиза

**Debug:**
```bash
./rust/target/debug/apiai --help
```

**Release:**
```bash
./rust/target/release/apiai --help
```

**Windows:**
```bash
.\rust\target\release\apiai.exe --help
```

### Логи

**Режим разработки:**
- Логи выводятся в stdout/stderr

**Включить подробные логи:**
```bash
RUST_LOG=debug cargo run
```

**Уровни логирования:**
- `RUST_LOG=error` - только ошибки
- `RUST_LOG=warn` - предупреждения и ошибки
- `RUST_LOG=info` - информация (по умолчанию)
- `RUST_LOG=debug` - отладочная информация
- `RUST_LOG=trace` - максимально подробно

**Сохранить логи в файл:**
```bash
cargo run 2>&1 | tee logs/app.log
```

---

## 🔧 Общие инструменты

### Управление версиями

Из папки `rust/`:
```bash
make version-status        # Текущая версия
make version-bump-patch    # 1.0.5 → 1.0.6
make version-bump-minor    # 1.0.5 → 1.1.0
```

Это обновит версии во всех файлах:
- `config_qt.json`
- `python/config/config_qt.json.template`
- `rust/Cargo.toml`
- `tauri-app/src-tauri/Cargo.toml`

### Очистка артефактов

**Tauri:**
```bash
cd tauri-app
rm -rf node_modules src-tauri/target
npm install
```

**Rust CLI:**
```bash
cd rust
cargo clean
```

**Python:**
```bash
cd python
rm -rf build dist __pycache__ *.spec
```

---

## 📊 Сравнение производительности

### Размер приложения

| Версия | macOS | Windows | Linux |
|--------|-------|---------|-------|
| Tauri | ~15 MB | ~12 MB | ~18 MB |
| Python | ~80 MB | ~60 MB | ~90 MB |
| Rust CLI | ~5 MB | ~4 MB | ~6 MB |

### Время запуска

| Версия | Холодный старт | Горячий старт |
|--------|----------------|---------------|
| Tauri | ~1-2 сек | <1 сек |
| Python | ~2-3 сек | ~1 сек |
| Rust CLI | <0.5 сек | <0.1 сек |

---

## 🐛 Решение проблем

### Tauri не компилируется

**Ошибка:** `Unable to find required tools`

**Решение:**
```bash
# Установить/обновить Rust
rustup update stable

# Очистить кеш
cd tauri-app
rm -rf src-tauri/target
cargo clean
```

### Python - ошибка импорта модулей

**Ошибка:** `ModuleNotFoundError: No module named 'PySide6'`

**Решение:**
```bash
cd python
python3 -m pip install --upgrade pip
python3 -m pip install -r requirements.txt
```

### Rust - ошибки линковки

**Ошибка:** `linking with 'cc' failed`

**Решение (macOS):**
```bash
# Установить Xcode Command Line Tools
xcode-select --install
```

**Решение (Linux):**
```bash
# Установить build-essential
sudo apt-get install build-essential
```

### Логи не создаются

**Проверить права доступа:**
```bash
# macOS/Linux
chmod -R 755 ~/Library/Application\ Support/ApiAi/

# Создать папку логов вручную
mkdir -p ~/Library/Application\ Support/ApiAi/logs/
```

---

## 📝 Дополнительные команды

### Тестирование

**Tauri:**
```bash
cd tauri-app/src-tauri
cargo test
```

**Rust CLI:**
```bash
cd rust
cargo test
```

**Python:**
```bash
cd python
python3 -m pytest tests/
```

### Линтинг

**Tauri/Rust:**
```bash
cargo clippy -- -D warnings
cargo fmt --check
```

**Python:**
```bash
cd python
python3 -m pylint *.py
python3 -m black --check .
```

### Документация

**Rust (локальная документация):**
```bash
cargo doc --open
```

---

## 🚀 Рекомендуемый workflow

### Для разработки
1. Используйте **Tauri** для GUI разработки
2. Режим разработки: `npm run tauri dev`
3. Логи в терминале + DevTools

### Для тестирования
1. Соберите release версию
2. Тестируйте на чистой системе
3. Проверяйте логи в user directories

### Для релиза
1. Обновите версию: `make version-bump-patch`
2. Соберите все версии
3. Протестируйте установщики
4. Проверьте логи после установки

---

## 📞 Поддержка

При возникновении проблем:
1. Проверьте логи (см. разделы выше)
2. Очистите артефакты сборки
3. Переустановите зависимости
4. Проверьте версии инструментов

**Разработчик:** Maksim Kurein  
**Версия:** 1.0.5  
**Дата релиза:** 2025-12-01
