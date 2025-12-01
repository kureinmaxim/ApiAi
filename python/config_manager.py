"""
Модуль для управления конфигурационными файлами
"""
import os
import shutil
import json


class ConfigManager:
    """Класс для управления конфигурацией"""
    
    def __init__(self, config_name="config_qt.json"):
        self.config_name = config_name
        # Base dir is project root (parent of python/)
        self.base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
        self.config_path = os.path.join(self.base_dir, config_name)
        self.config = {}
        self.load_config()
        
    def load_config(self):
        """Загружает конфигурацию из файла"""
        # Сначала убедимся, что конфиг существует (инициализируем если нет)
        initialize_config_from_template(self.config_name)
        
        try:
            if os.path.exists(self.config_path):
                with open(self.config_path, 'r', encoding='utf-8') as f:
                    self.config = json.load(f)
            else:
                print(f"⚠️ Конфиг не найден: {self.config_path}")
                self.config = {}
        except Exception as e:
            print(f"❌ Ошибка загрузки конфига: {e}")
            self.config = {}
            
    def save_config(self):
        """Сохраняет текущую конфигурацию в файл"""
        try:
            with open(self.config_path, 'w', encoding='utf-8') as f:
                json.dump(self.config, f, indent=2, ensure_ascii=False)
        except Exception as e:
            print(f"❌ Ошибка сохранения конфига: {e}")


def initialize_config_from_template(config_name="config_qt.json"):
    """
    Инициализирует конфиг из шаблона при первом запуске.
    """
    # Определяем пути
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    config_path = os.path.join(base_dir, config_name)
    
    # Если конфиг уже есть - ничего не делаем
    if os.path.exists(config_path):
        return False
    
    # Создаем базовый конфиг
    print(f"⚠️  Создаю минимальный конфиг: {config_name}")
    
    default_config = {
        "app_info": {
            "name": "ApiAi",
            "version": "1.0.0",
            "edition": "Standard",
            "description": "ApiAi Application",
            "release_date": "2025-11-30",
            "developer_en": "Kurein M.N."
        },
        "security": {
            "pin_code": "1234",
            "require_pin": True
        },
        "window": {
            "width": 900,
            "height": 700,
            "scale_factor": 1.0,
            "remember_size": True
        },
        "ui": {
            "theme": "dark",
            "font_size": 12,
            "scale_factor": 1.0,
            "view_mode": "expert",
            "log_timestamps": False,
            "auto_open_output": False,
            "auto_export_pdf": False,
            "ai_classifier_enabled": True,
            "ai_auto_classify": False
        },
        "api_keys": {
            "telegram_bot_url": "http://localhost:8000/ai_query",
            "telegram_bot_port": 8000,
            "telegram_use_encryption": True
        },
        "telegram_security": {
            "app_id": "apiai-v1",
            "enable_signature": True,
            "verify_ssl": True
        }
    }
    
    with open(config_path, 'w', encoding='utf-8') as f:
        json.dump(default_config, f, indent=2, ensure_ascii=False)
    
    print(f"✅ Создан минимальный конфиг: {config_name}")
    return True


def initialize_all_configs():
    """
    Инициализирует все конфигурационные файлы.
    """
    configs = ["config_qt.json"]
    created_count = 0
    
    for config_name in configs:
        if initialize_config_from_template(config_name):
            created_count += 1
    
    return created_count
