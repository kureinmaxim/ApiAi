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


def get_project_root():
    """Получает корень проекта"""
    return Path(__file__).parent.parent


def get_template_path():
    """Путь к шаблону конфига"""
    return get_project_root() / "config" / "config_qt.json.template"


def get_local_config_path():
    """Путь к локальному конфигу"""
    return get_project_root() / "config_qt.json"


def load_template():
    """Загружает шаблон"""
    template_path = get_template_path()
    if not template_path.exists():
        print(f"❌ Шаблон не найден: {template_path}")
        sys.exit(1)
    
    with open(template_path, 'r', encoding='utf-8') as f:
        return json.load(f)


def save_template(data):
    """Сохраняет шаблон"""
    template_path = get_template_path()
    with open(template_path, 'w', encoding='utf-8') as f:
        json.dump(data, f, indent=2, ensure_ascii=False)
    print(f"✅ Шаблон обновлен: {template_path}")


def parse_version(version_str):
    """Парсит версию в компоненты"""
    try:
        parts = version_str.split('.')
        return {
            'major': int(parts[0]),
            'minor': int(parts[1]) if len(parts) > 1 else 0,
            'patch': int(parts[2]) if len(parts) > 2 else 0
        }
    except (ValueError, IndexError):
        print(f"❌ Неверный формат версии: {version_str}")
        sys.exit(1)


def version_to_string(v):
    """Преобразует словарь версии в строку"""
    return f"{v['major']}.{v['minor']}.{v['patch']}"


def bump_version(current_version, bump_type):
    """Увеличивает версию"""
    v = parse_version(current_version)
    
    if bump_type == 'major':
        v['major'] += 1
        v['minor'] = 0
        v['patch'] = 0
    elif bump_type == 'minor':
        v['minor'] += 1
        v['patch'] = 0
    elif bump_type == 'patch':
        v['patch'] += 1
    else:
        print(f"❌ Неизвестный тип обновления: {bump_type}")
        sys.exit(1)
    
    return version_to_string(v)


def get_current_date():
    """Получает текущую дату в формате DD.MM.YYYY"""
    return datetime.now().strftime("%d.%m.%Y")


def get_current_date_iso():
    """Получает текущую дату в формате ISO"""
    return datetime.now().strftime("%Y-%m-%d")


def update_version(bump_type=None, version=None, release_date=None, developer=None, dry_run=False, no_release_date=False):
    """Обновляет версию в шаблоне"""
    
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
        return
    
    # Сохраняем
    save_template(template)
    
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
