# -*- coding: utf-8 -*-
"""
Диалог настроек для ApiAi
"""

import os
import json
import base64
from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout, QTabWidget,
    QWidget, QGroupBox, QGridLayout, QLabel, 
    QLineEdit, QCheckBox, QSpinBox, QPushButton,
    QDialogButtonBox, QMessageBox, QApplication
)
from PySide6.QtCore import QTimer, Qt

# Импорт SecureMessenger для теста соединения
try:
    from encryption import SecureMessenger
except ImportError:
    SecureMessenger = None


class SettingsDialog(QDialog):
    """Окно настроек API ключей"""
    
    def __init__(self, parent, config: dict):
        super().__init__(parent)
        self.config = config.copy()
        self.parent_window = parent
        
        self.setWindowTitle("⚙️ Настройки API")
        self.setModal(True)
        self.resize(600, 450)
        
        self._create_ui()
        self._load_settings()
    
    def _create_ui(self):
        """Создает интерфейс"""
        layout = QVBoxLayout(self)
        
        # Вкладка API ключей (единственная)
        self.api_keys_widget = self._create_api_keys_widget()
        layout.addWidget(self.api_keys_widget)
        
        # Кнопки
        buttons = QDialogButtonBox(QDialogButtonBox.Ok | QDialogButtonBox.Cancel)
        buttons.accepted.connect(self._save_settings)
        buttons.rejected.connect(self.reject)
        layout.addWidget(buttons)

    def _create_api_keys_widget(self):
        """Создает виджет настроек API ключей"""
        widget = QWidget()
        layout = QVBoxLayout(widget)
        
        api_group = QGroupBox("Ключи доступа для облачных сервисов")
        api_layout = QGridLayout()

        # Anthropic
        anthropic_label = QLabel("Anthropic Claude API Key:")
        self.anthropic_key_input = QLineEdit()
        self.anthropic_key_input.setEchoMode(QLineEdit.Password)
        self.anthropic_key_input.setPlaceholderText("sk-ant-...")
        
        show_anthropic_btn = QCheckBox("Показать")
        show_anthropic_btn.stateChanged.connect(
            lambda state: self.anthropic_key_input.setEchoMode(
                QLineEdit.Normal if state else QLineEdit.Password
            )
        )
        
        api_layout.addWidget(anthropic_label, 0, 0)
        api_layout.addWidget(self.anthropic_key_input, 0, 1)
        api_layout.addWidget(show_anthropic_btn, 0, 2)
        
        # OpenAI
        openai_label = QLabel("OpenAI GPT API Key:")
        self.openai_key_input = QLineEdit()
        self.openai_key_input.setEchoMode(QLineEdit.Password)
        self.openai_key_input.setPlaceholderText("sk-...")

        show_openai_btn = QCheckBox("Показать")
        show_openai_btn.stateChanged.connect(
            lambda state: self.openai_key_input.setEchoMode(
                QLineEdit.Normal if state else QLineEdit.Password
            )
        )
        
        api_layout.addWidget(openai_label, 1, 0)
        api_layout.addWidget(self.openai_key_input, 1, 1)
        api_layout.addWidget(show_openai_btn, 1, 2)
        
        api_group.setLayout(api_layout)
        layout.addWidget(api_group)

        # AI Server API
        telegram_group = QGroupBox("Настройки AI Server API")
        telegram_layout = QGridLayout()

        telegram_url_label = QLabel("Server API URL:")
        self.telegram_url_input = QLineEdit()
        self.telegram_url_input.setPlaceholderText("http://localhost:8000/ai_query")
        self.telegram_url_input.textChanged.connect(self._on_telegram_url_changed)
        
        telegram_port_label = QLabel("Порт:")
        self.telegram_port_input = QSpinBox()
        self.telegram_port_input.setRange(1, 65535)
        self.telegram_port_input.setValue(8000)
        self.telegram_port_input.valueChanged.connect(self._on_telegram_port_changed)
        
        # Checkbox for encryption
        self.use_encryption_cb = QCheckBox("Шифрование")
        self.use_encryption_cb.setToolTip("Если выключено, используется обычный HTTP без шифрования (небезопасно)")
        self.use_encryption_cb.setChecked(True)
        self.use_encryption_cb.toggled.connect(self._on_encryption_toggled)
        
        telegram_key_label = QLabel("Server API Key:")
        self.telegram_key_input = QLineEdit()
        self.telegram_key_input.setEchoMode(QLineEdit.Password)
        self.telegram_key_input.setPlaceholderText("secret_key")
        
        show_telegram_btn = QCheckBox("Показать")
        show_telegram_btn.stateChanged.connect(
            lambda state: self.telegram_key_input.setEchoMode(
                QLineEdit.Normal if state else QLineEdit.Password
            )
        )

        self.telegram_enc_label = QLabel("Encryption Key:")
        self.telegram_enc_input = QLineEdit()
        self.telegram_enc_input.setEchoMode(QLineEdit.Password)
        self.telegram_enc_input.setPlaceholderText("32-byte hex key")
        
        show_enc_btn = QCheckBox("Показать")
        show_enc_btn.stateChanged.connect(
            lambda state: self.telegram_enc_input.setEchoMode(
                QLineEdit.Normal if state else QLineEdit.Password
            )
        )

        telegram_layout.addWidget(telegram_url_label, 0, 0)
        telegram_layout.addWidget(self.telegram_url_input, 0, 1)
        telegram_layout.addWidget(telegram_port_label, 0, 2)
        telegram_layout.addWidget(self.telegram_port_input, 0, 3)
        telegram_layout.addWidget(self.use_encryption_cb, 0, 4)
        
        telegram_layout.addWidget(telegram_key_label, 1, 0)
        telegram_layout.addWidget(self.telegram_key_input, 1, 1, 1, 3)
        telegram_layout.addWidget(show_telegram_btn, 1, 4)

        telegram_layout.addWidget(self.telegram_enc_label, 2, 0)
        telegram_layout.addWidget(self.telegram_enc_input, 2, 1, 1, 3)
        telegram_layout.addWidget(show_enc_btn, 2, 4)
        
        # Test Connection Button
        test_conn_btn = QPushButton("🔄 Проверить соединение")
        test_conn_btn.setToolTip("Отправить тестовый запрос для проверки связи и шифрования")
        test_conn_btn.clicked.connect(self._test_connection)
        telegram_layout.addWidget(test_conn_btn, 3, 0, 1, 5)

        telegram_group.setLayout(telegram_layout)
        layout.addWidget(telegram_group)
        
        # Помощь
        help_label = QLabel(
            "💡 <b>Как получить API ключи:</b><br>"
            "• <b>Anthropic:</b> <a href='https://console.anthropic.com/'>console.anthropic.com</a><br>"
            "• <b>OpenAI:</b> <a href='https://platform.openai.com/api-keys'>platform.openai.com/api-keys</a><br>"
            "• <b>AI Server:</b> Обратитесь к администратору сервера"
        )
        help_label.setOpenExternalLinks(True)
        help_label.setWordWrap(True)
        layout.addWidget(help_label)
        
        layout.addStretch()
        return widget

    def _on_telegram_url_changed(self, text: str):
        """Обработчик изменения URL: обновляет поле порта"""
        from urllib.parse import urlparse
        
        self.telegram_port_input.blockSignals(True)
        try:
            if not text.startswith(('http://', 'https://')):
                pass
            else:
                parsed = urlparse(text)
                if parsed.port:
                    self.telegram_port_input.setValue(parsed.port)
                else:
                    if parsed.scheme == 'https':
                        self.telegram_port_input.setValue(443)
                    elif parsed.scheme == 'http':
                        self.telegram_port_input.setValue(80)
        except Exception:
            pass
        finally:
            self.telegram_port_input.blockSignals(False)

    def _on_telegram_port_changed(self, port: int):
        """Обработчик изменения порта: обновляет URL"""
        text = self.telegram_url_input.text()
        if not text:
            return
            
        self.telegram_url_input.blockSignals(True)
        try:
            from urllib.parse import urlparse, urlunparse
            parsed = urlparse(text)
            netloc_parts = parsed.netloc.split(':')
            host = netloc_parts[0]
            new_netloc = f"{host}:{port}"
            
            new_parsed = parsed._replace(netloc=new_netloc)
            new_url = urlunparse(new_parsed)
            
            self.telegram_url_input.setText(new_url)
        except Exception as e:
            print(f"Error updating URL port: {e}")
        finally:
            self.telegram_url_input.blockSignals(False)

    def _on_encryption_toggled(self, checked):
        """Обработчик переключения шифрования"""
        self.telegram_enc_input.setEnabled(checked)
        self.telegram_enc_label.setEnabled(checked)
    
    def _test_connection(self):
        """Проверяет соединение с TelegramHelper API"""
        url = self.telegram_url_input.text().strip()
        api_key = self.telegram_key_input.text().strip()
        enc_key = self.telegram_enc_input.text().strip()
        use_encryption = self.use_encryption_cb.isChecked()
        
        if not url:
            QMessageBox.warning(self, "Ошибка", "URL не может быть пустым")
            return
            
        sender = self.sender()
        original_text = sender.text()
        sender.setText("⏳ Проверка...")
        sender.setEnabled(False)
        QApplication.processEvents()
        
        try:
            import requests
            
            test_payload = {
                "prompt": "Test connection",
                "provider": "anthropic",
                "max_tokens": 10
            }
            
            headers = {
                "Content-Type": "application/json",
                "X-API-KEY": api_key,
                "X-APP-ID": "bomcategorizer-v5"
            }
            
            if use_encryption:
                if not enc_key:
                    raise ValueError("Ключ шифрования обязателен при включенном шифровании")
                
                if not SecureMessenger:
                    raise ImportError("Модуль шифрования недоступен")
                    
                messenger = SecureMessenger(enc_key)
                encrypted_bytes = messenger.encrypt(test_payload)
                b64_data = base64.b64encode(encrypted_bytes).decode('utf-8')
                json_data = {"data": b64_data}
                
                # Adjust URL for secure endpoint if needed
                if not url.endswith('/secure') and 'ai_query' in url:
                     # Simple heuristic, might need refinement based on exact server logic
                     pass
            else:
                json_data = test_payload
            
            # Note: In real usage, we should probably use the same logic as AIPDFSearcher regarding URL modification for secure endpoint
            # For simplicity here, we assume user enters correct URL or we rely on server handling
            # But let's try to be smart like AIPDFSearcher
            req_url = url
            if use_encryption:
                 if req_url.endswith('/ai_query'):
                    req_url = req_url.replace('/ai_query', '/ai_query/secure')
                 elif not req_url.endswith('/secure') and '/ai_query' in req_url:
                    # If it has ai_query but not at end?
                    pass 
                 elif not req_url.endswith('/secure'):
                     # Try appending if it looks like base url
                     if not req_url.endswith('/'):
                         req_url += '/'
                     req_url += 'ai_query/secure'

            try:
                response = requests.post(req_url, json=json_data, headers=headers, timeout=10)
                response.raise_for_status()
                result = response.json()
                
                success_msg = "✅ Соединение успешно установлено!\n\n"
                
                if use_encryption:
                    if result.get("mode") != "encrypted":
                        success_msg += "⚠️ Внимание: Сервер ответил без шифрования!\n"
                    
                    if "data" in result:
                        try:
                            encrypted_response = base64.b64decode(result["data"])
                            decrypted = messenger.decrypt(encrypted_response)
                            decrypted_json = json.loads(decrypted.decode('utf-8'))
                            success_msg += "🔐 Шифрование работает корректно.\n"
                        except Exception as e:
                            success_msg += f"❌ Ошибка расшифровки ответа: {e}"
                    else:
                        success_msg += "❌ Ответ сервера не содержит зашифрованных данных"
                else:
                    success_msg += "📡 Обычное соединение работает."
                
                QMessageBox.information(self, "Успех", success_msg)
                
            except Exception as e:
                QMessageBox.critical(self, "Ошибка запроса", f"Ошибка при отправке:\n{e}")
                
        except Exception as e:
            QMessageBox.critical(self, "Ошибка", f"Не удалось выполнить проверку:\n{e}")
        finally:
            sender.setText(original_text)
            sender.setEnabled(True)
    
    def _load_settings(self):
        """Загружает настройки из config"""
        api_keys = self.config.get("api_keys", {})
        
        self.anthropic_key_input.setText(api_keys.get("anthropic", ""))
        self.openai_key_input.setText(api_keys.get("openai", ""))
        
        self.telegram_url_input.setText(api_keys.get("telegram_url", ""))
        self.telegram_key_input.setText(api_keys.get("telegram_key", ""))
        self.telegram_enc_input.setText(api_keys.get("telegram_enc_key", ""))
        
        use_encryption = api_keys.get("telegram_use_encryption", True)
        self.use_encryption_cb.setChecked(use_encryption)
        self._on_encryption_toggled(use_encryption)
        
        # Init port
        self._on_telegram_url_changed(self.telegram_url_input.text())

    def _save_settings(self):
        """Сохраняет настройки"""
        if "api_keys" not in self.config:
            self.config["api_keys"] = {}
            
        self.config["api_keys"]["anthropic"] = self.anthropic_key_input.text().strip()
        self.config["api_keys"]["openai"] = self.openai_key_input.text().strip()
        
        self.config["api_keys"]["telegram_url"] = self.telegram_url_input.text().strip()
        self.config["api_keys"]["telegram_key"] = self.telegram_key_input.text().strip()
        self.config["api_keys"]["telegram_enc_key"] = self.telegram_enc_input.text().strip()
        self.config["api_keys"]["telegram_use_encryption"] = self.use_encryption_cb.isChecked()
        
        # Сохраняем в файл
        try:
            # config_qt.json в корне проекта
            project_root = os.path.dirname(os.path.dirname(os.path.dirname(__file__)))
            config_path = os.path.join(project_root, "config_qt.json")
            
            # Если мы не можем найти корень через __file__, попробуем через cwd или аргумент
            # Но здесь мы в gui/settings_dialog.py, так что 3 уровня вверх ок.
            # /Users/olgazaharova/Project/ApiAi/gui/settings_dialog.py -> .../ApiAi
            
            with open(config_path, 'w', encoding='utf-8') as f:
                json.dump(self.config, f, indent=2, ensure_ascii=False)
            
            self.accept()
        except Exception as e:
            QMessageBox.warning(self, "Ошибка", f"Не удалось сохранить настройки: {e}")

    def get_config(self) -> dict:
        """Возвращает обновленный конфиг"""
        return self.config
