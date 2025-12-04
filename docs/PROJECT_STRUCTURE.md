# 🏗 Структура проекта ApiAi

Этот документ описывает структуру проекта ApiAi (Tauri Edition).

## 📂 Общая структура проекта

```
ApiAi/
├── shared-rs/             # 📦 Общая библиотека (API & Encryption)
├── tauri-app/             # 🌐 Tauri версия (GUI приложение)
├── docs/                  # 📚 Документация проекта
│   ├── CHEAT_SHEET.md     # Шпаргалка команд
│   ├── ENCRYPTION.md      # Протокол шифрования
│   ├── KEYS_MANAGEMENT.md # Управление ключами API
│   ├── PORTS.md           # Сетевые порты
│   └── PROJECT_STRUCTURE.md  # Этот файл
├── BUILD.md               # 🔨 Инструкции по сборке
├── CLEANUP_GUIDE.md       # 🧹 Инструкции по очистке
├── VERSION_MANAGEMENT_TAURI.md  # 📦 Управление версиями
└── README.md              # 📄 Главная документация
```

---

## 📦 Общая библиотека (`shared-rs/`)

Общая Rust библиотека для API клиентов и шифрования.

### 📂 Структура

```
shared-rs/
├── src/
│   ├── lib.rs            # Экспорт модулей
│   ├── api.rs            # API клиенты (Anthropic, OpenAI, Telegram)
│   └── encryption.rs     # AES-256-GCM шифрование
├── Cargo.toml            # Манифест библиотеки
└── README.md             # Документация
```

### 🎯 Назначение

**Используется в:**
- `tauri-app/src-tauri/` - Tauri backend

**Содержит:**
- `ApiClient` trait - общий интерфейс для всех провайдеров
- `AnthropicClient` - Anthropic Claude API
- `OpenAIClient` - OpenAI GPT API
- `TelegramClient` - Telegram bot API с шифрованием
- `SecureMessenger` - ChaCha20-Poly1305 криптография

---

## 🌐 Tauri версия (`tauri-app/`)

Современная версия с веб-интерфейсом и Rust бэкендом.

### 📂 Структура

```
tauri-app/
├── src/                    # Frontend (Web)
│   ├── index.html         # HTML структура
│   ├── styles.css         # Стили с градиентами
│   ├── network-monitor.css # Стили монитора сети
│   ├── main.js            # JavaScript логика
│   ├── provider-settings.js  # Provider Settings modal
│   ├── file-editor.js     # File Editor mode
│   ├── chat-history.js    # Chat persistence
│   └── network-monitor.js # Network logging
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Точка входа Tauri
│   │   └── lib.rs         # Библиотечный код (использует shared-rs)
│   ├── Cargo.toml         # Rust зависимости
│   └── tauri.conf.json    # Конфигурация Tauri
├── scripts/               # Утилиты
│   ├── update_version.py  # Управление версиями
│   ├── backup_config.sh   # Бэкап конфига
│   └── restore_config.sh  # Восстановление конфига
├── config_templates/      # Шаблоны конфигов
├── Makefile               # Команды управления
└── README.md              # Документация Tauri версии
```

### 🎯 Ключевые файлы

#### Frontend

- **`src/index.html`**
  - Основной UI, модальные окна, сайдбар
  - Network Monitor sidebar (NEW!)

- **`src/styles.css`**
  - Современный дизайн с purple/indigo градиентами
  - Анимации и transitions
  - Glassmorphism эффекты
  - Стили для модальных окон и защищенных полей

- **`src/main.js`**
  - PIN логика (разблокировка для Settings)
  - Event listeners централизованно в `init()`
  - Отслеживание размера окна
  - Chat mode & File Editor mode
  - Echo тест с детальной статистикой

- **`src/provider-settings.js`**
  - Модальное окно для настройки провайдера
  - Сохранение в `config_qt.json` через Tauri backend
  - Автозагрузка сохраненных настроек
  - Показ/скрытие API ключей

- **`src/file-editor.js`**
  - File Editor mode для редактирования файлов через AI
  - Выбор файла, отправка AI, сохранение изменений
  - Версионирование (_AI_1, _AI_2 и т.д.)
  - Клик по имени файла → копирование пути

- **`src/network-monitor.js`** (NEW!)
  - Логирование сетевых запросов
  - Индикация шифрования (🔒/⚠️)
  - История запросов

- **`src/chat-history.js`**
  - Сохранение и загрузка истории чатов
  - Управление библиотекой чатов

#### Backend

- **`src-tauri/src/main.rs`**
  - Инициализация Tauri приложения
  - Регистрация команд
  
- **`src-tauri/src/lib.rs`**
  - Tauri команды (perform_search, check_pin, etc.)
  - Импортирует API клиенты из `shared-rs`
  - Управление конфигурацией и состоянием

---

## 🔗 Конфигурация

Конфигурация хранится в **App Data Directory** операционной системы:

- **macOS**: `~/Library/Application Support/com.apiai.app/config.json`
- **Windows**: `%APPDATA%\com.apiai.app\config.json`
- **Linux**: `~/.local/share/com.apiai.app/config.json`

**Управление:**
- Через UI (Settings)
- Через скрипты `make config-backup` / `make config-restore`

---

## 📂 `docs/` (документация)

Вся документация проекта собрана в одном месте:

- **`CHEAT_SHEET.md`** - Быстрая шпаргалка команд для всех версий
- **`ENCRYPTION.md`** - Протокол шифрования AES-256-GCM
- **`PORTS.md`** - Документация сетевых портов
- **`PROJECT_STRUCTURE.md`** - Этот файл (структура проекта)

### 📄 Корневые документы

- **`README.md`** - Главная документация проекта
- **`BUILD.md`** - Инструкции по сборке, запуску и логам
- **`VERSION_MANAGEMENT_TAURI.md`** - Руководство по управлению версиями

---

## 📦 Артефакты сборки

### Shared Library
- `shared-rs/target/` - Скомпилированная библиотека

### Tauri
- `tauri-app/src-tauri/target/` - Скомпилированные файлы (~5.4GB)

**Примечание:** Папки `target/` находятся в `.gitignore` из-за большого размера (общий размер ~8.4GB).

**Очистка:** Используйте `make clean-all` для освобождения места. См. [CLEANUP_GUIDE.md](../CLEANUP_GUIDE.md)

---

## 🏗 Архитектура кода

```
┌─────────────────────────────────────────┐
│         shared-rs (библиотека)          │
│  ┌────────────────────────────────────┐ │
│  │ API Clients:                       │ │
│  │  - AnthropicClient                 │ │
│  │  - OpenAIClient                    │ │
│  │  - TelegramClient                  │ │
│  │ Encryption: SecureMessenger        │ │
│  └────────────────────────────────────┘ │
└─────────────────────────────────────────┘
                   ▲
                   │
            ┌──────┴──────┐
            │ tauri-app/  │
            │  (GUI App)  │
            └─────────────┘
```

---

## ⚠️ Архивные компоненты

Следующие компоненты больше не поддерживаются и подлежат удалению:
- `rust/` (CLI версия)
- `python/` (Python версия)
- `config_qt.json` (старый конфиг)
