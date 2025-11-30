# ApiAi - Python Version

This is the **stable production version** of ApiAi, written in Python with PySide6.

## Quick Start

### Prerequisites
- Python 3.8+
- pip

### Installation

1. **Navigate to python directory:**
   ```bash
   cd python
   ```

2. **Create virtual environment:**
   
   **Windows:**
   ```bash
   python -m venv .venv
   .venv\Scripts\activate
   ```
   
   **macOS/Linux:**
   ```bash
   python3 -m venv venv
   source venv/bin/activate
   ```

3. **Install dependencies:**
   ```bash
   pip install -r requirements.txt
   ```

4. **Run application:**
   ```bash
   python main.py
   ```

## Configuration

On first run, `config_qt.json` will be created automatically from the template.

To configure API keys:
1. Double-click developer name in footer
2. Enter PIN (default: `1234`)
3. Open Settings → API Keys
4. Enter your keys

## Documentation

- [Setup Guide](../SETUP.md) - Detailed installation instructions
- [Build Guide](../BUILD.md) - Creating installers and releases
- [Logs Guide](../LOGS_GUIDE.md) - Understanding application logs

## Support

See main [README.md](../README.md) for full documentation.
