# 📋 Шпаргалка по логам ApiAi

## GUI Логи (Окно приложения)

### Встроенное окно логов

ApiAi выводит логи прямо в интерфейсе:

1. **Экспертный режим** → видно окно логов внизу
2. **Простой режим** → логи скрыты, переключитесь в экспертный

### Важные события в логах

- ✅ `Настройки API и AI сохранены` - настройки успешно сохранены
- 🔄 `Проверить соединение` - тест API подключения
- ⚠️ `Ошибка сервера: 403` - проблема с авторизацией
- ⚠️ `Ошибка сервера: 404` - неверный URL
- ⚠️ `Timeout` - сервер не отвечает
- 🔒 `Шифрование включено` - запросы шифруются
- 📡 `Отправка запроса...` - запрос к AI

## Отладочные логи (консоль)

### Запуск из командной строки

Для просмотра детальных логов запустите через консоль:

```cmd
# Windows - из папки проекта
cd c:\Project\ApiAi
python main.py

# Или если установлен
ApiAi.exe
```

### Что показывается в консоли

- Загрузка конфигурации
- Подключение к БД
- Ошибки импорта модулей
- Traceback при крашах
- Предупреждения Qt

## Файловые логи

### Где хранятся логи

```
Windows: %APPDATA%\ApiAi\logs\
         C:\Users\{USER}\AppData\Roaming\ApiAi\logs\

Linux:   ~/.local/share/ApiAi/logs/
macOS:   ~/Library/Application Support/ApiAi/logs/
```

### Основные файлы логов

Если включено логирование в файл (проверить в настройках):

- `app.log` - основной лог приложения
- `api_requests.log` - все API запросы
- `errors.log` - только ошибки

### Просмотр файловых логов

```cmd
# Windows
type %APPDATA%\ApiAi\logs\app.log

# Последние строки
powershell "Get-Content %APPDATA%\ApiAi\logs\app.log -Tail 50"
```

```bash
# Linux/macOS
cat ~/.local/share/ApiAi/logs/app.log

# Последние 50 строк
tail -50 ~/.local/share/ApiAi/logs/app.log

# В реальном времени
tail -f ~/.local/share/ApiAi/logs/app.log
```

## Конфигурация

### Где хранится конфиг

```
Из проекта:      c:\Project\ApiAi\config_qt.json
После установки: %LOCALAPPDATA%\ApiAi\config_qt.json
                 C:\Users\{USER}\AppData\Local\ApiAi\config_qt.json
```

### Где виртуальное окружение

```
Из проекта:      c:\Project\ApiAi\.venv\
После установки: %LOCALAPPDATA%\ApiAi\.venv\
                 C:\Users\{USER}\AppData\Local\ApiAi\.venv\
```

> **Важно:** Инсталлятор устанавливает приложение в `%LOCALAPPDATA%\ApiAi` (папка пользователя), поэтому права администратора НЕ требуются.

### Просмотр текущего конфига

```cmd
# Windows
type c:\Project\ApiAi\config_qt.json

# Или после установки
type %LOCALAPPDATA%\ApiAi\config_qt.json
```

### Важные секции конфига

```json
{
  "api_keys": {
    "telegram_url": "http://YOUR_SERVER:8000/ai_query",
    "telegram_key": "your_api_key",
    "telegram_enc_key": "hex_encryption_key",
    "telegram_use_encryption": true,
    "app_id": "apiai-v1"
  },
  "telegram_security": {
    "app_id": "apiai-v1"
  }
}
```

## Отладка проблем

### Проблема: Настройки не сохраняются

**Проверить:**

```cmd
# 1. Права на запись
icacls %LOCALAPPDATA%\ApiAi

# 2. Наличие файла
dir %LOCALAPPDATA%\ApiAi\config_qt.json

# 3. Валидность JSON
type %LOCALAPPDATA%\ApiAi\config_qt.json | python -m json.tool
```

### Проблема: Connection Timeout

**Диагностика:**

1. **Проверить URL доступен:**
   ```cmd
   curl http://YOUR_SERVER:8000/health
   ```

2. **Проверить конфиг:**
   ```cmd
   type config_qt.json | findstr telegram_url
   ```

