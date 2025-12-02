# ApiAi - Tauri Application

Modern web-based AI assistant with beautiful UI and PIN-protected settings.

![Version](https://img.shields.io/badge/version-1.0.5-blue)
![Tauri](https://img.shields.io/badge/Tauri-2.0-yellow)

## ✨ Features

- 🎨 **Modern Design**: Beautiful purple/indigo gradient UI with smooth animations
- 🔒 **Smart Security**: PIN protection for sensitive settings only
- 💬 **AI Chat**: Support for multiple AI providers (Anthropic, OpenAI, Telegram)
- 📱 **Responsive**: Adaptive layout for different window sizes
- 🌙 **Dark Theme**: Eye-friendly dark color scheme
- ⚡ **Fast**: Built with Rust backend and lightweight web frontend

## 🚀 Quick Start

### Prerequisites
- Node.js (v16 or higher)
- Rust (latest stable)
- npm or yarn

### Installation

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev

# Build for production
npm run tauri build
```

## 🔐 Security Features

### PIN Protection
- **Default PIN**: `1234`
- **Protected Settings**: Telegram URL, Port, Encryption Key, API Key
- **Unlock Methods**:
  - Click the lock icon (🔒) in sidebar
  - Double-click "Maksim Kurein" in footer
- **Smart UX**: App opens directly without PIN screen for immediate use

### Encryption
- AES-256-GCM for Telegram Bot communication
- API keys stored locally (never transmitted)

## 🎯 Usage

1. **Launch Application**: Opens directly to chat interface
2. **Select Provider**: Choose from Telegram, Anthropic, or OpenAI
3. **Unlock Settings** (if needed):
   - Click lock icon 🔒 or double-click developer name
   - Enter PIN: `1234`
   - Configure Telegram URL, Port, Encryption, or API Key
4. **Start Chatting**: Type message and press Send or Ctrl+Enter

## 🎨 UI Components

### Main Interface
- **Sidebar**: Provider selection, mode settings, protected configuration fields
- **Chat Area**: Message history with user/AI message formatting
- **Input Area**: Message composition with keyboard shortcuts
- **Footer**: Developer info, version, release date, live window size

### Design Elements
- **Colors**: Purple/indigo gradients (`#667eea` → `#764ba2`)
- **Effects**: Glassmorphism, backdrop blur, smooth transitions
- **Animations**: Slide-in messages, unlock animation, hover effects
- **Typography**: Inter font family for modern look

## 📁 Project Structure

```
tauri-app/
├── src/                    # Frontend (Web)
│   ├── index.html         # Main HTML structure
│   ├── styles.css         # Modern CSS with gradients
│   └── main.js            # JavaScript logic
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── main.rs        # Tauri app entry point
│   │   ├── lib.rs         # Library code
│   │   └── api.rs         # API client logic
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
└── package.json           # Node.js dependencies
```

## ⚙️ Configuration

Configuration is loaded from `config_qt.json` in the project root:

```json
{
  "app_info": {
    "name": "ApiAi",
    "version": "1.0.5",
    "developer_en": "Maksim Kurein"
  },
  "security": {
    "pin_code": "1234",
    "require_pin": true
  }
}
```

## 🔧 Development

### Available Commands

```bash
# Development
npm run tauri dev          # Run with hot-reload

# Building
npm run tauri build        # Create production bundle

# Rust only
cd src-tauri
cargo check               # Check Rust code
cargo test                # Run tests
cargo clippy              # Lint Rust code
```

### Technologies Used

**Frontend:**
- HTML5, CSS3, JavaScript (ES6+)
- Modern CSS features (gradients, animations, backdrop-filter)

**Backend:**
- Rust
- Tauri 2.0
- reqwest (HTTP client)
- aes-gcm (encryption)
- serde (JSON serialization)

## 🐛 Troubleshooting

### App won't start
```bash
# Clean and reinstall
rm -rf node_modules package-lock.json
npm install
```

### Rust compilation errors
```bash
# Update Rust
rustup update stable
```

### Settings won't unlock
- Default PIN is `1234`
- Try clicking lock icon directly
- Or double-click developer name in footer

## 📝 Changelog

### v1.0.5 (2025-12-01)
- ✨ Modern UI redesign with gradients and animations
- 🔒 Smart PIN protection (settings-only)
- 📊 Footer with developer, version, release date, window size
- 🎨 Purple/indigo color scheme
- 🔓 Double-click unlock on developer name

## 👨‍💻 Developer

**Maksim Kurein**  
Version: 1.0.5  
Release: 2025-12-01

## 📄 License

Private use
