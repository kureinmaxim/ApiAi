# ApiAi v2.4.2 Release Notes

**Release Date:** December 4, 2025

## 🎉 What's New

### Chat History Persistence
- **Save/Load Chat Sessions**: Save your conversations in JSON format with full context and metadata
- **Chat Library**: Browse and manage all saved chat sessions in one place
- **Import from TXT/MD**: Convert plain text conversations to chat format
- **Export Options**: Export chats to Markdown, TXT, PDF, or DOCX

### Improved User Experience
- **Detailed Tooltips**: Added comprehensive tooltips to all export buttons explaining their purpose
- **Window Size**: Optimized default window size to 1100x1000 for better usability
- **Better Save Dialog**: File Editor now has proper Cancel option with two-step save dialog

## 🐛 Bug Fixes

### Critical Fixes
- **Chat Save Error**: Fixed "ReferenceError: Can't find variable: invoke" when saving chats
- **ACL Permissions**: Added `fs:allow-mkdir` and `fs:allow-read-dir` permissions for chat library
- **File Editor Dialog**: Fixed confusing three-option dialog that only had two buttons

### Server Fixes (TelegramHelper)
- **Syntax Errors**: Fixed incomplete try block in `api.py` causing bot crashes
- **Indentation Errors**: Corrected missing indentation on line 626
- **Bot Responsiveness**: Restored missing function definitions for `timer_command` and `gen_chacha_key_command`

## 🔧 Technical Improvements

### APP_ID Management
- **Version Synchronization**: Refactored `show_keys.py` to import `ALLOWED_APPS` from `security.py`
- **Single Source of Truth**: Eliminated code duplication for version management
- **Documentation**: Added comprehensive APP_ID versioning guide in `VPS_GUIDE.md`

### Code Quality
- **Import Organization**: Added proper `invoke` import to `chat-history.js`
- **Dialog Flow**: Improved save dialog logic with clear two-step process

## 📝 Documentation Updates

- **VPS_GUIDE.md**: Added section on APP_ID versioning and key management
- **Tooltips**: Export buttons now have detailed descriptions explaining differences between:
  - **Load**: Restore from JSON (full context)
  - **Import**: Convert from TXT/MD (plain text)
  - **Save**: Preserve to JSON format

## 🔐 Security & Server

### TelegramHelper Server Updates
- **Version**: Updated to 2.2.1
- **Key Management**: Improved key deletion commands
- **Whitelist**: Properly configured `apiai-v2` in `ALLOWED_APPS`

## 📦 Installation

### macOS
Download the `.dmg` file from the releases page and drag ApiAi to your Applications folder.

### Building from Source
```bash
cd tauri-app
npm install
npm run tauri build
```

## ⚙️ Configuration

### First Time Setup
1. Open ApiAi
2. Click Settings (⚙️)
3. Configure your provider:
   - **Telegram**: Enter server URL and keys
   - **Anthropic**: Enter API key
   - **OpenAI**: Enter API key
4. Save settings and start chatting!

### Chat History Location
Saved chats are stored in: `~/ApiAi_Chats/`

## 🙏 Contributors

- **Developer**: Куреин М.Н. (Kurein M.N.)

## 📋 Known Issues

None at this time. If you encounter any issues, please report them on the GitHub Issues page.

---

## Full Changelog

**Features:**
- Chat history persistence with JSON format
- Chat library browser
- Import from TXT/MD files
- Improved tooltips for all buttons
- Two-step save dialog in File Editor

**Bug Fixes:**
- Fixed invoke import error in chat-history.js
- Fixed file editor save dialog confusion
- Fixed ACL permissions for mkdir and readdir
- Fixed TelegramHelper bot syntax errors
- Fixed APP_ID version synchronization

**Documentation:**
- Added APP_ID versioning guide
- Updated VPS deployment guide
- Improved tooltip descriptions

**Version Updates:**
- Client: 2.4.2
- Server (TelegramHelper): 2.2.1
- APP_ID: apiai-v2