3. **Тест подключения из Python:**
   ```python
   import requests
   url = "http://YOUR_SERVER:8000/ai_query"
   headers = {"X-API-KEY": "your_key", "X-APP-ID": "apiai-v1"}
   payload = {"prompt": "test", "provider": "anthropic", "max_tokens": 10}
   r = requests.post(url, json=payload, headers=headers, timeout=10)
   print(r.status_code, r.text)
   ```

### Проблема: Ошибка шифрования

**Проверить:**

1. Длина ключа (должна быть 64 символа hex):
   ```python
   enc_key = "your_hex_key"
   print(len(enc_key), "should be 64")
   ```

2. Тест шифрования:
   ```python
   from encryption import SecureMessenger
   messenger = SecureMessenger("your_hex_key")
   data = {"test": "data"}
   encrypted = messenger.encrypt(data)
   decrypted = messenger.decrypt(encrypted)
   print(decrypted)
   ```

### Проблема: API ключ не работает

**Проверить на сервере:**

```bash
# Логи сервера
docker logs telegram-api | grep "your_api_key_prefix"
docker logs telegram-api | grep "Invalid API"
```

## Режим отладки

### Включить verbose logging

Редактировать `main.py`:

```python
import logging
logging.basicConfig(
    level=logging.DEBUG,  # Вместо INFO
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
```

### Отладка API запросов

В `gui/pdf_search.py` добавить:

```python
# Перед requests.post
print(f"URL: {url}")
print(f"Headers: {headers}")
print(f"Payload: {json.dumps(payload)[:200]}")

# После response
print(f"Status: {response.status_code}")
print(f"Response: {response.text[:500]}")
```

## Тестирование API вручную

### Скрипт для теста подключения

Создать `test_api.py`:

```python
import requests
import json

url = "http://YOUR_SERVER:8000/ai_query"
api_key = "your_api_key"
app_id = "apiai-v1"

payload = {
    "prompt": "Test connection",
    "provider": "anthropic",
    "max_tokens": 10
}

headers = {
    "Content-Type": "application/json",
    "X-API-KEY": api_key,
    "X-APP-ID": "apiai-v1"
}

try:
    response = requests.post(url, json=payload, headers=headers, timeout=10)
    print(f"Status: {response.status_code}")
    print(f"Response: {response.json()}")
except Exception as e:
    print(f"Error: {e}")
```

Запуск:
```cmd
python test_api.py
```

## Экспорт логов для поддержки

### Собрать все логи

```cmd
# Windows
mkdir logs_export
copy %LOCALAPPDATA%\ApiAi\config_qt.json logs_export\
copy %LOCALAPPDATA%\ApiAi\logs\*.log logs_export\
powershell Compress-Archive -Path logs_export -DestinationPath ApiAi_logs_%date%.zip
```

```bash
# Linux/macOS
mkdir logs_export
cp ~/.local/share/ApiAi/config_qt.json logs_export/
cp ~/.local/share/ApiAi/logs/*.log logs_export/
tar -czf ApiAi_logs_$(date +%Y%m%d).tar.gz logs_export/
```

**⚠️ Важно:** Удалите API ключи из конфига перед отправкой!

## Типичные ошибки и решения

| Ошибка | Причина | Решение |
|--------|---------|---------|
| `ModuleNotFoundError: requests` | Не установлены зависимости | `pip install -r requirements.txt` |
| `config_qt.json not found` | Запуск из неправильной папки | Запустить из корня проекта |
| `Настройки не сохранились` | Не хватает метода `get_config()` | Обновить до последней версии |
| `Connection timeout` | Сервер недоступен или URL пустой | Проверить URL и доступность сервера |
| `403 Forbidden` | Неверный API ключ или APP_ID | Проверить ключ и whitelist на сервере |
| `Encryption failed` | Неверный ключ шифрования | Проверить длину (64 символа hex) |

## Быстрая диагностика

### Чек-лист для отладки

- [ ] Сервер доступен: `curl http://SERVER:8000/health`
- [ ] Конфиг существует: `dir config_qt.json`
- [ ] URL заполнен в конфиге
- [ ] API ключ заполнен
- [ ] APP_ID = `apiai-v1`
- [ ] Если шифрование - ключ 64 символа
- [ ] Логи сервера показывают запросы
- [ ] Нет ошибок в GUI логах
