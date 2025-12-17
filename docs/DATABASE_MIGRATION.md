# Database Migration Guide

This guide helps you migrate from the old database API to the new modular database system.

## What Changed?

The database module has been refactored to support multiple storage backends through a common `DatabaseProvider` trait. This makes the bot more flexible and testable while maintaining backward compatibility.

### Before (Legacy API)
```rust
use telegram_bot_template::database::Database;

let db = Database::new("sqlite:bot.db").await?;
db.create_user("alice", 12345).await?;
```

### After (New API)
```rust
use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};

let config = DatabaseConfig::SQLite {
    database_url: "sqlite:bot.db".to_string(),
};
let db = DatabaseManager::create_provider(config).await?;
db.create_user("alice", 12345).await?;
```

## Migration Steps

### Step 1: Update Your Dependencies (Optional)

The new system doesn't require any new dependencies. All existing dependencies continue to work.

### Step 2: Choose Your Migration Path

#### Option A: Keep Using Legacy API (Easiest)
Your existing code continues to work without changes:

```rust
use telegram_bot_template::database::Database;

let db = Database::new("sqlite:bot.db").await?;
// All your existing code works unchanged
```

#### Option B: Migrate to New API (Recommended)
Update your code to use the new provider system:

```rust
use telegram_bot_template::database::{DatabaseManager, DatabaseConfig};

// From config
let config = Config::from_env()?;
let db_config = config.get_database_config();
let db = DatabaseManager::create_provider(db_config).await?;

// Or directly
let db = DatabaseManager::create_from_url("sqlite:bot.db").await?;
```

### Step 3: Update Configuration (Optional)

Add new environment variables to your `.env` file:

```bash
# Method 1: Explicit provider (recommended)
DATABASE_PROVIDER=sqlite
DATABASE_URL=sqlite:bot.db

# Method 2: URL-based (provider inferred)
DATABASE_URL=sqlite:bot.db
```

### Step 4: Update Your Application Code

#### Before
```rust
use telegram_bot_template::database::Database;

async fn setup_database() -> Result<Database> {
    let db = Database::new("sqlite:bot.db").await?;
    Ok(db)
}
```

#### After
```rust
use telegram_bot_template::database::{DatabaseManager, DatabaseProvider};
use std::sync::Arc;

async fn setup_database() -> Result<Arc<dyn DatabaseProvider>> {
    let db = DatabaseManager::create_from_url("sqlite:bot.db").await?;
    Ok(db)
}
```

## Benefits of Migration

### 1. Multiple Storage Backends
```rust
// SQLite for production
let db = DatabaseManager::create_from_url("sqlite:bot.db").await?;

// In-memory for testing
let db = DatabaseManager::create_from_url("memory:").await?;

// No database for stateless bots
let db = DatabaseManager::create_from_url("none:").await?;
```

### 2. Better Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    async fn setup_test_db() -> Arc<dyn DatabaseProvider> {
        DatabaseManager::create_provider(DatabaseConfig::InMemory).await.unwrap()
    }
    
    #[tokio::test]
    async fn test_user_operations() {
        let db = setup_test_db().await;
        // Fast, isolated tests with in-memory storage
    }
}
```

### 3. Configuration-Driven Setup
```rust
// Users can switch providers via environment variables
let config = Config::from_env()?;
let db = DatabaseManager::create_provider(config.get_database_config()).await?;
```

## Common Migration Patterns

### Pattern 1: Dependency Injection
```rust
// Before
struct BotService {
    db: Database,
}

// After
struct BotService {
    db: Arc<dyn DatabaseProvider>,
}

impl BotService {
    pub fn new(db: Arc<dyn DatabaseProvider>) -> Self {
        Self { db }
    }
}
```

### Pattern 2: Factory Functions
```rust
// Before
async fn create_bot_service() -> Result<BotService> {
    let db = Database::new("sqlite:bot.db").await?;
    Ok(BotService::new(db))
}

// After
async fn create_bot_service(config: &Config) -> Result<BotService> {
    let db = DatabaseManager::create_provider(config.get_database_config()).await?;
    Ok(BotService::new(db))
}
```

### Pattern 3: Test Helpers
```rust
// Create test utilities
#[cfg(test)]
pub async fn create_test_database() -> Arc<dyn DatabaseProvider> {
    DatabaseManager::create_provider(DatabaseConfig::InMemory).await.unwrap()
}

#[cfg(test)]
pub async fn create_noop_database() -> Arc<dyn DatabaseProvider> {
    DatabaseManager::create_provider(DatabaseConfig::None).await.unwrap()
}
```

## Troubleshooting

### Issue: Compilation Errors After Migration

**Problem**: Type mismatches when switching from `Database` to `Arc<dyn DatabaseProvider>`

**Solution**: Update your type annotations:
```rust
// Before
fn process_data(db: &Database) -> Result<()> { ... }

// After
fn process_data(db: &dyn DatabaseProvider) -> Result<()> { ... }
// Or
fn process_data(db: Arc<dyn DatabaseProvider>) -> Result<()> { ... }
```

### Issue: Configuration Not Working

**Problem**: Database provider not being selected correctly

**Solution**: Check your environment variables:
```bash
# Debug your configuration
DATABASE_PROVIDER=sqlite
DATABASE_URL=sqlite:bot.db

# Or use URL-based configuration
DATABASE_URL=sqlite:bot.db  # Will infer SQLite provider
```

### Issue: Tests Running Slowly

**Problem**: Tests are still using SQLite instead of in-memory storage

**Solution**: Use in-memory provider for tests:
```rust
#[tokio::test]
async fn test_something() {
    let db = DatabaseManager::create_provider(DatabaseConfig::InMemory).await.unwrap();
    // Your test code here
}
```

## Rollback Plan

If you need to rollback to the old system:

1. **Keep using the legacy API**: The `Database` struct still works exactly as before
2. **No code changes needed**: Your existing code continues to work
3. **No data migration needed**: The underlying SQLite database format is unchanged

## Next Steps

1. **Start with legacy API**: Keep your existing code working
2. **Migrate gradually**: Update one module at a time to use the new API
3. **Add testing**: Use in-memory provider for faster, isolated tests
4. **Consider other providers**: Evaluate PostgreSQL or no-op providers for your use case

## Questions?

- Check the [DATABASE.md](../DATABASE.md) for detailed API documentation
- Run `cargo run --example database_providers` to see all providers in action
- Look at the test files for usage examples