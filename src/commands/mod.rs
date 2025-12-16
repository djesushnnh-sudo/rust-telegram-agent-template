use std::collections::HashMap;
use teloxide::{Bot, types::Message, prelude::Requester};
use crate::error::{BotError, BotResult};

pub mod handler;
pub mod start;
pub mod help;
pub mod echo;
pub mod status;

pub use handler::CommandHandler;
pub use start::StartCommand;
pub use help::{HelpCommand, CommandInfo};
pub use echo::EchoCommand;
pub use status::StatusCommand;

/// Command router that manages command registration and routing
/// 
/// The CommandRouter is responsible for:
/// - Registering command handlers with their associated command names
/// - Parsing incoming messages to extract commands and arguments
/// - Routing commands to the appropriate handlers
/// - Handling unknown commands with helpful error messages
pub struct CommandRouter {
    /// Map of command names to their handlers
    handlers: HashMap<String, Box<dyn CommandHandler>>,
}

impl CommandRouter {
    /// Create a new empty command router
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
    
    /// Register a command handler with the router
    /// 
    /// # Arguments
    /// * `name` - The command name (without the leading '/')
    /// * `handler` - The command handler implementation
    /// 
    /// # Example
    /// ```ignore
    /// let mut router = CommandRouter::new();
    /// router.register_command("start", Box::new(StartCommand::new()));
    /// ```
    pub fn register_command(&mut self, name: &str, handler: Box<dyn CommandHandler>) {
        let command_name = name.to_lowercase();
        log::debug!("Registering command: {}", command_name);
        self.handlers.insert(command_name, handler);
    }
    
    /// Handle an incoming message, routing it to the appropriate command handler
    /// 
    /// This method implements the core message processing pipeline:
    /// 1. Extracts and validates message text
    /// 2. Checks if the message is a command (starts with '/')
    /// 3. Parses the command name and arguments
    /// 4. Routes to the appropriate handler or returns an unknown command error
    /// 
    /// ## Extending with Middleware
    /// 
    /// You can extend this method to add preprocessing middleware by adding
    /// code in the designated middleware section below. Common middleware includes:
    /// 
    /// ### Rate Limiting
    /// Extract user ID and check against rate limiter, sending appropriate
    /// response if the user is being rate limited.
    /// 
    /// ### Permission Checks
    /// Verify user authorization before executing commands, especially for
    /// admin or restricted functionality.
    /// 
    /// ### Command Analytics
    /// Log command usage for monitoring, analytics, and debugging purposes.
    /// 
    /// ### Input Sanitization
    /// Clean and validate user input to prevent injection attacks and
    /// ensure data integrity.
    /// 
    /// # Arguments
    /// * `bot` - The Telegram bot instance
    /// * `message` - The incoming message to process
    /// 
    /// # Returns
    /// * `Ok(())` if the message was processed successfully
    /// * `Err(BotError)` if there was an error processing the message
    pub async fn handle_message(&self, bot: &Bot, message: &Message) -> BotResult<()> {
        // Extract text from the message
        let text = match message.text() {
            Some(text) => text,
            None => {
                log::debug!("Received non-text message, ignoring");
                return Ok(()); // Non-text messages are not commands
            }
        };
        
        // Check if this is a command (starts with '/')
        if !text.starts_with('/') {
            log::debug!("Message is not a command, ignoring");
            return Ok(()); // Not a command, ignore
        }
        
        // =====================================================================
        // MIDDLEWARE PROCESSING SECTION
        // =====================================================================
        // Add your middleware processing here. This is executed for every
        // command before it reaches the handler.
        // 
        // Examples of middleware you might want to add:
        
        // Rate limiting (prevent spam)
        // Extract user ID for rate limiting
        // let user_id = message.from().map(|u| u.id.0).unwrap_or(0);
        // Check rate limits and send appropriate response if limited
        
        // Permission/authorization checks
        // Verify user has permission to execute the command
        // Send permission denied message if unauthorized
        
        // Command usage analytics
        // Log command execution for analytics and monitoring
        
        // Input validation and sanitization
        // Validate command format and sanitize arguments
        // Reject unsafe or malformed commands
        
        // Maintenance mode check
        // Block non-admin users during maintenance periods
        // ====================================================================
        
        // Parse the command and arguments
        let (command_name, args) = self.parse_command(text)?;
        
        log::debug!("Processing command: {} with {} args", command_name, args.len());
        
        // Find and execute the appropriate handler
        match self.handlers.get(&command_name) {
            Some(handler) => {
                log::debug!("Found handler for command: {}", command_name);
                handler.handle(bot, message, args).await
            }
            None => {
                log::debug!("Unknown command: {}", command_name);
                self.handle_unknown_command(bot, message, &command_name).await
            }
        }
    }
    
