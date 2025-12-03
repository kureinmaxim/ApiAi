# ApiAi - Tauri Application

Modern web-based AI assistant with Rust backend.

![Version](https://img.shields.io/badge/version-2.1.1-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.0-yellow)

## � Quick Start

```bash
# From this directory
npm install
npm run tauri dev
```

## � Documentation

Complete documentation available in the project root:

- **[BUILD.md](../BUILD.md)** - Build instructions, run commands, troubleshooting
- **[docs/PROJECT_STRUCTURE.md](../docs/PROJECT_STRUCTURE.md)** - Project structure and architecture
- **[VERSION_MANAGEMENT.md](../VERSION_MANAGEMENT.md)** - Version management guide
- **[CLEANUP.md](../CLEANUP.md)** - Cleanup instructions for build artifacts
- **[README.md](../README.md)** - Main project documentation

## 🎯 Features

- 🎨 Modern purple/indigo gradient UI
- 🔒 PIN-protected settings
- 💬 Multiple AI providers (Anthropic, OpenAI, Telegram)
- 📱 Responsive design
- 🌙 Dark theme
- ⚡ Fast Rust + Web stack

## 🏗 Architecture

**Backend:** Uses [`shared-rs`](../shared-rs/) library for API clients and encryption.

**Frontend:** HTML/CSS/JS with modern design.

See [PROJECT_STRUCTURE.md](../docs/PROJECT_STRUCTURE.md) for details.

## 🔧 Development

```bash
# Development mode with hot-reload
npm run tauri dev

# Build for production
npm run tauri build

# Clean build artifacts
cd src-tauri && cargo clean
```

## 📦 Version

Current version: **2.1.1** (synced with all project components)

See [VERSION_MANAGEMENT.md](../VERSION_MANAGEMENT.md) for version management.

---

**Developer:** Maksim Kurein  
**Repository:** Main project README at [../README.md](../README.md)
