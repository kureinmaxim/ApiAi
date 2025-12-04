// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Use shared library for API and encryption
use apiai_shared::{ApiClient, AnthropicClient, OpenAIClient, TelegramClient};

use tauri::{State, Manager};
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
    window_width: Option<f64>,
    window_height: Option<f64>,
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
                window_width: None,
                window_height: None,
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
    model: Option<String>,
    conversation_id: Option<String>,
    request_id: Option<String>,
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
            model: result.model,
            conversation_id: result.conversation_id,
            request_id: result.request_id,
        }),
        Err(e) => Err(format!("Error: {}", e)),
    }
}

#[tauri::command]
fn save_window_size(width: f64, height: f64, state: State<AppState>) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.ui.window_width = Some(width);
    config.ui.window_height = Some(height);
    
    // Save to file
    let path = get_config_path();
    let content = serde_json::to_string_pretty(&*config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| format!("Failed to write config to {:?}: {}", path, e))?;
    
    Ok(())
}

#[tauri::command]
fn reset_window_size(state: State<AppState>) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.ui.window_width = None;
    config.ui.window_height = None;
    
    // Save to file
    let path = get_config_path();
    let content = serde_json::to_string_pretty(&*config).map_err(|e| e.to_string())?;
    fs::write(&path, content).map_err(|e| format!("Failed to write config to {:?}: {}", path, e))?;
    
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct CancelRequestBody {
    request_id: String,
}

#[tauri::command]
async fn cancel_request(
    request_id: String,
    telegram_url: String,
    api_key: String,
) -> Result<String, String> {
    // Build cancel endpoint URL
    let cancel_url = if telegram_url.contains("/ai_query") {
        telegram_url.replace("/ai_query", "/cancel_request")
    } else {
        format!("{}/cancel_request", telegram_url.trim_end_matches('/'))
    };
    
    // Create HTTP client
    let client = reqwest::Client::new();
    
    // Prepare request body
    let body = CancelRequestBody { request_id };
    
    // Send POST request
    match client.post(&cancel_url)
        .header("X-API-KEY", api_key)
        .header("X-APP-ID", "apiai-v2")
        .json(&body)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                Ok("Request cancelled successfully".to_string())
            } else {
                Ok(format!("Cancel request returned status: {}", response.status()))
            }
        }
        Err(e) => {
            // Don't fail if cancel fails - request might have already completed
            Ok(format!("Cancel request failed (request may have completed): {}", e))
        }
    }
}

