# Commands Documentation

This document provides comprehensive information about the command system in the Rust Telegram Bot Template, including built-in commands and how to create custom ones.

## Command System Overview

The bot uses a flexible command system built around the `CommandHandler` trait. Commands are:

- **Case-insensitive**: `/START` and `/start` work the same way
- **Argument-aware**: Commands can accept and parse arguments
- **Extensible**: Easy to add new commands without modifying existing code
- **Error-resilient**: Unknown commands provide helpful error messages

## Built-in Commands

### `/start`
**Purpose**: Welcome new users and introduce the bot

**Usage**: `/start`

**Description**: 
- Sends a welcome message to new users
- Provides basic information about the bot's capabilities
- Typically the first command users interact with

**Example**:
```
User: /start
Bot: 🤖 Welcome to the Telegram Bot Template!

This bot demonstrates a clean, extensible foundation for building Telegram bots in Rust.

Available features:
• Simple command system
• Configuration management  
• AI integration placeholder
• Comprehensive error handling

Type /help to see all available commands.
```

### `/help`
**Purpose**: Display available commands and their usage

**Usage**: `/help`

**Description**:
- Lists all registered commands
- Shows brief descriptions for each command
- Provides usage examples
- Dynamically updates as new commands are added

**Example**:
```
User: /help
Bot: 📚 Available Commands:

/start - Welcome message and bot introduction
/help - Show this help message
/echo <message> - Echo back your message
/status - Show bot status and health information

💡 Tip: Commands are case-insensitive and work in both private chats and groups.
```

### `/echo`
**Purpose**: Echo back user messages (useful for testing)

**Usage**: `/echo <message>`

**Arguments**:
- `<message>`: Any text you want the bot to repeat

**Description**:
- Repeats whatever text you send after the command
- Useful for testing bot responsiveness
- Demonstrates argument parsing

**Examples**:
```
User: /echo Hello, World!
Bot: Hello, World!

User: /echo This is a test message with multiple words
Bot: This is a test message with multiple words

User: /echo
Bot: ❌ Please provide a message to echo.
Usage: /echo <message>
```

### `/status`
**Purpose**: Display bot health and configuration information

**Usage**: `/status`

**Description**:
- Shows current bot status and uptime
- Displays configuration information (without sensitive data)
- Useful for monitoring and debugging
- Provides system health indicators

**Example**:
```
User: /status
Bot: 🤖 Bot Status Report

✅ Status: Online and operational
⏱️ Uptime: 2 hours, 34 minutes
🔧 Version: 1.0.0
📊 Commands processed: 127
🧠 AI Integration: Disabled
🔗 Mode: Polling

Configuration:
• Log Level: info
• Commands registered: 4
• Webhook: Not configured

All systems operational! 🚀
```

## Command Architecture

### Command Handler Trait

All commands implement the `CommandHandler` trait:

```rust
#[async_trait]
pub trait CommandHandler: Send + Sync {
    /// Execute the command
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> Result<(), BotError>;
    
    /// Get command description for help text
    fn description(&self) -> &str;
    
    /// Get usage information
    fn usage(&self) -> &str;
}
```

### Command Registration

Commands are registered in `main.rs`:

```rust
async fn register_commands(bot_service: &mut BotService, _config: &Config) -> Result<()> {
    let router = bot_service.command_router_mut();
    
    // Register built-in commands
    router.register_command("start", Box::new(StartCommand::new()));
    router.register_command("help", Box::new(HelpCommand::new()));
    router.register_command("echo", Box::new(EchoCommand::new()));
    router.register_command("status", Box::new(StatusCommand::new()));
    
    Ok(())
}
```

### Command Parsing

The command router automatically handles:

- **Command extraction**: Identifies commands starting with `/`
- **Argument parsing**: Splits command text into command name and arguments
- **Bot name handling**: Strips `@botname` suffixes (e.g., `/start@mybot` → `/start`)
- **Case normalization**: Converts commands to lowercase for matching

## Creating Custom Commands

### Step 1: Create Command Structure

Create a new file in `src/commands/` (e.g., `src/commands/weather.rs`):

```rust
use async_trait::async_trait;
use teloxide::{Bot, types::Message, prelude::Requester};
use crate::commands::CommandHandler;
use crate::error::{BotError, BotResult};

pub struct WeatherCommand;

impl WeatherCommand {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandHandler for WeatherCommand {
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
        // Check if location argument is provided
        if args.is_empty() {
            let usage_msg = "❌ Please provide a location.\nUsage: /weather <city>";
            bot.send_message(message.chat.id, usage_msg).await
                .map_err(BotError::Telegram)?;
            return Ok(());
        }
        
        let location = args.join(" ");
        
        // TODO: Integrate with weather API
        let response = format!("🌤️ Weather for {}: Sunny, 22°C\n\n(This is a placeholder - integrate with a real weather API)", location);
        
        bot.send_message(message.chat.id, response).await
            .map_err(BotError::Telegram)?;
        
        Ok(())
    }
    
    fn description(&self) -> &str {
        "Get weather information for a location"
    }
    
    fn usage(&self) -> &str {
        "/weather <city> - Get current weather for the specified city"
    }
}
```

### Step 2: Export the Command

Add to `src/commands/mod.rs`:

```rust
pub mod weather;
pub use weather::WeatherCommand;
```

### Step 3: Register the Command

