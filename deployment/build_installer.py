# -*- coding: utf-8 -*-
"""
Скрипт для автоматической сборки инсталлятора ApiAi (Windows)

Этот скрипт:
1. Копирует все необходимые файлы в temp_installer/
2. Запускает Inno Setup Compiler для создания .exe инсталлятора
3. Очищает временные файлы после сборки

Использование:
    python deployment/build_installer.py
"""

import os
import shutil
import subprocess
import sys
import json
import re

# Конфигурация
TEMP_DIR = "temp_installer"
INNO_SETUP_PATH = r"C:\Program Files (x86)\Inno Setup 6\ISCC.exe"


def read_version_from_config():
    """Читает версию из config файла"""
    try:
        template_path = "config/config_qt.json.template"
        if os.path.exists(template_path):
            with open(template_path, "r", encoding="utf-8") as f:
                config = json.load(f)
                return config.get("app_info", {}).get("version", "1.0.0")
        return "1.0.0"
    except Exception as e:
        print(f"⚠️  Ошибка чтения версии: {e}")
        return "1.0.0"


VERSION = read_version_from_config()
OUTPUT_FILE = f"ApiAi-Setup-{VERSION}.exe"

# Файлы для копирования
FILES_TO_COPY = [
    "main.py",
    "config_qt.json",
    "config/config_qt.json.template",
    "requirements.txt",
    "utils.py",
    "styles.py",
    "config_manager.py",
    "encryption.py",
    "README.md",
    "BUILD.md",
    "SETUP.md",
    "update_config.py"
]

# Директории для копирования
DIRECTORIES_TO_COPY = [
    "gui",
    "docs",
    "fonts",
    "offline_packages",
    "config",
    "scripts"
]


def print_step(message):
    """Вывод шага выполнения"""
    print(f"\n{'='*60}")
    print(f"  {message}")
    print(f"{'='*60}")


def clean_temp_dir():
    """Удаляет временную директорию если она существует"""
    if os.path.exists(TEMP_DIR):
        print(f"Удаляю старую директорию {TEMP_DIR}...")
        shutil.rmtree(TEMP_DIR)
    os.makedirs(TEMP_DIR)
    print(f"Создана директория {TEMP_DIR}")


def copytree_exclude(src, dst, exclude_dirs=None, exclude_files=None):
    """
    Копирует директорию с исключением указанных директорий и файлов.
    """
    if exclude_dirs is None:
        exclude_dirs = ['__pycache__', '.git', '.pytest_cache', '.mypy_cache', '.venv', 'venv']
    if exclude_files is None:
        exclude_files = ['*.pyc', '*.pyo', '*.pyd']
    
    os.makedirs(dst, exist_ok=True)
    
    for item in os.listdir(src):
        src_path = os.path.join(src, item)
        dst_path = os.path.join(dst, item)
        
        # Пропускаем исключенные директории
        if os.path.isdir(src_path) and item in exclude_dirs:
            continue
        
        # Пропускаем исключенные файлы
        if os.path.isfile(src_path):
            skip = False
            for pattern in exclude_files:
                if item.endswith(pattern.replace('*', '')):
                    skip = True
                    break
            if skip:
                continue
            shutil.copy2(src_path, dst_path)
        else:
            # Рекурсивно копируем поддиректории
            copytree_exclude(src_path, dst_path, exclude_dirs, exclude_files)


def copy_files():
    """Копирует необходимые файлы в temp_installer"""
    print("\nКопирую файлы...")
    
    for file in FILES_TO_COPY:
        if os.path.exists(file):
            dest = os.path.join(TEMP_DIR, file)
            
            # Создаем директорию назначения
            os.makedirs(os.path.dirname(dest), exist_ok=True)
            
            shutil.copy2(file, dest)
            print(f"  [OK] {file}")
        else:
            print(f"  [SKIP] {file} (не найден)")
    
    # Копируем директории с исключением ненужных файлов
    for directory in DIRECTORIES_TO_COPY:
        if os.path.exists(directory):
            dest = os.path.join(TEMP_DIR, directory)
            if os.path.exists(dest):
                shutil.rmtree(dest)
            copytree_exclude(directory, dest)
            print(f"  [OK] {directory}/ (директория)")
        else:
            print(f"  [SKIP] {directory}/ (не найдена)")


