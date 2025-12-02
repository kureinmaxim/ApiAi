// ApiAi - Rust implementation
// Main entry point with egui GUI

mod api;
mod encryption;

use eframe::egui;
use egui_extras::install_image_loaders;

// Default window size (placeholders for future use)
const _WINDOW_WIDTH: f32 = 800.0;
const _WINDOW_HEIGHT: f32 = 600.0;

const _COMPONENT_ANALYSIS_TEMPLATE: &str = r#"Роль: Инженер-разработчик радиоэлектронной аппаратуры с опытом во всех направлениях: силовая электроника, аналоговая и цифровая схемотехника, СВЧ-техника, датчики и измерения.

Задача: Составь краткое техническое описание компонента: {component_name}

ОБЯЗАТЕЛЬНО:
1. Выполни поиск на сайте производителя
2. Открой datasheet (и application note если есть)
3. Определи тип компонента и подбери релевантные параметры

СТРУКТУРА ОПИСАНИЯ:

【Идентификация】
- Полное название с суффиксами маркировки
- Производитель (оригинальный)
- Статус: Active / NRND / Obsolete
- Категория компонента (ИС, транзистор, модуль, пассивный, датчик, СВЧ и т.д.)

【Назначение】
- Функция компонента (1-2 предложения)
- Типовые области применения

【Ключевые параметры】
Выбери параметры в зависимости от типа компонента:

▸ DC-DC / LDO / ИВП:
  Vin, Vout, Iout, КПД, Fsw, Iq, защиты (OVP/OCP/OTP), корпус

▸ Операционные усилители:
  Vcc, GBW, Slew Rate, Vos, Ib, шум, число каналов, Rail-to-Rail, корпус

▸ АЦП/ЦАП:
  Разрядность, скорость (SPS/MSPS), интерфейс, Vref, INL/DNL, корпус

▸ Микроконтроллеры / ПЛИС:
  Ядро, Flash/RAM, частота, периферия, интерфейсы, корпус

▸ Транзисторы (MOSFET/BJT/IGBT):
  Тип (N/P), Vds/Vce, Id/Ic, Rds(on)/hFE, Qg, корпус

▸ СВЧ-компоненты (усилители, смесители, генераторы):
  Диапазон частот, Gain, P1dB, NF, OIP3, Vcc/Icc, корпус

▸ Датчики:
  Измеряемая величина, диапазон, точность, интерфейс, Vcc, корпус

▸ Пассивные (если требуется описание):
  Номинал, допуск, напряжение/мощность, температурный коэфф., корпус

【Особенности и преимущества】
- 2-3 ключевых отличия от конкурентов (если известны)

【Аналоги】
- Pin-to-pin совместимые (других производителей)
- Отечественные аналоги: конкретные парт-номера или указание, какие параметры не покрываются
- Вывод: оригинал предпочтителен / есть адекватная замена

【Источники】
- Прямая ссылка на datasheet (PDF)
- Ссылка на страницу продукта

Формат: структурированный текст, 150-300 слов (в зависимости от сложности)
Язык: русский, технические термины допустимы на английском"#;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0])
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "ApiAi",
        options,
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            
            Ok(Box::new(ApiAiApp::new(cc)))
        }),
    )
}

// App state
pub struct ApiAiApp {
    search_query: String,
    prompt_mode: PromptMode,
    selected_provider: Provider,
    dark_mode: bool,
    config: AppConfig,
    show_settings: bool,
    show_about: bool,
    
    // Security State
    unlocked: bool,
    show_pin_dialog: bool,
    pin_input: String,
    pin_error: String,

    // Settings dialog temporary state
    settings_anthropic_key: String,
    settings_openai_key: String,
    settings_telegram_url: String,
    settings_telegram_port: u16,
    settings_telegram_key: String,
    settings_telegram_enc_key: String,
    settings_use_encryption: bool,
    show_anthropic_password: bool,
    show_openai_password: bool,
    show_telegram_password: bool,
    show_enc_password: bool,
    
    // Response state
    response_text: String,
    is_loading: bool,
    
    // Conversation history
    conversation_history: Vec<(String, String)>, // (user_message, ai_response)
    
    // Chat mode
    chat_mode_enabled: bool,
    conversation_id: Option<String>,
    
