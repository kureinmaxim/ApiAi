# Офлайн пакеты для ApiAi

Эта папка предназначена для хранения `.whl` файлов Python пакетов для офлайн установки.

## Как подготовить

```bash
# Скачать все зависимости для офлайн установки
pip download -r ../requirements.txt -d .

# Для конкретной платформы (например, Windows 64-bit)
pip download -r ../requirements.txt \
    -d . \
    --platform win_amd64 \
    --python-version 38 \
    --only-binary=:all:
```

## Использование

На компьютере без интернета:

```bash
# Установить зависимости из этой папки
pip install --no-index --find-links=../offline_packages -r ../requirements.txt
```

## Размер

~100-150 MB (основной вес - PySide6)

## Что будет скачано

- PySide6 + shiboken6 (~80 MB)
- requests (~1 MB)
- cryptography (~5 MB)
- Транзитивные зависимости (~20 MB)

## Внимание

Эта папка добавлена в `.gitignore` - файлы не хранятся в Git, их нужно скачивать каждый раз для сборки релиза.
