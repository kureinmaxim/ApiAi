# -*- coding: utf-8 -*-
"""
Главное окно приложения ApiAi
"""

import os
import sys
from PySide6.QtWidgets import (
    QMainWindow, QWidget, QVBoxLayout, QMessageBox, 
    QApplication, QMenu
)
from PySide6.QtCore import Qt, QTimer
from PySide6.QtGui import QAction, QIcon, QFont

from config_manager import ConfigManager
from styles import apply_theme
from .sections import create_footer
from .dialogs import PinDialog
from .search_widget import SearchWidget
from .settings_dialog import SettingsDialog


class ApiAiMainWindow(QMainWindow):
    """Главное окно приложения ApiAi"""
    
    def __init__(self):
        super().__init__()
        
        # Загрузка конфигурации
        self.config_manager = ConfigManager()
        self.cfg = self.config_manager.config
        
        # Состояние
        self.scale_factor = self.cfg.get("window", {}).get("scale_factor", 1.0)
        self.expert_mode = False
        self.unlocked = False
        self.require_pin = self.cfg.get("security", {}).get("require_pin", True)
        self.correct_pin = self.cfg.get("security", {}).get("pin_code", "1234")
        
        # Настройка окна
        self.setWindowTitle(f"{self.cfg['app_info']['name']} v{self.cfg['app_info']['version']}")
        self.resize(
            self.cfg.get("window", {}).get("width", 1200),
            self.cfg.get("window", {}).get("height", 800)
        )
        
        # Применение темы
        apply_theme(self, self.cfg.get("ui", {}).get("theme", "dark"))
        
        # Создание UI
        self._create_ui()
        
        # Таймер для обновления времени в футере (если нужно, но пока просто статика или обновление при действиях)
        # В оригинале create_footer создает статические лейблы, кроме размера.
        
    def _create_ui(self):
        """Создает интерфейс главного окна"""
        # Центральный виджет
        central_widget = QWidget()
        self.setCentralWidget(central_widget)
        
        main_layout = QVBoxLayout(central_widget)
        main_layout.setContentsMargins(0, 0, 0, 0)
        main_layout.setSpacing(0)
        
        # Меню
        self._create_menu()
        
        # Основной контент (SearchWidget)
        self.search_widget = SearchWidget(self, self.cfg)
        main_layout.addWidget(self.search_widget)
        
        # Подвал
        self.footer = create_footer(self)
        main_layout.addWidget(self.footer)
        
    def _create_menu(self):
        """Создает меню приложения"""
        menubar = self.menuBar()
        
        # View Menu
        view_menu = menubar.addMenu("View")
        
        # Scaling
        scale_menu = QMenu("Scaling", self)
        view_menu.addMenu(scale_menu)
        
        for scale in [0.8, 0.9, 1.0, 1.1, 1.2, 1.5]:
            action = QAction(f"{int(scale*100)}%", self)
            action.setCheckable(True)
            action.setChecked(abs(self.scale_factor - scale) < 0.01)
            action.triggered.connect(lambda checked, s=scale: self.set_scale(s))
            scale_menu.addAction(action)
            
        # PDF Search Menu (как в требованиях)
        search_menu = menubar.addMenu("PDF Search")
        
        search_action = QAction("AI Search", self)
        search_action.triggered.connect(lambda: self.search_widget.search_input.setFocus())
        search_menu.addAction(search_action)
        
        settings_action = QAction("Settings (API Keys)", self)
        settings_action.triggered.connect(self.open_settings)
        search_menu.addAction(settings_action)
        
        # Help Menu
        help_menu = menubar.addMenu("Help")
        
        about_action = QAction("About", self)
        about_action.triggered.connect(self.show_about)
        help_menu.addAction(about_action)

    def set_scale(self, scale):
        """Устанавливает масштаб интерфейса"""
        self.scale_factor = scale
        self.cfg["window"]["scale_factor"] = scale
        self.config_manager.save_config()
        
        # Применяем масштаб (простой способ - через шрифт и ресайз, но полный рескейл сложен без перезагрузки)
        # Для простоты пока просто сохраняем и просим перезапустить, или меняем шрифт глобально
        QMessageBox.information(self, "Масштаб", "Изменения масштаба вступят в силу после перезапуска приложения.")

    def open_settings(self):
        """Открывает настройки"""
        # Проверка PIN если заблокировано (или требование экспертного режима)
        # В оригинале настройки доступны только в экспертном режиме
        if not self.unlocked and self.require_pin:
             QMessageBox.information(self, "Доступ запрещен", "Для доступа к настройкам разблокируйте режим эксперта (двойной клик по разработчику внизу).")
             return

        dialog = SettingsDialog(self, self.cfg)
        if dialog.exec():
            self.cfg = dialog.get_config()
            self.config_manager.config = self.cfg
            # Обновляем конфиг в виджете поиска
            self.search_widget.config = self.cfg

    def on_developer_double_click(self):
        """Обработчик двойного клика по разработчику (PIN)"""
        if not self.unlocked and self.require_pin:
            dialog = PinDialog(self.correct_pin, self)
            if dialog.exec():
                self.unlocked = True
                self.expert_mode = True
                QMessageBox.information(self, "Успех", "Режим эксперта активирован!\nТеперь доступны настройки API.")
        else:
            QMessageBox.information(self, "Info", "Режим эксперта уже активен.")

    def show_about(self):
        """Показывает информацию о программе"""
        QMessageBox.about(
            self,
            "About ApiAi",
            f"<b>{self.cfg['app_info']['name']}</b><br>"
            f"Version: {self.cfg['app_info']['version']}<br>"
            f"Developer: {self.cfg['app_info']['developer_en']}<br><br>"
            "Application for AI-powered component search."
        )

    def resizeEvent(self, event):
        """Обновление размера в футере"""
        if hasattr(self, 'size_label'):
            self.size_label.setText(f"{self.width()}x{self.height()}")
        super().resizeEvent(event)

def main():
    """Запуск приложения"""
    app = QApplication(sys.argv)
    window = ApiAiMainWindow()
    window.show()
    sys.exit(app.exec())
