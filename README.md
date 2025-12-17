# Rust Telegram Bot Template

A clean, extensible foundation for building Telegram bots in Rust. This template provides a well-structured starting point with a robust command system, comprehensive configuration management, and designated areas for AI integration.

## ✨ Features

- **🏗️ Production-Ready Architecture**: Based on patterns from real production bots with concurrent state management
- **⚡ Modern Command System**: Type-safe command handling using BotCommands derive macro
- **⚙️ Simple Configuration Management**: Easy setup with .env file for all environments
- **🗄️ SQLite Database Integration**: Persistent storage with automatic migrations and state management
- **🤖 Enhanced AI Router**: Sophisticated message routing with session management and conversation context
- **🔄 Concurrent State Management**: Thread-safe state using DashMap for high-performance concurrent access
- **🛡️ Comprehensive Error Handling**: Production-grade error handling with graceful degradation
- **📚 Extensive Documentation**: Complete guides, inline comments, and real-world examples
- **🧪 Property-Based Testing**: Comprehensive test coverage with correctness validation
- **🐳 Docker Support**: Multi-stage builds with development and production configurations
- **🔒 Security & Performance**: Input validation, rate limiting, and optimized for production use

## 🚀 Quick Start

### 1. Create a Telegram Bot
- Message [@BotFather](https://t.me/BotFather) on Telegram
- Use `/newbot` to create a new bot
- Choose a name and username for your bot
- Save the bot token provided (keep it secure!)

### 2. Set up the Project

**On Linux/macOS:**
```bash
# Clone the repository
git clone <repository-url>
cd rust-telegram-agent-template

# Copy environment template
cp .env.example .env
```

**On Windows:**
```cmd
# Clone the repository
git clone <repository-url>
cd rust-telegram-agent-template

# Copy environment template
copy .env.example .env
```

### 3. Configure Your Bot
Edit `.env` and add your bot token:
```bash
# Linux/Mac
nano .env

# Windows
notepad .env
```

Replace `your_bot_token_here` with your actual bot token:
```bash
TELEGRAM_BOT_TOKEN=123456789:ABCdefGHIjklMNOpqrsTUVwxyz
```

### 4. Build and Run
```bash
# Build the project
cargo build

# Run the bot
cargo run
```

### 5. Test Your Bot
- Find your bot on Telegram
- Send `/start` to begin
- Try `/help` to see available commands
- Use `/echo hello` to test functionality

**🎉 Your bot is now running!** See the [Setup Guide](docs/SETUP.md) for detailed configuration options.

## 📁 Project Structure

```
telegram-bot-template/
├── src/
│   ├── main.rs              # Application entry point and orchestration
│   ├── config.rs            # Configuration management with validation
│   ├── bot.rs               # Core bot service and Telegram API integration
│   ├── error.rs             # Error types and recovery mechanisms
│   ├── lib.rs               # Library exports and module declarations
│   ├── commands/            # Command system and handlers
│   │   ├── mod.rs           # Command router and registration
│   │   ├── handler.rs       # Command handler trait definition
│   │   ├── start.rs         # Welcome command implementation
│   │   ├── help.rs          # Help system with dynamic command listing
│   │   ├── echo.rs          # Echo command for testing
│   │   └── status.rs        # Bot status and health information
│   ├── ai/                  # AI integration system (extensible)
│   │   ├── mod.rs           # AI router and decision logic
│   │   └── processor.rs     # AI processing placeholder with clear extension points
│   └── database/            # Database integration (optional)
│       ├── mod.rs           # Database connection and initialization
│       ├── models.rs        # Data models and database interactions
│       ├── migrations.rs    # Database schema migrations
│       └── errors.rs        # Database-specific error handling
├── tests/                   # Comprehensive test suite
│   ├── *_property_tests.rs  # Property-based tests for correctness validation
│   └── property/            # Property test utilities and generators
├── docs/                    # Comprehensive documentation
│   ├── SETUP.md             # Detailed setup and configuration guide
│   ├── COMMANDS.md          # Command system documentation and examples
│   └── EXTENDING.md         # Guide for adding new features and integrations
├── .env.example             # Environment variables template with examples
├── Cargo.toml               # Dependencies and project configuration
├── Dockerfile               # Docker containerization
├── docker-compose.yml       # Development environment setup
└── README.md               # This comprehensive guide
```

## ⚙️ Configuration

The bot uses environment variables for configuration. Copy `.env.example` to `.env` and customize:

**Linux/macOS:** `cp .env.example .env`  
**Windows:** `copy .env.example .env`

### Required Configuration
```bash
# Your bot token from @BotFather (REQUIRED)
TELEGRAM_BOT_TOKEN=your_bot_token_here

# SQLite database for persistent storage (REQUIRED)
DATABASE_URL=sqlite:./bot_database.db
```

### Simple Configuration
The bot uses a single `.env` file for all configuration. This keeps setup simple and beginner-friendly.

### Optional Configuration
```bash
# Logging configuration
RUST_LOG=info                    # Logging level: debug, info, warn, error

# Network configuration
BOT_WEBHOOK_URL=                 # Webhook URL (leave empty for polling mode)
BOT_PORT=8080                    # Port for webhook server

# Feature toggles
AI_ENABLED=false                 # Enable AI processing capabilities

# External API keys (optional)
OPENAI_API_KEY=                  # OpenAI API key for AI features
ANTHROPIC_API_KEY=               # Anthropic Claude API key
```

See the [Setup Guide](docs/SETUP.md) for detailed configuration instructions.

## 🤖 Built-in Commands

The template includes a complete set of functional commands:

| Command | Description | Example |
|---------|-------------|---------|
| `/start` | Welcome message and bot introduction | `/start` |
| `/help` | List all available commands with descriptions | `/help` |
| `/echo <message>` | Echo back your message (useful for testing) | `/echo Hello World!` |
| `/status` | Show bot health and configuration information | `/status` |

### Command Features
- **Case-insensitive**: `/START` and `/start` work the same way
- **Argument parsing**: Commands automatically parse and validate arguments
- **Error handling**: Invalid commands show helpful error messages
- **Dynamic help**: Help command automatically updates as you add new commands
- **Extensible**: Easy to add new commands following the established patterns

See the [Commands Guide](docs/COMMANDS.md) for detailed information about each command and how to create custom ones.

## 🛠️ Development

### Prerequisites
- **Rust 1.70 or later** - Install from [rustup.rs](https://rustup.rs/)
- **Git** - For version control
- **A Telegram bot token** from [@BotFather](https://t.me/BotFather)

### Development Commands
```bash
# Build the project
cargo build

# Run with debug logging
RUST_LOG=debug cargo run

# Run all tests
cargo test

# Run property-based tests specifically
cargo test property

# Check code without building (faster)
cargo check

# Format code
cargo fmt

# Run linting
cargo clippy

# Build optimized release version
cargo build --release
```

### Hot Reloading (Optional)
For faster development cycles:
```bash
# Install cargo-watch
cargo install cargo-watch

# Auto-rebuild and restart on file changes
cargo watch -x run
```

### Adding New Commands
The template makes it easy to add new commands:

1. **Create** a new command file in `src/commands/`
2. **Implement** the `CommandHandler` trait
3. **Register** the command in `main.rs`
4. **Test** your command

Example:
```rust
// src/commands/weather.rs
#[async_trait]
impl CommandHandler for WeatherCommand {
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
        // Your command logic here
    }
    
    fn description(&self) -> &str { "Get weather information" }
    fn usage(&self) -> &str { "/weather <city>" }
}
```

See the [Extension Guide](docs/EXTENDING.md) for comprehensive examples and patterns.

### AI Integration
The template includes a complete AI integration framework in `src/ai/`:

- **AI Router**: Decides when to use AI vs regular commands
- **AI Processor**: Handles AI service integration (OpenAI, local models, etc.)
- **Context Management**: Maintains conversation context and user preferences
- **Graceful Fallback**: Continues working when AI services are unavailable

Key extension points are marked with `TODO` comments for easy identification.

## 🐳 Docker Support

The template includes complete Docker configuration for easy deployment:

### Quick Docker Run
```bash
# Build and run with Docker
docker build -t telegram-bot-template .
docker run -e TELEGRAM_BOT_TOKEN=your_token -e AI_ENABLED=false telegram-bot-template
```

### Development with Docker Compose
```bash
# Start development environment (includes database)
docker-compose up -d

# View logs
docker-compose logs -f bot

# Stop services
docker-compose down
```

### Production Deployment
```bash
# Build optimized image
docker build --target production -t telegram-bot-template:latest .

# Run in production
docker run -d \
  --name telegram-bot \
  -e TELEGRAM_BOT_TOKEN=your_token \
  -e RUST_LOG=info \
  --restart unless-stopped \
  telegram-bot-template:latest
```

The Docker setup includes:
- **Multi-stage builds** for optimized production images
- **Health checks** for container monitoring
- **Volume mounts** for persistent data
- **Environment variable** configuration
- **Development** and **production** configurations

## 📚 Documentation

Comprehensive documentation is available in the `docs/` directory:

| Document | Description |
|----------|-------------|
| **[Setup Guide](docs/SETUP.md)** | Detailed setup instructions, environment configuration, and troubleshooting |
| **[Commands Guide](docs/COMMANDS.md)** | Complete command system documentation with examples and best practices |
| **[Extension Guide](docs/EXTENDING.md)** | Comprehensive guide for adding new features, AI integration, and database setup |

### Additional Resources
- **Inline Code Comments**: Every module and function is thoroughly documented
- **Example Implementations**: Built-in commands serve as examples for creating new ones
- **Test Examples**: Property-based and unit tests demonstrate testing patterns
- **Configuration Examples**: `.env.example` shows all available configuration options

### Architecture Documentation
The codebase follows clean architecture principles with:
- **Clear separation of concerns** between layers
- **Well-defined interfaces** for easy testing and extension
- **Comprehensive error handling** with user-friendly messages
- **Async-first design** for optimal performance
- **Modular structure** allowing independent development of features

## 🧪 Testing

The template includes comprehensive testing infrastructure:

### Test Types
- **Unit Tests**: Test individual components and functions
- **Property-Based Tests**: Validate correctness properties across many inputs
- **Integration Tests**: Test complete workflows and component interactions

### Running Tests
```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run property-based tests specifically
cargo test property

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out html
```

### Test Coverage
The template includes property-based tests for:
- ✅ Configuration loading and validation
- ✅ Command routing and parsing
- ✅ Message processing completeness
- ✅ API error recovery
- ✅ AI routing decisions
- ✅ Message response delivery

## 🚀 Production Deployment

### Environment Setup
```bash
# Production environment variables
export TELEGRAM_BOT_TOKEN="your_production_token"
export RUST_LOG="info"
export DATABASE_URL="postgresql://user:pass@host/db"
```

### Systemd Service (Linux)
```ini
[Unit]
Description=Telegram Bot Template
After=network.target

[Service]
Type=simple
User=botuser
WorkingDirectory=/opt/telegram-bot-template
ExecStart=/opt/telegram-bot-template/target/release/telegram-bot-template
Environment=TELEGRAM_BOT_TOKEN=your_token
Environment=RUST_LOG=info
Restart=always

[Install]
WantedBy=multi-user.target
```

### Security Considerations
- 🔒 **Never commit** your `.env` file or bot tokens
- 🔐 **Use secure storage** for production secrets (AWS Secrets Manager, etc.)
- 🔄 **Rotate bot tokens** periodically using @BotFather
- 📊 **Monitor logs** for suspicious activity
- 🌐 **Use HTTPS** for webhook URLs in production

## 🤝 Contributing

This template is designed to be forked and customized for your specific needs:

1. **Fork the repository** for your project
2. **Customize the structure** as needed
3. **Add your specific features** following the established patterns
4. **Update documentation** to reflect your changes
5. **Share improvements** back to the community (optional)

### Development Guidelines
- Follow Rust conventions and best practices
- Maintain comprehensive test coverage
- Document all public APIs and extension points
- Use meaningful commit messages
- Keep the architecture clean and modular

## 📄 License

This template is provided as-is for educational and development purposes. Customize the license as needed for your project.

## 🆘 Getting Help

If you encounter issues:

1. **Check the documentation** in the `docs/` directory
2. **Review the logs** with `RUST_LOG=debug` for detailed information
3. **Look at the test files** for usage examples
4. **Examine the source code** - it's well-documented with inline comments
5. **Consult the Telegram Bot API** documentation at https://core.telegram.org/bots/api

## 🎯 What's Next?

After setting up your bot:

1. **Explore the built-in commands** to understand the patterns
2. **Add your first custom command** following the examples
3. **Integrate external APIs** for enhanced functionality
4. **Set up a database** if you need persistent storage
5. **Add AI capabilities** using the provided framework
6. **Deploy to production** using Docker or systemd

**Happy bot building! 🤖✨**