// ============================================================================
// Chat History Persistence
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMessage {
    role: String, // "user" or "assistant"
    content: String,
    timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatMetadata {
    created_at: String,
    last_modified: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conversation_id: Option<String>,
    chat_mode: bool,
    encryption_used: bool,
    message_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatHistory {
    version: String,
    metadata: ChatMetadata,
    messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedChatInfo {
    filename: String,
    path: String,
    metadata: ChatMetadata,
    preview: String, // First user message
}

#[tauri::command]
fn save_chat_history(
    chat_data: ChatHistory,
    file_path: String,
) -> Result<(), String> {
    let content = serde_json::to_string_pretty(&chat_data)
        .map_err(|e| format!("Failed to serialize chat history: {}", e))?;
    
    fs::write(&file_path, content)
        .map_err(|e| format!("Failed to write chat history to {:?}: {}", file_path, e))?;
    
    Ok(())
}

#[tauri::command]
fn load_chat_history(file_path: String) -> Result<ChatHistory, String> {
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read chat history from {:?}: {}", file_path, e))?;
    
    let chat_history: ChatHistory = serde_json::from_str(&content)
        .map_err(|e| format!("Failed to parse chat history: {}", e))?;
    
    Ok(chat_history)
}

#[tauri::command]
fn list_saved_chats(directory: String) -> Result<Vec<SavedChatInfo>, String> {
    let dir_path = std::path::Path::new(&directory);
    
    if !dir_path.exists() {
        return Ok(Vec::new());
    }
    
    let mut chats = Vec::new();
    
    let entries = fs::read_dir(dir_path)
        .map_err(|e| format!("Failed to read directory {:?}: {}", directory, e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            
            // Only process .json files
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(chat_history) = serde_json::from_str::<ChatHistory>(&content) {
                        // Get preview from first user message
                        let preview = chat_history.messages
                            .iter()
                            .find(|m| m.role == "user")
                            .map(|m| {
                                let content = &m.content;
                                if content.len() > 100 {
                                    format!("{}...", &content[..100])
                                } else {
                                    content.clone()
                                }
                            })
                            .unwrap_or_else(|| "Empty chat".to_string());
                        
                        chats.push(SavedChatInfo {
                            filename: entry.file_name().to_string_lossy().to_string(),
                            path: path.to_string_lossy().to_string(),
                            metadata: chat_history.metadata,
                            preview,
                        });
                    }
                }
            }
        }
    }
    
    // Sort by last_modified (newest first)
    chats.sort_by(|a, b| b.metadata.last_modified.cmp(&a.metadata.last_modified));
    
    Ok(chats)
}

#[tauri::command]
fn import_text_chat(file_path: String) -> Result<ChatHistory, String> {
    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    let mut messages = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    
    let mut current_role: Option<String> = None;
    let mut current_content = String::new();
    
    for line in lines {
        let trimmed = line.trim();
        
        // Detect role markers
        if trimmed.starts_with("User:") || trimmed.starts_with("USER:") {
            // Save previous message if exists
            if let Some(role) = current_role.take() {
                if !current_content.trim().is_empty() {
                    messages.push(ChatMessage {
                        role,
                        content: current_content.trim().to_string(),
                        timestamp: chrono::Local::now().to_rfc3339(),
                        provider: None,
                        model: None,
                    });
                }
            }
            current_role = Some("user".to_string());
            current_content = trimmed.strip_prefix("User:").or_else(|| trimmed.strip_prefix("USER:"))
                .unwrap_or("").trim().to_string();
        } else if trimmed.starts_with("AI:") || trimmed.starts_with("Assistant:") || trimmed.starts_with("ASSISTANT:") {
            // Save previous message if exists
            if let Some(role) = current_role.take() {
                if !current_content.trim().is_empty() {
                    messages.push(ChatMessage {
                        role,
                        content: current_content.trim().to_string(),
                        timestamp: chrono::Local::now().to_rfc3339(),
                        provider: None,
                        model: None,
                    });
                }
            }
            current_role = Some("assistant".to_string());
            current_content = trimmed.strip_prefix("AI:").or_else(|| trimmed.strip_prefix("Assistant:"))
                .or_else(|| trimmed.strip_prefix("ASSISTANT:"))
                .unwrap_or("").trim().to_string();
        } else if !trimmed.is_empty() && !trimmed.starts_with("=") && !trimmed.starts_with("-") && !trimmed.starts_with("Exported:") {
            // Continue current message
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(trimmed);
        }
    }
    
    // Save last message
    if let Some(role) = current_role {
        if !current_content.trim().is_empty() {
            messages.push(ChatMessage {
                role,
                content: current_content.trim().to_string(),
                timestamp: chrono::Local::now().to_rfc3339(),
                provider: None,
                model: None,
            });
        }
    }
    
    let now = chrono::Local::now().to_rfc3339();
    
    Ok(ChatHistory {
        version: "1.0".to_string(),
        metadata: ChatMetadata {
            created_at: now.clone(),
            last_modified: now,
            provider: None,
            model: None,
            conversation_id: None,
            chat_mode: false,
            encryption_used: false,
            message_count: messages.len(),
        },
        messages,
    })
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

    let config_clone = config.clone();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(AppState { config: Mutex::new(config) })
        .setup(move |app| {
            // Set window size on startup if saved in config
            if let Some(window) = app.get_webview_window("main") {
                if let Some(width) = config_clone.ui.window_width {
                    if let Some(height) = config_clone.ui.window_height {
                        println!("Setting window size on startup: {}x{}", width, height);
                        let _ = window.set_size(tauri::LogicalSize::new(width, height));
                    }
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            perform_search, 
            check_pin, 
            is_pin_required, 
            get_config, 
            save_config, 
            save_window_size, 
            reset_window_size,
            cancel_request,
            save_chat_history,
            load_chat_history,
            list_saved_chats,
            import_text_chat
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
