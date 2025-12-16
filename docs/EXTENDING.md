# Extending the Bot

This guide explains how to extend the Rust Telegram Bot Template with new features, commands, and integrations.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Adding New Commands](#adding-new-commands)
3. [Integrating AI Services](#integrating-ai-services)
4. [Database Integration](#database-integration)
5. [Adding Middleware](#adding-middleware)
6. [External API Integration](#external-api-integration)
7. [Custom Error Handling](#custom-error-handling)
8. [Configuration Extensions](#configuration-extensions)
9. [Testing Your Extensions](#testing-your-extensions)
10. [Deployment Considerations](#deployment-considerations)

## Architecture Overview

The bot template follows a modular architecture designed for easy extension:

```
┌─────────────────────────────────────────┐
│              Main Application           │  ← Entry point, orchestration
├─────────────────────────────────────────┤
│            Command Router               │  ← Routes messages to handlers
├─────────────────────────────────────────┤
│     Command Handlers    │  AI Router    │  ← Business logic layer
├─────────────────────────────────────────┤
│          Bot Service Layer              │  ← Telegram API abstraction
├─────────────────────────────────────────┤
│         Telegram API (teloxide)         │  ← External dependency
└─────────────────────────────────────────┘
```

### Key Extension Points

The template provides several designated extension points marked with `TODO` comments:

1. **Command Handlers** (`src/commands/`) - Add new bot commands
2. **AI Router** (`src/ai/`) - Integrate AI/ML services
3. **Configuration** (`src/config.rs`) - Add new configuration options
4. **Error Handling** (`src/error.rs`) - Custom error types
5. **Main Application** (`src/main.rs`) - Application-level extensions

## Adding New Commands

### Basic Command Structure

Every command follows the same pattern. Here's a template for creating new commands:

```rust
// src/commands/my_command.rs
use async_trait::async_trait;
use teloxide::{Bot, types::Message, prelude::Requester};
use crate::commands::CommandHandler;
use crate::error::{BotError, BotResult};

/// TODO: Replace with your command description
/// 
/// This command demonstrates the basic structure for implementing
/// new bot commands. Replace this documentation and implementation
/// with your specific command logic.
pub struct MyCommand {
    // TODO: Add any state or configuration your command needs
    // For example: api_client, database_pool, cache, etc.
}

impl MyCommand {
    /// Create a new instance of the command
    pub fn new() -> Self {
        Self {
            // TODO: Initialize any required state
        }
    }
    
    // TODO: Add any helper methods your command needs
    // For example: validate_input, format_response, call_external_api, etc.
}

#[async_trait]
impl CommandHandler for MyCommand {
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
        // TODO: Implement your command logic here
        
        // Example: Basic argument validation
        if args.is_empty() {
            let usage_msg = "❌ Please provide required arguments.\nUsage: /mycommand <arg>";
            bot.send_message(message.chat.id, usage_msg).await
                .map_err(BotError::Telegram)?;
            return Ok(());
        }
        
        // TODO: Process the command arguments
        let user_input = args.join(" ");
        
        // TODO: Implement your business logic
        let response = format!("You said: {}", user_input);
        
        // Send response to user
        bot.send_message(message.chat.id, response).await
            .map_err(BotError::Telegram)?;
        
        Ok(())
    }
    
    fn description(&self) -> &str {
        // TODO: Replace with your command description
        "Description of what this command does"
    }
    
    fn usage(&self) -> &str {
        // TODO: Replace with your command usage
        "/mycommand <arg> - Detailed usage information"
    }
}
```

### Registration Steps

1. **Create the command file** in `src/commands/`
2. **Export in mod.rs**:
   ```rust
   // src/commands/mod.rs
   pub mod my_command;
   pub use my_command::MyCommand;
   ```

3. **Register in main.rs**:
   ```rust
   // src/main.rs - in register_commands function
   router.register_command("mycommand", Box::new(MyCommand::new()));
   ```

### Advanced Command Examples

#### Command with External API Integration

```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ApiResponse {
    result: String,
    status: String,
}

pub struct ApiCommand {
    client: Client,
    api_key: String,
}

impl ApiCommand {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }
    
    async fn call_external_api(&self, query: &str) -> Result<String, reqwest::Error> {
        let response: ApiResponse = self.client
            .get("https://api.example.com/search")
            .query(&[("q", query), ("key", &self.api_key)])
            .send()
            .await?
            .json()
            .await?;
        
        Ok(response.result)
    }
}

#[async_trait]
impl CommandHandler for ApiCommand {
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
        if args.is_empty() {
            bot.send_message(message.chat.id, "❌ Please provide a search query").await?;
            return Ok(());
        }
        
        let query = args.join(" ");
        
        // Show typing indicator while processing
        bot.send_chat_action(message.chat.id, teloxide::types::ChatAction::Typing).await?;
        
        match self.call_external_api(&query).await {
            Ok(result) => {
                bot.send_message(message.chat.id, format!("🔍 Result: {}", result)).await?;
            }
            Err(e) => {
                log::error!("API call failed: {}", e);
                bot.send_message(message.chat.id, "❌ Sorry, the service is temporarily unavailable").await?;
            }
        }
        
        Ok(())
    }
    
    fn description(&self) -> &str {
        "Search using external API"
    }
    
    fn usage(&self) -> &str {
        "/search <query> - Search for information using external API"
    }
}
```

#### Command with Database Integration

```rust
use sqlx::{PgPool, Row};

pub struct DatabaseCommand {
    db_pool: PgPool,
}

impl DatabaseCommand {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
    
    async fn save_user_data(&self, user_id: i64, data: &str) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO user_data (user_id, data) VALUES ($1, $2) ON CONFLICT (user_id) DO UPDATE SET data = $2")
            .bind(user_id)
            .bind(data)
            .execute(&self.db_pool)
            .await?;
        Ok(())
    }
    
    async fn get_user_data(&self, user_id: i64) -> Result<Option<String>, sqlx::Error> {
        let row = sqlx::query("SELECT data FROM user_data WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.db_pool)
            .await?;
        
        Ok(row.map(|r| r.get("data")))
    }
}
```

## Integrating AI Services

The template includes a placeholder AI system in `src/ai/` that you can extend:

### AI Router Extension Points

```rust
// src/ai/processor.rs - Key TODO locations for AI integration

impl AIProcessor {
    pub async fn process_message(&self, message_text: &str, context: &AIContext) -> Result<String> {
        if !self.enabled {
            return Ok("AI service is not properly configured.".to_string());
        }

        // TODO: Replace this placeholder with actual AI service integration
        // Examples of what you might integrate here:
        // - OpenAI GPT API
        // - Anthropic Claude API  
        // - Local LLM via Ollama
        // - Custom ML model endpoints
        // - Hugging Face Transformers
        
        // Placeholder implementation
        if let Some(ref _api_key) = self.api_key {
            // TODO: Implement actual AI service call
            // Example structure:
            // let response = self.call_ai_service(message_text, context).await?;
            // Ok(response)
            
            Ok(format!("AI Response: Hello! You said '{}'", message_text))
        } else {
            Ok("AI service is not properly configured.".to_string())
        }
    }
}
```

### OpenAI Integration Example

```rust
use reqwest::Client;
use serde_json::json;

pub struct OpenAIProcessor {
    client: Client,
    api_key: String,
    model: String,
}

impl OpenAIProcessor {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "gpt-3.5-turbo".to_string(),
        }
    }
    
    pub async fn generate_response(&self, message: &str, context: &AIContext) -> Result<String, Box<dyn std::error::Error>> {
        let payload = json!({
            "model": self.model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant in a Telegram bot."
                },
                {
                    "role": "user", 
                    "content": message
                }
            ],
            "max_tokens": 150
        });
        
        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await?;
            
        // TODO: Parse response and extract generated text
        let response_text = response.text().await?;
        Ok(format!("AI: {}", response_text))
    }
}
```

### Local LLM Integration (Ollama)

```rust
pub struct OllamaProcessor {
    client: Client,
    base_url: String,
    model: String,
}

impl OllamaProcessor {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            model,
        }
    }
    
    pub async fn generate_response(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error>> {
        let payload = json!({
            "model": self.model,
            "prompt": prompt,
            "stream": false
        });
        
        let response = self.client
            .post(&format!("{}/api/generate", self.base_url))
            .json(&payload)
            .send()
            .await?;
            
        // TODO: Parse Ollama response format
        let response_text = response.text().await?;
        Ok(response_text)
    }
}
```

## Database Integration

### Adding Database Support

1. **Add database dependencies** to `Cargo.toml`:
   ```toml
   [dependencies]
   sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "chrono", "uuid"] }
   # or for SQLite:
   # sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "sqlite", "chrono"] }
   ```

2. **Extend configuration** in `src/config.rs`:
   ```rust
   #[derive(Debug, Clone)]
   pub struct Config {
       // ... existing fields
       
       /// Database connection URL
       pub database_url: Option<String>,
       
       /// Maximum database connections
       pub database_max_connections: Option<u32>,
   }
   
   impl Config {
       pub fn from_env() -> ConfigResult<Self> {
           // ... existing code
           
           let database_url = env::var("DATABASE_URL").ok();
           let database_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
               .ok()
               .map(|s| s.parse().unwrap_or(10));
           
           // TODO: Add database_url and database_max_connections to Config construction
       }
   }
   ```

3. **Create database module** `src/database/mod.rs`:
   ```rust
   use sqlx::{PgPool, Pool, Postgres};
   use crate::config::Config;
   use crate::error::{BotError, BotResult};
   
   pub mod models;
   pub mod migrations;
   
   pub type DatabasePool = Pool<Postgres>;
   
   /// Initialize database connection pool
   pub async fn init_database(config: &Config) -> BotResult<Option<DatabasePool>> {
       let database_url = match &config.database_url {
           Some(url) => url,
           None => {
               log::info!("No database URL configured, skipping database initialization");
               return Ok(None);
           }
       };
       
       log::info!("Connecting to database...");
       
       let pool = PgPool::connect(database_url)
           .await
           .map_err(|e| BotError::Database(format!("Failed to connect to database: {}", e)))?;
       
       // TODO: Run migrations here
       // migrations::run_migrations(&pool).await?;
       
       log::info!("Database connection established");
       Ok(Some(pool))
   }
   ```

4. **Update main.rs** to initialize database:
   ```rust
   #[tokio::main]
   async fn main() -> Result<()> {
       // ... existing initialization code
       
       // TODO: Initialize database if configured
       let database_pool = database::init_database(&config).await?;
       
       // Pass database pool to bot service
       let mut bot_service = BotService::new(&config, database_pool).await?;
       
       // ... rest of main function
   }
   ```

### Database Models Example

```rust
// src/database/models.rs
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub telegram_id: i64,
    pub username: Option<String>,
    pub first_name: String,
    pub last_name: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct UserSession {
    pub id: i64,
    pub user_id: i64,
    pub session_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl User {
    pub async fn find_by_telegram_id(pool: &DatabasePool, telegram_id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE telegram_id = $1",
            telegram_id
        )
        .fetch_optional(pool)
        .await
    }
    
    pub async fn create_or_update(pool: &DatabasePool, telegram_user: &teloxide::types::User) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (telegram_id, username, first_name, last_name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            ON CONFLICT (telegram_id) 
            DO UPDATE SET 
                username = $2,
                first_name = $3,
                last_name = $4,
                updated_at = NOW()
            RETURNING *
            "#,
            telegram_user.id.0 as i64,
            telegram_user.username.as_deref(),
            telegram_user.first_name,
            telegram_user.last_name.as_deref()
        )
        .fetch_one(pool)
        .await
    }
}
```

## Adding Middleware

Middleware allows you to process messages before they reach command handlers:

### Creating Middleware

```rust
// src/middleware/mod.rs
use async_trait::async_trait;
use teloxide::{Bot, types::Message};
use crate::error::BotResult;

#[async_trait]
pub trait Middleware: Send + Sync {
    /// Process message before command handling
    /// Return false to stop processing (message handled by middleware)
    /// Return true to continue to command handlers
    async fn process(&self, bot: &Bot, message: &Message) -> BotResult<bool>;
}

pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }
    
    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
    }
    
    pub async fn process(&self, bot: &Bot, message: &Message) -> BotResult<bool> {
        for middleware in &self.middlewares {
            if !middleware.process(bot, message).await? {
                return Ok(false); // Stop processing
            }
        }
        Ok(true) // Continue to command handlers
    }
}
```

### Example Middleware Implementations

```rust
// Rate limiting middleware
pub struct RateLimitMiddleware {
    // TODO: Implement rate limiting logic
    // Could use Redis, in-memory cache, or database
}

#[async_trait]
impl Middleware for RateLimitMiddleware {
    async fn process(&self, bot: &Bot, message: &Message) -> BotResult<bool> {
        let user_id = message.from().map(|u| u.id.0).unwrap_or(0);
        
        // TODO: Check rate limit for user
        // if rate_limit_exceeded(user_id) {
        //     bot.send_message(message.chat.id, "⏰ Please wait before sending another message").await?;
        //     return Ok(false); // Stop processing
        // }
        
        Ok(true) // Continue processing
    }
}

// Logging middleware
pub struct LoggingMiddleware;

#[async_trait]
impl Middleware for LoggingMiddleware {
    async fn process(&self, bot: &Bot, message: &Message) -> BotResult<bool> {
        let user_id = message.from().map(|u| u.id.0).unwrap_or(0);
        let text = message.text().unwrap_or("<non-text>");
        
        log::info!("Message from user {}: {}", user_id, text);
        
        Ok(true) // Always continue processing
    }
}
```

## External API Integration

### HTTP Client Setup

```rust
// src/services/mod.rs
use reqwest::Client;
use std::time::Duration;

pub struct ApiClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl ApiClient {
    pub fn new(base_url: String, api_key: Option<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            base_url,
            api_key,
        }
    }
    
    pub async fn get<T>(&self, endpoint: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: serde::de::DeserializeOwned,
    {
        let mut request = self.client.get(&format!("{}/{}", self.base_url, endpoint));
        
        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
        
        let response = request.send().await?;
        let data = response.json().await?;
        Ok(data)
    }
    
    pub async fn post<T, U>(&self, endpoint: &str, body: &T) -> Result<U, Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
        U: serde::de::DeserializeOwned,
    {
        let mut request = self.client
            .post(&format!("{}/{}", self.base_url, endpoint))
            .json(body);
            
        if let Some(ref key) = self.api_key {
            request = request.header("Authorization", format!("Bearer {}", key));
        }
        
        let response = request.send().await?;
        let data = response.json().await?;
        Ok(data)
    }
}
```

### Service Integration Pattern

```rust
// src/services/weather.rs
use serde::{Deserialize, Serialize};
use super::ApiClient;

#[derive(Deserialize)]
pub struct WeatherResponse {
    pub temperature: f64,
    pub description: String,
    pub humidity: u32,
    pub wind_speed: f64,
}

pub struct WeatherService {
    client: ApiClient,
}

impl WeatherService {
    pub fn new(api_key: String) -> Self {
        let client = ApiClient::new(
            "https://api.openweathermap.org/data/2.5".to_string(),
            Some(api_key),
        );
        
        Self { client }
    }
    
    pub async fn get_weather(&self, city: &str) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
        let endpoint = format!("weather?q={}&units=metric", city);
        self.client.get(&endpoint).await
    }
}
```

## Custom Error Handling

### Extending Error Types

```rust
// src/error.rs - Add new error variants
#[derive(Debug, thiserror::Error)]
pub enum BotError {
    // ... existing variants
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("External API error: {0}")]
    ExternalApi(String),
    
    #[error("Rate limit exceeded")]
    RateLimit,
    
    #[error("Permission denied: {0}")]
    Permission(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
}

// Add conversion implementations
impl From<sqlx::Error> for BotError {
    fn from(err: sqlx::Error) -> Self {
        BotError::Database(err.to_string())
    }
}

impl From<reqwest::Error> for BotError {
    fn from(err: reqwest::Error) -> Self {
        BotError::ExternalApi(err.to_string())
    }
}
```

### Error Recovery Strategies

```rust
impl ErrorRecovery {
    pub fn user_friendly_message(error: &BotError) -> String {
        match error {
            BotError::Database(_) => {
                "🔧 We're experiencing technical difficulties. Please try again later.".to_string()
            }
            BotError::ExternalApi(_) => {
                "🌐 External service is temporarily unavailable. Please try again later.".to_string()
            }
            BotError::RateLimit => {
                "⏰ You're sending messages too quickly. Please wait a moment and try again.".to_string()
            }
            BotError::Permission(msg) => {
                format!("🚫 Permission denied: {}", msg)
            }
            BotError::Validation(msg) => {
                format!("❌ Invalid input: {}", msg)
            }
            // ... handle other error types
            _ => "❌ An unexpected error occurred. Please try again.".to_string(),
        }
    }
}
```

## Configuration Extensions

### Adding New Configuration Options

```rust
// src/config.rs - Extend the Config struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Config {
    // ... existing fields
    
    /// External API configurations
    pub weather_api_key: Option<String>,
    pub openai_api_key: Option<String>,
    pub database_url: Option<String>,
    
    /// Feature flags
    pub enable_weather: bool,
    pub enable_ai_chat: bool,
    pub enable_user_tracking: bool,
    
    /// Rate limiting
    pub rate_limit_messages_per_minute: u32,
    pub rate_limit_enabled: bool,
    
    /// Custom settings
    pub default_language: String,
    pub timezone: String,
}

impl Config {
    pub fn from_env() -> ConfigResult<Self> {
        // ... existing code
        
        // TODO: Load new configuration values
        let weather_api_key = env::var("WEATHER_API_KEY").ok();
        let openai_api_key = env::var("OPENAI_API_KEY").ok();
        let database_url = env::var("DATABASE_URL").ok();
        
        let enable_weather = env::var("ENABLE_WEATHER")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
            
        let enable_ai_chat = env::var("ENABLE_AI_CHAT")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);
            
        let rate_limit_messages_per_minute = env::var("RATE_LIMIT_MESSAGES_PER_MINUTE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .unwrap_or(10);
            
        let default_language = env::var("DEFAULT_LANGUAGE")
            .unwrap_or_else(|_| "en".to_string());
            
        // TODO: Add these to Config construction
    }
}
```

### Environment Variable Documentation

Update your `.env.example`:

```bash
# Core bot configuration
TELEGRAM_BOT_TOKEN=your_bot_token_here
RUST_LOG=info

# External API keys (optional)
WEATHER_API_KEY=your_openweather_api_key
OPENAI_API_KEY=your_openai_api_key

# Database configuration (optional)
DATABASE_URL=postgresql://user:password@localhost/botdb
DATABASE_MAX_CONNECTIONS=10

# Feature flags
ENABLE_WEATHER=true
ENABLE_AI_CHAT=false
ENABLE_USER_TRACKING=true

# Rate limiting
RATE_LIMIT_ENABLED=true
RATE_LIMIT_MESSAGES_PER_MINUTE=10

# Localization
DEFAULT_LANGUAGE=en
TIMEZONE=UTC
```

## Testing Your Extensions

### Unit Tests for Commands

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;
    
    #[tokio::test]
    async fn test_weather_command_with_valid_city() {
        let command = WeatherCommand::new("test_api_key".to_string());
        
        // TODO: Mock the external API call
        // You might want to use mockall or similar for this
        
        assert_eq!(command.description(), "Get weather information for a location");
    }
    
    #[tokio::test]
    async fn test_weather_command_with_no_args() {
        // TODO: Test error handling when no arguments provided
    }
}
```

### Integration Tests

```rust
// tests/integration_tests.rs
use telegram_bot_template::*;

#[tokio::test]
async fn test_full_bot_workflow() {
    // TODO: Set up test bot instance
    // TODO: Send test messages
    // TODO: Verify responses
}
```

### Property-Based Tests

```rust
use quickcheck::{quickcheck, TestResult};

#[quickcheck]
fn test_command_parsing_properties(input: String) -> TestResult {
    // TODO: Test that command parsing is robust across various inputs
    TestResult::passed()
}
```

## Deployment Considerations

### Docker Extensions

```dockerfile
# Dockerfile extensions for additional dependencies
FROM rust:1.70 as builder

# Install additional system dependencies if needed
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

# Build with all features
RUN cargo build --release --all-features

FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/telegram-bot-template /usr/local/bin/

# TODO: Add any additional runtime setup

CMD ["telegram-bot-template"]
```

### Environment-Specific Configuration

```yaml
# docker-compose.yml for development
version: '3.8'
services:
  bot:
    build: .
    environment:
      - TELEGRAM_BOT_TOKEN=${TELEGRAM_BOT_TOKEN}
      - DATABASE_URL=postgresql://postgres:password@db:5432/botdb
      - RUST_LOG=debug
    depends_on:
      - db
    
  db:
    image: postgres:15
    environment:
      - POSTGRES_DB=botdb
      - POSTGRES_PASSWORD=password
    volumes:
      - postgres_data:/var/lib/postgresql/data
    
volumes:
  postgres_data:
```

## Best Practices for Extensions

### 1. Follow Rust Conventions
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and structs
- Add comprehensive documentation with `///`
- Use `Result<T, E>` for error handling

### 2. Maintain Backwards Compatibility
- Don't break existing command interfaces
- Use feature flags for new functionality
- Provide migration paths for configuration changes

### 3. Error Handling
- Always handle errors gracefully
- Provide user-friendly error messages
- Log detailed errors for debugging
- Use appropriate error recovery strategies

### 4. Performance Considerations
- Use async/await properly
- Implement connection pooling for databases
- Cache frequently accessed data
- Set appropriate timeouts for external calls

### 5. Security
- Validate all user inputs
- Sanitize data before external API calls
- Use environment variables for secrets
- Implement rate limiting
- Log security-relevant events

### 6. Testing
- Write unit tests for business logic
- Create integration tests for full workflows
- Use property-based testing for robust validation
- Mock external dependencies in tests

Remember: The template is designed to be extended and modified. Don't hesitate to restructure it to fit your specific needs while maintaining the core architectural principles!