# Database Module

This project includes a modular, database-agnostic system for storing bot data. The architecture supports multiple storage backends through a common interface.

## Architecture

```
src/
├── database/
│   ├── mod.rs              # Main module with legacy compatibility
│   ├── provider.rs         # DatabaseProvider trait definition
│   ├── manager.rs          # DatabaseManager for provider creation
│   ├── providers/
│   │   ├── mod.rs          # Provider module exports
│   │   ├── sqlite.rs       # SQLite implementation
│   │   └── memory.rs       # In-memory implementation
│   ├── models.rs           # Data models (User, ManagedGroup, etc.)
│   ├── migrations.rs       # Database schema migration system
│   └── errors.rs           # Custom error types
└── lib.rs                  # Library exports
```

## Usage

### Modern API (Recommended)

```rust
use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};

// SQLite provider
let config = DatabaseConfig::SQLite {
    database_url: "sqlite:bot.db".to_string(),
};
let db = DatabaseManager::create_provider(config).await?;

// In-memory provider (for testing)
let config = DatabaseConfig::InMemory;
let db = DatabaseManager::create_provider(config).await?;

// No-op provider (disable database)
let config = DatabaseConfig::None;
let db = DatabaseManager::create_provider(config).await?;
```

### URL-Based Creation

```rust
use telegram_bot_template::database::DatabaseManager;

// Create from URL string
let db = DatabaseManager::create_from_url("sqlite:bot.db").await?;
let db = DatabaseManager::create_from_url("memory:").await?;
let db = DatabaseManager::create_from_url("none:").await?;
```

### Legacy API (Backward Compatible)

```rust
use telegram_bot_template::database::Database;

// Still works for backward compatibility
let db = Database::new("sqlite:bot.db").await?;
```

### Database Operations

All providers implement the same `DatabaseProvider` trait:

```rust
// User operations
db.create_user("john_doe", 12345).await?;
if let Some(user) = db.get_user(12345).await? {
    println!("Found: {}", user.username);
}
let users = db.get_all_users().await?;
let deleted = db.delete_user(12345).await?;

// Group management
db.store_managed_group(chat_id, "My Group").await?;
let groups = db.get_managed_groups().await?;
db.remove_managed_group(chat_id).await?;

// Message forwarding
db.store_forwarded_message(admin_msg_id, original_chat_id, original_msg_id).await?;
if let Some((chat_id, msg_id)) = db.get_original_message(admin_msg_id).await? {
    println!("Original message: {} in {}", msg_id.0, chat_id.0);
}

// User preferences
db.set_user_ai_enabled(user_id, true).await?;
let ai_enabled = db.get_user_ai_enabled(user_id).await?;

// Statistics and health
let stats = db.get_stats().await?;
db.health_check().await?;
```

## Available Providers

### SQLite Provider
- **Use case**: Production deployments, persistent storage
- **Features**: File-based storage, ACID transactions, migrations
- **Configuration**: `sqlite:path/to/database.db`

### In-Memory Provider
- **Use case**: Testing, development, temporary storage
- **Features**: Fast access, no persistence, thread-safe
- **Configuration**: `memory:` or `mem:`

### No-Op Provider
- **Use case**: Disable database functionality entirely
- **Features**: All operations are no-ops, returns empty results
- **Configuration**: `none:` or `noop:`

## Configuration

### Environment Variables

```bash
# Method 1: Explicit provider type
DATABASE_PROVIDER=sqlite
DATABASE_URL=sqlite:bot.db

# Method 2: URL-based (provider inferred)
DATABASE_URL=sqlite:bot.db    # SQLite
DATABASE_URL=memory:          # In-memory
DATABASE_URL=none:            # No-op
```

### Programmatic Configuration

```rust
use telegram_bot_template::config::Config;

let config = Config::from_env()?;
let db_config = config.get_database_config();
let db = DatabaseManager::create_provider(db_config).await?;
```

## Features

- **Database-agnostic**: Switch providers without changing application code
- **Auto-initialization**: Tables/storage created automatically
- **Migration system**: Schema versioning for safe database updates (SQLite)
- **Thread-safe**: All providers support concurrent access
- **Async/await**: Fully async for use with Tokio
- **Type safety**: Strongly typed models with Serde support
- **Backward compatible**: Legacy `Database` struct still works
- **Testing friendly**: In-memory provider for unit tests

## Examples

```bash
# Run the provider comparison example
cargo run --example database_providers
```

## Database Schema (SQLite Provider)

### Users Table
- `id`: Auto-incrementing primary key
- `telegram_id`: Unique Telegram user ID
- `username`: User's display name
- `created_at`: Timestamp of user creation

### Managed Groups Table
- `chat_id`: Telegram chat ID (primary key)
- `group_name`: Display name for the group
- `created_at`: Timestamp of group creation
- `updated_at`: Timestamp of last update

### Forwarded Messages Table
- `admin_msg_id`: Admin message ID (primary key)
- `original_chat_id`: Original chat where message came from
- `original_msg_id`: Original message ID
- `created_at`: Timestamp of message forwarding

### User Preferences Table
- `user_id`: Telegram user ID (primary key)
- `ai_enabled`: Whether AI is enabled for this user
- `language`: User's preferred language
- `created_at`: Timestamp of preference creation
- `updated_at`: Timestamp of last update

## Migration Guide

### From Legacy Database API

```rust
// Old way
use telegram_bot_template::database::Database;
let db = Database::new("sqlite:bot.db").await?;

// New way (recommended)
use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};
let config = DatabaseConfig::SQLite { database_url: "sqlite:bot.db".to_string() };
let db = DatabaseManager::create_provider(config).await?;

// Or using URL
let db = DatabaseManager::create_from_url("sqlite:bot.db").await?;
```

### Adding New Providers

To add a new database provider (e.g., PostgreSQL):

1. Create `src/database/providers/postgres.rs`
2. Implement `DatabaseProvider` trait
3. Add to `src/database/providers/mod.rs`
4. Update `DatabaseManager::create_provider()`
5. Add configuration support in `DatabaseConfig`

## Dependencies

- `sqlx`: Async SQL toolkit with SQLite support
- `serde`: Serialization for data models
- `anyhow`: General error handling
- `thiserror`: Custom error types
- `log`: Logging framework
- `tokio`: Async runtime
- `async-trait`: Async trait support
- `dashmap`: Concurrent HashMap (for in-memory provider)

## Best Practices

### For Template Users

1. **Start with SQLite**: Good for most use cases, easy deployment
2. **Use in-memory for tests**: Fast, isolated, no cleanup needed
3. **Consider no-op for stateless bots**: When persistence isn't needed
4. **Plan for growth**: Easy to migrate to PostgreSQL later

### For Development

1. **Use the trait**: Always code against `DatabaseProvider` trait
2. **Test with multiple providers**: Ensure your code works with all backends
3. **Handle errors gracefully**: Database operations can fail
4. **Use connection pooling**: SQLx provides this automatically

### Example Test Setup

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};
    
    async fn setup_test_db() -> Arc<dyn DatabaseProvider> {
        DatabaseManager::create_provider(DatabaseConfig::InMemory).await.unwrap()
    }
    
    #[tokio::test]
    async fn test_user_operations() {
        let db = setup_test_db().await;
        // Your tests here...
    }
}
```