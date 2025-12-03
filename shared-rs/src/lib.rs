// ApiAi Shared Library
// Common API clients and encryption utilities

pub mod api;
pub mod encryption;

// Re-export commonly used types
pub use api::{ApiClient, AnthropicClient, OpenAIClient, TelegramClient, SearchResult};
pub use encryption::SecureMessenger;
