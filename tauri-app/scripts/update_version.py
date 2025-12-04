#!/usr/bin/env python3
"""
Version Management for Tauri App
Синхронизирует версии между Cargo.toml, tauri.conf.json и index.html
"""

import json
import re
import sys
from pathlib import Path

# Paths
SCRIPT_DIR = Path(__file__).parent
TAURI_DIR = SCRIPT_DIR.parent
CARGO_TOML = TAURI_DIR / "src-tauri" / "Cargo.toml"
TAURI_CONF = TAURI_DIR / "src-tauri" / "tauri.conf.json"
INDEX_HTML = TAURI_DIR / "src" / "index.html"


def get_version_from_cargo():
    """Читает версию из Cargo.toml"""
    with open(CARGO_TOML, 'r') as f:
        for line in f:
            if line.startswith('version ='):
                match = re.search(r'"([^"]+)"', line)
                if match:
                    return match.group(1)
    return None


def set_version_in_cargo(version):
    """Устанавливает версию в Cargo.toml"""
    lines = []
    with open(CARGO_TOML, 'r') as f:
        for line in f:
            if line.startswith('version ='):
                lines.append(f'version = "{version}"\n')
            else:
                lines.append(line)
    
    with open(CARGO_TOML, 'w') as f:
        f.writelines(lines)


def set_version_in_tauri_conf(version):
    """Устанавливает версию в tauri.conf.json"""
    with open(TAURI_CONF, 'r') as f:
        config = json.load(f)
    
    config['version'] = version
    
    with open(TAURI_CONF, 'w') as f:
        json.dump(config, f, indent=2)
        f.write('\n')


def set_version_in_html(version):
    """Устанавливает версию в index.html footer"""
    with open(INDEX_HTML, 'r') as f:
        content = f.read()
    
    # Find and replace version in footer
    pattern = r'(<span class="footer-label">Version:</span>\s*<span class="footer-value">)[^<]+(</span>)'
    replacement = f'\\g<1>{version}\\g<2>'
    new_content = re.sub(pattern, replacement, content)
    
    with open(INDEX_HTML, 'w') as f:
        f.write(new_content)


def sync_version(version):
    """Синхронизирует версию во всех файлах"""
    print(f"Syncing version {version} to all files...")
    set_version_in_cargo(version)
    set_version_in_tauri_conf(version)
    set_version_in_html(version)
    print("✓ Cargo.toml")
    print("✓ tauri.conf.json")
    print("✓ index.html")


def bump_version(bump_type):
    """Увеличивает версию"""
    current = get_version_from_cargo()
    if not current:
        print("Error: Could not read current version")
        sys.exit(1)
    
    parts = current.split('.')
    major, minor, patch = int(parts[0]), int(parts[1]), int(parts[2])
    
    if bump_type == 'patch':
        patch += 1
    elif bump_type == 'minor':
        minor += 1
        patch = 0
    elif bump_type == 'major':
        major += 1
        minor = 0
        patch = 0
    
    new_version = f"{major}.{minor}.{patch}"
    print(f"Bumping version: {current} → {new_version}")
    sync_version(new_version)
    print(f"\n✅ Version updated to {new_version}")


def status():
    """Показывает текущую версию"""
    version = get_version_from_cargo()
    print(f"Current version: {version}")
    print("\nFiles to sync:")
    print(f"  ✓ {CARGO_TOML.relative_to(TAURI_DIR.parent)}")
    print(f"  ✓ {TAURI_CONF.relative_to(TAURI_DIR.parent)}")
    print(f"  ✓ {INDEX_HTML.relative_to(TAURI_DIR.parent)}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage:")
        print("  python update_version.py status")
        print("  python update_version.py sync [version]")
        print("  python update_version.py bump <patch|minor|major>")
        sys.exit(1)
    
    command = sys.argv[1]
    
    if command == "status":
        status()
    elif command == "sync":
        version = sys.argv[2] if len(sys.argv) > 2 else get_version_from_cargo()
        sync_version(version)
    elif command == "bump":
        if len(sys.argv) < 3:
            print("Error: Specify bump type (patch, minor, major)")
            sys.exit(1)
        bump_version(sys.argv[2])
    else:
        print(f"Unknown command: {command}")
        sys.exit(1)
