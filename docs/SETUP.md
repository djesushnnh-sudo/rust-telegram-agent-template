# Setup Guide

This guide provides detailed instructions for setting up and configuring the Rust Telegram Bot Template.

## Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.70 or later**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository
- **A Telegram account**: To create and test your bot

## Step 1: Create a Telegram Bot

1. **Open Telegram** and search for [@BotFather](https://t.me/BotFather)
2. **Start a conversation** with BotFather by sending `/start`
3. **Create a new bot** by sending `/newbot`
4. **Choose a name** for your bot (this is the display name users will see)
5. **Choose a username** for your bot (must end with 'bot', e.g., `my_awesome_bot`)
6. **Save the bot token** - BotFather will provide a token like `123456789:ABCdefGHIjklMNOpqrsTUVwxyz`

⚠️ **Important**: Keep your bot token secure! Never commit it to version control or share it publicly.

## Step 2: Clone and Setup the Project

```bash
# Clone the repository
git clone <repository-url>
cd telegram-bot-template

# Copy the environment template
cp .env.example .env

# Open the .env file in your preferred editor
nano .env  # or vim, code, etc.
```

## Step 3: Configure Environment Variables

Edit your `.env` file with the following configuration:

### Required Variables

```bash
# Your bot token from BotFather (REQUIRED)
TELEGRAM_BOT_TOKEN=your_bot_token_here
```

### Optional Variables

```bash
# Logging level - controls how much information is logged
# Options: debug, info, warn, error
# Default: info
RUST_LOG=info

# Webhook configuration (advanced users only)
# Leave empty to use polling mode (recommended for development)
BOT_WEBHOOK_URL=

# Port for webhook server (only used if webhook URL is set)
# Default: 8080
BOT_PORT=8080

# AI functionality toggle
# Set to true when you're ready to integrate AI features
# Default: false
AI_ENABLED=false
```

## Step 4: Build and Run

```bash
# Build the project (this may take a few minutes the first time)
cargo build

# Run the bot
cargo run
```

If everything is configured correctly, you should see output similar to:

```
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Starting Telegram Bot Template
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Configuration loaded successfully
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Bot authenticated successfully: @your_bot_username
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Registered 4 commands
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Available commands: start, help, echo, status
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Bot initialization complete, starting main loop
[2024-01-01T12:00:00Z INFO  telegram_bot_template] Starting bot in polling mode
```

## Step 5: Test Your Bot

1. **Find your bot** on Telegram by searching for its username
2. **Start a conversation** by clicking "Start" or sending `/start`
3. **Try the built-in commands**:
   - `/start` - Welcome message
   - `/help` - List available commands
   - `/echo hello world` - Echo back "hello world"
   - `/status` - Show bot status

## Development Setup

### IDE Configuration

For the best development experience, we recommend:

- **VS Code** with the Rust Analyzer extension
- **IntelliJ IDEA** with the Rust plugin
- **Vim/Neovim** with rust.vim and coc-rust-analyzer

### Useful Development Commands

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code without building
cargo check

# Format code
cargo fmt

# Run clippy for linting
cargo clippy

# Build for release (optimized)
cargo build --release
```

### Hot Reloading During Development

For faster development cycles, you can use `cargo-watch`:

```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild and restart on file changes
cargo watch -x run
```

## Troubleshooting

### Common Issues

#### "Failed to authenticate bot"
- **Cause**: Invalid or missing bot token
- **Solution**: Double-check your `TELEGRAM_BOT_TOKEN` in the `.env` file
- **Verify**: Make sure there are no extra spaces or characters

#### "Configuration error: Missing variable TELEGRAM_BOT_TOKEN"
- **Cause**: The `.env` file is missing or the variable is not set
- **Solution**: Ensure you've copied `.env.example` to `.env` and set the token

#### "Permission denied" or "Address already in use"
- **Cause**: Port conflict (usually only in webhook mode)
- **Solution**: Change the `BOT_PORT` in your `.env` file or stop other services using that port

#### Bot doesn't respond to commands
- **Cause**: Bot might not be started, or there might be a network issue
- **Solution**: 
  1. Check that the bot is running and shows "Starting bot in polling mode"
  2. Verify your internet connection
  3. Try restarting the bot

### Debug Mode

To get more detailed logging for troubleshooting:

```bash
# Run with debug logging
RUST_LOG=debug cargo run
```

This will show detailed information about:
- Configuration loading
- Bot authentication
- Command registration
- Message processing
- API calls to Telegram

### Environment Variable Validation

The bot validates all configuration on startup. If you see validation errors:

1. **Check the format** of your bot token (should contain a colon)
2. **Verify log level** is one of: debug, info, warn, error
3. **Check webhook URL** starts with http:// or https:// (if provided)
4. **Ensure port number** is 1024 or higher (if provided)

## Production Deployment

### Environment Variables in Production

Never use `.env` files in production. Instead, set environment variables directly:

```bash
# Example for systemd service
Environment=TELEGRAM_BOT_TOKEN=your_actual_token
Environment=RUST_LOG=info
Environment=AI_ENABLED=false
```

### Docker Deployment

```bash
# Build the Docker image
docker build -t telegram-bot-template .

# Run with environment variables
docker run -e TELEGRAM_BOT_TOKEN=your_token telegram-bot-template
```

### Security Considerations

1. **Never commit** your `.env` file to version control
2. **Use secure storage** for your bot token in production (e.g., AWS Secrets Manager, HashiCorp Vault)
3. **Rotate your bot token** periodically using BotFather
4. **Monitor logs** for suspicious activity
5. **Use HTTPS** for webhook URLs in production

## Next Steps

Once your bot is running successfully:

1. **Read the [Commands Guide](COMMANDS.md)** to understand the built-in commands
2. **Check the [Extension Guide](EXTENDING.md)** to learn how to add new features
3. **Explore the source code** to understand the architecture
4. **Add your own commands** following the examples in `src/commands/`

## Getting Help

If you encounter issues not covered in this guide:

1. **Check the logs** with `RUST_LOG=debug` for detailed error information
2. **Review the source code** - it's well-documented with inline comments
3. **Look at the test files** for usage examples
4. **Consult the Telegram Bot API documentation** at https://core.telegram.org/bots/api

Remember: This template is designed to be modified and extended for your specific needs. Don't hesitate to customize it!