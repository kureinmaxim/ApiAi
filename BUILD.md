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

**Вариант 1: Через Makefile (macOS/Linux)**
```bash
cd tauri-app
make dev
```

**Вариант 2: Через npm (рекомендуется для Windows)**
```bash
cd tauri-app
npm run tauri dev
```

> **⚠️ Важно для Windows:** Команда `make` не доступна по умолчанию на Windows. Используйте `npm run tauri dev` вместо `make dev`.

**Что происходит:**
- Frontend запускается с hot-reload
- Rust backend компилируется автоматически
- Открывается окно приложения
- Изменения в HTML/CSS/JS применяются моментально

### Сборка релиза

**Вариант 1: Через Makefile (macOS/Linux)**
```bash
cd tauri-app
make build
```

**Вариант 2: Через npm (рекомендуется для Windows)**
```bash
cd tauri-app
npm run tauri build
```

> **⚠️ Важно для Windows:** Команда `make` не доступна по умолчанию на Windows. Используйте `npm run tauri build` вместо `make build`. Если нужно установить make на Windows, можно использовать [Chocolatey](https://chocolatey.org/) или [MSYS2](https://www.msys2.org/), но это не обязательно - `npm run tauri build` работает отлично.

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
- **macOS**: `~/Library/Logs/com.apiai.desktop/`
- **Windows**: `%APPDATA%\com.apiai.desktop\logs\`
- **Linux**: `~/.local/share/com.apiai.desktop/logs/`

---



## 🔧 Управление версиями

### Текущая версия: 2.4.2

Все команды выполняются из папки `tauri-app/`.

### Проверить версию

```bash
cd tauri-app
make version-status
```

**Вывод:**
```
Current version: 2.4.2

Files to sync:
  ✓ tauri-app/src-tauri/Cargo.toml
  ✓ tauri-app/src-tauri/tauri.conf.json
  ✓ tauri-app/src/index.html
```

### Увеличить версию

**Patch (исправления багов):**
```bash
cd tauri-app
make version-bump-patch  # 2.4.2 → 2.4.3
```

**Minor (новые функции):**
```bash
cd tauri-app
make version-bump-minor  # 2.4.2 → 2.5.0
```

**Major (крупные изменения):**
```bash
cd tauri-app
make version-bump-major  # 2.4.2 → 3.0.0
```

### Установить конкретную версию

```bash
cd tauri-app
make version-set v=2.5.0
```

### Синхронизировать файлы

Если вы вручную изменили версию в одном из файлов:

```bash
cd tauri-app
make version-sync
```

### Что обновляется

При изменении версии автоматически синхронизируются:
- `src-tauri/Cargo.toml` - Rust package version
- `src-tauri/tauri.conf.json` - Tauri app version
- `src/index.html` - UI footer version

### Полная документация

См. [VERSION_MANAGEMENT_TAURI.md](VERSION_MANAGEMENT_TAURI.md) для подробностей.

### Очистка артефактов

**Быстрая очистка:**
```bash
cd tauri-app
make clean  # Очистить build artifacts (~5GB)
```

**Или вручную:**
```bash
cd tauri-app/src-tauri
cargo clean
```

**Shared library:**
```bash
cd shared-rs
cargo clean
```

Подробнее см. [CLEANUP_GUIDE.md](CLEANUP_GUIDE.md)

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
chmod -R 755 ~/Library/Application\ Support/com.apiai.desktop/

# Создать папку логов вручную
mkdir -p ~/Library/Application\ Support/com.apiai.desktop/logs/
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
1. Используйте **Tauri** для разработки
2. Режим разработки: `cd tauri-app && npm run tauri dev`
3. Логи в терминале + DevTools (правый клик → Inspect)
4. Hot-reload для HTML/CSS/JS
5. Network Monitor для отладки запросов

### Для тестирования
1. Соберите release версию: 
   - **macOS/Linux:** `cd tauri-app && make build`
   - **Windows:** `cd tauri-app && npm run tauri build`
2. Тестируйте на чистой системе
3. Проверяйте логи в `~/Library/Application Support/com.apiai.desktop/` (macOS) или `%APPDATA%\com.apiai.desktop\` (Windows)
4. Используйте Echo Test для проверки соединения
5. Проверьте Console (📡) для network logs

### Для релиза
1. Обновите версию: 
   - **macOS/Linux:** `cd tauri-app && make version-bump-patch`
   - **Windows:** `cd tauri-app && python scripts/update_version.py bump patch`
2. Соберите: 
   - **macOS/Linux:** `make build`
   - **Windows:** `npm run tauri build`
3. Протестируйте установщик (.dmg для macOS, .msi/.exe для Windows)
4. Создайте GitHub Release: `gh release create vX.Y.Z`
5. Приложите установщик из `src-tauri/target/release/bundle/`

---

## 📞 Поддержка

При возникновении проблем:
1. Проверьте логи (см. разделы выше)
2. Очистите артефакты сборки: 
   - **macOS/Linux:** `cd tauri-app && make clean`
   - **Windows:** `cd tauri-app\src-tauri && cargo clean`
3. Переустановите зависимости: `cd tauri-app && npm install`
4. Проверьте версии инструментов: `node --version`, `rustc --version`
5. Проверьте Console logs (📡 Network Monitor)

**Разработчик:** Kurein M.N.  
**Версия:** 2.4.2  
**Дата релиза:** 04.12.2025

## 📦 Активные компоненты

Текущая версия **2.4.2**:
- `tauri-app/` - Tauri GUI приложение
- `shared-rs/` - Общая Rust библиотека (API и ChaCha20-Poly1305 шифрование)

**Архивные компоненты** (не используются):
- `rust/` - Старая CLI версия (egui)
- `python/` - Python Qt версия
