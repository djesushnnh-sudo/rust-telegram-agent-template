use anyhow::Result;
use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};

/// Example demonstrating different database providers
/// 
/// This example shows how to use the database abstraction layer
/// to switch between different storage backends.
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("🗄️ Database Providers Example");
    println!("=============================\n");
    
    // Example 1: SQLite provider
    println!("1. SQLite Provider");
    println!("------------------");
    let sqlite_config = DatabaseConfig::SQLite {
        database_url: "sqlite::memory:".to_string(), // Use in-memory for example
    };
    let sqlite_db = DatabaseManager::create_provider(sqlite_config).await?;
    
    // Test basic operations
    sqlite_db.create_user("alice", 12345).await?;
    if let Some(user) = sqlite_db.get_user(12345).await? {
        println!("✅ SQLite: Created and retrieved user: {}", user.username);
    }
    
    let stats = sqlite_db.get_stats().await?;
    println!("📊 SQLite stats: {} users", stats.users_count);
    println!();
    
    // Example 2: In-memory provider
    println!("2. In-Memory Provider");
    println!("---------------------");
    let memory_config = DatabaseConfig::InMemory;
    let memory_db = DatabaseManager::create_provider(memory_config).await?;
    
    memory_db.create_user("bob", 67890).await?;
    if let Some(user) = memory_db.get_user(67890).await? {
        println!("✅ Memory: Created and retrieved user: {}", user.username);
    }
    
    let stats = memory_db.get_stats().await?;
    println!("📊 Memory stats: {} users", stats.users_count);
    println!();
    
    // Example 3: No-op provider
    println!("3. No-Op Provider");
    println!("-----------------");
    let noop_config = DatabaseConfig::None;
    let noop_db = DatabaseManager::create_provider(noop_config).await?;
    
    noop_db.create_user("charlie", 11111).await?;
    let user = noop_db.get_user(11111).await?;
    if user.is_some() {
        println!("❌ No-op: User should be None: {:?}", user);
    } else {
        println!("✅ No-op: Correctly returned None for user lookup");
    }
    
    let stats = noop_db.get_stats().await?;
    println!("📊 No-op stats: {} users", stats.users_count);
    println!();
    
    // Example 4: Using URL-based creation
    println!("4. URL-Based Provider Creation");
    println!("------------------------------");
    
    let providers = vec![
        ("sqlite::memory:", "SQLite in-memory"),
        ("memory:", "In-memory provider"),
        ("none:", "No-op provider"),
    ];
    
    for (url, description) in providers {
        let db = DatabaseManager::create_from_url(url).await?;
        db.create_user("test_user", 99999).await?;
        let stats = db.get_stats().await?;
        println!("✅ {}: {} users", description, stats.users_count);
    }
    
    println!("\n🎉 All database providers working correctly!");
    
    Ok(())
}