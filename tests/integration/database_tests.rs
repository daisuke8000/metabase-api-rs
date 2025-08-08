//! Database API integration tests

use metabase_api_rs::core::models::MetabaseId;
use crate::integration::setup::get_test_client;

#[tokio::test]
async fn test_list_databases() {
    let client = get_test_client().await;
    
    let databases = client.list_databases().await;
    assert!(databases.is_ok(), "Should be able to list databases");
    
    let databases = databases.unwrap();
    assert!(databases.len() > 0, "Should have at least one database");
    
    // Metabase always has at least the sample database
    assert!(databases.iter().any(|db| db.name.contains("Sample") || db.name.contains("H2")));
}

#[tokio::test]
async fn test_get_database_metadata() {
    let client = get_test_client().await;
    
    // Get the first database (usually the sample database)
    let databases = client.list_databases().await.expect("Should list databases");
    if databases.is_empty() {
        println!("No databases available, skipping test");
        return;
    }
    
    let db_id = databases[0].id;
    let metadata = client.get_database_metadata(db_id).await;
    
    assert!(metadata.is_ok(), "Should be able to get database metadata");
    let metadata = metadata.unwrap();
    assert_eq!(metadata.id, db_id);
    assert!(metadata.tables.len() > 0, "Database should have tables");
    
    // Check that tables have fields
    for table in &metadata.tables {
        assert!(table.fields.len() > 0, "Table {} should have fields", table.name);
    }
}

#[tokio::test]
async fn test_get_database_fields() {
    let client = get_test_client().await;
    
    // Get the first database
    let databases = client.list_databases().await.expect("Should list databases");
    if databases.is_empty() {
        println!("No databases available, skipping test");
        return;
    }
    
    let db_id = databases[0].id;
    let fields = client.get_database_fields(db_id).await;
    
    assert!(fields.is_ok(), "Should be able to get database fields");
    let fields = fields.unwrap();
    assert!(fields.len() > 0, "Database should have fields");
    
    // Verify field structure
    for field in &fields {
        assert!(!field.name.is_empty(), "Field should have a name");
        assert!(!field.base_type.is_empty(), "Field should have a base type");
    }
}

#[tokio::test]
async fn test_get_database_schemas() {
    let client = get_test_client().await;
    
    // Get the first database
    let databases = client.list_databases().await.expect("Should list databases");
    if databases.is_empty() {
        println!("No databases available, skipping test");
        return;
    }
    
    let db_id = databases[0].id;
    let schemas = client.get_database_schemas(db_id).await;
    
    assert!(schemas.is_ok(), "Should be able to get database schemas");
    let schemas = schemas.unwrap();
    
    // Most databases have at least one schema (public, dbo, etc.)
    if !schemas.is_empty() {
        assert!(!schemas[0].is_empty(), "Schema name should not be empty");
    }
}

#[tokio::test]
async fn test_sync_database_schema() {
    let client = get_test_client().await;
    
    // Get the first database
    let databases = client.list_databases().await.expect("Should list databases");
    if databases.is_empty() {
        println!("No databases available, skipping test");
        return;
    }
    
    let db_id = databases[0].id;
    let sync_result = client.sync_database_schema(db_id).await;
    
    assert!(sync_result.is_ok(), "Should be able to sync database schema");
    let sync_result = sync_result.unwrap();
    assert!(!sync_result.id.is_empty(), "Sync should return a task ID");
    assert!(!sync_result.status.is_empty(), "Sync should have a status");
}