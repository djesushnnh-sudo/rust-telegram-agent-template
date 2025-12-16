# Database Module

This project includes a simple SQLite database adapter for storing bot data.

## Structure

```
src/
├── database/
│   ├── mod.rs         # Main database adapter with connection logic
│   ├── models.rs      # Data models (User struct)
│   ├── migrations.rs  # Database schema migration system
│   └── errors.rs      # Custom error types
└── lib.rs             # Library exports
```

## Usage

### Basic Connection

```rust
use telegram_bot_template::database::Database;

// Initialize database (creates file if it doesn't exist)
let db = Database::new("bot.db").await?;
```

### User Operations

```rust
// Create a user
db.create_user("john_doe", 12345).await?;

// Get a user by Telegram ID
if let Some(user) = db.get_user(12345).await? {
    println!("Found: {}", user.username);
}

// Get all users
let users = db.get_all_users().await?;

// Delete a user
let deleted = db.delete_user(12345).await?;
```

## Features

- **Auto-initialization**: Tables are created automatically via migrations
- **Simple API**: Basic CRUD operations for user management
- **Migration system**: Schema versioning for safe database updates
- **Custom errors**: Granular error types with `thiserror`
- **Logging**: Debug and info logging for database operations
- **Indexed queries**: Optimized lookups with database indexes
- **Connection pooling**: Uses SQLx connection pool for performance
- **Async/await**: Fully async for use with Tokio
- **Type safety**: Strongly typed models with Serde support

## Running the Example

```bash
# Run the database usage example
cargo run --example database_usage
```

## Database Schema

### Users Table
- `id`: Auto-incrementing primary key
- `telegram_id`: Unique Telegram user ID
- `username`: User's display name
- `created_at`: Timestamp of user creation

## Dependencies

- `sqlx`: Async SQL toolkit with SQLite support
- `serde`: Serialization for data models
- `anyhow`: General error handling
- `thiserror`: Custom error types
- `log`: Logging framework
- `tokio`: Async runtime

## Advanced Features

### Migration System
The database uses a simple migration system to handle schema changes:

```rust
// Migrations are applied automatically on connection
let db = Database::new("bot.db").await?; // Runs all pending migrations
```

### Custom Error Types
Use `DatabaseError` for more specific error handling:

```rust
use telegram_bot_template::database::errors::{DatabaseError, DatabaseResult};

// More granular error handling
match db.get_user(12345).await {
    Ok(Some(user)) => println!("Found: {}", user.username),
    Ok(None) => println!("User not found"),
    Err(DatabaseError::ConnectionFailed(e)) => println!("DB connection issue: {}", e),
    Err(e) => println!("Other error: {}", e),
}
```

### Logging
Enable logging to see database operations:

```rust
env_logger::init(); // Shows debug info about DB operations
```