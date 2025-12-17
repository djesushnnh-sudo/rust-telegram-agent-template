# Optional Database Configuration

The Rust Telegram Bot Template now supports **optional database configuration**, making it easier to get started while still providing full persistence capabilities when needed.

## 🚀 Quick Start (No Database Required)

The bot works perfectly without any database configuration! Simply:

1. Copy `.env.example` to `.env`
2. Set your `TELEGRAM_BOT_TOKEN`
3. Run the bot

```bash
cp .env.example .env
# Edit .env and set TELEGRAM_BOT_TOKEN=your_bot_token_here
cargo run
```

The bot will run in **memory-only mode** - all functionality works, but data won't persist across restarts.

## 🗄️ Enabling Database (Optional)

When you're ready for persistent storage, simply uncomment and configure the database settings in your `.env` file:

```env
# Enable SQLite database
DATABASE_PROVIDER=sqlite
DATABASE_URL=sqlite:./bot_database.db
```

## Configuration Options

### 1. No Database (Default)
```env
# No database configuration needed
# Bot runs in memory-only mode
```

### 2. SQLite Database (Recommended)
```env
DATABASE_PROVIDER=sqlite
DATABASE_URL=sqlite:./bot_database.db
```

### 3. In-Memory Database (Testing)
```env
DATABASE_PROVIDER=memory
DATABASE_URL=memory:
```

### 4. Explicitly Disable Database
```env
DATABASE_PROVIDER=none
DATABASE_URL=none:
```

## What Works Without Database?

**Everything!** The bot is fully functional without a database:

- ✅ All commands work normally
- ✅ User sessions and AI routing
- ✅ Group management
- ✅ Message forwarding
- ✅ All bot features

**What you lose without database:**
- ❌ Data doesn't persist across bot restarts
- ❌ No historical data storage
- ❌ User preferences reset on restart

## Migration Path

You can easily migrate from no-database to database mode:

1. **Start without database** - Get your bot working quickly
2. **Add database later** - When you need persistence, just update your `.env`
3. **No code changes needed** - The bot automatically adapts

### Example Migration

**Step 1: Start simple**
```env
TELEGRAM_BOT_TOKEN=your_token_here
# No database configuration
```

**Step 2: Add persistence later**
```env
TELEGRAM_BOT_TOKEN=your_token_here
DATABASE_PROVIDER=sqlite
DATABASE_URL=sqlite:./bot_database.db
```

## Technical Details

### How It Works

The bot uses a **provider pattern** for database operations:

- **No Database**: `NoOpProvider` - all operations succeed but do nothing
- **SQLite**: `SqliteProvider` - full persistence with SQLite
- **Memory**: `InMemoryProvider` - in-memory storage for testing

### Configuration Resolution

The bot determines which database provider to use in this order:

1. **`DATABASE_PROVIDER`** environment variable (if set)
2. **`DATABASE_URL`** prefix (if set):
   - `sqlite:` → SQLite provider
   - `memory:` → Memory provider  
   - `none:` → No-op provider
3. **Default**: No database (memory-only mode)

### Code Example

```rust
use telegram_bot_template::config::Config;
use telegram_bot_template::state::AppState;

// Load configuration
let config = Config::from_env()?;

// Create state (automatically handles database configuration)
let state = AppState::from_config(&config).await?;

// Use normally - works with or without database
state.register_group(chat_id, "My Group".to_string()).await;
```

## Best Practices

### For Development
- Start without database for quick iteration
- Use in-memory database for testing
- Add SQLite when you need persistence

### For Production
- Use SQLite for most use cases
- Consider PostgreSQL for high-scale deployments
- Always backup your database files

### For Templates/Examples
- Default to no database for easy setup
- Provide clear instructions for enabling database
- Include examples for all provider types

## Backward Compatibility

This change is **100% backward compatible**:

- Existing configurations continue to work
- Legacy `Database` struct still supported
- All existing database code unchanged
- Migration path is optional and gradual

## Examples

See the `examples/` directory for complete examples:

- `examples/test_no_database.rs` - Running without database
- `examples/database_providers.rs` - All provider types
- `examples/legacy_database.rs` - Legacy API compatibility

## Troubleshooting

### Bot starts but data doesn't persist
- Check if `DATABASE_URL` is set in your `.env` file
- Verify the database file path is writable
- Check logs for database initialization messages

### Database connection errors
- Ensure the database file directory exists
- Check file permissions
- Verify the `DATABASE_URL` format is correct

### Migration from memory-only to database
- Stop the bot
- Add database configuration to `.env`
- Restart the bot
- Previous in-memory data will be lost (this is expected)

## Summary

The optional database feature makes the Rust Telegram Bot Template more accessible:

- **Beginners**: Can start immediately without database setup
- **Developers**: Can iterate quickly in memory-only mode
- **Production**: Can add persistence when needed
- **Everyone**: Benefits from a simpler, more flexible architecture

The bot is designed to work great both with and without a database, giving you the flexibility to choose the right approach for your use case.