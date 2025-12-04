use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub text: String,
    pub provider: String,
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

#[async_trait::async_trait]
#[allow(dead_code)]
pub trait ApiClient: Send + Sync {
    async fn search(&self, query: &str) -> Result<SearchResult, Box<dyn Error + Send + Sync>>;
}

#[allow(dead_code)]
pub struct AnthropicClient {
    api_key: String,
    _client: reqwest::Client,
}

#[allow(dead_code)]
impl AnthropicClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            _client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
#[async_trait::async_trait]
impl ApiClient for AnthropicClient {
    async fn search(&self, query: &str) -> Result<SearchResult, Box<dyn Error + Send + Sync>> {
        if self.api_key.is_empty() {
             return Err("Anthropic API key is missing".into());
        }

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&serde_json::json!({
                "model": "claude-3-sonnet-20240229",
                "max_tokens": 1024,
                "messages": [
                    {"role": "user", "content": query}
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("Anthropic API error: {}", error_text).into());
        }

        let json: serde_json::Value = response.json().await?;
        let text = json["content"][0]["text"]
            .as_str()
            .unwrap_or("No response text found")
            .to_string();

        Ok(SearchResult {
            text,
            provider: "Anthropic".to_string(),
            model: Some("claude-3-sonnet-20240229".to_string()),
            conversation_id: None,
            request_id: None,
        })
    }
}

#[allow(dead_code)]
pub struct OpenAIClient {
    api_key: String,
    _client: reqwest::Client,
}

#[allow(dead_code)]
impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            _client: reqwest::Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl ApiClient for OpenAIClient {
    async fn search(&self, query: &str) -> Result<SearchResult, Box<dyn Error + Send + Sync>> {
        if self.api_key.is_empty() {
             return Err("OpenAI API key is missing".into());
        }

        let client = reqwest::Client::new();
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "model": "gpt-4o",
                "messages": [
                    {"role": "user", "content": query}
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(format!("OpenAI API error: {}", error_text).into());
        }

        let json: serde_json::Value = response.json().await?;
        let text = json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response text found")
            .to_string();

        Ok(SearchResult {
            text,
            provider: "OpenAI".to_string(),
            model: Some("gpt-4o".to_string()),
            conversation_id: None,
            request_id: None,
        })
    }
}

#[allow(dead_code)]
use crate::encryption::SecureMessenger;

pub struct TelegramClient {
    url: String,
    api_key: String,
    encryption_key: Option<String>,
    use_encryption: bool,
    chat_mode: bool,
    conversation_id: Option<String>,
    _client: reqwest::Client,
}

impl TelegramClient {
    pub fn new(url: String, api_key: String, encryption_key: Option<String>, use_encryption: bool, chat_mode: bool, conversation_id: Option<String>) -> Self {
        Self {
            url,
            api_key,
            encryption_key,
            use_encryption,
            chat_mode,
            conversation_id,
            _client: reqwest::Client::new(),
        }
    }

    async fn search_encrypted(&self, query: &str) -> Result<SearchResult, Box<dyn Error + Send + Sync>> {
        let enc_key = self.encryption_key.as_ref().ok_or("Encryption key is missing")?;
        let messenger = SecureMessenger::new(enc_key).map_err(|e| format!("Failed to init encryption: {}", e))?;

        // Prepare payload - server expects "prompt" field
        let mut payload = serde_json::json!({
            "prompt": query,
            "provider": "anthropic",
            "max_tokens": 1024,
            "chat_mode": self.chat_mode
        });
        
        // Add conversation_id if exists
        if let Some(ref conv_id) = self.conversation_id {
            payload["conversation_id"] = serde_json::json!(conv_id);
        }

        // Encrypt payload
        let encrypted_data = messenger.encrypt_json(&payload)
            .map_err(|e| format!("Encryption failed: {}", e))?;

        // Use the URL as provided - don't modify it
        // This allows using /echo, /ai_query/secure, or any other endpoint
        let url = self.url.trim_end_matches('/').to_string();

        let client = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("X-API-KEY", reqwest::header::HeaderValue::from_str(&self.api_key)?);
        }
        headers.insert("X-APP-ID", reqwest::header::HeaderValue::from_static("apiai-v2"));

        let response = client
            .post(&url)
            .headers(headers)
            .json(&serde_json::json!({
                "data": encrypted_data
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(format!("Server error ({}): {}", status, error_text).into());
        }

        let response_json: serde_json::Value = response.json().await?;
        let encrypted_response = response_json["data"].as_str()
            .ok_or("Invalid response format: missing 'data'")?;

        // Decrypt response
        let decrypted_data: serde_json::Value = messenger.decrypt_json(encrypted_response)
            .map_err(|e| format!("Decryption failed: {}", e))?;

        let result_text = if let Some(resp) = decrypted_data.get("response") {
            resp.as_str().unwrap_or("").to_string()
        } else {
            decrypted_data.to_string()
        };
        
        let conversation_id = decrypted_data.get("conversation_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        // Extract provider and model from server response
        let provider = decrypted_data.get("provider")
            .and_then(|v| v.as_str())
            .unwrap_or("anthropic")
            .to_string();
        let model = decrypted_data.get("model")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
            
        let request_id = decrypted_data.get("request_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(SearchResult {
            text: result_text,
            provider,
            model,
            conversation_id,
            request_id,
        })
    }
}

#[async_trait::async_trait]
impl ApiClient for TelegramClient {
    async fn search(&self, query: &str) -> Result<SearchResult, Box<dyn Error + Send + Sync>> {
        if self.use_encryption && self.encryption_key.is_some() {
            return self.search_encrypted(query).await;
        }

        if self.url.is_empty() {
             return Err("Server URL is missing".into());
        }

        let client = reqwest::Client::new();
        
        // Construct headers
        let mut headers = reqwest::header::HeaderMap::new();
        if !self.api_key.is_empty() {
            headers.insert("X-API-KEY", reqwest::header::HeaderValue::from_str(&self.api_key)?);
        }
        headers.insert("X-APP-ID", reqwest::header::HeaderValue::from_static("apiai-v2"));

        // Prepare payload
        let mut payload = serde_json::json!({
            "prompt": query,
            "provider": "anthropic",
            "max_tokens": 1024,
            "chat_mode": self.chat_mode
        });
        
        // Add conversation_id if exists
        if let Some(ref conv_id) = self.conversation_id {
            payload["conversation_id"] = serde_json::json!(conv_id);
        }

        let response = client
            .post(&self.url)
            .headers(headers)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(format!("Server error ({}): {}", status, error_text).into());
        }

        let text = response.text().await?;
        
        // Try to parse as JSON first
        let (result_text, conversation_id, provider, model, request_id) = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
            let text_content = if let Some(resp) = json.get("response") {
                resp.as_str().unwrap_or(&text).to_string()
            } else if let Some(content) = json.get("content") {
                 content.as_str().unwrap_or(&text).to_string()
            } else {
                text.clone()
            };
            
            let conv_id = json.get("conversation_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            // Extract provider and model from response if available
            let provider_str = json.get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("anthropic")
                .to_string();
            let model_str = json.get("model")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
                
            let req_id = json.get("request_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            
            (text_content, conv_id, provider_str, model_str, req_id)
        } else {
            (text, None, "Telegram".to_string(), None, None)
        };

        Ok(SearchResult {
            text: result_text,
            provider,
            model,
            conversation_id,
            request_id,
        })
    }
}
