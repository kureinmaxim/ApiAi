# 🦀 ApiAi - Cheat Sheet

Быстрые команды для работы с разными версиями ApiAi.

---

## 🌐 Tauri Version (Recommended)

### Development
```bash
cd tauri-app
npm install                 # First time setup
npm run tauri dev          # Run with hot-reload
```

### Building
```bash
npm run tauri build        # Production build
```

### Debugging
```bash
cd src-tauri
cargo check                # Check Rust code
cargo clippy              # Lint
cargo test                # Run tests
```

---

## 🦀 Rust CLI Version

### 🛠 Build & Run
```bash
cd rust
cargo run                  # Run (Debug)
cargo build --release     # Build (Release)
cargo check               # Check code
cargo test                # Run tests
```

### 📦 Version Management
Use `make` commands to manage versions (syncs Rust, Python, Tauri).

```bash
make version-status              # Check current version
make version-sync                # Sync config
make version-bump-patch          # 1.0.0 → 1.0.1
make version-bump-minor          # 1.0.0 → 1.1.0
make version-set v=1.2.3         # Set specific version
```

### 🧹 Clean
```bash
cargo clean               # Clean build artifacts
```

### 📚 Documentation
```bash
cargo doc --open          # Generate and open docs
```

---

## 🐍 Python Version

### Run
```bash
cd python
python3 main.py           # Run application
```

### Build
```bash
python3 scripts/build.py  # Create installer
```

---

## 🔐 Security

### PIN Access
- **Default PIN:** `1234`
- **Unlock methods:**
  - Click lock icon 🔒 in sidebar
  - Double-click "Maksim Kurein" in footer
- **Protected:** Telegram URL, Port, Encryption Key, API Key

---

## ⚙️ Configuration

### Location
```
ApiAi/config_qt.json      # Shared by all versions
```

### Structure
```json
{
  "app_info": {
    "version": "1.0.5",
    "developer_en": "Maksim Kurein"
  },
  "security": {
    "pin_code": "1234"
  }
}
```

---

## 🎨 Tauri UI Features

### Core Features
- ✨ Modern gradient design (purple/indigo)
- 🔒 Smart PIN protection (settings only)
- 📱 Responsive layout
- 🌙 Dark theme
- ⚡ Fast Rust backend

### Provider Settings Modal (NEW!)
- Click **"⚙️ Settings"** button to open
- Settings saved to `config_qt.json` automatically
- Настройки восстанавливаются при перезапуске
- Show/Hide keys toggle

### File Editor Mode (NEW!)
1. Click **"📄 Select File"** to choose file
2. Write instructions (e.g., "Add new section about features")
3. Click **Send** - AI processes file
4. Choose: Overwrite original OR Create new with _AI_N suffix
5. Click file name → copy path to clipboard

### Echo Test
- Click **"🔊 Echo"** to test connection
- Shows: Round-trip time, Server time, Network time

---

## 📝 Quick Reference

| Task | Command |
|------|---------|
| Run Tauri dev | `cd tauri-app && npm run tauri dev` |
| Run Rust CLI | `cd rust && cargo run` |
| Run Python | `cd python && python3 main.py` |
| Check version | `cd rust && make version-status` |
| Bump version | `cd rust && make version-bump-patch` |

---

## 🆘 Troubleshooting

### Tauri won't start
```bash
cd tauri-app
rm -rf node_modules package-lock.json
npm install
npm run tauri dev
```

### Rust compilation errors
```bash
rustup update stable
cargo clean
cargo build
```

### Settings locked
- Default PIN: `1234`
- Try: Click 🔒 or double-click developer name
