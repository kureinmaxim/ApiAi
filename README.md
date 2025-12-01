# ApiAi

AI-powered component search application with support for multiple AI providers.

## 📁 Project Structure

This repository contains **two versions** of ApiAi:

- **`python/`** - **Stable Python version** (v1.0.3) - Production-ready, fully functional
- **`rust/`** - **Experimental Rust version** - Work in progress, for testing and migration

> [!NOTE]
> If you're a user, use the **Python version**. The Rust version is for developers experimenting with a Rust rewrite.

```
ApiAi/
├── python/          # 🐍 Python version (stable)
│   ├── main.py      # Entry point
│   ├── gui/         # GUI modules
│   └── config/      # Configuration
├── rust/            # 🦀 Rust version (experimental)
│   ├── Cargo.toml
│   └── src/
├── docs/            # Documentation
└── README.md        # This file
```

## Documentation

### General
- [Encryption Protocol](docs/ENCRYPTION.md)
- [Network Ports](docs/PORTS.md)

### Versions
- [**Python Version Documentation**](python/README.md)
- [**Rust Version Documentation**](rust/README.md)

See `python/` directory for detailed setup, build, and logs guides.

#### 2. Install Dependencies

**Windows:**
```bash
# Install dependencies
python -m pip install -r requirements.txt
```

**macOS/Linux:**
```bash
# Install dependencies
python3 -m pip install -r requirements.txt
```

> **Note:** Using `python -m pip` (or `python3 -m pip`) is more reliable than just `pip`, especially in virtual environments.

**Required dependencies:**
- PySide6 (GUI)
- requests (HTTP)
- cryptography (encryption)

**Optional dependencies (for Word/PDF export):**
- python-docx
- reportlab

#### 3. Run the Application

**Windows:**
```bash
python main.py
```

**macOS/Linux:**
```bash
python3 main.py
```

> **Note:** Always activate the virtual environment before running the application. On Windows, use `.venv\Scripts\activate`, on macOS/Linux use `source venv/bin/activate`.

> **Installed Version:** The Windows installer does NOT require administrator privileges and installs to `%LOCALAPPDATA%\ApiAi` (user directory). The virtual environment `.venv` is created automatically in the same folder on first run.

## Configuration
On first run, the application creates `config_qt.json` automatically in the **project root**.

To configure API keys:
1. Double-click on developer name in footer
2. Enter PIN (default: `1234`)
3. Open Settings via Menu → PDF Search → Settings
4. Enter your API keys

### Supported AI Providers
- **Anthropic Claude**: Get API key from [console.anthropic.com](https://console.anthropic.com/)
- **OpenAI GPT**: Get API key from [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **Telegram Bot**: Configure custom bot API endpoint

## Version Management

The project uses a synchronized versioning system for both Python and Rust.

### From Rust Directory (`rust/`)
Use the provided `Makefile` for simple commands:

```bash
make version-status       # Check current version
make version-sync         # Sync local config
make version-bump-patch   # 1.0.0 -> 1.0.1
make version-bump-minor   # 1.0.0 -> 1.1.0
```

### From Python Directory (`python/`)
Use the python script directly:

```bash
python scripts/update_version.py status
python scripts/update_version.py bump --type patch
```

## Usage
1. Launch the application
2. Select AI provider from dropdown
3. Enter component name or custom query
4. Click Search
5. View and save results

## Security
- PIN protection for settings access
- AES-256-GCM encryption for Telegram Bot communication
- API keys stored locally in `config_qt.json` (git-ignored)

## License
Private use

## Developer
Kurein M.N.
