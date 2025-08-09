//! Simple SQL query execution example for metabase-api-rs
//!
//! This example demonstrates how to execute SQL queries

use metabase_api_rs::{api::Credentials, core::models::MetabaseId, ClientBuilder, Result};
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let base_url =
        std::env::var("METABASE_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

    println!("🚀 SQL Query Execution Example");
    println!("{}", "=".repeat(50));

    // Create and authenticate client
    let mut client = ClientBuilder::new(&base_url).build()?;

    let email = std::env::var("METABASE_EMAIL").unwrap_or_else(|_| "admin@example.com".to_string());
    let password = std::env::var("METABASE_PASSWORD").unwrap_or_else(|_| "password123".to_string());

    client
        .authenticate(Credentials::EmailPassword { email, password })
        .await?;

    println!("✅ Authenticated successfully");

    // Note: For SQL execution, you need a database ID
    // In a real scenario, you would get this from list_databases()
    // For this example, we'll use database ID 1 (commonly the sample database)
    let database_id = MetabaseId(1);

    // Example 1: Simple SQL query
    println!("\n1️⃣ Simple SQL Query");
    println!("{}", "-".repeat(30));

    let simple_sql = "SELECT COUNT(*) as total FROM orders";
    println!("Query: {}", simple_sql);

    match client.execute_sql(database_id, simple_sql).await {
        Ok(result) => {
            println!("✅ Query executed successfully");
            if let Some(row_count) = result.row_count {
                println!("   Rows returned: {}", row_count);
            }
            if let Some(running_time) = result.running_time {
                println!("   Execution time: {}ms", running_time);
            }
        }
        Err(e) => {
            println!("⚠️ Query failed: {}", e);
        }
    }

    // Example 2: Parameterized SQL query
    println!("\n2️⃣ Parameterized SQL Query");
    println!("{}", "-".repeat(30));

    let parameterized_sql = "SELECT * FROM orders WHERE status = {{status}} LIMIT 10";

    let mut params = HashMap::new();
    params.insert("status".to_string(), json!("completed"));

    println!("Query: {}", parameterized_sql);
    println!("Parameters: status = 'completed'");

    match client
        .execute_sql_with_params(database_id, parameterized_sql, params)
        .await
    {
        Ok(result) => {
            println!("✅ Parameterized query executed successfully");
            if let Some(row_count) = result.row_count {
                println!("   Rows returned: {}", row_count);
            }
            if let Some(running_time) = result.running_time {
                println!("   Execution time: {}ms", running_time);
            }

            // Show column names if available
            if !result.data.cols.is_empty() {
                let cols = &result.data.cols;
                let col_names: Vec<_> = cols.iter().map(|c| &c.name).collect();
                println!("   Columns: {:?}", col_names);
            }
        }
        Err(e) => {
            println!("⚠️ Query failed: {}", e);
        }
    }

    // Example 3: Using NativeQuery builder
    println!("\n3️⃣ NativeQuery Builder");
    println!("{}", "-".repeat(30));

    use metabase_api_rs::core::models::query::NativeQuery;

    let query = NativeQuery::builder("SELECT * FROM products WHERE price > {{min_price}} LIMIT 5")
        .add_number_param("min_price", 50.0)
        .build();

    println!("Built query with typed parameter: min_price = 50.0");

    match client.execute_native_query(database_id, query).await {
        Ok(result) => {
            println!("✅ Native query executed successfully");
            if let Some(row_count) = result.row_count {
                println!("   Rows returned: {}", row_count);
            }
        }
        Err(e) => {
            println!("⚠️ Query failed: {}", e);
        }
    }

    println!("\n✅ SQL query execution examples completed!");

    Ok(())
}
