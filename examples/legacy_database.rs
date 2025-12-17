use anyhow::Result;
use telegram_bot_template::database::Database;

/// Example demonstrating backward compatibility with legacy Database API
#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("🔄 Legacy Database API Example");
    println!("==============================\n");
    
    // Legacy API still works
    let db = Database::new("sqlite::memory:").await?;
    
    // Test basic operations
    db.create_user("legacy_user", 99999).await?;
    if let Some(user) = db.get_user(99999).await? {
        println!("✅ Legacy API: Created and retrieved user: {}", user.username);
    }
    
    let stats = db.get_stats().await?;
    println!("📊 Legacy API stats: {} users", stats.users_count);
    
    // Access the underlying provider if needed
    let provider = db.provider();
    let stats2 = provider.get_stats().await?;
    println!("📊 Provider stats: {} users", stats2.users_count);
    
    println!("\n🎉 Legacy API working correctly!");
    
    Ok(())
}