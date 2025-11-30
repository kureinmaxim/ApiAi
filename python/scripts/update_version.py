#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Инструмент для управления версиями ApiAi

Обновляет версию в шаблоне конфигурации и синхронизирует с файлами проекта.
"""

import json
import sys
import os
from datetime import datetime
from pathlib import Path


def get_repo_root():
    """Получает корень всего репозитория (над python/)"""
    return get_project_root().parent


def update_rust_files(new_version, dry_run=False):
    """Обновляет версию в файлах Rust"""
    repo_root = get_repo_root()
    cargo_toml = repo_root / "rust" / "Cargo.toml"
    main_rs = repo_root / "rust" / "src" / "main.rs"
    
    if not cargo_toml.exists():
        print(f"⚠️ Rust проект не найден: {cargo_toml}")
        return

    # 1. Update Cargo.toml
    try:
        with open(cargo_toml, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Simple regex-like replacement for version = "..."
        import re
        new_content = re.sub(r'version = "\d+\.\d+\.\d+"', f'version = "{new_version}"', content, count=1)
        
        if content != new_content:
            if not dry_run:
                with open(cargo_toml, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"✅ Обновлен Cargo.toml: {new_version}")
            else:
                print(f"🔍 [Dry Run] Обновлен бы Cargo.toml: {new_version}")
    except Exception as e:
        print(f"❌ Ошибка обновления Cargo.toml: {e}")

    # 2. Update main.rs (default config)
    try:
        with open(main_rs, 'r', encoding='utf-8') as f:
            content = f.read()
            
        new_content = re.sub(r'version: "\d+\.\d+\.\d+".to_string\(\)', f'version: "{new_version}".to_string()', content)
        
        if content != new_content:
            if not dry_run:
                with open(main_rs, 'w', encoding='utf-8') as f:
                    f.write(new_content)
                print(f"✅ Обновлен main.rs: {new_version}")
            else:
                print(f"🔍 [Dry Run] Обновлен бы main.rs: {new_version}")
    except Exception as e:
        print(f"❌ Ошибка обновления main.rs: {e}")


def update_version(bump_type=None, version=None, release_date=None, developer=None, dry_run=False, no_release_date=False):
    """Обновляет версию в шаблоне и Rust файлах"""
    
    # Загружаем шаблон
    template = load_template()
    app_info = template.get('app_info', {})
    
    current_version = app_info.get('version', '1.0.0')
    
    # Определяем новую версию
    if version:
        new_version = version
    elif bump_type:
        new_version = bump_version(current_version, bump_type)
    else:
        print("❌ Необходимо указать --bump или --version")
        sys.exit(1)
    
    # Обновляем app_info
    app_info['version'] = new_version
    
    if not no_release_date:
        if release_date:
            app_info['release_date'] = release_date
        else:
            app_info['release_date'] = get_current_date()
        
        app_info['last_updated'] = get_current_date_iso()
    
    if developer:
        app_info['developer_en'] = developer
    
    template['app_info'] = app_info
    
    # Сообщаем об изменениях
    print(f"\n{'='*60}")
    print(f"📦 ОБНОВЛЕНИЕ ВЕРСИИ ApiAi")
    print(f"{'='*60}")
    print(f"  Старая версия: {current_version}")
    print(f"  Новая версия:  {new_version}")
    if not no_release_date:
        print(f"  Дата релиза:   {app_info['release_date']}")
    print(f"{'='*60}\n")
    
    if dry_run:
        print("🔍 Режим тестирования - изменения не сохранены")
        update_rust_files(new_version, dry_run=True)
        return
    
    # Сохраняем шаблон
    save_template(template)
    
    # Обновляем Rust файлы
    update_rust_files(new_version, dry_run=False)
    
    print("\n💡 Следующие шаги:")
    print("   1. Синхронизируйте: python scripts/update_version.py sync")
    print("   2. Проверьте статус: python scripts/update_version.py status")
    print("   3. Закоммитьте: git add config/ && git commit -m 'Release: v{}'".format(new_version))


def show_status():
    """Показывает текущий статус версий"""
    template = load_template()
    app_info = template.get('app_info', {})
    
    print(f"\n{'='*60}")
    print(f"📊 ТЕКУЩАЯ ВЕРСИЯ ApiAi")
    print(f"{'='*60}")
    print(f"  Версия:       {app_info.get('version', 'N/A')}")
    print(f"  Дата релиза:  {app_info.get('release_date', 'N/A')}")
    print(f"  Обновлена:    {app_info.get('last_updated', 'N/A')}")
    print(f"  Разработчик:  {app_info.get('developer_en', 'N/A')}")
    print(f"{'='*60}")
    
    # Проверяем локальный конфиг
    local_config = get_local_config_path()
    if local_config.exists():
        with open(local_config, 'r', encoding='utf-8') as f:
            local = json.load(f)
        local_version = local.get('app_info', {}).get('version', 'N/A')
        
        if local_version == app_info.get('version'):
            print(f"✅ Локальный config синхронизирован")
        else:
            print(f"⚠️  Локальный config устарел ({local_version})")
            print(f"   Выполните: python scripts/update_version.py sync")
    else:
        print(f"ℹ️  Локальный config не создан (будет создан при запуске)")
    
    print()


def sync_config():
    """Синхронизирует локальный конфиг с шаблоном"""
    template = load_template()
    local_config = get_local_config_path()
    
    if not local_config.exists():
        print("ℹ️  Локальный config не найден - создаю из шаблона")
        with open(local_config, 'w', encoding='utf-8') as f:
            json.dump(template, f, indent=2, ensure_ascii=False)
        print(f"✅ Создан: {local_config}")
        return
    
    # Загружаем локальный конфиг
    with open(local_config, 'r', encoding='utf-8') as f:
        local = json.load(f)
    
    # Синхронизируем только app_info
    local['app_info'] = template['app_info']
    
    # Сохраняем
    with open(local_config, 'w', encoding='utf-8') as f:
        json.dump(local, f, indent=2, ensure_ascii=False)
    
    print(f"✅ Локальный config синхронизирован: {local_config}")
    print(f"   Версия: {template['app_info']['version']}")


def main():
    """Главная функция"""
    import argparse
    
    parser = argparse.ArgumentParser(description='Управление версиями ApiAi')
    
    subparsers = parser.add_subparsers(dest='command', help='Команда')
    
    # Команда status
    subparsers.add_parser('status', help='Показать текущий статус')
    
    # Команда sync
    subparsers.add_parser('sync', help='Синхронизировать локальный config')
    
    # Команда bump
    bump_parser = subparsers.add_parser('bump', help='Обновить версию')
    bump_parser.add_argument('--type', choices=['major', 'minor', 'patch'], required=True,
                            help='Тип обновления')
    bump_parser.add_argument('--no-release-date', action='store_true',
                            help='Не обновлять дату релиза')
    bump_parser.add_argument('--dry-run', action='store_true',
                            help='Тестовый запуск без сохранения')
    
    # Команда set
    set_parser = subparsers.add_parser('set', help='Установить версию')
    set_parser.add_argument('version', help='Версия (например: 1.2.0)')
    set_parser.add_argument('--release-date', help='Дата релиза (DD.MM.YYYY)')
    set_parser.add_argument('--developer', help='Имя разработчика')
    set_parser.add_argument('--dry-run', action='store_true',
                           help='Тестовый запуск без сохранения')
    
    # Устаревший формат для совместимости с BOMCategorizer
    parser.add_argument('--bump', choices=['major', 'minor', 'patch'],
                       help='[Устаревший] Тип обновления версии')
    parser.add_argument('--version', help='[Устаревший] Установить конкретную версию')
    parser.add_argument('--release-date', help='[Устаревший] Дата релиза')
    parser.add_argument('--developer', help='[Устаревший] Имя разработчика')
    parser.add_argument('--no-release-date', action='store_true',
                       help='[Устаревший] Не обновлять дату')
    parser.add_argument('--dry-run', action='store_true',
                       help='[Устаревший] Тестовый запуск')
    
    args = parser.parse_args()
    
    # Создаем папку config если её нет
    config_dir = get_project_root() / "config"
    config_dir.mkdir(exist_ok=True)
    
    # Обработка команд
    if args.command == 'status':
        show_status()
    elif args.command == 'sync':
        sync_config()
    elif args.command == 'bump':
        update_version(
            bump_type=args.type,
            no_release_date=args.no_release_date,
            dry_run=args.dry_run
        )
    elif args.command == 'set':
        update_version(
            version=args.version,
            release_date=args.release_date,
            developer=args.developer,
            dry_run=args.dry_run
        )
    # Устаревший формат
    elif args.bump or args.version:
        update_version(
            bump_type=args.bump,
            version=args.version,
            release_date=args.release_date,
            developer=args.developer,
            no_release_date=args.no_release_date,
            dry_run=args.dry_run
        )
    else:
        parser.print_help()


if __name__ == '__main__':
    main()
