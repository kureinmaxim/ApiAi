// ApiAi - Rust implementation
// Main entry point with egui GUI

mod api;

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use api::{ApiClient, AnthropicClient, OpenAIClient, TelegramClient, SearchResult};

const COMPONENT_ANALYSIS_TEMPLATE: &str = r#"Роль: Инженер-разработчик радиоэлектронной аппаратуры с опытом во всех направлениях: силовая электроника, аналоговая и цифровая схемотехника, СВЧ-техника, датчики и измерения.

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
    // Initialize tokio runtime
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime");

    // Enter the runtime context so we can spawn tasks
    let _enter = rt.enter();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 600.0]) // User requested 600x600
            .with_min_inner_size([500.0, 400.0])
            .with_title("ApiAi"),
        ..Default::default()
    };
    
    eframe::run_native(
        "ApiAi",
        options,
        Box::new(|cc| {
            // Customize fonts
            let fonts = egui::FontDefinitions::default();
            cc.egui_ctx.set_fonts(fonts);
            
            Ok(Box::new(ApiAiApp::new(cc)))
        }),
    )
}

// ... (struct definitions remain the same)

    fn configure_theme(&self, ctx: &egui::Context) {
        let visuals = if self.dark_mode {
            let mut v = egui::Visuals::dark();
            
            // Modern Tech Theme (Deep Navy/Black)
            let bg_color = egui::Color32::from_rgb(10, 13, 20); // Very deep navy, almost black
            let panel_color = egui::Color32::from_rgb(20, 25, 35); // Slightly lighter for panels
            let accent_color = egui::Color32::from_rgb(50, 160, 255); // Electric Blue
            let text_color = egui::Color32::from_rgb(245, 245, 250); // Bright white-ish
            let faint_bg = egui::Color32::from_rgb(30, 35, 50); // Input fields
            
            v.widgets.noninteractive.bg_fill = bg_color;
            v.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, text_color);
            
            // Window & Panel backgrounds
            v.window_fill = panel_color;
            v.panel_fill = bg_color;
            
            // Inputs
            v.widgets.inactive.bg_fill = faint_bg;
            v.widgets.hovered.bg_fill = faint_bg.linear_multiply(1.2); 
            v.widgets.active.bg_fill = faint_bg.linear_multiply(1.4);
            
            // Rounding - Modern soft rounding
            v.window_rounding = egui::Rounding::same(10.0);
            v.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
            v.widgets.inactive.rounding = egui::Rounding::same(6.0);
            v.widgets.hovered.rounding = egui::Rounding::same(6.0);
            v.widgets.active.rounding = egui::Rounding::same(6.0);
            v.widgets.open.rounding = egui::Rounding::same(6.0);
            
            // Selection
            v.selection.bg_fill = accent_color;
            v.selection.stroke = egui::Stroke::new(1.0, accent_color);
            
            v
        } else {
            let mut v = egui::Visuals::light();
            v.window_rounding = egui::Rounding::same(10.0);
            v.widgets.noninteractive.rounding = egui::Rounding::same(6.0);
            v.widgets.inactive.rounding = egui::Rounding::same(6.0);
            v.widgets.hovered.rounding = egui::Rounding::same(6.0);
            v.widgets.active.rounding = egui::Rounding::same(6.0);
            v
        };
        
        ctx.set_visuals(visuals);
    }
    }

impl eframe::App for ApiAiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ... (async check remains same)

        self.configure_theme(ctx);
        
        // ... (menu bar and footer remain same)
        
        // Main content panel
        egui::CentralPanel::default().show(ctx, |ui| {
            let available_height = ui.available_height();
            
            ui.vertical_centered(|ui| {
                ui.add_space(available_height * 0.05);
                
                // Title
                ui.heading(egui::RichText::new(&self.config.app_info.name).size(32.0).strong().color(egui::Color32::from_rgb(50, 160, 255)));
                ui.label(egui::RichText::new(&self.config.app_info.description).size(14.0).weak());
                
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
                                .margin(egui::vec2(10.0, 10.0))
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
                
                // ... (Results area logic)

