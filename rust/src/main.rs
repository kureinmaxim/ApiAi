// ApiAi - Rust implementation
// Main entry point with egui GUI

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0])
            .with_min_inner_size([600.0, 400.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "ApiAi",
        options,
        Box::new(|cc| Ok(Box::new(ApiAiApp::new(cc)))),
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppInfo {
    name: String,
    version: String,
    edition: String,
    description: String,
    release_date: String,
    developer_en: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config {
    app_info: AppInfo,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            app_info: AppInfo {
                name: "ApiAi".to_string(),
                version: "1.0.3".to_string(),
                edition: "Rust Experimental".to_string(),
                description: "AI-powered component search (Rust)".to_string(),
                release_date: "2025-11-30".to_string(),
                developer_en: "Kurein M.N.".to_string(),
            },
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
enum Provider {
    Anthropic,
    OpenAI,
    Telegram,
}

impl Provider {
    fn as_str(&self) -> &str {
        match self {
            Provider::Anthropic => "Anthropic Claude",
            Provider::OpenAI => "OpenAI GPT",
            Provider::Telegram => "Telegram Bot",
        }
    }
}

struct ApiAiApp {
    config: Config,
    dark_mode: bool,
    
    // Search state
    selected_provider: Provider,
    search_query: String,
    search_result: String,
    
    // UI state
    show_about: bool,
}

impl ApiAiApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Try to load config
        let config = Self::load_config().unwrap_or_default();
        
        Self {
            config,
            dark_mode: true,
            selected_provider: Provider::Anthropic,
            search_query: String::new(),
            search_result: String::new(),
            show_about: false,
        }
    }
    
    fn load_config() -> Option<Config> {
        let config_path = PathBuf::from("../python/config/config_qt.json.template");
        if let Ok(content) = fs::read_to_string(config_path) {
            serde_json::from_str(&content).ok()
        } else {
            None
        }
    }
    
    fn perform_search(&mut self) {
        if self.search_query.trim().is_empty() {
            self.search_result = "❌ Please enter a search query".to_string();
            return;
        }
        
        self.search_result = format!(
            "🔍 Searching for: \"{}\"\n📡 Provider: {}\n\n⚙️ API integration coming soon...\n\nThis is the Rust experimental version of ApiAi.",
            self.search_query,
            self.selected_provider.as_str()
        );
    }
}

impl eframe::App for ApiAiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set theme
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }
        
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("View", |ui| {
                    if ui.button(if self.dark_mode { "☀ Light Theme" } else { "🌙 Dark Theme" }).clicked() {
                        self.dark_mode = !self.dark_mode;
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("PDF Search", |ui| {
                    if ui.button("AI Search").clicked() {
                        ui.close_menu();
                    }
                    if ui.button("Settings (API Keys)").clicked() {
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.show_about = true;
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Footer
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Developer: {}", self.config.app_info.developer_en));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let window_size = ctx.input(|i| i.viewport().inner_rect.unwrap_or(egui::Rect::NOTHING).size());
                    ui.label(format!("{}x{}", window_size.x as i32, window_size.y as i32));
                    ui.separator();
                    ui.label(format!("Release: {}", self.config.app_info.release_date));
                });
            });
        });
        
        // Main content panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);
                
                // Title
                ui.heading(format!("{} v{}", self.config.app_info.name, self.config.app_info.version));
                ui.label(&self.config.app_info.description);
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(20.0);
                
                // Provider selection
                ui.horizontal(|ui| {
                    ui.label("Провайдер:");
                    ui.radio_value(&mut self.selected_provider, Provider::Anthropic, Provider::Anthropic.as_str());
                    ui.radio_value(&mut self.selected_provider, Provider::OpenAI, Provider::OpenAI.as_str());
                    ui.radio_value(&mut self.selected_provider, Provider::Telegram, Provider::Telegram.as_str());
                });
                
                ui.add_space(10.0);
                
                // Search input
                ui.horizontal(|ui| {
                    ui.label("Запрос:");
                    let response = ui.text_edit_singleline(&mut self.search_query);
                    
                    // Enter key to search
                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.perform_search();
                    }
                });
                
                ui.add_space(10.0);
                
                // Search button
                if ui.button("🔍 Поиск").clicked() {
                    self.perform_search();
                }
                
                ui.add_space(20.0);
                ui.separator();
                ui.add_space(10.0);
                
                // Results
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(&mut self.search_result)
                            .desired_width(f32::INFINITY)
                            .desired_rows(15)
                            .font(egui::TextStyle::Monospace)
                    );
                });
                
                // Copy button
                if !self.search_result.is_empty() {
                    ui.add_space(10.0);
                    if ui.button("📋 Скопировать результат").clicked() {
                        ui.output_mut(|o| o.copied_text = self.search_result.clone());
                    }
                }
            });
        });
        
        // About dialog
        if self.show_about {
            egui::Window::new("About ApiAi")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(&self.config.app_info.name);
                        ui.label(format!("Version: {}", self.config.app_info.version));
                        ui.label(format!("Developer: {}", self.config.app_info.developer_en));
                        ui.add_space(10.0);
                        ui.label(&self.config.app_info.description);
                        ui.add_space(10.0);
                        if ui.button("OK").clicked() {
                            self.show_about = false;
                        }
                    });
                });
        }
    }
}