def update_iss_version(iss_file, version):
    """Обновляет версию в .iss файле"""
    try:
        with open(iss_file, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Ищем строку с версией и заменяем её
        content = re.sub(
            r'#define MyAppVersion ".*?"',
            f'#define MyAppVersion "{version}"',
            content
        )
        
        with open(iss_file, 'w', encoding='utf-8') as f:
            f.write(content)
        
        print(f"[OK] Версия в {iss_file} обновлена на {version}")
        return True
    except Exception as e:
        print(f"⚠️  Ошибка обновления версии в {iss_file}: {e}")
        return False


def run_inno_setup(iss_file):
    """Запускает Inno Setup Compiler"""
    if not os.path.exists(INNO_SETUP_PATH):
        print(f"\n[ERROR] Inno Setup не найден: {INNO_SETUP_PATH}")
        print("Установите Inno Setup или укажите правильный путь в переменной INNO_SETUP_PATH")
        return False
    
    print(f"\nЗапуск Inno Setup Compiler...")
    try:
        result = subprocess.run(
            [INNO_SETUP_PATH, iss_file],
            capture_output=True,
            text=True,
            encoding='utf-8',
            errors='replace'
        )
        
        if result.returncode == 0:
            print("[OK] Инсталлятор успешно собран!")
            
            # Проверяем размер файла
            if os.path.exists(OUTPUT_FILE):
                size_mb = os.path.getsize(OUTPUT_FILE) / (1024 * 1024)
                print(f"\nРазмер инсталлятора: {size_mb:.2f} MB")
            
            return True
        else:
            print(f"[ERROR] Ошибка при сборке инсталлятора")
            print(f"Код возврата: {result.returncode}")
            if result.stdout:
                print(f"Вывод:\n{result.stdout}")
            if result.stderr:
                print(f"Ошибки:\n{result.stderr}")
            return False
    
    except Exception as e:
        print(f"[ERROR] Исключение при запуске Inno Setup: {e}")
        return False


def main():
    """Главная функция"""
    print_step(f"Сборка инсталлятора ApiAi v{VERSION}")
    
    # Шаг 1: Очистка и создание temp_installer
    print_step("Шаг 1: Подготовка временной директории")
    clean_temp_dir()
    
    # Шаг 2: Копирование файлов
    print_step("Шаг 2: Копирование файлов проекта")
    copy_files()
    
    # Шаг 3: Подготовка Inno Setup скрипта
    print_step("Шаг 3: Подготовка скрипта Inno Setup")
    
    iss_source = "deployment/installer.iss"
    if os.path.exists(iss_source):
        shutil.copy2(iss_source, "installer_active.iss")
        print(f"[OK] Скопирован {iss_source}")
        
        # Обновляем версию в .iss файле
        update_iss_version("installer_active.iss", VERSION)
    else:
        print(f"[ERROR] Не найден {iss_source}")
        return 1
    
    # Шаг 4: Запуск Inno Setup
    print_step("Шаг 4: Компиляция инсталлятора")
    if not run_inno_setup("installer_active.iss"):
        print("\n[FAIL] Не удалось собрать инсталлятор")
        return 1
    
    # Успех
    print_step(f"УСПЕХ! Инсталлятор готов")
    print(f"\nВерсия: ApiAi v{VERSION}")
    print(f"Файл: {OUTPUT_FILE}")
    print("\nВы можете распространять этот файл для установки на других компьютерах.")
    
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("\n\n[ОТМЕНЕНО] Сборка прервана пользователем")
        sys.exit(1)
    except Exception as e:
        print(f"\n[ERROR] Неожиданная ошибка: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)
