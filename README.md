# metabase-api-rs

A simplified and efficient Rust client for the Metabase API.

## Features

- 🚀 **Simple API**: Clean and intuitive interface
- 🔐 **Automatic Authentication**: Handles session management automatically
- 🔄 **Retry Logic**: Built-in exponential backoff for failed requests
- 📦 **Modular Design**: Use only what you need with feature flags
- 🦀 **Type Safe**: Leverages Rust's type system for safety

## Quick Start

```rust
use metabase_api_rs::MetabaseClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client with email/password authentication
    let client = MetabaseClient::builder()
        .base_url("https://metabase.example.com")
        .email_password("user@example.com", "password")
        .build()
        .await?;

    // Get a card
    let card = client.get_card(123).await?;
    println!("Card: {}", card.name);

    Ok(())
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
metabase-api-rs = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Features

- `cache` - Enable in-memory caching
- `performance` - Enable performance optimizations
- `query-builder` - Enable MBQL query builder
- `full` - Enable all features

## Architecture

This library uses a simplified 3-layer architecture:

- **API Layer**: Public interface and client
- **Core Layer**: Business logic and models
- **Transport Layer**: HTTP communication and retry logic

## License

MIT