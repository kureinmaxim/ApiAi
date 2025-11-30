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
```bash
# Clone or download the repository
cd ApiAi

# Install dependencies
pip install -r requirements.txt

# Run the application
python main.py
```

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
