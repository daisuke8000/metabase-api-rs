# metabase-api-rs

⚠️ **Alpha Release** - This is an experimental version. API may change. Not recommended for production use.

A simplified and efficient Rust client for the Metabase API.

## Features

- 🚀 **Simple API**: Clean and intuitive interface
- 🔐 **Automatic Authentication**: Handles session management automatically
- 🔄 **Retry Logic**: Built-in exponential backoff for failed requests
- 📦 **Modular Design**: Use only what you need with feature flags
- 🦀 **Type Safe**: Leverages Rust's type system for safety
- 🧪 **Well Tested**: 100+ tests with ~80% coverage
- 📚 **Examples**: Ready-to-use sample code included

## Quick Start

```rust
use metabase_api_rs::{ClientBuilder, api::Credentials, Result};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Create and authenticate client
    let mut client = ClientBuilder::new("https://metabase.example.com")
        .timeout(Duration::from_secs(30))
        .build()?;
    
    client.authenticate(Credentials::EmailPassword {
        email: "user@example.com".to_string(),
        password: "password".to_string(),
    }).await?;

    // Get a card
    let card = client.get_card(123).await?;
    println!("Card: {}", card.name());

    Ok(())
}
```

## Examples

See the `examples/` directory for more comprehensive examples:
- `simple_example.rs` - Basic authentication and API usage
- `crud_simple.rs` - CRUD operations on Collections
- `sql_query_simple.rs` - Execute SQL queries directly

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
metabase-api-rs = "0.1.0-alpha.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
serde_json = "1.0"
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

## Documentation

For detailed documentation and development guidelines, see the `docs/` directory:
- [Architecture Overview](docs/ARCHITECTURE.md)
- [API Structure](docs/API_STRUCTURE.md)
- [Development Rules](docs/DEVELOPMENT_RULES.md)
- [Documentation Index](docs/INDEX.md)

## Development

This project follows strict TDD (Test-Driven Development) practices. All development should use the provided Taskfile commands:

```bash
task dev        # Run development cycle (fmt, build, test)
task test       # Run all tests
task check      # Run all quality checks
```

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.