# -*- coding: utf-8 -*-
"""
Секции интерфейса для ApiAi
"""

from PySide6.QtWidgets import (
    QWidget, QHBoxLayout, QLabel, QFrame
)
from PySide6.QtCore import Qt, QTimer
from PySide6.QtGui import QCursor


def create_footer(parent_window):
    """
    Создает подвал (status bar)
    """
    footer_widget = QWidget()
    footer_layout = QHBoxLayout(footer_widget)
    footer_layout.setContentsMargins(10, 2, 10, 2)
    
    # Разработчик (с секретной кнопкой для PIN)
    dev_label = QLabel(f"Developer: {parent_window.cfg['app_info']['developer_en']}")
    dev_label.setStyleSheet("color: #6c7086; font-size: 11pt;")
    dev_label.setCursor(QCursor(Qt.PointingHandCursor))
    # Обработка двойного клика для PIN
    dev_label.mouseDoubleClickEvent = lambda e: parent_window.on_developer_double_click()
    dev_label.setToolTip("Double click to unlock expert mode")
    footer_layout.addWidget(dev_label)
    
    footer_layout.addStretch()
    
    # Дата релиза
    date_label = QLabel(f"Release: {parent_window.cfg['app_info']['release_date']}")
    date_label.setStyleSheet("color: #6c7086; font-size: 11pt;")
    footer_layout.addWidget(date_label)
    
    # Разделитель
    sep = QFrame()
    sep.setFrameShape(QFrame.VLine)
    sep.setFrameShadow(QFrame.Sunken)
    sep.setStyleSheet("color: #45475a;")
    footer_layout.addWidget(sep)
    
    # Размер окна
    parent_window.size_label = QLabel(f"{parent_window.width()}x{parent_window.height()}")
    parent_window.size_label.setStyleSheet("color: #6c7086; font-size: 11pt;")
    footer_layout.addWidget(parent_window.size_label)
    
    return footer_widget
