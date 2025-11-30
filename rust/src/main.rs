// ApiAi - Rust implementation
// Main entry point with egui GUI

mod api;

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
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([500.0, 400.0])
            .with_title("ApiAi"),
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
    search_results: Vec<String>,
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
    settings_telegram_key: String,
    settings_telegram_enc_key: String,
    settings_use_encryption: bool,
    show_anthropic_password: bool,
    show_openai_password: bool,
    show_telegram_password: bool,
    show_enc_password: bool,
}

#[derive(Default, PartialEq)]
enum PromptMode {
    #[default]
    GeneralChat,
    ComponentAnalysis,
}

#[derive(Default, PartialEq)]
enum Provider {
    #[default]
    Anthropic,
    OpenAI,
    Telegram,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct SecurityConfig {
    pin_code: String,
    require_pin: bool,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct AppConfig {
    app_info: AppInfo,
    api_keys: ApiKeys,
    ui: UiConfig,
    security: SecurityConfig,
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
                version: "1.0.0".to_string(),
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
        Self {
            config: AppConfig::default(),
            dark_mode: true,
            search_query: String::new(),
            search_results: Vec::new(),
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
            settings_telegram_key: String::new(),
            settings_telegram_enc_key: String::new(),
            settings_use_encryption: true,
            show_anthropic_password: false,
            show_openai_password: false,
            show_telegram_password: false,
            show_enc_password: false,
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
        self.config.api_keys.telegram_url = self.settings_telegram_url.clone();
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
        // Implement search functionality here
        self.search_results.push(format!("Search for: {}", self.search_query));
        self.search_query.clear();
    }
    
    fn configure_theme(&self, ctx: &egui::Context) {
        let visuals = if self.dark_mode {
            let mut v = egui::Visuals::dark();
            
            // Catppuccin Macchiato (Dark Theme from Python)
            let base = egui::Color32::from_rgb(30, 30, 46);          // #1e1e2e (Base)
            let surface1 = egui::Color32::from_rgb(49, 50, 68);      // #313244 (Surface1)
            let surface2 = egui::Color32::from_rgb(69, 71, 90);      // #45475a (Surface2)
            let text = egui::Color32::from_rgb(224, 229, 255);       // #e0e5ff (Text)
            let blue = egui::Color32::from_rgb(90, 143, 234);        // #5a8fea (Darker Blue)
            let lavender = egui::Color32::from_rgb(203, 166, 247);   // #cba6f7 (Lavender)
            let darker_bg = egui::Color32::from_rgb(26, 27, 38);     // #1a1b26 (Darker Base for inputs)
            
            // Background colors
            v.window_fill = base;
            v.panel_fill = base;
            v.faint_bg_color = darker_bg; // Darker input fields background
            
            // Text colors
            v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
            v.widgets.noninteractive.weak_bg_fill = darker_bg;
            v.widgets.noninteractive.bg_fill = base;
            
            // Interactive widgets (buttons, inputs)
            v.widgets.inactive.bg_fill = darker_bg;
            v.widgets.inactive.weak_bg_fill = darker_bg;
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
            let surface1 = egui::Color32::from_rgb(204, 208, 218);   // #ccd0da (Surface1)
            let text = egui::Color32::from_rgb(60, 63, 81);          // #3c3f51 (Text)
            let blue = egui::Color32::from_rgb(30, 102, 245);        // #1e66f5 (Blue)
            
            v.window_fill = base;
            v.panel_fill = base;
            v.faint_bg_color = surface1;
            
            v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text);
            v.widgets.inactive.bg_fill = surface1;
            v.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, text);
            
            v.widgets.active.bg_fill = blue;
            v.widgets.active.fg_stroke = egui::Stroke::new(2.0, egui::Color32::WHITE);
            
            v.selection.bg_fill = blue.linear_multiply(0.4);
            v.selection.stroke = egui::Stroke::new(1.0, egui::Color32::WHITE);
            
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
        self.configure_theme(ctx);

        // Footer (Status Bar)
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    let developer_label = ui.label(egui::RichText::new(format!("Developer: {}", self.config.app_info.developer_en)).strong());
                    
                    // Double-click to unlock
                    if developer_label.double_clicked() {
                        if !self.unlocked && self.config.security.require_pin {
                            self.show_pin_dialog = true;
                            self.pin_input.clear();
                            self.pin_error.clear();
                        }
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label("730x550"); // Placeholder for resolution/status
                        ui.label("Local");   // Placeholder for connection status
                        ui.label("Mode: Simple"); // Placeholder for mode
                        ui.label("DB: 1.1 (2025-11-24)"); // Placeholder for DB version
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
            egui::Window::new("⚙️ Settings - API Keys")
                .collapsible(false)
                .resizable(true)
                .default_width(600.0)
                .show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.heading("Cloud Services API Keys");
                        ui.add_space(10.0);
                        
                        // Anthropic
                        ui.group(|ui| {
                            ui.label(egui::RichText::new("Anthropic Claude API Key:").strong());
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
                            ui.label(egui::RichText::new("OpenAI GPT API Key:").strong());
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
                            ui.label(egui::RichText::new("Server API URL:").strong());
                            ui.text_edit_singleline(&mut self.settings_telegram_url);
                            
                            ui.add_space(8.0);
                            
                            ui.label(egui::RichText::new("Server API Key:").strong());
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
                                ui.label(egui::RichText::new("Encryption Key:").strong());
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
                        ui.label(egui::RichText::new("💡 How to get API keys:").strong());
                        ui.label("• Anthropic: console.anthropic.com");
                        ui.label("• OpenAI: platform.openai.com/api-keys");
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
        
        // Main content panel
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();
            
            ui.vertical_centered(|ui| {
                ui.add_space(available_height * 0.05);
                
                // Title
                ui.heading(egui::RichText::new(&self.config.app_info.name).size(32.0).strong().color(egui::Color32::from_rgb(50, 160, 255)));
                ui.label(egui::RichText::new("AI-powered component analysis and chat assistant").size(14.0).weak());
                
                ui.add_space(20.0);
                
                // Search Container - Adjusted width for 600px window
                let search_panel_width = 500.0;
                
                ui.allocate_ui_with_layout(
                    egui::vec2(search_panel_width, 0.0),
                    egui::Layout::top_down(egui::Align::Center), 
                    |ui| {
                        // Mode Selection
                        ui.group(|ui| {
                            ui.horizontal(|ui| {
                                ui.label(egui::RichText::new("Mode:").strong().color(ui.visuals().text_color()));
                                ui.radio_value(&mut self.prompt_mode, PromptMode::GeneralChat, "💬 General Chat");
                                ui.radio_value(&mut self.prompt_mode, PromptMode::ComponentAnalysis, "🔧 Component Analysis");
                            });
                        });
                        
                        ui.add_space(10.0);

                        // Provider selection
                        ui.horizontal(|ui| {
                            ui.label(egui::RichText::new("Provider:").strong().color(ui.visuals().text_color()));
                            ui.add_space(10.0);
                            ui.selectable_value(&mut self.selected_provider, Provider::Anthropic, "Anthropic");
                            ui.selectable_value(&mut self.selected_provider, Provider::OpenAI, "OpenAI");
                            ui.selectable_value(&mut self.selected_provider, Provider::Telegram, "Telegram");
                        });
                        
                        ui.add_space(15.0);
                        
                        // Input Label
                        let input_label = match self.prompt_mode {
                            PromptMode::GeneralChat => "Enter your prompt:",
                            PromptMode::ComponentAnalysis => "Enter component name (e.g., STM32F103C8T6):",
                        };
                        
                        ui.label(egui::RichText::new(input_label).size(14.0).color(ui.visuals().text_color()));
                        ui.add_space(5.0);

                        // Search input - Multiline for better UX
                        let response = ui.add(
                            egui::TextEdit::multiline(&mut self.search_query)
                                .hint_text(if self.prompt_mode == PromptMode::GeneralChat { "Ask anything..." } else { "Component part number..." })
                                .desired_width(search_panel_width)
                                .desired_rows(3) // More space for typing
                                .font(egui::TextStyle::Body)
                        );
                        
                        // Ctrl+Enter to search (since Enter creates new line in multiline)
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter) && i.modifiers.command) {
                            self.perform_search();
                        }
                        
                        ui.add_space(20.0);
                        
                        // Search button
                        let btn_text = match self.prompt_mode {
                            PromptMode::GeneralChat => "   🚀 Send Request   ",
                            PromptMode::ComponentAnalysis => "   🔍 Analyze Component   ",
                        };

                        let btn = egui::Button::new(egui::RichText::new(btn_text).size(16.0).strong())
                            .min_size(egui::vec2(180.0, 36.0));
                            
                        if ui.add(btn).clicked() {
                            self.perform_search();
                        }
                    }
                );
                
                // Results area
                if !self.search_results.is_empty() {
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(10.0);
                    
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            ui.vertical(|ui| {
                                for result in &self.search_results {
                                    ui.label(egui::RichText::new(result).text_style(egui::TextStyle::Body));
                                    ui.add_space(10.0);
                                }
                            });
                        });
                }
            });
        });
        
        // Keyboard shortcuts
        if ctx.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.command) {
            self.toggle_theme();
        }
    }
}
