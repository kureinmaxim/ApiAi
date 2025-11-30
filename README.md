# ApiAi

AI-powered component search application with support for multiple AI providers.

## Features
- 🤖 Multiple AI providers (Anthropic Claude, OpenAI GPT, Telegram Bot)
- 🔒 Secure API key storage with encryption support
- 🎨 Modern dark/light themes
- 🔐 PIN protection for expert mode
- 📱 Cross-platform (Windows, macOS, Linux)

## Installation

### Prerequisites
- Python 3.8+
- pip

### Setup

#### 1. Create Virtual Environment

**Windows:**
```bash
# Clone or download the repository
cd ApiAi

# Create virtual environment
python -m venv .venv

# Activate virtual environment
.venv\Scripts\activate
```

**macOS/Linux:**
```bash
# Clone or download the repository
cd ApiAi

# Create virtual environment
python3 -m venv venv

# Activate virtual environment
source venv/bin/activate
```

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
On first run, the application creates `config_qt.json` automatically. 

To configure API keys:
1. Double-click on developer name in footer
2. Enter PIN (default: `1234`)
3. Open Settings via Menu → PDF Search → Settings
4. Enter your API keys

### Supported AI Providers
- **Anthropic Claude**: Get API key from [console.anthropic.com](https://console.anthropic.com/)
- **OpenAI GPT**: Get API key from [platform.openai.com/api-keys](https://platform.openai.com/api-keys)
- **Telegram Bot**: Configure custom bot API endpoint

## Usage
1. Launch the application
2. Select AI provider from dropdown
3. Enter component name or custom query
4. Click Search
5. View and save results

## Security
- PIN protection for settings access
- AES-256-GCM encryption for Telegram Bot communication
- API keys stored locally in `config_qt.json`

## License
Private use

## Developer
Kurein M.N.
