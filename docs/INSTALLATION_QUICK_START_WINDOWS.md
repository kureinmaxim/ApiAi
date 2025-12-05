# ⚡ Быстрая установка ApiAi на Windows

Краткая шпаргалка для опытных пользователей.

---

## 📋 Чек-лист установки

### 1. Проверка инструментов
```cmd
node --version    # Должно быть v16+
npm --version
rustc --version   # Если нет - установите через rustup-init.exe
```

### 2. Установка необходимых инструментов (если нужно)

**Node.js:**
- Скачайте с [nodejs.org](https://nodejs.org) и установите

**Rust:**
```powershell
# Скачайте rustup-init.exe с rustup.rs
# Или через PowerShell:
Invoke-WebRequest https://win.rustup.rs/x86_64 -OutFile rustup-init.exe
.\rustup-init.exe
```

**Microsoft C++ Build Tools:**
- Скачайте с [visualstudio.microsoft.com/visual-cpp-build-tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Установите "C++ build tools"

### 3. Клонирование проекта
```cmd
cd C:\Projects
git clone https://github.com/ВАШ_USERNAME/ApiAi.git
cd ApiAi
```

### 4. Установка зависимостей
```cmd
cd tauri-app
npm install
```

### 5. Первый запуск
```cmd
npm run tauri dev
```

**Примечание:** Первая компиляция Rust может занять 10-15 минут.

---

## 🔧 Если что-то пошло не так

### Rust не установлен
```powershell
# Скачайте rustup-init.exe с rustup.rs
# Запустите установщик
.\rustup-init.exe
```

### Microsoft C++ Build Tools не установлены
- Скачайте с [visualstudio.microsoft.com/visual-cpp-build-tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Установите "C++ build tools"
- Перезапустите терминал

### Ошибки компиляции
```cmd
cd tauri-app\src-tauri
cargo clean
cd ..\..
npm run tauri dev
```

### npm не найден
- Переустановите Node.js с опцией "Add to PATH"
- Перезапустите терминал

---

## ⚙️ Первоначальная настройка

1. Запустите приложение: `cd tauri-app && npm run tauri dev`
2. Откройте Settings (⚙️)
3. Введите PIN: `1234`
4. Настройте AI провайдера
5. Сохраните настройки

---

## 📍 Расположение конфигурации

```
C:\Users\ВАШ_USERNAME\AppData\Roaming\com.apiai.app\config.json
```

**Быстрый доступ (PowerShell):**
```powershell
cd $env:APPDATA\com.apiai.app
```

---

## 📚 Полная инструкция

См. [INSTALLATION_WINDOWS.md](INSTALLATION_WINDOWS.md) для подробной документации.

---

## ✅ Готово!

Теперь можно использовать ApiAi! 🎉

