# 📦 Version Management - Tauri Edition

> **ВАЖНО:** Этот документ описывает управление версиями для **Tauri приложения**. 
> Старый `rust/` проект (egui) больше не используется.

---

## 🎯 Текущая версия: **2.4.2**

**Релиз:** 04.12.2025  
**Платформа:** macOS (ARM64)

---

## 📁 Файлы версий

### 1. **tauri-app/src-tauri/Cargo.toml**
```toml
[package]
name = "tauri-app"
version = "2.4.2"  # ← Обновить
```

### 2. **tauri-app/src-tauri/tauri.conf.json**
```json
{
  "version": "2.4.2"  # ← Обновить
}
```

### 3. **tauri-app/src/index.html**
```html
<span class="footer-value">2.4.2</span>  <!-- Обновить -->
```

---

---

## 🛠️ Команды управления версиями

Все команды выполняются из папки `tauri-app/`:

```bash
cd tauri-app
```

### Проверить текущую версию

```bash
make version-status
```

**Вывод:**
```
Current version: 2.4.2

Files to sync:
  ✓ tauri-app/src-tauri/Cargo.toml
  ✓ tauri-app/src-tauri/tauri.conf.json
  ✓ tauri-app/src/index.html
```

### Синхронизировать конфигурацию

```bash
make version-sync
```

Обновляет все файлы если версия была изменена вручную.

### Увеличить версию

**Patch (2.4.2 → 2.4.3):**
```bash
make version-bump-patch
```

**Minor (2.4.2 → 2.5.0):**
```bash
make version-bump-minor
```

**Major (2.4.2 → 3.0.0):**
```bash
make version-bump-major
```

### Установить конкретную версию

```bash
make version-set v=X.Y.Z
```

**Пример:**
```bash
make version-set v=2.5.0
```

---

## 🚀 Типичные сценарии

### Исправление бага

```bash
cd tauri-app

# 1. Проверить текущую версию
make version-status

# 2. Увеличить patch версию
make version-bump-patch
# 2.4.2 → 2.4.3

# 3. Проверить изменения
git diff src-tauri/Cargo.toml src-tauri/tauri.conf.json src/index.html
```

### Новая функция

```bash
cd tauri-app

# Увеличить minor версию
make version-bump-minor
# 2.4.2 → 2.5.0
```

### Крупное обновление

```bash
cd tauri-app

# Увеличить major версию
make version-bump-major
# 2.4.2 → 3.0.0
```

---

## 🚀 Процесс релиза

```bash
# 1. Обновить версию в 3 файлах (см. выше)

# 2. Коммит
git add tauri-app/src-tauri/Cargo.toml tauri-app/src-tauri/tauri.conf.json tauri-app/src/index.html
git commit -m "Release: Version X.Y.Z"

# 3. Тег
git tag -a vX.Y.Z -m "ApiAi vX.Y.Z - Description"
git push origin vX.Y.Z

# 4. Сборка
cd tauri-app
npm run tauri build

# 5. GitHub Release
gh release create vX.Y.Z \
  "tauri-app/src-tauri/target/release/bundle/dmg/tauri-app_X.Y.Z_aarch64.dmg#ApiAi-vX.Y.Z-macOS-ARM64.dmg" \
  --title "ApiAi vX.Y.Z" \
  --notes-file RELEASE_NOTES_vX.Y.Z.md
```
---

## ✅ Активный проект

**Используется только:**
- ✅ `tauri-app/` - Tauri приложение
- ✅ `shared-rs/` - Общая Rust библиотека

Всё остальное - legacy код.
