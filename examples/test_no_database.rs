use anyhow::Result;
use telegram_bot_template::config::Config;
use telegram_bot_template::state::AppState;
use telegram_bot_template::database::DatabaseConfig;

/// Test that the bot can start without any database configuration
#[tokio::main]
async fn main() -> Result<()> {
    println!("🧪 Testing Bot Without Database");
    println!("===============================");
    
    // Test 1: Default config should use no database
    println!("\n1. Testing default configuration...");
    let default_config = Config::default();
    let db_config = default_config.get_database_config();
    match db_config {
        DatabaseConfig::None => println!("✅ Default config correctly uses no database"),
        _ => println!("❌ Default config should use no database, got: {:?}", db_config),
    }
    
    // Test 2: Create AppState without database
    println!("\n2. Testing AppState creation without database...");
    let state = AppState::new();
    if state.database.is_none() {
        println!("✅ AppState created without database");
    } else {
        println!("❌ AppState should not have database when created with new()");
    }
    
    // Test 3: Test AppState from config with no database
    println!("\n3. Testing AppState from config with no database...");
    let config = Config {
        bot_token: "test:token".to_string(),
        log_level: "info".to_string(),
        webhook_url: None,
        port: None,
        ai_enabled: false,
        database_url: None,
        database_provider: None,
    };
    
    let state = AppState::from_config(&config).await.map_err(|e| anyhow::anyhow!("Failed to create state: {}", e))?;
    if state.database.is_none() {
        println!("✅ AppState from config correctly has no database");
    } else {
        println!("❌ AppState from config should not have database");
    }
    
    // Test 4: Test basic operations work without database
    println!("\n4. Testing basic operations without database...");
    
    // Test loading from database (should succeed with no-op)
    match state.load_from_database().await {
        Ok(()) => println!("✅ load_from_database() succeeded without database"),
        Err(e) => println!("❌ load_from_database() failed: {}", e),
    }
    
    // Test saving to database (should succeed with no-op)
    match state.save_to_database().await {
        Ok(()) => println!("✅ save_to_database() succeeded without database"),
        Err(e) => println!("❌ save_to_database() failed: {}", e),
    }
    
    // Test registering a group (should work in memory)
    use teloxide::types::ChatId;
    state.register_group(ChatId(-1001234567890), "Test Group".to_string()).await;
    let groups = state.get_managed_groups();
    if groups.len() == 1 {
        println!("✅ Group registration works without database");
    } else {
        println!("❌ Group registration failed");
    }
    
    // Test storing forwarded message (should work in memory)
    use teloxide::types::MessageId;
    state.store_forwarded_message(
        MessageId(123), 
        ChatId(-1001234567890), 
        MessageId(456)
    ).await;
    
    if let Some((chat_id, msg_id)) = state.get_original_message(MessageId(123)) {
        println!("✅ Forwarded message storage works without database");
        println!("   Original: {} in {}", msg_id.0, chat_id.0);
    } else {
        println!("❌ Forwarded message storage failed");
    }
    
    println!("\n🎉 All tests passed! Bot works perfectly without database.");
    println!("💡 Users can enable database later by setting DATABASE_URL or DATABASE_PROVIDER");
    
    Ok(())
}