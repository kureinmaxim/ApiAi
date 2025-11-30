# -*- coding: utf-8 -*-
"""
Диалоговые окна для ApiAi
"""

from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout,
    QPushButton, QLabel, QLineEdit
)
from PySide6.QtCore import Qt
from PySide6.QtGui import QFont


class PinDialog(QDialog):
    """Диалог ввода PIN-кода"""

    def __init__(self, correct_pin: str, parent=None):
        super().__init__(parent)
        self.correct_pin = correct_pin
        self.is_authenticated = False

        self.setWindowTitle("Авторизация")
        self.setFixedSize(380, 220)
        self.setModal(True)
        
        # Получаем scale_factor от родительского окна
        self.scale_factor = getattr(parent, 'scale_factor', 1.0) if parent else 1.0

        self._create_ui()

        # Центрируем окно
        if parent:
            parent_geo = parent.geometry()
            x = parent_geo.x() + (parent_geo.width() - self.width()) // 2
            y = parent_geo.y() + (parent_geo.height() - self.height()) // 2
            self.move(x, y)

    def _create_ui(self):
        """Создает элементы диалога"""
        layout = QVBoxLayout()
        layout.setContentsMargins(15, 15, 15, 15)
        layout.setSpacing(10)

        # Заголовок
        title_label = QLabel("Введите PIN-код:")
        title_font = QFont()
        title_font.setPointSize(int(14 * self.scale_factor))
        title_font.setBold(True)
        title_label.setFont(title_font)
        title_label.setAlignment(Qt.AlignCenter)
        layout.addWidget(title_label)

        # Поле ввода PIN
        self.pin_entry = QLineEdit()
        self.pin_entry.setEchoMode(QLineEdit.Password)
        self.pin_entry.setAlignment(Qt.AlignCenter)
        pin_font = QFont()
        pin_font.setPointSize(int(18 * self.scale_factor))
        self.pin_entry.setFont(pin_font)
        self.pin_entry.setMaxLength(10)
        self.pin_entry.returnPressed.connect(self.check_pin)
        layout.addWidget(self.pin_entry)

        # Метка ошибки
        self.error_label = QLabel("")
        self.error_label.setAlignment(Qt.AlignCenter)
        error_font = QFont()
        error_font.setPointSize(12)
        self.error_label.setFont(error_font)
        self.error_label.setStyleSheet("color: #DE350B;")
        layout.addWidget(self.error_label)

        # Кнопки
        buttons_layout = QHBoxLayout()

        ok_btn = QPushButton("OK")
        ok_btn.setMinimumWidth(100)
        ok_btn.clicked.connect(self.check_pin)
        ok_btn.setDefault(True)
        buttons_layout.addWidget(ok_btn)

        cancel_btn = QPushButton("Отмена")
        cancel_btn.setMinimumWidth(100)
        cancel_btn.clicked.connect(self.reject)
        buttons_layout.addWidget(cancel_btn)

        layout.addLayout(buttons_layout)

        self.setLayout(layout)

        # Фокус на поле ввода
        self.pin_entry.setFocus()

    def check_pin(self):
        """Проверяет введенный PIN"""
        entered_pin = self.pin_entry.text().strip()

        if entered_pin == self.correct_pin:
            self.is_authenticated = True
            self.accept()
        else:
            self.error_label.setText("Неверный PIN-код")
            self.pin_entry.clear()
            self.pin_entry.setFocus()

    def keyPressEvent(self, event):
        """Обработка нажатий клавиш"""
        if event.key() == Qt.Key_Escape:
            self.reject()
        else:
            super().keyPressEvent(event)
