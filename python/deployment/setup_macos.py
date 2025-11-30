"""
py2app setup script для создания macOS .app bundle

Usage:
    python deployment/setup_macos.py py2app
"""

from setuptools import setup
import json
import os

# Читаем версию из конфига
def get_version():
    try:
        with open('config/config_qt.json.template', 'r') as f:
            config = json.load(f)
            return config['app_info']['version']
    except:
        return '1.0.0'

VERSION = get_version()

APP_NAME = 'ApiAi'
APP = ['main.py']
DATA_FILES = [
    ('', ['config_qt.json']),
    ('config', ['config/config_qt.json.template']),
    ('fonts', ['fonts/DejaVuSans.ttf', 'fonts/DejaVuSans-Bold.ttf']),
    ('docs', [
        'docs/AI_INTEGRATION_GUIDE.md',
        'docs/API_MANAGEMENT.md',
        'docs/VERSION_MANAGEMENT.md',
        'docs/DISPLAY_FIXES.md',
        'docs/OFFLINE_INSTALLATION_GUIDE.md',
        'docs/FONT_SETUP_QUICK.md'
    ]),
    ('', ['README.md', 'BUILD.md', 'SETUP.md'])
]

OPTIONS = {
    'argv_emulation': False,
    'packages': ['PySide6', 'gui', 'cryptography', 'requests'],
    'includes': [
        'PySide6.QtCore',
        'PySide6.QtGui', 
        'PySide6.QtWidgets',
        'encryption',
        'utils',
        'styles',
        'config_manager'
    ],
    'excludes': [
        'matplotlib',
        'numpy',
        'scipy',
        'pandas',
        'tkinter',
        'PyQt5',
        'PyQt6'
    ],
    'iconfile': None,  # Можно добавить иконку позже
    'plist': {
        'CFBundleName': APP_NAME,
        'CFBundleDisplayName': APP_NAME,
        'CFBundleGetInfoString': "AI Search Interface",
        'CFBundleIdentifier': "com.kurein.apiai",
        'CFBundleVersion': VERSION,
        'CFBundleShortVersionString': VERSION,
        'NSHumanReadableCopyright': '© 2025 Kurein M.N.',
        'NSHighResolutionCapable': True,
    }
}

setup(
    app=APP,
    name=APP_NAME,
    data_files=DATA_FILES,
    options={'py2app': OPTIONS},
    setup_requires=['py2app'],
)
