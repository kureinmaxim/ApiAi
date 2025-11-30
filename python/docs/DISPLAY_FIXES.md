# 🔤 Решение проблем отображения в ApiAi

Документ описывает решения проблем с отображением текста, шрифтов и символов на разных платформах.

---

## 📋 Содержание

1. [Шрифты на macOS Retina](#-шрифты-на-macos-retina)
2. [Эмодзи в Windows консоли](#-эмодзи-в-windows-консоли)
3. [Масштабирование интерфейса](#-масштабирование-интерфейса)

---

## 🖥️ Шрифты на macOS Retina

### Проблема

На macOS с Retina дисплеями (2x/3x DPI) шрифты в приложении могут отображаться **слишком мелко** или **слишком крупно**.

**Симптомы:**
- Мелкие метки полей
- Мелкий текст на кнопках
- Мелкие плейсхолдеры
- Или наоборот - огромные элементы

### Решение

ApiAi использует PySide6, которая автоматически поддерживает High DPI. Реализованы macOS-специфичные исправления.

#### 1. High DPI Support в Qt

```python
# В main.py перед созданием QApplication:
import os
import sys
from PySide6.QtWidgets import QApplication
from PySide6.QtCore import Qt

if __name__ == "__main__":
    # Настройка кодировки для Windows
    if sys.platform == 'win32':
        import io
        try:
            if hasattr(sys.stdout, 'buffer'):
                sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8', errors='replace')
            if hasattr(sys.stderr, 'buffer'):
                sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8', errors='replace')
        except Exception:
            pass
    
    # High DPI для macOS
    if sys.platform == 'darwin':
        os.environ['QT_AUTO_SCREEN_SCALE_FACTOR'] = '1'
        os.environ['QT_ENABLE_HIGHDPI_SCALING'] = '1'
        os.environ['QT_SCALE_FACTOR_ROUNDING_POLICY'] = 'PassThrough'
    
    # Включаем High DPI
    QApplication.setAttribute(Qt.AA_EnableHighDpiScaling, True)
    QApplication.setAttribute(Qt.AA_UseHighDpiPixmaps, True)
    
    from gui.main_window import main
    main()
```

#### 2. Определение DPI

```python
from PySide6.QtGui import QGuiApplication

# В ApiAiMainWindow.__init__:
def __init__(self):
    super().__init__()
    
    # Определяем DPI экрана
    screens = QGuiApplication.screens()
    if screens:
        dpr = screens[0].devicePixelRatio()
        print(f"Device Pixel Ratio: {dpr}")
        
        # Корректируем масштаб для Retina
        if sys.platform == 'darwin' and dpr >= 2:
            # На Retina используем увеличенный масштаб
            self.scale_factor = 1.2
        else:
            self.scale_factor = 1.0
```

#### 3. Настройка масштаба через меню

ApiAi имеет встроенное меню **View → Scaling** с опциями:
- 80%
- 90%
- 100%
- 110%
- 120%
- 150%

Пользователь может выбрать комфортный масштаб вручную.

### Рекомендуемые настройки

| Платформа | DPI | Масштаб | Примечание |
|-----------|-----|---------|------------|
| **Windows** | 1x | 100% | По умолчанию |
| **Windows** | 1.25x | 100% | Qt автоматически |
| **Windows** | 1.5x+ | 90-100% | Qt автоматически |
| **macOS Retina** | 2x | 110-120% | Вручную |
| **macOS обычный** | 1x | 100% | По умолчанию |
| **Linux** | 1x | 100% | По умолчанию |

---

## 💻 Эмодзи в Windows консоли

### Проблема

При выводе эмодзи (✅ 💡 ℹ️) в Windows консоли возникает ошибка:

```
UnicodeEncodeError: 'charmap' codec can't encode character '\u2705'
```

**Причина:** Windows консоль по умолчанию использует cp1251/cp866, которые не поддерживают Unicode.

### Решение: UTF-8 для консоли

Реализовано в `main.py`:

```python
import sys
import io

if sys.platform == 'win32':
    try:
        if hasattr(sys.stdout, 'buffer'):
            sys.stdout = io.TextIOWrapper(
                sys.stdout.buffer, 
                encoding='utf-8', 
                errors='replace'
            )
        if hasattr(sys.stderr, 'buffer'):
            sys.stderr = io.TextIOWrapper(
                sys.stderr.buffer, 
                encoding='utf-8', 
                errors='replace'
            )
    except Exception:
        pass
```

### Результат

После исправления консольные скрипты корректно отображают эмодзи:

```
✅ Создан минимальный конфиг: config_qt.json

============================================================
📊 ТЕКУЩАЯ ВЕРСИЯ ApiAi
============================================================
  Версия:       1.0.0
  Дата релиза:  30.11.2025
============================================================
```

### Альтернатива: Windows Terminal

Для лучшей поддержки Unicode используйте **Windows Terminal** вместо cmd.exe:

```powershell
# Установка через winget
winget install Microsoft.WindowsTerminal

# Или через Microsoft Store
# Поиск "Windows Terminal"
```

---

## 📐 Масштабирование интерфейса

### Через меню приложения

1. Откройте **View → Scaling**
2. Выберите нужный масштаб (80-150%)
3. Перезапустите приложение (требуется для полного применения)

### Через конфигурационный файл

Отредактируйте `config_qt.json`:

```json
{
  "window": {
    "scale_factor": 1.2
  },
  "ui": {
    "font_size": 12,
    "scale_factor": 1.2
  }
}
```

**Значения:**
- `0.8` = 80% (мелкий текст)
- `1.0` = 100% (по умолчанию)
- `1.2` = 120% (крупный текст)
- `1.5` = 150% (очень крупный)

### Автоматическое определение

```python
# В ApiAiMainWindow
def _detect_optimal_scale(self):
    """Определяет оптимальный масштаб для текущего дисплея"""
    screens = QGuiApplication.screens()
    if not screens:
        return 1.0
    
    dpr = screens[0].devicePixelRatio()
    
    # Логика определения
    if sys.platform == 'darwin':
        # macOS Retina
        return 1.2 if dpr >= 2 else 1.0
    elif sys.platform == 'win32':
        # Windows High DPI
        return 1.0  # Qt сам разберётся
    else:
        # Linux
        return 1.0
```

---

## 🔤 Шрифты

### Системные шрифты

ApiAi использует системные шрифты по умолчанию:

| Платформа | Шрифт |
|-----------|-------|
| **Windows** | Segoe UI |
| **macOS** | SF Pro / Helvetica Neue |
| **Linux** | Liberation Sans / DejaVu Sans |

### Пользовательские шрифты

Для изменения шрифта отредактируйте `styles.py`:

```python
# В DARK_THEME или LIGHT_THEME
QMainWindow, QDialog, QWidget {
    font-family: "Your Font Name", sans-serif;
    font-size: 10pt;
}
```

Доступные шрифты в `styles.py`:
- Consolas (для QTextEdit - моноширинный)
- Можно добавить Google Fonts или локальные

---

## ✅ Чек-лист для разработчиков

### Перед релизом:

- [ ] Протестировано на Windows (1x, 1.25x, 1.5x DPI)
- [ ] Протестировано на macOS Retina (2x DPI)
- [ ] Протестировано на Linux (1x DPI)
- [ ] Эмодзи в консоли работают на Windows
- [ ] Масштабирование сохраняется в config
- [ ] Меню View → Scaling доступно

---

## 🔧 Диагностика

### Проверка DPI

```python
from PySide6.QtWidgets import QApplication
from PySide6.QtGui import QGuiApplication

app = QApplication([])
screens = QGuiApplication.screens()
if screens:
    print(f"Screen: {screens[0].name()}")
    print(f"DPI: {screens[0].logicalDotsPerInch()}")
    print(f"Device Pixel Ratio: {screens[0].devicePixelRatio()}")
    print(f"Physical DPI: {screens[0].physicalDotsPerInch()}")
```

### Проверка кодировки

```python
import sys
print(f"stdout encoding: {sys.stdout.encoding}")
print(f"stderr encoding: {sys.stderr.encoding}")
print(f"Default encoding: {sys.getdefaultencoding()}")

# Тест эмодзи
print("✅ ℹ️ 💡 ❌ 🔄")
```

---

## 📚 Внешние ресурсы

- [Qt High DPI Documentation](https://doc.qt.io/qt-6/highdpi.html)
- [PySide6 High DPI](https://doc.qt.io/qtforpython-6/overviews/highdpi.html)
- [Windows Terminal](https://aka.ms/terminal)

---

**Последнее обновление:** 30.11.2025  
**Версия документа:** 1.0
