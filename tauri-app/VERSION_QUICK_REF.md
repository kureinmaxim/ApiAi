# 🎯 Быстрая справка - Version Management

## Команды (из `tauri-app/`)

```bash
cd tauri-app

# Проверить версию
make version-status

# Синхронизировать файлы
make version-sync

# Увеличить версию
make version-bump-patch     # 2.4.2 → 2.4.3
make version-bump-minor     # 2.4.2 → 2.5.0  
make version-bump-major     # 2.4.2 → 3.0.0

# Установить конкретную
make version-set v=X.Y.Z

# Сборка
make build                  # Production build
make dev                    # Development mode
```

## Файлы версий

- `src-tauri/Cargo.toml` - Rust версия
- `src-tauri/tauri.conf.json` - Tauri версия
- `src/index.html` - UI footer версия

## Полная документация

См. [VERSION_MANAGEMENT_TAURI.md](VERSION_MANAGEMENT_TAURI.md)
