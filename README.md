# metabase-api-rs

⚠️ **Beta Release** - API may change. Production ready with no known security vulnerabilities.

A simplified Rust client for the Metabase API.

## Installation

```toml
[dependencies]
metabase-api-rs = "0.1.0-beta.3"
tokio = { version = "1", features = ["rt-multi-thread", "macros", "time"] }
serde_json = "1.0"
```

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
    
    client.authenticate(Credentials::email_password(
        "user@example.com",
        "password"
    )).await?;

    // Get a card
    let card = client.get_card(123).await?;
    println!("Card: {}", card.name);

    Ok(())
}
```

## Features

| Feature | Description |
|---------|-------------|
| `cache` | Enable in-memory LRU caching |
| `performance` | Performance optimizations |
| `query-builder` | MBQL query builder |
| `full` | Enable all features |

## Examples

See working examples in the [`examples/`](examples/) directory or on [docs.rs](https://docs.rs/metabase-api-rs).

## Documentation

Full API documentation is available on [docs.rs](https://docs.rs/metabase-api-rs).

## License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.