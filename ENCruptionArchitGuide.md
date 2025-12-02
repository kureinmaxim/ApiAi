# 📘 ApiAi Encryption Architecture Guide

> **Шпаргалка по архитектуре шифрования и работе с ключами**

## 🎯 Общая архитектура

```mermaid
graph TB
    subgraph "ApiAi Client (Tauri)"
        A[User Interface<br/>main.js] --> B[Rust Backend<br/>api.rs]
        B --> C[Encryption Module<br/>encryption.rs]
        C --> D[AES-256-GCM]
    end
    
    subgraph "TelegramHelper Server (VPS)"
        E[FastAPI Endpoint<br/>api.py] --> F[Security Module<br/>security.py]
        F --> G[Key Management<br/>app_keys.py]
        E --> H[Encryption Module<br/>encryption.py]
        H --> I[AES-256-GCM]
    end
    
    B -->|"HTTPS Request<br/>X-APP-ID: apiai-v1<br/>X-API-KEY: [key]<br/>Base64(encrypted)"| E
    E -->|"Encrypted Response<br/>Base64(encrypted)"| B
    
    style A fill:#667eea,color:#fff
    style E fill:#764ba2,color:#fff
    style D fill:#f093fb,color:#000
    style I fill:#f093fb,color:#000
```

## 🔐 Как работает шифрование

### 1️⃣ Этап подготовки (Клиент)

```mermaid
sequenceDiagram
    participant UI as User Interface
    participant Storage as config_qt.json
    participant Rust as Rust Backend
    
    UI->>Storage: Загрузить конфигурацию
    Storage-->>UI: API Key + Encryption Key
    UI->>Rust: Инициализировать SecureMessenger<br/>с ключом шифрования
    Note over Rust: SecureMessenger готов<br/>для шифрования/дешифрования
```

**Что происходит:**
1. При запуске приложения загружается `config_qt.json`
2. Из него извлекаются:
   - `telegram_key` → API ключ для аутентификации
   - `telegram_enc_key` → Ключ шифрования (hex 64 символа)
   - `telegram_url` → URL сервера
3. Создается экземпляр `SecureMessenger` с ключом шифрования

### 2️⃣ Процесс шифрования запроса

```mermaid
graph LR
    A["Запрос пользователя<br/>{prompt: 'Hello'}"] --> B[Serialize to JSON]
    B --> C["JSON bytes<br/>[123, 34, 112...]"]
    C --> D[Generate Random Nonce<br/>12 байт]
    D --> E[AES-256-GCM Encrypt]
    E --> F["Encrypted bytes<br/>[nonce + ciphertext]"]
    F --> G[Base64 Encode]
    G --> H["Base64 String<br/>'wTAgQ+/zJtfe...'"]
    
    style A fill:#e1f5ff
    style E fill:#fff3cd
    style H fill:#d4edda
```

**Детали шифрования:**
- **Алгоритм**: AES-256-GCM (Authenticated Encryption)
- **Размер ключа**: 256 бит (32 байта, 64 hex символа)
- **Nonce**: 12 байт (генерируется случайно для каждого сообщения)
- **Формат пакета**: `[nonce(12 байт)][ciphertext + auth_tag]`
- **Кодирование**: Base64 для передачи по HTTP

### 3️⃣ Отправка запроса на сервер

```mermaid
sequenceDiagram
    participant Client as ApiAi Client
    participant Network as HTTPS
    participant Server as TelegramHelper<br/>Server
    
    Client->>Network: POST /ai_query/secure
    Note over Network: Headers:<br/>X-APP-ID: apiai-v1<br/>X-API-KEY: b398f203...
    Note over Network: Body (JSON):<br/>{<br/>  "data": "wTAgQ+/z..."<br/>}
    Network->>Server: Доставка запроса
    
    Server->>Server: 1. Проверка API ключа
    Server->>Server: 2. Получение ключа шифрования<br/>для apiai-v1
    Server->>Server: 3. Декодирование Base64
    Server->>Server: 4. Расшифровка AES-GCM
    Server->>Server: 5. Парсинг JSON
    Server->>Server: 6. Обработка запроса AI
    
    Server-->>Client: Зашифрованный ответ
```