    // Async runtime and channel
    runtime: tokio::runtime::Runtime,
    response_rx: std::sync::mpsc::Receiver<Result<(String, String, Option<String>), String>>,
    response_tx: std::sync::mpsc::Sender<Result<(String, String, Option<String>), String>>,
}

#[derive(Default, PartialEq)]
enum PromptMode {
    #[default]
    GeneralChat,
    ComponentAnalysis,
}

#[derive(Default, PartialEq)]
enum Provider {
    Anthropic,
    OpenAI,
    #[default]
    Telegram,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct SecurityConfig {
    pin_code: String,
    require_pin: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct TelegramSecurityConfig {
    app_id: String,
    enable_signature: bool,
    verify_ssl: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AppConfig {
    app_info: AppInfo,
    api_keys: ApiKeys,
    ui: UiConfig,
    security: SecurityConfig,
    telegram_security: TelegramSecurityConfig,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AppInfo {
    name: String,
    version: String,
    developer_en: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct ApiKeys {
    anthropic: String,
    openai: String,
    telegram_url: String,
    telegram_key: String,
    telegram_enc_key: String,
    telegram_use_encryption: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct UiConfig {
    theme: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            app_info: AppInfo {
                name: "ApiAi".to_string(),
                version: "2.1.1".to_string(),
                developer_en: "Maksim Kurein".to_string(),
            },
            api_keys: ApiKeys {
                anthropic: String::new(),
                openai: String::new(),
                telegram_url: "http://localhost:8000/ai_query".to_string(),
                telegram_key: String::new(),
                telegram_enc_key: String::new(),
                telegram_use_encryption: true,
            },
            ui: UiConfig {
                theme: "dark".to_string(),
            },
            security: SecurityConfig {
                pin_code: "1234".to_string(),
                require_pin: true,
            },
            telegram_security: TelegramSecurityConfig {
                app_id: "apiai-v1".to_string(),
                enable_signature: true,
                verify_ssl: true,
            },
        }
    }
}

impl AppConfig {
    fn config_path() -> std::path::PathBuf {
        // Try to find config in project root first
        let current_dir = std::env::current_dir().unwrap_or_default();
    let project_config = current_dir.join("config_qt.json");
    
    if project_config.exists() {
        return project_config;
    }
    
    // Check parent directory (project root if running from rust/)
    if let Some(parent) = current_dir.parent() {
        let parent_config = parent.join("config_qt.json");
        if parent_config.exists() {
            return parent_config;
        }
    }    
        // Otherwise use user's home directory
        if let Some(home) = dirs::home_dir() {
            home.join(".config").join("apiai").join("config_qt.json")
        } else {
            std::path::PathBuf::from("config_qt.json")
        }
    }
    
    fn load() -> Self {
        let path = Self::config_path();
        
        if let Ok(contents) = std::fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }
        
        Self::default()
    }
    
    fn save(&self) -> anyhow::Result<()> {
        let path = Self::config_path();
        
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let contents = serde_json::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}

impl Default for ApiAiApp {
    fn default() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let runtime = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
        
        Self {
            config: AppConfig::default(),
            dark_mode: true,
            search_query: String::new(),
            prompt_mode: PromptMode::default(),
            selected_provider: Provider::default(),
            show_settings: false,
            show_about: false,
            unlocked: false,
            show_pin_dialog: false,
            pin_input: String::new(),
            pin_error: String::new(),
            settings_anthropic_key: String::new(),
            settings_openai_key: String::new(),
            settings_telegram_url: String::new(),
            settings_telegram_port: 8000,
            settings_telegram_key: String::new(),
            settings_telegram_enc_key: String::new(),
            settings_use_encryption: true,
            show_anthropic_password: false,
            show_openai_password: false,
            show_telegram_password: false,
            show_enc_password: false,
            response_text: String::new(),
            is_loading: false,
            conversation_history: Vec::new(),
            chat_mode_enabled: true, // Enable chat mode by default
            conversation_id: None,
            runtime,
            response_rx: rx,
            response_tx: tx,
        }
    }
}

impl ApiAiApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load configuration
        let config = AppConfig::load();
        let dark_mode = config.ui.theme == "dark";
        
        let app = Self {
            config: config.clone(),
            dark_mode,
            // If PIN is not required, start unlocked
            unlocked: !config.security.require_pin,
            ..Default::default()
        };
        
        app.configure_theme(&cc.egui_ctx);
        app
    }
    
    fn open_settings(&mut self) {
        // Load current settings into dialog fields
        self.settings_anthropic_key = self.config.api_keys.anthropic.clone();
        self.settings_openai_key = self.config.api_keys.openai.clone();
        self.settings_telegram_url = self.config.api_keys.telegram_url.clone();
        self.settings_telegram_key = self.config.api_keys.telegram_key.clone();
        self.settings_telegram_enc_key = self.config.api_keys.telegram_enc_key.clone();
        self.settings_use_encryption = self.config.api_keys.telegram_use_encryption;
        self.show_settings = true;
    }
    
    fn save_settings(&mut self) {
        // Save from dialog fields to config
        self.config.api_keys.anthropic = self.settings_anthropic_key.clone();
        self.config.api_keys.openai = self.settings_openai_key.clone();
        
        // Handle telegram URL - extract base URL and apply port if needed
        let telegram_url = if self.settings_telegram_url.contains("://") {
            // If URL contains protocol, use it as-is (user manually set full URL)
            self.settings_telegram_url.clone()
        } else {
            // Otherwise construct URL with port
            format!("http://{}:{}/ai_query", self.settings_telegram_url, self.settings_telegram_port)
        };
        
        self.config.api_keys.telegram_url = telegram_url;
        self.config.api_keys.telegram_key = self.settings_telegram_key.clone();
        self.config.api_keys.telegram_enc_key = self.settings_telegram_enc_key.clone();
        self.config.api_keys.telegram_use_encryption = self.settings_use_encryption;
        
        // Save to file
        if let Err(e) = self.config.save() {
            eprintln!("Failed to save config: {}", e);
        }
    }
    
    fn toggle_theme(&mut self) {
        self.dark_mode = !self.dark_mode;
        self.config.ui.theme = if self.dark_mode { "dark" } else { "light" }.to_string();
        
        // Save to file
        if let Err(e) = self.config.save() {
            eprintln!("Failed to save theme preference: {}", e);
        }
    }
    
    
    fn perform_search(&mut self) {
        if self.search_query.is_empty() {
            return;
        }
        
        // Set loading state
        self.is_loading = true;
        
        // Get query and save to history
        let query = self.search_query.clone();
        self.search_query.clear();
        
        // Get API credentials
        let api_key = match self.selected_provider {
            Provider::Anthropic => self.config.api_keys.anthropic.clone(),
            Provider::OpenAI => self.config.api_keys.openai.clone(),
            Provider::Telegram => self.config.api_keys.telegram_key.clone(),
        };
        
        let telegram_url = self.config.api_keys.telegram_url.clone();
        let encryption_key = if !self.config.api_keys.telegram_enc_key.is_empty() {
            Some(self.config.api_keys.telegram_enc_key.clone())
        } else {
            None
        };
        let use_encryption = self.config.api_keys.telegram_use_encryption;
        let chat_mode = self.chat_mode_enabled;
        let conversation_id = self.conversation_id.clone();
        
        // Create the appropriate client
        let client: Box<dyn api::ApiClient> = match self.selected_provider {
            Provider::Anthropic => Box::new(api::AnthropicClient::new(api_key)),
            Provider::OpenAI => Box::new(api::OpenAIClient::new(api_key)),
            Provider::Telegram => Box::new(api::TelegramClient::new(telegram_url, api_key, encryption_key, use_encryption, chat_mode, conversation_id)),
        };
        
        // Spawn async task
        let tx = self.response_tx.clone();
        let user_query = query.clone();
        self.runtime.spawn(async move {
            let result = client.search(&query).await;
            
            let message = match result {
                Ok(search_result) => {
                    let conv_id = search_result.conversation_id.clone();
                    Ok((user_query, search_result.text, conv_id))
                }
                Err(e) => Err(format!("Error: {}", e)),
            };
            
            let _ = tx.send(message);
        });
    }

    
    fn configure_theme(&self, ctx: &egui::Context) {
        let visuals = if self.dark_mode {
            let mut v = egui::Visuals::dark();
            
            // Catppuccin Macchiato (Dark Theme from Python)
            let base = egui::Color32::from_rgb(30, 30, 46);          // #1e1e2e (Base)
            let surface1 = egui::Color32::from_rgb(49, 50, 68);      // #313244 (Surface1 - for inputs)
            let surface2 = egui::Color32::from_rgb(69, 71, 90);      // #45475a (Surface2)
            let text = egui::Color32::from_rgb(224, 229, 255);       // #e0e5ff (Text)
            let blue = egui::Color32::from_rgb(90, 143, 234);        // #5a8fea (Darker Blue)
            let lavender = egui::Color32::from_rgb(203, 166, 247);   // #cba6f7 (Lavender)
            
            // Background colors
            v.window_fill = base;
            v.panel_fill = base;
            v.faint_bg_color = surface1; // Input fields background - not too dark
            v.extreme_bg_color = surface1; // TextEdit background - crucial for not being black
            
            // Text colors
            v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
            v.widgets.noninteractive.weak_bg_fill = surface1;
            v.widgets.noninteractive.bg_fill = base;
            
            // Interactive widgets (buttons, inputs)
            v.widgets.inactive.bg_fill = surface1;
            v.widgets.inactive.weak_bg_fill = surface1;
            v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text);
            v.widgets.inactive.rounding = egui::Rounding::same(4.0);
            
            // Hovered state
            v.widgets.hovered.bg_fill = surface2;
            v.widgets.hovered.weak_bg_fill = surface2;
            v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            v.widgets.hovered.rounding = egui::Rounding::same(4.0);
            
            // Active/Selected state
            v.widgets.active.bg_fill = blue;
            v.widgets.active.weak_bg_fill = blue;
            v.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            v.widgets.active.rounding = egui::Rounding::same(4.0);
            
            // Open widgets
            v.widgets.open.bg_fill = surface2;
            v.widgets.open.weak_bg_fill = surface2;
            v.widgets.open.fg_stroke = egui::Stroke::new(1.0, text);
            v.widgets.open.rounding = egui::Rounding::same(4.0);
            
            // Selection highlight
            v.selection.bg_fill = blue; // Opaque blue for better visibility
            v.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            
            // Window styling
            v.window_rounding = egui::Rounding::same(8.0);
            v.window_shadow = egui::epaint::Shadow {
                offset: egui::vec2(0.0, 4.0),
                blur: 15.0,
                spread: 0.0,
                color: egui::Color32::from_black_alpha(120),
            };
            
            // Hyperlinks
            v.hyperlink_color = lavender;
            
            v
        } else {
            let mut v = egui::Visuals::light();
            
            // Catppuccin Latte (Light Theme from Python)
            let base = egui::Color32::from_rgb(239, 241, 245);       // #eff1f5 (Base)
            let text = egui::Color32::from_rgb(76, 79, 105);         // #4c4f69 (Text) - Darker and more readable
            let blue = egui::Color32::from_rgb(30, 102, 245);        // #1e66f5 (Blue)
            
            v.window_fill = base;
            v.panel_fill = base;
            v.faint_bg_color = egui::Color32::WHITE;   // White inputs
            v.extreme_bg_color = egui::Color32::WHITE; // White text edits
            
            v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
            v.widgets.inactive.bg_fill = egui::Color32::WHITE;
            v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text);
            
            // Hovered state - lighter blue for selectable buttons
            v.widgets.hovered.bg_fill = blue.linear_multiply(0.7); // Lighter blue for hover
            v.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            
            v.widgets.active.bg_fill = blue;
            v.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            
            v.selection.bg_fill = blue.linear_multiply(0.2);
            v.selection.stroke = egui::Stroke::new(1.0, text); // Dark text selection
            
            v.window_rounding = egui::Rounding::same(8.0);
            v.widgets.noninteractive.rounding = egui::Rounding::same(4.0);
            v.widgets.inactive.rounding = egui::Rounding::same(4.0);
            v.widgets.hovered.rounding = egui::Rounding::same(4.0);
            v.widgets.active.rounding = egui::Rounding::same(4.0);
            
            v
        };
        
