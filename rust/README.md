# ApiAi - Rust Implementation

> [!WARNING]
> **Experimental Version** - This is a work-in-progress rewrite of ApiAi in Rust. For production use, see the [Python version](../python/).

Экспериментальная версия ApiAi на языке Rust.

## 🚀 Запуск

Для запуска экспериментальной Rust версии:

```bash
cd rust
cargo run
```

## 📦 Управление версиями

Версия проекта синхронизируется с Python-версией через общий скрипт.

**Обновить версию (Patch / Minor / Major):**

```bash
# Из корня проекта или из папки python/
python python/scripts/update_version.py bump --type patch
```

Это автоматически обновит:
1. `python/config/config_qt.json.template`
2. `rust/Cargo.toml`
3. `rust/src/main.rs`

## 🛠 Сборка

```bash
# Проверка кода
cargo check

# Компиляция (Debug)
cargo build

# Релизная сборка (Optimized)
cargo build --release
```

## Структура проекта

```
rust/
├── Cargo.toml       # Конфигурация проекта и зависимости
├── src/
│   ├── main.rs      # Точка входа
│   └── lib.rs       # Библиотечный код (будет создан)
└── tests/           # Тесты (будет создан)
```

## Статус миграции

- [ ] Config Manager
- [x] Encryption Module (Basic)
- [x] API Client
- [x] GUI Layer
- [x] Settings Dialog
- [ ] Main Window
