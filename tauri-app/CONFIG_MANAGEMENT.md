# 🔧 Configuration Management

## 📁 Где хранится конфигурация

### macOS
```
~/Library/Application Support/com.apiai.app/config.json
```

### Windows
```
C:\Users\USERNAME\AppData\Roaming\com.apiai.app\config.json
```

### Linux
```
~/.local/share/com.apiai.app/config.json
```

---

## 💾 Бэкап и восстановление

### Создать бэкап текущей конфигурации

```bash
cd tauri-app
./scripts/backup_config.sh
```

**Результат:**
- Создается файл в `~/Documents/ApiAi_Backups/config_YYYYMMDD_HHMMSS.json`
- Исходный конфиг не изменяется

### Восстановить конфигурацию из шаблона

```bash
cd tauri-app
./scripts/restore_config.sh
```

**Что делает:**
1. Создает бэкап текущего config.json (если существует)
2. Копирует `config_templates/config.json.template` в app data
3. Перезапустите приложение - настройки загрузятся

---

## 📝 Шаблон конфигурации

### Редактировать шаблон

```bash
# Откройте в редакторе
code config_templates/config.json.template
```

**Структура:**
```json
{
  "api_keys": {
    "anthropic": "",
    "openai": "",
    "telegram_url": "http://...",
    "telegram_key": "...",
    "telegram_enc_key": "...",
    "telegram_use_encryption": true
  },
  "ui": {
    "window_width": 1100,
    "window_height": 1000
  },
  "security": {
    "require_pin": true
  }
}
```

### Обновить шаблон из текущего конфига

```bash
# macOS
cp ~/Library/Application\ Support/com.apiai.app/config.json \
   config_templates/config.json.template
```

---

## ⚠️ Безопасность

### .gitignore

```gitignore
# Конфиг с реальными ключами НЕ коммитится
config_templates/config.json
config_templates/*.backup

# Шаблон можно коммитить (без ключей)
# или добавить в .gitignore если содержит секреты
```

### Хранение ключей

**Рекомендации:**
1. ✅ Храните `config.json.template` **локально** (не в git)
2. ✅ Создайте `config.json.template.example` **без ключей** для git
3. ✅ Регулярно делайте бэкапы в `~/Documents/ApiAi_Backups/`

---

## 🔄 Типичные сценарии

### Первая установка на новом компьютере

```bash
cd tauri-app

# 1. Скопировать шаблон из безопасного места (облако, USB)
cp /path/to/backup/config.json.template config_templates/

# 2. Восстановить конфиг
./scripts/restore_config.sh

# 3. Запустить приложение
npm run tauri dev
```

### Переустановка приложения

```bash
cd tauri-app

# Восстановить из шаблона
./scripts/restore_config.sh

# Готово! Настройки восстановлены
```

### Регулярный бэкап

```bash
cd tauri-app

# Создать бэкап (автоматически с timestamp)
./scripts/backup_config.sh

# Результат в ~/Documents/ApiAi_Backups/
```

---

## 📦 Что происходит при удалении приложения

### Стандартное удаление (Drag to Trash)
- ❌ Приложение удалено
- ✅ Конфиг **остается** в `~/Library/Application Support/`
- ✅ При переустановке настройки восстановятся

### Полное удаление (AppCleaner, CleanMyMac)
- ❌ Приложение удалено
- ❌ Конфиг **тоже удален**
- ✅ Используйте `./scripts/restore_config.sh` для восстановления

---

## 🎯 Быстрая справка

```bash
# В папке tauri-app/

# Бэкап
./scripts/backup_config.sh

# Восстановление
./scripts/restore_config.sh

# Редактировать шаблон
code config_templates/config.json.template

# Посмотреть текущий конфиг (macOS)
cat ~/Library/Application\ Support/com.apiai.app/config.json
```