Add to the registration function in `main.rs`:

```rust
router.register_command("weather", Box::new(WeatherCommand::new()));
```

## Advanced Command Features

### Argument Validation

```rust
async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
    // Validate argument count
    if args.len() != 2 {
        let error_msg = "❌ Invalid arguments.\nUsage: /command <arg1> <arg2>";
        bot.send_message(message.chat.id, error_msg).await?;
        return Ok(());
    }
    
    // Validate argument format
    let number = match args[0].parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            bot.send_message(message.chat.id, "❌ First argument must be a number").await?;
            return Ok(());
        }
    };
    
    // Process valid arguments...
}
```

### User Permission Checks

```rust
async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
    // Check if user is authorized (example: admin-only command)
    let user_id = message.from().map(|u| u.id.0).unwrap_or(0);
    
    if !self.is_admin(user_id) {
        bot.send_message(message.chat.id, "❌ This command requires admin privileges").await?;
        return Ok(());
    }
    
    // Process admin command...
}
```

### Rich Message Formatting

```rust
use teloxide::types::{ParseMode, InlineKeyboardMarkup, InlineKeyboardButton};

async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
    // Send message with Markdown formatting
    let formatted_text = "*Bold text* and _italic text_\n\n`Code block`\n\n[Link](https://example.com)";
    
    bot.send_message(message.chat.id, formatted_text)
        .parse_mode(ParseMode::Markdown)
        .await?;
    
    // Send message with inline keyboard
    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::callback("Option 1", "opt1")],
        vec![InlineKeyboardButton::callback("Option 2", "opt2")],
    ]);
    
    bot.send_message(message.chat.id, "Choose an option:")
        .reply_markup(keyboard)
        .await?;
    
    Ok(())
}
```

## Command Best Practices

### 1. Error Handling
Always handle errors gracefully and provide helpful messages:

```rust
// Good: Specific error message
if args.is_empty() {
    bot.send_message(message.chat.id, "❌ Please provide a search term.\nUsage: /search <query>").await?;
    return Ok(());
}

// Bad: Generic or no error message
// Just returning an error without user feedback
```

### 2. Input Validation
Validate and sanitize user input:

```rust
// Validate input length
if args.join(" ").len() > 1000 {
    bot.send_message(message.chat.id, "❌ Message too long (max 1000 characters)").await?;
    return Ok(());
}

// Sanitize input for external APIs
let safe_query = args.join(" ").chars()
    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
    .collect::<String>();
```

### 3. Async Best Practices
Use proper async patterns:

```rust
// Good: Proper error handling with ?
bot.send_message(message.chat.id, response).await
    .map_err(BotError::Telegram)?;

// Good: Concurrent operations when possible
let (weather, news) = tokio::join!(
    fetch_weather(&location),
    fetch_news(&location)
);
```

### 4. User Experience
Make commands intuitive and helpful:

```rust
// Provide clear usage instructions
fn usage(&self) -> &str {
    "/weather <city> - Get current weather (e.g., /weather London)"
}

// Use emojis for better visual feedback
let response = format!("🌤️ Weather for {}: {}°C", city, temp);

// Provide examples in error messages
"❌ Invalid date format. Use YYYY-MM-DD (e.g., 2024-01-15)"
```

## Testing Commands

### Unit Tests
Create tests for your commands in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_command_description() {
        let cmd = WeatherCommand::new();
        assert!(!cmd.description().is_empty());
        assert!(!cmd.usage().is_empty());
    }
    
    // Add more specific tests for your command logic
}
```

### Integration Tests
Test commands with the full bot context in `tests/`:

```rust
#[tokio::test]
async fn test_weather_command_integration() {
    // Set up test bot and message
    // Call command handler
    // Verify response
}
```

## Command Ideas and Extensions

Here are some ideas for additional commands you might want to implement:

### Utility Commands
- `/time [timezone]` - Show current time
- `/calc <expression>` - Simple calculator
- `/qr <text>` - Generate QR codes
- `/shorten <url>` - URL shortener

### Information Commands
- `/news [category]` - Latest news
- `/weather <location>` - Weather information
- `/translate <text>` - Language translation
- `/define <word>` - Dictionary lookup

### Interactive Commands
- `/poll <question> <options>` - Create polls
- `/remind <time> <message>` - Set reminders
- `/quiz` - Interactive quizzes
- `/game` - Simple games

### Admin Commands
- `/stats` - Bot usage statistics
- `/users` - User management
- `/broadcast <message>` - Send to all users
- `/maintenance` - Toggle maintenance mode

## Troubleshooting Commands

### Command Not Responding
1. **Check registration**: Ensure the command is registered in `main.rs`
2. **Verify spelling**: Command names must match exactly
3. **Check logs**: Look for error messages in the console
4. **Test with `/help`**: See if the command appears in the list

### Arguments Not Working
1. **Debug argument parsing**: Add logging to see what arguments are received
2. **Check whitespace**: Arguments are split by whitespace
3. **Validate input**: Ensure your validation logic is correct

### Error Messages
1. **Check error handling**: Ensure all error paths send user-friendly messages
2. **Verify permissions**: Make sure the bot can send messages to the chat
3. **Test error scenarios**: Try invalid inputs to test error handling

Remember: The command system is designed to be flexible and extensible. Don't hesitate to modify it to fit your specific needs!