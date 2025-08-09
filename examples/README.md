# metabase-api-rs Examples

This directory contains practical examples demonstrating how to use the `metabase-api-rs` library.

## Prerequisites

Before running the examples, ensure you have:

1. A running Metabase instance (local or remote)
2. Valid credentials (email/password or API key)
3. Environment variables configured (optional):

```bash
export METABASE_URL="http://localhost:3000"
export METABASE_EMAIL="admin@example.com"
export METABASE_PASSWORD="password123"
```

## Available Examples

### 1. Simple Example (`simple_example.rs`)
The simplest starting point:
- Client creation with ClientBuilder
- Authentication using email/password
- Getting current user information
- Listing collections and cards

```bash
cargo run --example simple_example
```

### 2. Basic Usage (`basic_usage.rs`)
Demonstrates fundamental operations:
- Creating and configuring a client
- Authentication methods
- Getting user details
- Listing collections and cards
- Proper logout

```bash
cargo run --example basic_usage
```

### 3. Query Execution (`query_execution.rs`)
Shows various ways to execute queries:
- Simple SQL queries
- Parameterized SQL queries with type safety
- NativeQuery builder pattern
- MBQL (Metabase Query Language) queries
- Exporting results in different formats (CSV, JSON, XLSX)

```bash
cargo run --example query_execution
# With MBQL feature:
cargo run --example query_execution --features query-builder
```

### 4. CRUD Simple (`crud_simple.rs`)
Simplified CRUD examples for main entities:
- Collections: List and read operations
- Cards (Questions): List, read, and query execution
- Dashboards: List and read operations
- Update operations using JSON

```bash
cargo run --example crud_simple
```

### 5. Advanced Features (`advanced_features.rs`)
Advanced functionality demonstration:
- Cache control and performance optimization
- Custom retry policies for resilience
- Comprehensive error handling patterns
- Session management and auto-refresh
- Performance tuning techniques

```bash
cargo run --example advanced_features
# With cache feature:
cargo run --example advanced_features --features cache
```

## Running Examples

### Run a specific example:
```bash
cargo run --example <example_name>
```

### Run with all features enabled:
```bash
cargo run --example <example_name> --features full
```

### Run with verbose output:
```bash
RUST_LOG=debug cargo run --example <example_name>
```

## Docker Setup for Testing

If you need a local Metabase instance for testing:

```bash
# Start Metabase with Docker
docker run -d -p 3000:3000 --name metabase metabase/metabase:latest

# Wait for Metabase to start (takes about 30 seconds)
sleep 30

# Access Metabase at http://localhost:3000
# Complete the setup wizard to create an admin account
```

## Common Issues

### Connection Refused
- Ensure Metabase is running and accessible
- Check the URL in environment variables or code
- Verify firewall/network settings

### Authentication Failed
- Verify credentials are correct
- Ensure the user account is active
- Check if API access is enabled in Metabase

### Missing Features
Some examples require specific features to be enabled:
```bash
# Enable all features
cargo run --example <example_name> --features full

# Enable specific features
cargo run --example <example_name> --features "cache,query-builder"
```

## Example Output

### Basic Usage Example
```
🚀 Connecting to Metabase at: http://localhost:3000
✅ Client created successfully
🔐 Authenticating as: admin@example.com
✅ Authentication successful!

👤 Current User:
  - ID: 1
  - Email: admin@example.com
  - Name: Admin User
  - Active: true
  - Superuser: true

💾 Available Databases:
  - [1] Sample Database: h2

📁 Collections:
  - [1] Our analytics (our-analytics)

📊 Cards (Questions):
  - [1] Orders over time
    Description: Track order trends

🔚 Logging out...
✅ Logged out successfully

🎉 Basic usage example completed successfully!
```

## Next Steps

1. Review the example code to understand patterns
2. Adapt examples to your specific use case
3. Explore the API documentation for more details
4. Build your own Metabase integrations!

## Contributing

Found an issue or have a suggestion for the examples? Please open an issue or submit a PR!