        ctx.set_visuals(visuals);
    }
}

impl eframe::App for ApiAiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for responses from async tasks
        if let Ok(result) = self.response_rx.try_recv() {
            self.is_loading = false;
            match result {
                Ok((user_query, ai_response, conversation_id)) => {
                    // Save conversation_id if provided
                    if let Some(conv_id) = conversation_id {
                        self.conversation_id = Some(conv_id);
                    }
                    
                    // Remove provider information from response text if present
                    // Filter out lines containing "Provider:" or "(Secure)"
                    let cleaned_response: String = ai_response
                        .lines()
                        .filter(|line| {
                            let trimmed = line.trim();
                            !trimmed.starts_with("Provider:") && 
                            !trimmed.starts_with("provider:") &&
                            !trimmed.contains("Provider:") &&
                            !trimmed.contains("(Secure)")
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    
                    // Save to conversation history
                    self.conversation_history.push((user_query.clone(), cleaned_response.clone()));
                    
                    // Append to response text (keep conversation context)
                    if !self.response_text.is_empty() {
                        self.response_text.push_str("\n\n---\n\n");
                    }
                    self.response_text.push_str(&format!("👤 You: {}\n\n🤖 AI: {}", user_query, cleaned_response));
                }
                Err(error) => {
                    if !self.response_text.is_empty() {
                        self.response_text.push_str("\n\n---\n\n");
                    }
                    self.response_text.push_str(&format!("❌ Error: {}", error));
                }
            }
        }
        
        // Request repaint if loading (to show spinner/animation)
        if self.is_loading {
            ctx.request_repaint();
        }
        
        self.configure_theme(ctx);

        // Footer (Status Bar)
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            // Add background color for better visibility
            let footer_bg = if self.dark_mode {
                egui::Color32::from_rgb(24, 24, 37)  // Darker in dark mode
            } else {
                egui::Color32::from_rgb(220, 222, 228)  // Light gray in light mode
            };
            
            let footer_text_color = egui::Color32::from_rgb(108, 112, 134); // #6c7086
            
            ui.visuals_mut().widgets.noninteractive.bg_fill = footer_bg;
            
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let developer_text = format!("Developer: {}", self.config.app_info.developer_en);
                    let developer_label = ui.add(
                        egui::Label::new(egui::RichText::new(developer_text).color(footer_text_color).strong())
                            .sense(egui::Sense::click())
                    );
                    
                    // Double-click to unlock
                    if developer_label.double_clicked() {
                        if !self.unlocked && self.config.security.require_pin {
                            self.show_pin_dialog = true;
                            self.pin_input.clear();
                            self.pin_error.clear();
                        }
                    }
                    
                    // Tooltip
                    developer_label.on_hover_text("Double click to unlock expert mode");
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        // Get actual window size
                        let screen_rect = ctx.input(|i| i.screen_rect());
                        let size_text = format!("{}x{}", screen_rect.width() as i32, screen_rect.height() as i32);
                        ui.label(egui::RichText::new(size_text).color(footer_text_color));
                        
                        // Separator
                        ui.label(egui::RichText::new("|").color(footer_text_color));
                        
                        // Release date from config
                        ui.label(egui::RichText::new(format!("Release: {}", self.config.app_info.version)).color(footer_text_color));
                    });
                });
            });
        });

        // PIN Dialog
        if self.show_pin_dialog {
            egui::Window::new("Enter PIN")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.heading("Enter PIN code");
                    ui.add_space(10.0);
                    
                    let response = ui.add(egui::TextEdit::singleline(&mut self.pin_input).password(true).desired_width(150.0));
                    
                    if !self.pin_error.is_empty() {
                        ui.add_space(5.0);
                        ui.colored_label(egui::Color32::RED, &self.pin_error);
                    }
                    
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Unlock").clicked() || (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                            if self.pin_input == self.config.security.pin_code {
                                self.unlocked = true;
                                self.show_pin_dialog = false;
                                self.pin_input.clear();
                                self.pin_error.clear();
                            } else {
                                self.pin_error = "Incorrect PIN".to_string();
                                self.pin_input.clear();
                                response.request_focus();
                            }
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_pin_dialog = false;
                            self.pin_input.clear();
                            self.pin_error.clear();
                        }
                    });
                    
                    // Request focus on open
                    if self.show_pin_dialog {
                        response.request_focus();
                    }
                });
        }

        // Menu Bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui.button("🌓 Toggle Theme (Ctrl+T)").clicked() {
                        self.toggle_theme();
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Settings", |ui| {
                    if self.unlocked {
                        if ui.button("🔑 API Keys").clicked() {
                            self.open_settings();
                            ui.close_menu();
                        }
                    } else {
                        ui.label("🔒 API Keys (Locked)");
                        if ui.button("🔓 Unlock").clicked() {
                             self.show_pin_dialog = true;
                             self.pin_input.clear();
                             self.pin_error.clear();
                             ui.close_menu();
                        }
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("ℹ️ About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Settings Dialog
        if self.show_settings {
            // Define label color for settings dialog
            let settings_label_color = if self.dark_mode {
                egui::Color32::WHITE
            } else {
                egui::Color32::from_rgb(76, 79, 105) // Dark text for light mode
            };
            
            egui::Window::new("⚙️ Settings - API Keys")
                .collapsible(false)
                .resizable(false)
                .auto_sized()
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading("Cloud Services API Keys");
                        ui.add_space(10.0);
                        
                        // Anthropic
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Anthropic Claude API Key:").strong().color(settings_label_color));
                            ui.horizontal(|ui| {
                                let text_edit = if self.show_anthropic_password {
                                    egui::TextEdit::singleline(&mut self.settings_anthropic_key)
                                } else {
                                    egui::TextEdit::singleline(&mut self.settings_anthropic_key).password(true)
                                };
                                ui.add(text_edit.hint_text("sk-ant-...").desired_width(400.0));
                                ui.checkbox(&mut self.show_anthropic_password, "Show");
                            });
                        });
                        
                        ui.add_space(10.0);
                        
                        // OpenAI
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("OpenAI GPT API Key:").strong().color(settings_label_color));
                            ui.horizontal(|ui| {
                                let text_edit = if self.show_openai_password {
                                    egui::TextEdit::singleline(&mut self.settings_openai_key)
                                } else {
                                    egui::TextEdit::singleline(&mut self.settings_openai_key).password(true)
                                };
                                ui.add(text_edit.hint_text("sk-...").desired_width(400.0));
                                ui.checkbox(&mut self.show_openai_password, "Show");
                            });
                        });
                        
                        ui.add_space(15.0);
                        ui.separator();
                        ui.add_space(10.0);
                        
                        ui.heading("AI Server API Settings");
                        ui.add_space(10.0);
                        
                        // Telegram/Server API
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Server API URL:").strong().color(settings_label_color));
                                    ui.text_edit_singleline(&mut self.settings_telegram_url);
                                });
                                ui.add_space(10.0);
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new("Port:").strong().color(settings_label_color));
                                    ui.add(egui::DragValue::new(&mut self.settings_telegram_port).range(1..=65535));
                                });
                            });
                            
                            ui.add_space(8.0);
                            
                            ui.label(egui::RichText::new("Server API Key:").strong().color(settings_label_color));
                            ui.horizontal(|ui| {
                                let text_edit = if self.show_telegram_password {
                                    egui::TextEdit::singleline(&mut self.settings_telegram_key)
                                } else {
                                    egui::TextEdit::singleline(&mut self.settings_telegram_key).password(true)
                                };
                                ui.add(text_edit.hint_text("secret_key").desired_width(400.0));
                                ui.checkbox(&mut self.show_telegram_password, "Show");
                            });
                            
                            ui.add_space(8.0);
                            
                            ui.checkbox(&mut self.settings_use_encryption, "🔐 Use Encryption");
                            
                            ui.add_space(8.0);
                            
                            ui.add_enabled_ui(self.settings_use_encryption, |ui| {
                                ui.label(egui::RichText::new("Encryption Key:").strong().color(settings_label_color));
                                ui.horizontal(|ui| {
                                    let text_edit = if self.show_enc_password {
                                        egui::TextEdit::singleline(&mut self.settings_telegram_enc_key)
                                    } else {
                                        egui::TextEdit::singleline(&mut self.settings_telegram_enc_key).password(true)
                                    };
                                    ui.add(text_edit.hint_text("32-byte hex key").desired_width(400.0));
                                    ui.checkbox(&mut self.show_enc_password, "Show");
                                });
                            });
                        });
                        
                        ui.add_space(15.0);
                        
                        // Help
                        ui.label(egui::RichText::new("💡 How to get API keys:").strong().color(settings_label_color));
                        ui.horizontal(|ui| {
                            ui.label("• Anthropic: ");
                            ui.hyperlink_to("console.anthropic.com", "https://console.anthropic.com");
                        });
                        ui.horizontal(|ui| {
                            ui.label("• OpenAI: ");
                            ui.hyperlink_to("platform.openai.com/api-keys", "https://platform.openai.com/api-keys");
                        });
                        ui.label("• AI Server: Contact your server administrator");
                        
                        ui.add_space(20.0);
                        
                        // Buttons
                        ui.horizontal(|ui| {
                            if ui.button("💾 Save").clicked() {
                                self.save_settings();
                                self.show_settings = false;
                            }
                            if ui.button("❌ Cancel").clicked() {
                                self.show_settings = false;
                            }
                        });
                    });
                });
        }
        
        // About Dialog
        if self.show_about {
            egui::Window::new("About ApiAi")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(&self.config.app_info.name);
                        ui.label(format!("Version: {}", &self.config.app_info.version));
                        ui.label(format!("Developer: {}", &self.config.app_info.developer_en));
                        ui.add_space(10.0);
                        ui.label("Application for AI-powered component search.");
                    });
                    ui.add_space(10.0);
                    if ui.button("Close").clicked() {
                        self.show_about = false;
                    }
                });
        }
        
        // --- NEW LAYOUT STRUCTURE ---
        
        // Define label color explicitly based on theme to ensure visibility
        let label_color = if self.dark_mode {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(76, 79, 105) // Dark text for light mode
        };
        
        // 1. Header Panel (Top)
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                ui.heading(egui::RichText::new(&self.config.app_info.name).size(18.0).strong().color(egui::Color32::from_rgb(50, 160, 255)));
                ui.label(egui::RichText::new("AI-powered component analysis and chat assistant").size(11.0).weak());
                
                // Provider and model information
                let provider_info = match self.selected_provider {
                    Provider::Anthropic => "Anthropic Claude (claude-3-sonnet-20240229)",
                    Provider::OpenAI => "OpenAI GPT (gpt-4o)",
                    Provider::Telegram => {
                        // For Telegram, we don't know the exact model, so just show it's via Telegram
                        "AI via Telegram Server"
                    }
                };
                ui.add_space(5.0);
                ui.label(egui::RichText::new(provider_info).size(10.0).weak().color(if self.dark_mode {
                    egui::Color32::from_rgb(180, 190, 254) // Light lavender in dark mode
                } else {
                    egui::Color32::from_rgb(76, 79, 105) // Dark text in light mode
                }));
                
                ui.add_space(10.0);
            });
        });

        // 2. Input Panel (Top)
        egui::TopBottomPanel::top("input_panel").show(ctx, |ui| {
            ui.add_space(20.0);
            ui.vertical(|ui| {
                let panel_width = ui.available_width();
                    
                    // Mode Selection
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Mode:").strong().color(label_color));
                        ui.radio_value(&mut self.prompt_mode, PromptMode::GeneralChat, "💬 General Chat");
                        ui.radio_value(&mut self.prompt_mode, PromptMode::ComponentAnalysis, "🔧 Component Analysis");
                    });
                    
                    ui.add_space(5.0);
                    
                    // Chat Mode Toggle and Clear Button
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.chat_mode_enabled, "🔄 Chat Mode (remember context)");
                        
                        ui.add_space(10.0);
                        
                        // Clear Chat button - only enabled if conversation_id exists
                        let has_conversation = self.conversation_id.is_some();
                        ui.add_enabled_ui(has_conversation, |ui| {
                            if ui.button("🗑️ Clear Chat").clicked() {
                                self.conversation_id = None;
                                self.conversation_history.clear();
                                self.response_text.clear();
                            }
                        });
                        
                        // Show conversation ID if exists
                        if let Some(ref conv_id) = self.conversation_id {
                            ui.label(egui::RichText::new(format!("ID: {}", &conv_id[..12.min(conv_id.len())])).weak().small());
                        }
                    });
                    
                    ui.add_space(8.0);

                    // Provider selection
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Provider:").strong().color(label_color));
                        ui.selectable_value(&mut self.selected_provider, Provider::Anthropic, "Anthropic");
                        ui.selectable_value(&mut self.selected_provider, Provider::OpenAI, "OpenAI");
                        ui.selectable_value(&mut self.selected_provider, Provider::Telegram, "Telegram");
                    });
                    
                    ui.add_space(10.0);
                    
                    // Input Label
                    let input_label = match self.prompt_mode {
                        PromptMode::GeneralChat => "Enter your prompt:",
                        PromptMode::ComponentAnalysis => "Enter component name:",
                    };
                    ui.label(egui::RichText::new(input_label).size(12.0).color(label_color));
                    ui.add_space(3.0);

                    // Search input - scrollable with fixed height
                    egui::ScrollArea::vertical()
                        .max_height(150.0)
                        .show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.search_query)
                                    .hint_text(if self.prompt_mode == PromptMode::GeneralChat { "Ask anything..." } else { "Component part number..." })
                                    .desired_width(panel_width)
                                    .font(egui::TextStyle::Body)
                            );
                        });
                    
                    // Ctrl+Enter to search
                    if ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command) {
                        self.perform_search();
                    }
                    
                    ui.add_space(8.0);
                    
                    // Send Button - Compact and Centered
                    let btn_text = match self.prompt_mode {
                        PromptMode::GeneralChat => "🚀 Send Request",
                        PromptMode::ComponentAnalysis => "🔍 Analyze Component",
                    };
                    
                    ui.horizontal(|ui| {
                        // Center the button
                        let button_width = 200.0;
                        let available_width = panel_width;
                        let left_padding = (available_width - button_width) / 2.0;
                        ui.add_space(left_padding);
                        
                        // Create button with same color as active provider background in each theme
                        let button_color = if self.dark_mode {
                            egui::Color32::from_rgb(90, 143, 234)   // Dark: #5a8fea
                        } else {
                            // Use the exact same color as active provider background
                            // Match the visual appearance of active selectable_value (light blue)
                            // The active provider appears as light blue (#A0D9F5 or similar)
                            // Use the hovered color which matches the visual appearance
                            ctx.style().visuals.widgets.hovered.bg_fill
                        };
                        
                        let send_button = egui::Button::new(egui::RichText::new(btn_text).size(14.0))
                            .fill(button_color)
                            .min_size(egui::vec2(button_width, 36.0));
                        
                        if ui.add(send_button).clicked() {
                            self.perform_search();
                        }
                    });
                    
                    ui.add_space(5.0);
                });
            ui.add_space(20.0);
        });

        // 3. Export Panel (Bottom) - Docked above the footer
        egui::TopBottomPanel::bottom("export_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(20.0);
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("Export:").strong().color(label_color));
                        
                        let has_response = !self.response_text.is_empty();
                        
                        ui.add_enabled_ui(has_response, |ui| {
                            if ui.button("📄 TXT").clicked() { /* TODO */ }
                            if ui.button("📘 DOCX").clicked() { /* TODO */ }
                            if ui.button("📕 PDF").clicked() { /* TODO */ }
                            if ui.button("🌐 HTML").clicked() { /* TODO */ }
                        });
                    });
                    ui.add_space(5.0);
                });
            });
        });

        // 4. Response Area (Central) - Fills remaining space
        egui::CentralPanel::default().show(ctx, |ui| {
            // Remove all padding to maximize space
            let mut frame = egui::Frame::central_panel(ui.style());
            frame.inner_margin = egui::Margin::symmetric(20.0, 0.0);
            
            frame.show(ui, |ui| {
                ui.vertical(|ui| {
                    // Use the same width as input panel
                    let panel_width = ui.available_width() - 20.0;
                    
                    ui.label(egui::RichText::new("Response:").size(12.0).strong().color(label_color));
                    ui.add_space(3.0);
                    
                    // Calculate exact rows to fill remaining space
                    let available_height = ui.available_height();
                    let line_height = ui.text_style_height(&egui::TextStyle::Body);
                    let rows = ((available_height - 10.0) / line_height).floor() as usize;
                    
                    ui.add(
                        egui::TextEdit::multiline(&mut self.response_text)
                            .desired_width(panel_width)
                            .desired_rows(rows.max(15))
                            .font(egui::TextStyle::Body)
                    );
                });
            });
        });
        
        // Keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.command) {
            self.toggle_theme();
        }
    }
}
