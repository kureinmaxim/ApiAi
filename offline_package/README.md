# Офлайн пакеты для ApiAi

Эта папка предназначена для хранения `.whl` файлов Python пакетов для офлайн установки.

## Как подготовить

### Вариант 1: Из корня проекта (рекомендуется)

**Windows:**
```powershell
# Перейдите в корень проекта (если еще не там)
cd C:\Project\ApiAi

# Скачать все зависимости для офлайн установки
python -m pip download -r requirements.txt -d offline_package

# Для конкретной платформы (например, Windows 64-bit)
python -m pip download -r requirements.txt `
    -d offline_package `
    --platform win_amd64 `
    --python-version 38 `
    --only-binary=:all:
```

**macOS/Linux:**
```bash
# Перейдите в корень проекта (если еще не там)
cd /path/to/ApiAi

# Скачать все зависимости для офлайн установки
python3 -m pip download -r requirements.txt -d offline_package

# Для конкретной платформы (например, Linux 64-bit)
python3 -m pip download -r requirements.txt \
    -d offline_package \
    --platform linux_x86_64 \
    --python-version 38 \
    --only-binary=:all:
```

### Вариант 2: Из папки offline_package

**Windows:**
```powershell
# Перейдите в папку offline_package
cd offline_package

# Скачать все зависимости для офлайн установки
python -m pip download -r ../requirements.txt -d .
```

**macOS/Linux:**
```bash
# Перейдите в папку offline_package
cd offline_package

# Скачать все зависимости для офлайн установки
python3 -m pip download -r ../requirements.txt -d .
```

## Использование

На компьютере без интернета:

**Windows:**
```powershell
# Из корня проекта
python -m pip install --no-index --find-links=offline_package -r requirements.txt

# Или из папки offline_package
cd offline_package
python -m pip install --no-index --find-links=. -r ../requirements.txt
```

**macOS/Linux:**
```bash
# Из корня проекта
python3 -m pip install --no-index --find-links=offline_package -r requirements.txt

# Или из папки offline_package
cd offline_package
python3 -m pip install --no-index --find-links=. -r ../requirements.txt
```

> **Примечание:** Использование `python -m pip` (или `python3 -m pip`) более надежно, чем просто `pip`, особенно в виртуальных окружениях.

## Размер

~100-150 MB (основной вес - PySide6)

## Что будет скачано

- PySide6 + shiboken6 (~80 MB)
- requests (~1 MB)
- cryptography (~5 MB)
- Транзитивные зависимости (~20 MB)

## Внимание

Эта папка добавлена в `.gitignore` - файлы не хранятся в Git, их нужно скачивать каждый раз для сборки релиза.