### 4️⃣ Управление ключами на сервере

```mermaid
graph TB
    subgraph "Server Key Management"
        A[Запрос приходит<br/>X-APP-ID: apiai-v1] --> B{app_keys.json<br/>существует?}
        B -->|Да| C{Есть ключи<br/>для apiai-v1?}
        B -->|Нет| D[Использовать DEFAULT<br/>из .env]
        
        C -->|Да| E["INDIVIDUAL KEYS<br/>API: b398f203...<br/>ENC: 83d68210..."]
        C -->|Нет| D
        
        E --> F[Проверка API ключа]
        D --> F
        
        F -->|Совпадает| G[Получить encryption_key]
        F -->|Не совпадает| H[403 Forbidden]
        
        G --> I[Расшифровать запрос]
        I -->|Успех| J[Обработать]
        I -->|Ошибка| K[400 Decryption Failed]
    end
    
    style E fill:#d4edda,color:#000
    style D fill:#fff3cd,color:#000
    style H fill:#f8d7da,color:#000
    style K fill:#f8d7da,color:#000
    style J fill:#d4edda,color:#000
```

**Приоритет ключей:**
1. **Individual keys** из `app_keys.json` (для конкретного app_id)
2. **Default keys** из переменных окружения `.env`

**Файл app_keys.json структура:**
```json
{
  "app_keys": {
    "apiai-v1": {
      "api_key": "b398f203ffb4a57afe3c5eff4239151404c85b8803507a58dca8ed64becfd392",
      "encryption_key": "83d68210ec84a39e2c3eb82a7a6b7afe488ae5684d45a8546c22b1fc0016cb10",
      "name": "ApiAi Experimental Rust version",
      "created_at": "2025-12-01T19:43:09.823466"
    }
  }
}
```

## 🔄 Полный цикл запроса

```mermaid
sequenceDiagram
    autonumber
    participant User as 👤 Пользователь
    participant UI as ApiAi UI
    participant Enc as Encryption<br/>(Client)
    participant API as Server API
    participant Sec as Security Check
    participant Keys as Key Manager
    participant Dec as Encryption<br/>(Server)
    participant AI as AI Provider
    
    User->>UI: Вводит запрос "Hello"
    UI->>Enc: encrypt({prompt: "Hello"})
    Enc->>Enc: 1. JSON → bytes<br/>2. Generate nonce<br/>3. AES encrypt<br/>4. Base64 encode
    Enc-->>UI: "wTAgQ+/zJtfe..."
    
    UI->>API: POST /ai_query/secure<br/>X-APP-ID: apiai-v1<br/>X-API-KEY: b398f...
    API->>Sec: Verify credentials
    Sec->>Keys: get_api_key("apiai-v1")
    Keys-->>Sec: b398f203...
    Sec->>Sec: Compare with X-API-KEY
    
    alt ✅ Keys Match
        Sec-->>API: Authorized
        API->>Keys: get_encryption_key("apiai-v1")
        Keys-->>API: 83d68210...
        API->>Dec: decrypt("wTAgQ+/zJtfe...", key)
        Dec->>Dec: 1. Base64 decode<br/>2. Extract nonce<br/>3. AES decrypt<br/>4. Parse JSON
        Dec-->>API: {prompt: "Hello"}
        
        API->>AI: Process query
        AI-->>API: "AI Response..."
        
        API->>Dec: encrypt({response: "..."})
        Dec-->>API: Encrypted response
        API-->>UI: {data: "encrypted..."}
        
        UI->>Enc: decrypt(response)
        Enc-->>UI: {response: "AI Response..."}
        UI-->>User: Отобразить ответ
        
    else ❌ Keys Don't Match
        Sec-->>API: 403 Forbidden
        API-->>UI: Error: Invalid API key
        UI-->>User: Показать ошибку
    end
```

## 🛠️ Устранение проблем

### Ошибка: "Decryption failed"

