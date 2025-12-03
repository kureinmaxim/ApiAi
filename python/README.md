# ApiAi - Python Version

Python scripts and utilities for project management.

![Version](https://img.shields.io/badge/version-2.1.1-blue)
![Python](https://img.shields.io/badge/Python-3.8+-green)

## 🎯 Purpose

Python version serves as **development utilities**:
- ✅ Version management scripts (`scripts/update_version.py`)
- ✅ Build automation (future)
- ✅ CI/CD tools (future)
- ⚠️ Legacy GUI application (stable, but deprecated in favor of Tauri)

## 🚀 Quick Start

### Run GUI Application

```bash
cd python
python3 -m venv venv
source venv/bin/activate  # Linux/macOS
# or: venv\Scripts\activate  # Windows

pip install -r requirements.txt
python main.py
```

### Use Version Management Scripts

```bash
# From rust/ directory (required!)
cd ../rust

# Check version
make version-status

# Bump version
make version-bump-patch

# Sync all files
make version-sync
```

See [VERSION_MANAGEMENT.md](../VERSION_MANAGEMENT.md) for details.

## 📚 Documentation

Complete documentation is in the project root:

- **[BUILD.md](../BUILD.md)** - Build and run instructions
- **[VERSION_MANAGEMENT.md](../VERSION_MANAGEMENT.md)** - Version management
- **[CLEANUP.md](../CLEANUP.md)** - Cleanup build artifacts
- **[README.md](../README.md)** - Main project README

## 📦 Dependencies

Main dependencies (`requirements.txt`):
- **PySide6** - Qt GUI framework
- **requests** - HTTP client
- **cryptography** - Encryption support

Optional (for export features):
- **python-docx** - Word export
- **reportlab** - PDF export

## 🔧 Scripts

### Version Management
```bash
# All commands from python/
python scripts/update_version.py status
python scripts/update_version.py bump --type patch
python scripts/update_version.py sync
```

### Font Download (if missing)
```bash
python scripts/download_fonts.py
```

## ⚙️ Configuration

Uses shared `config_qt.json` in project root (created automatically on first run).

See main [README.md](../README.md) for configuration details.

## 📝 Version

Current version: **2.1.1** (synced with all project components)

---

**Note:** For modern GUI experience, use the **Tauri version** instead:
```bash
cd ../tauri-app
npm run tauri dev
```

**Developer:** Maksim Kurein  
**Documentation:** See [../README.md](../README.md)
