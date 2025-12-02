# ApiAi - Rust CLI Implementation

> [!WARNING]
> **Experimental Version** - This is a command-line experimental version. For modern GUI, see the [**Tauri version**](../tauri-app/). For stable desktop app, see the [Python version](../python/).

Экспериментальная версия ApiAi на языке Rust (CLI).

## 🚀 Запуск

Для запуска экспериментальной Rust CLI версии:

```bash
cd rust
cargo run
```

## 📦 Управление версиями
 
 Используйте `make` для управления версиями:
 
 ```bash
 make version-status       # Проверить версию
 make version-sync         # Синхронизировать конфиг
 make version-bump-patch   # 1.0.0 → 1.0.1
 ```
 
 Это автоматически обновит:
 1. `python/config/config_qt.json.template`
 2. `rust/Cargo.toml`
 3. `rust/src/main.rs`
 4. `config_qt.json` (в корне)
 
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
 ├── Makefile         # Команды управления
 ├── src/
 │   ├── main.rs      # Точка входа
 │   └── api.rs       # API клиент
 ```
 
 ## Статус миграции
 
 - [x] Config Manager (Shared)
 - [x] Encryption Module (Basic)
 - [x] API Client
 - [x] GUI Layer (egui - deprecated)
 - [ ] Modern GUI (see Tauri version)
 
 ## Рекомендация
 
 Для современного GUI используйте [**Tauri версию**](../tauri-app/):
 - ✨ Современный веб-интерфейс
 - 🔒 PIN-защита настроек
 - 🎨 Красивый дизайн с градиентами
 - 📱 Адаптивная верстка
