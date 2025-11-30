# 🚀 Быстрая настройка шрифтов для ApiAi

## Для разработчиков (подготовка инфраструктуры)

```bash
# Скачать шрифты одной командой
python scripts/download_fonts.py

# Или через curl (для macOS с SSL проблемами)
curl -L -o fonts/dejavu.zip "https://github.com/dejavu-fonts/dejavu-fonts/releases/download/version_2.37/dejavu-fonts-ttf-2.37.zip"
cd fonts
unzip -j dejavu.zip "*/DejaVuSans.ttf" "*/DejaVuSans-Bold.ttf"
rm dejavu.zip

# Проверить (должно быть 2 файла)
ls fonts/
```

✅ **Готово!** Шрифты готовы для будущего использования (например, экспорт результатов в PDF).

---

## Для пользователей (если в будущем появится PDF экспорт)

### Windows

1. **Скачайте:** https://dejavu-fonts.github.io/
2. **Установите:** Правый клик на файлы → "Установить"
3. **Перезапустите** программу

### macOS

```bash
# Через Homebrew
brew tap homebrew/cask-fonts
brew install --cask font-dejavu
```

### Linux

```bash
# Ubuntu/Debian
sudo apt-get install fonts-dejavu fonts-dejavu-core

# Fedora
sudo dnf install dejavu-sans-fonts

# Arch
sudo pacman -S ttf-dejavu
```

---

## Примечание

ApiAi в настоящее время **не генерирует PDF** (в отличие от BOMCategorizer).

Шрифты подготовлены на будущее, если добавите функцию экспорта результатов AI поиска в PDF через ReportLab.

---

## Подробнее

- 📖 [Полная документация](docs/DISPLAY_FIXES.md)
- 📦 [Структура шрифтов](fonts/README.md)
