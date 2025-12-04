# ApiAi

AI-powered assistant application with support for multiple AI providers and encryption.

## 📁 Project Structure

This repository contains the **Tauri-based** ApiAi application with modern web UI.

> [!NOTE]
> **Current Version**: v2.4.2 (Tauri Edition)  
> Previous Python and Rust CLI versions have been archived.

```
ApiAi/
├── tauri-app/              # 🌐 Main Tauri application
│   ├── src/                # Web frontend (HTML/CSS/JS)
│   │   ├── main.js         # Main application logic
│   │   ├── chat-history.js # Chat persistence
│   │   ├── file-editor.js  # File editing mode
│   │   ├── network-monitor.js # Network logging
│   │   └── index.html      # UI
│   ├── src-tauri/          # Rust backend
│   │   ├── src/lib.rs      # Tauri commands
│   │   └── Cargo.toml      # Dependencies
│   ├── scripts/            # Utility scripts
│   │   ├── update_version.py    # Version management
│   │   ├── backup_config.sh     # Config backup
│   │   └── restore_config.sh    # Config restore
│   ├── config_templates/   # Config templates
│   ├── Makefile            # Build & version commands
│   └── README.md           # Tauri-specific docs
├── shared-rs/              # 📦 Shared Rust library
│   └── src/                # Common API & encryption
│       ├── api.rs          # API clients
│       └── encryption.rs   # ChaCha20-Poly1305
├── docs/                   # 📚 Documentation
│   ├── ENCRYPTION.md
│   └── PORTS.md
├── VERSION_MANAGEMENT_TAURI.md  # Version management guide
├── CLEANUP_GUIDE.md        # Cleanup old files
└── README.md               # This file
```

## Documentation

### General
- [Encryption Protocol](docs/ENCRYPTION.md) - ChaCha20-Poly1305 encryption details
- [Network Ports](docs/PORTS.md) - Port usage and configuration
- [Version Management](VERSION_MANAGEMENT_TAURI.md) - Version bump and sync commands
- [Config Management](tauri-app/CONFIG_MANAGEMENT.md) - Backup and restore settings

### Application
- [**Tauri App Documentation**](tauri-app/README.md) - Full Tauri app guide
- [Cleanup Guide](CLEANUP_GUIDE.md) - Remove archived files

## Quick Start

### Installation & Setup

```bash
cd tauri-app
npm install
npm run tauri dev
```

**Features:**
- ✨ Modern web UI with dark theme and smooth animations
- 🔒 PIN-protected settings (default PIN: 1234)
- 💬 **Chat History Persistence** - Save, load, and manage conversations
- 📚 **Chat Library** - Browse all saved sessions
- 📥 **Import/Export** - TXT, MD, PDF, DOCX formats
- 📄 **File Editor Mode** - AI-powered file editing with versioning
- 🔊 **Echo Test** - Connection testing with timing statistics
- 📡 **Network Monitor** - Request history sidebar with encryption status
- 💾 **Auto-save Settings** - Configuration persists between sessions
- 🎨 Beautiful gradient UI with purple/indigo theme

## Configuration

### Location

**Development and Production:**
```
~/Library/Application Support/com.apiai.app/config.json  (macOS)
C:\Users\USERNAME\AppData\Roaming\com.apiai.app\config.json  (Windows)
~/.local/share/com.apiai.app/config.json  (Linux)
```

### Setup

**Option 1: Through UI (Recommended)**
1. Launch ApiAi
2. Click **"⚙️ Settings"** in sidebar
3. If locked, enter PIN (default: `1234`)
4. Configure:
   - Provider (Telegram/Anthropic/OpenAI)
   - Telegram URL, Port, API Key
   - Encryption settings
5. Click **"💾 Save Settings"**

**Option 2: Restore from Template**
```bash
cd tauri-app
make config-restore  # Restores from config_templates/
```

**Option 3: Backup Current Config**
```bash
cd tauri-app
make config-backup  # Saves to ~/Documents/ApiAi_Backups/
```

### Supported AI Providers
- **Anthropic Claude**: API key from [console.anthropic.com](https://console.anthropic.com/)
- **OpenAI GPT**: API key from [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **Telegram Bot**: Custom bot API endpoint with encryption support

## Version Management

All version commands run from `tauri-app/` directory.

### Quick Commands

```bash
cd tauri-app

# Check version
make version-status

# Bump version
make version-bump-patch   # 2.4.2 → 2.4.3
make version-bump-minor   # 2.4.2 → 2.5.0
make version-bump-major   # 2.4.2 → 3.0.0

# Set specific version
make version-set v=2.5.0

# Sync if manually edited
make version-sync
```

**Full documentation:** [VERSION_MANAGEMENT_TAURI.md](VERSION_MANAGEMENT_TAURI.md)

## Build & Cleanup

### Build Commands

```bash
cd tauri-app

make build    # Production build (.dmg for macOS)
make dev      # Development mode
make clean    # Clean build artifacts
```

### Cleanup Build Artifacts

```bash
cd tauri-app

# Clean Tauri build (~5GB)
rm -rf src-tauri/target

# Or use make
make clean
```

> [!TIP]
> Build artifacts are recreated automatically on next build.  
> Safe to clean regularly to save disk space.

## Usage

### Basic Chat
1. Launch ApiAi
2. Select AI provider (Telegram/Anthropic/OpenAI)
3. Type your query
4. Send (or Ctrl/Cmd+Enter)

### File Editor Mode
1. Switch to "📝 File Editor" mode
2. Select file to edit
3. Describe changes needed
4. AI processes and saves (overwrite or create new version)

### Chat Management
- **Save**: 💾 Save current conversation
- **Load**: 📂 Restore previous session
- **Library**: 📚 Browse all saved chats
- **Import**: 📥 Convert TXT/MD to chat format
- **Export**: 📄 MD, TXT, PDF, DOCX

### Network Monitoring
- Click **📡 Console** for developer console instructions
- View request history in sidebar (encrypted/unencrypted status)
- Filter by encryption status

## Security

- 🔒 **PIN Protection**: Settings locked by default (PIN: 1234)
- 🔐 **ChaCha20-Poly1305**: Military-grade encryption for Telegram
- 🔑 **Local Storage**: API keys stored locally (never sent to cloud)
- 📡 **Request Monitoring**: See which requests are encrypted
- ✅ **Secure by Default**: Encryption indicators on all network logs
- 🎯 **Console Logging**: Full request/response details for debugging

## License

Private use

## Developer

**Kurein M.N.**  
Version: 2.4.2  
Release: December 4, 2025