    /// Parse a command message into command name and arguments
    /// 
    /// # Arguments
    /// * `text` - The message text starting with '/'
    /// 
    /// # Returns
    /// * `Ok((command_name, args))` - The parsed command name and arguments
    /// * `Err(BotError)` - If the command format is invalid
    pub fn parse_command(&self, text: &str) -> BotResult<(String, Vec<String>)> {
        if !text.starts_with('/') {
            return Err(BotError::MessageParsing(
                "Command must start with '/'".to_string()
            ));
        }
        
        // Remove the leading '/' and split by whitespace
        let without_slash = &text[1..];
        let parts: Vec<&str> = without_slash.split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(BotError::MessageParsing(
                "Empty command".to_string()
            ));
        }
        
        let command_name = parts[0].to_lowercase();
        
        // Handle commands with @botname suffix (e.g., /start@mybotname)
        let command_name = if let Some(at_pos) = command_name.find('@') {
            command_name[..at_pos].to_string()
        } else {
            command_name
        };
        
        // Collect arguments (everything after the command name)
        let args: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();
        
        Ok((command_name, args))
    }
    
    /// Handle unknown commands with a helpful error message
    /// 
    /// # Arguments
    /// * `bot` - The Telegram bot instance
    /// * `message` - The original message
    /// * `command_name` - The unknown command name
    async fn handle_unknown_command(
        &self,
        bot: &Bot,
        message: &Message,
        command_name: &str,
    ) -> BotResult<()> {
        let available_commands: Vec<String> = self.handlers.keys().cloned().collect();
        
        let error_message = if available_commands.is_empty() {
            format!("Unknown command: /{}\n\nNo commands are currently available.", command_name)
        } else {
            format!(
                "Unknown command: /{}\n\nAvailable commands:\n{}",
                command_name,
                available_commands
                    .iter()
                    .map(|cmd| format!("/{}", cmd))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };
        
        bot.send_message(message.chat.id, error_message)
            .await
            .map_err(BotError::Telegram)?;
        
        Ok(())
    }
    
    /// Get a list of all registered command names
    pub fn get_registered_commands(&self) -> Vec<String> {
        self.handlers.keys().cloned().collect()
    }
    
    /// Check if a command is registered
    pub fn is_command_registered(&self, command_name: &str) -> bool {
        self.handlers.contains_key(&command_name.to_lowercase())
    }
    
    /// Get the number of registered commands
    pub fn command_count(&self) -> usize {
        self.handlers.len()
    }
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::commands::handler::tests::MockCommandHandler;
    
    #[test]
    fn test_command_registration() {
        let mut router = CommandRouter::new();
        let handler = Box::new(MockCommandHandler::new());
        
        router.register_command("test", handler);
        
        assert!(router.is_command_registered("test"));
        assert!(router.is_command_registered("TEST")); // Case insensitive
        assert!(!router.is_command_registered("nonexistent"));
        assert_eq!(router.command_count(), 1);
    }
    
    #[test]
    fn test_command_parsing() {
        let router = CommandRouter::new();
        
        // Test basic command
        let (cmd, args) = router.parse_command("/start").unwrap();
        assert_eq!(cmd, "start");
        assert!(args.is_empty());
        
        // Test command with arguments
        let (cmd, args) = router.parse_command("/echo hello world").unwrap();
        assert_eq!(cmd, "echo");
        assert_eq!(args, vec!["hello", "world"]);
        
        // Test command with @botname
        let (cmd, args) = router.parse_command("/start@testbot").unwrap();
        assert_eq!(cmd, "start");
        assert!(args.is_empty());
        
        // Test invalid command (no leading slash)
        assert!(router.parse_command("start").is_err());
        
        // Test empty command
        assert!(router.parse_command("/").is_err());
    }
    
    #[test]
    fn test_get_registered_commands() {
        let mut router = CommandRouter::new();
        
        assert!(router.get_registered_commands().is_empty());
        
        router.register_command("start", Box::new(MockCommandHandler::new()));
        router.register_command("help", Box::new(MockCommandHandler::new()));
        
        let commands = router.get_registered_commands();
        assert_eq!(commands.len(), 2);
        assert!(commands.contains(&"start".to_string()));
        assert!(commands.contains(&"help".to_string()));
    }
}