```mermaid
graph TD
    A[Ошибка: Decryption failed] --> B{Проверить<br/>encryption_key}
    B --> C[Клиент:<br/>config_qt.json]
    B --> D[Сервер:<br/>app_keys.json]
    
    C --> E{Ключи<br/>совпадают?}
    D --> E
    
    E -->|Нет| F[🔴 ПРОБЛЕМА:<br/>Разные ключи]
    E -->|Да| G{Проверить<br/>формат ключа}
    
    F --> H[Решение: Синхронизировать<br/>ключи между клиентом<br/>и сервером]
    
    G -->|Hex, 64 символа| I{Проверить<br/>endpoint}
    G -->|Неверный формат| J[🔴 ПРОБЛЕМА:<br/>Неправильный формат]
    
    I -->|/ai_query/secure| K{Сервер использует<br/>app-specific key?}
    I -->|Другой endpoint| L[🔴 ПРОБЛЕМА:<br/>Неверный endpoint]
    
    K -->|Да| M[✅ ВСЕ КОРРЕКТНО]
    K -->|Нет| N[🔴 ПРОБЛЕМА:<br/>Сервер использует<br/>глобальный ключ]
    
    style F fill:#f8d7da
    style J fill:#f8d7da
    style L fill:#f8d7da
    style N fill:#f8d7da
    style M fill:#d4edda
    style H fill:#fff3cd
```

### Ошибка: "Invalid API key" (403)

**Причины:**
1. API ключ в `config_qt.json` не совпадает с `app_keys.json` на сервере
2. `X-APP-ID` заголовок не передается или неправильный
3. На сервере нет ключей для указанного app_id

**Решение:**
1. Проверить ключ на сервере:
   ```bash
   ssh root@138.124.19.67
   cd /opt/TelegramHelper
   python3 scripts/show_keys.py --app-id apiai-v1
   ```

2. Скопировать правильный ключ в `config_qt.json`:
   ```json
   "telegram_key": "b398f203ffb4a57afe3c5eff4239151404c85b8803507a58dca8ed64becfd392"
   ```

## 📊 Сравнение ключей

### ❌ НЕПРАВИЛЬНО (Default keys)

```
┌─────────────────────────────────────────────────────────────┐
│ CLIENT (config_qt.json)                                     │
├─────────────────────────────────────────────────────────────┤
│ API Key:        13ab4a4f0c5d57ecf93727ad684f1ac46f359...   │
│ Encryption Key: 31d3636f5edb72ead0ccf07de041c5f24f530...   │
└─────────────────────────────────────────────────────────────┘
                              ↓ ❌ MISMATCH
┌─────────────────────────────────────────────────────────────┐
│ SERVER (app_keys.json → apiai-v1)                           │
├─────────────────────────────────────────────────────────────┤
│ API Key:        b398f203ffb4a57afe3c5eff4239151404c85b...   │
│ Encryption Key: 83d68210ec84a39e2c3eb82a7a6b7afe488ae...   │
└─────────────────────────────────────────────────────────────┘
```

### ✅ ПРАВИЛЬНО (Individual keys for apiai-v1)

```
┌─────────────────────────────────────────────────────────────┐
│ CLIENT (config_qt.json)                                     │
├─────────────────────────────────────────────────────────────┤
│ API Key:        b398f203ffb4a57afe3c5eff4239151404c85b...   │
│ Encryption Key: 83d68210ec84a39e2c3eb82a7a6b7afe488ae...   │
└─────────────────────────────────────────────────────────────┘
                              ↓ ✅ MATCH
┌─────────────────────────────────────────────────────────────┐
│ SERVER (app_keys.json → apiai-v1)                           │
├─────────────────────────────────────────────────────────────┤
│ API Key:        b398f203ffb4a57afe3c5eff4239151404c85b...   │
│ Encryption Key: 83d68210ec84a39e2c3eb82a7a6b7afe488ae...   │
└─────────────────────────────────────────────────────────────┘
```

## 🔧 Генерация новых ключей

Если нужно создать новые ключи для нового приложения:

```bash
# На сервере
ssh root@138.124.19.67
cd /opt/TelegramHelper

# Генерация ключей для нового app_id
python3 scripts/generate_keys.py --app-id my-new-app --name "My Application"
```

Это создаст:
- Новый случайный API ключ (64 hex символа)
- Новый случайный ключ шифрования (64 hex символа)
- Запись в `app_keys.json`

## 📝 Конфигурационные файлы

### Клиент: config_qt.json

