// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/


mod api;
mod encryption;

use api::{ApiClient, AnthropicClient, OpenAIClient, TelegramClient};
use tauri::State;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Serialize, Deserialize)]
struct SecurityConfig {
    pin_code: String,
    require_pin: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct ApiKeysConfig {
    anthropic: String,
    openai: String,
    telegram_url: String,
    telegram_key: String,
    telegram_enc_key: String,
    telegram_use_encryption: bool,
}

#[derive(Clone, Serialize, Deserialize)]
struct UiConfig {
    theme: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct AppConfig {
    security: SecurityConfig,
    api_keys: ApiKeysConfig,
    ui: UiConfig,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            security: SecurityConfig {
                pin_code: "1234".to_string(),
                require_pin: true,
            },
            api_keys: ApiKeysConfig {
                anthropic: "".to_string(),
                openai: "".to_string(),
                telegram_url: "http://localhost:8000".to_string(),
                telegram_key: "".to_string(),
                telegram_enc_key: "".to_string(),
                telegram_use_encryption: false,
            },
            ui: UiConfig {
                theme: "dark".to_string(),
            },
        }
    }
}

struct AppState {
    config: Mutex<AppConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SearchResponse {
    text: String,
    provider: String,
    conversation_id: Option<String>,
}

#[tauri::command]
fn get_config(state: State<AppState>) -> AppConfig {
    let config = state.config.lock().unwrap();
    config.clone()
}

#[tauri::command]
fn save_config(new_config: AppConfig, state: State<AppState>) -> Result<(), String> {
    // Update state
    {
        let mut config = state.config.lock().unwrap();
        *config = new_config.clone();
    }

    // Save to file
    let path = get_config_path();
    let content = serde_json::to_string_pretty(&new_config).map_err(|e| e.to_string())?;
    
    fs::write(&path, content).map_err(|e| format!("Failed to write config to {:?}: {}", path, e))?;
    
    Ok(())
}

#[tauri::command]
fn check_pin(pin: String, state: State<AppState>) -> bool {
    let config = state.config.lock().unwrap();
    if !config.security.require_pin {
        return true;
    }
    config.security.pin_code == pin
}

#[tauri::command]
fn is_pin_required(state: State<AppState>) -> bool {
    let config = state.config.lock().unwrap();
    config.security.require_pin
}

#[tauri::command]
async fn perform_search(
    query: String,
    provider: String,
    api_key: String,
    telegram_url: Option<String>,
    encryption_key: Option<String>,
    use_encryption: bool,
    chat_mode: bool,
    conversation_id: Option<String>
) -> Result<SearchResponse, String> {
    let client: Box<dyn ApiClient> = match provider.as_str() {
        "anthropic" => Box::new(AnthropicClient::new(api_key)),
        "openai" => Box::new(OpenAIClient::new(api_key)),
        "telegram" => {
            let url = telegram_url.unwrap_or_default();
            // Handle URL construction if needed (port logic can be done in frontend or here)
            Box::new(TelegramClient::new(url, api_key, encryption_key, use_encryption, chat_mode, conversation_id))
        },
        _ => return Err("Unknown provider".to_string()),
    };

    match client.search(&query).await {
        Ok(result) => Ok(SearchResponse {
            text: result.text,
            provider: result.provider,
            conversation_id: result.conversation_id,
        }),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

fn get_config_path() -> std::path::PathBuf {
    // 1. Check if config exists in current directory (Project root in dev)
    let current_dir_config = std::path::Path::new("config_qt.json");
    if current_dir_config.exists() {
        return current_dir_config.to_path_buf();
    }

    // 2. Check parent directory (Dev mode: src-tauri/../config_qt.json)
    let parent_dir_config = std::path::Path::new("../config_qt.json");
    if parent_dir_config.exists() {
        return parent_dir_config.to_path_buf();
    }

    // 3. Check executable directory (Production)
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let exe_config = exe_dir.join("config_qt.json");
            if exe_config.exists() {
                return exe_config;
            }
            // If none exists, default to writing to exe dir
            return exe_config;
        }
    }

    // Fallback to current directory
    std::path::PathBuf::from("config_qt.json")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let config_path = get_config_path();
    println!("Loading config from: {:?}", config_path);

    let config = if let Ok(content) = fs::read_to_string(&config_path) {
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        AppConfig::default()
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { config: Mutex::new(config) })
        .invoke_handler(tauri::generate_handler![perform_search, check_pin, is_pin_required, get_config, save_config])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
