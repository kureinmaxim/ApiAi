# Shared Rust Library (apiai-shared)

This is a shared library containing common API client and encryption functionality used by both the experimental Rust CLI and the Tauri application.

## Purpose

Eliminates code duplication between:
- `rust/` - Experimental CLI version
- `tauri-app/src-tauri/` - Tauri application backend

## Modules

### `api`
API clients for different AI providers:
- **AnthropicClient** - Anthropic Claude API
- **OpenAIClient** - OpenAI GPT API  
- **TelegramClient** - Custom Telegram bot API with encryption support

### `encryption`
**SecureMessenger** - AES-256-GCM encryption utilities for secure communication with Telegram bot

## Usage

In `Cargo.toml`:
```toml
[dependencies]
apiai-shared = { path = "../shared-rs" }
```

In your code:
```rust
use apiai_shared::{TelegramClient, SecureMessenger};
```

## Development

```bash
# Check code
cargo check

# Run tests (when added)
cargo test

# Build
cargo build --release
```

## Version

Current version: **2.1.1** (synced with main project)