```json
{
  "pin": "1234",
  "api_keys": {
    "anthropic": "",
    "openai": "",
    "telegram_url": "http://138.124.19.67:8000/ai_query",
    "telegram_key": "b398f203ffb4a57afe3c5eff4239151404c85b8803507a58dca8ed64becfd392",
    "telegram_enc_key": "83d68210ec84a39e2c3eb82a7a6b7afe488ae5684d45a8546c22b1fc0016cb10",
    "telegram_use_encryption": true
  }
}
```

**Важные поля:**
- `telegram_url` - URL сервера (может быть полным URL или host:port)
- `telegram_key` - API ключ для аутентификации
- `telegram_enc_key` - Ключ шифрования (HEX, 64 символа)
- `telegram_use_encryption` - Включить/выключить шифрование

### Сервер: app_keys.json

```json
{
  "default": {
    "api_key": "test_secret_key_32_bytes_long_12345",
    "encryption_key": "test_secret_key_32_bytes_long_12345"
  },
  "app_keys": {
    "apiai-v1": {
      "api_key": "b398f203ffb4a57afe3c5eff4239151404c85b8803507a58dca8ed64becfd392",
      "encryption_key": "83d68210ec84a39e2c3eb82a7a6b7afe488ae5684d45a8546c22b1fc0016cb10",
      "name": "ApiAi Experimental Rust version",
      "created_at": "2025-12-01T19:43:09.823466"
    },
    "bomcategorizer-v5": {
      "api_key": "7ec64a14...c3cb72bf",
      "encryption_key": "cc1f0e4b...cd05d41f8",
      "name": "BOM Categorizer Modern Edition v5",
      "created_at": "2025-12-02T07:20:07.823466"
    }
  }
}
```

## 🎓 Ключевые концепции

### 1. App ID
- Уникальный идентификатор приложения (например, `apiai-v1`)
- Передается в заголовке `X-APP-ID`
- Используется для поиска индивидуальных ключей

### 2. Двухуровневая аутентификация
1. **API Key** - аутентификация приложения
2. **Encryption Key** - шифрование данных

Оба ключа должны совпадать!

### 3. AES-256-GCM
- **Authenticated Encryption** - шифрование + проверка подлинности
- **256-bit key** - высокий уровень безопасности
- **GCM mode** - Galois/Counter Mode (быстрый и безопасный)
- **Nonce** - уникален для каждого сообщения

### 4. Endpoints

| Endpoint | Шифрование | App-specific keys |
|----------|------------|-------------------|
| `/ai_query` | Опционально | ✅ Да (если data поле) |
| `/ai_query/secure` | Обязательно | ✅ Да |
| `/ai_query/encrypted` | Обязательно (binary) | ✅ Да |

## 🚀 Быстрая проверка

### Проверить текущие ключи на сервере:
```bash
ssh root@138.124.19.67 "cd /opt/TelegramHelper && python3 scripts/show_keys.py --app-id apiai-v1"
```

### Проверить ключи в клиенте:
```bash
# На Mac
cat /Users/olgazaharova/Project/ApiAi/config_qt.json | jq '.api_keys | {telegram_key, telegram_enc_key}'
```

### Проверить, что сервер работает:
```bash
curl http://138.124.19.67:8000/health
```

## 📚 Дополнительные ресурсы

- [encryption.rs](file:///Users/olgazaharova/Project/ApiAi/tauri-app/src-tauri/src/encryption.rs) - Клиентское шифрование (Rust)
- [encryption.py](file:///Users/olgazaharova/Project/ProjectPython/TelegramHelper/encryption.py) - Серверное шифрование (Python)
- [api.rs](file:///Users/olgazaharova/Project/ApiAi/tauri-app/src-tauri/src/api.rs#L160-L234) - Клиентский API
- [api.py](file:///Users/olgazaharova/Project/ProjectPython/TelegramHelper/api.py#L639-L705) - Серверный endpoint
- [security.py](file:///Users/olgazaharova/Project/ProjectPython/TelegramHelper/security.py) - Модуль безопасности
- [app_keys.py](file:///Users/olgazaharova/Project/ProjectPython/TelegramHelper/app_keys.py) - Управление ключами
