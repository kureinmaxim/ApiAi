#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
Скрипт для скачивания шрифтов DejaVu Sans

Скачивает TrueType шрифты с поддержкой кириллицы для будущего использования в PDF.
"""

import os
import sys
import urllib.request
import zipfile
from pathlib import Path


def download_fonts():
    """Скачивает и распаковывает шрифты DejaVu Sans"""
    
    # URL с GitHub релизами DejaVu Fonts
    DEJAVU_VERSION = "2.37"
    DEJAVU_URL = f"https://github.com/dejavu-fonts/dejavu-fonts/releases/download/version_{DEJAVU_VERSION}/dejavu-fonts-ttf-{DEJAVU_VERSION}.zip"
    
    # Пути
    project_root = Path(__file__).parent.parent
    fonts_dir = project_root / "fonts"
    temp_zip = fonts_dir / "dejavu.zip"
    
    # Создаем папку fonts если её нет
    fonts_dir.mkdir(exist_ok=True)
    
    print(f"📦 Скачивание DejaVu Fonts {DEJAVU_VERSION}...")
    print(f"   URL: {DEJAVU_URL}")
    
    try:
        # Скачиваем
        urllib.request.urlretrieve(DEJAVU_URL, temp_zip)
        print(f"✅ Скачано: {temp_zip}")
        
        # Распаковываем
        print("📂 Распаковка...")
        with zipfile.ZipFile(temp_zip, 'r') as zip_ref:
            # Ищем нужные файлы в архиве
            for file in zip_ref.namelist():
                if file.endswith('DejaVuSans.ttf') or file.endswith('DejaVuSans-Bold.ttf'):
                    # Извлекаем только имя файла (без пути в архиве)
                    filename = os.path.basename(file)
                    target_path = fonts_dir / filename
                    
                    # Читаем из архива и пишем в нужное место
                    with zip_ref.open(file) as source, open(target_path, 'wb') as target:
                        target.write(source.read())
                    
                    print(f"✅ Извлечен: {filename}")
        
        # Удаляем временный zip
        temp_zip.unlink()
        print(f"🗑️  Удален временный файл")
        
        # Проверка
        regular_font = fonts_dir / "DejaVuSans.ttf"
        bold_font = fonts_dir / "DejaVuSans-Bold.ttf"
        
        if regular_font.exists() and bold_font.exists():
            print(f"\n✅ УСПЕХ! Шрифты установлены:")
            print(f"   - {regular_font.name} ({regular_font.stat().st_size // 1024} KB)")
            print(f"   - {bold_font.name} ({bold_font.stat().st_size // 1024} KB)")
            return True
        else:
            print(f"\n❌ ОШИБКА: Не все файлы найдены")
            return False
            
    except Exception as e:
        print(f"\n❌ ОШИБКА: {e}")
        if temp_zip.exists():
            temp_zip.unlink()
        return False


if __name__ == "__main__":
    print("=" * 60)
    print("🔤 Скачивание шрифтов DejaVu Sans для ApiAi")
    print("=" * 60)
    print()
    
    success = download_fonts()
    
    print()
    print("=" * 60)
    
    sys.exit(0 if success else 1)
