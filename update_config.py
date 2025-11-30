#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Скрипт для обновления структуры config_qt.json

Использование:
    python update_config.py

Что делает:
- Проверяет наличие config_qt.json
- Добавляет недостающие секции из template
- Сохраняет существующие настройки пользователя
"""

import json
import os
from pathlib import Path


def update_config():
    """Обновляет структуру config_qt.json"""
    
    config_path = Path('config_qt.json')
    template_path = Path('config/config_qt.json.template')
    
    # Проверка наличия файлов
    if not config_path.exists():
        print("❌ config_qt.json не найден")
        print("💡 Запустите приложение для создания config из template")
        return False
    
    if not template_path.exists():
        print("❌ Template файл не найден: config/config_qt.json.template")
        return False
    
    try:
        # Загрузка текущего config
        with open(config_path, 'r', encoding='utf-8') as f:
            current_config = json.load(f)
        
        # Загрузка template
        with open(template_path, 'r', encoding='utf-8') as f:
            template_config = json.load(f)
        
        print("📋 Проверка структуры config_qt.json...")
        updated = False
        
        # Проверка и добавление недостающих секций
        for section_name, section_template in template_config.items():
            if section_name == 'app_info':
                # app_info не трогаем - он управляется через update_version.py
                continue
            
            if section_name not in current_config:
                print(f"   ➕ Добавляю секцию: {section_name}")
                current_config[section_name] = section_template
                updated = True
            elif isinstance(section_template, dict):
                # Проверка полей внутри секции
                for key, value in section_template.items():
                    if key not in current_config[section_name]:
                        print(f"   ➕ Добавляю поле: {section_name}.{key}")
                        current_config[section_name][key] = value
                        updated = True
        
        if updated:
            # Сохранение обновленного config
            with open(config_path, 'w', encoding='utf-8') as f:
                json.dump(current_config, f, indent=2, ensure_ascii=False)
            print("\n✅ Config успешно обновлен!")
            print(f"📁 Сохранено: {config_path.absolute()}")
        else:
            print("\n✅ Config уже актуален, обновления не требуются")
        
        return True
        
    except json.JSONDecodeError as e:
        print(f"\n❌ Ошибка парсинга JSON: {e}")
        print("💡 Проверьте синтаксис файла config_qt.json")
        return False
    except Exception as e:
        print(f"\n❌ Ошибка: {e}")
        return False


if __name__ == '__main__':
    print("=" * 60)
    print("🔧 Обновление структуры config_qt.json")
    print("=" * 60)
    print()
    
    success = update_config()
    
    print()
    print("=" * 60)
    
    if success:
        print("\n💡 Полезные команды:")
        print("   python scripts/update_version.py status  # Проверка версии")
        print("   python scripts/update_version.py sync    # Синхронизация")
        print("   python main.py                           # Запуск приложения")
    
    exit(0 if success else 1)
