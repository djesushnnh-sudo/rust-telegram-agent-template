use async_trait::async_trait;
use teloxide::{Bot, types::Message, prelude::Requester, payloads::SendMessageSetters};
use crate::error::BotResult;

/// Trait for implementing command handlers
/// 
/// All command implementations must implement this trait to be registered
/// with the CommandRouter. The trait provides a standardized interface
/// for command execution and metadata.
#[async_trait]
pub trait CommandHandler: Send + Sync {
    /// Handle the execution of this command
    /// 
    /// # Arguments
    /// * `bot` - The Telegram bot instance for sending responses
    /// * `message` - The original message that triggered this command
    /// * `args` - Command arguments parsed from the message text
    /// 
    /// # Returns
    /// * `Ok(())` if the command was executed successfully
    /// * `Err(BotError)` if there was an error during execution
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()>;
    
    /// Get a human-readable description of what this command does
    /// 
    /// This description is used in help messages and documentation.
    /// 
    /// # Returns
    /// A string describing the command's functionality
    fn description(&self) -> &str;
    
    /// Get usage information for this command
    /// 
    /// This should include the command syntax and any required or optional arguments.
    /// 
    /// # Returns
    /// A string showing how to use the command
    /// 
    /// # Example
    /// ```ignore
    /// fn usage(&self) -> &str {
    ///     "/echo <message> - Repeats the provided message"
    /// }
    /// ```
    fn usage(&self) -> &str;
    
    /// Get the command name (without the leading slash)
    /// 
    /// This is used for command registration and routing.
    /// 
    /// # Returns
    /// The command name as a string
    fn name(&self) -> &str;
    
    /// Check if this command requires arguments
    /// 
    /// This can be used by the router to provide better error messages
    /// when commands are called without required arguments.
    /// 
    /// # Returns
    /// `true` if the command requires arguments, `false` otherwise
    fn requires_args(&self) -> bool {
        false // Default implementation - most commands don't require args
    }
    
    /// Get the minimum number of arguments required by this command
    /// 
    /// # Returns
    /// The minimum number of arguments required (0 by default)
    fn min_args(&self) -> usize {
        0 // Default implementation
    }
    
    /// Get the maximum number of arguments accepted by this command
    /// 
    /// # Returns
    /// The maximum number of arguments, or None for unlimited
    fn max_args(&self) -> Option<usize> {
        None // Default implementation - unlimited args
    }
    
    /// Validate the provided arguments
    /// 
    /// This method can be overridden to provide custom argument validation
    /// before the command is executed.
    /// 
    /// # Arguments
    /// * `args` - The command arguments to validate
    /// 
    /// # Returns
    /// * `Ok(())` if arguments are valid
    /// * `Err(BotError)` if arguments are invalid
    fn validate_args(&self, args: &[String]) -> BotResult<()> {
        // Check minimum arguments
        if args.len() < self.min_args() {
            return Err(crate::error::BotError::Command(format!(
                "Command '{}' requires at least {} arguments, got {}. Usage: {}",
                self.name(),
                self.min_args(),
                args.len(),
                self.usage()
            )));
        }
        
        // Check maximum arguments
        if let Some(max) = self.max_args() {
            if args.len() > max {
                return Err(crate::error::BotError::Command(format!(
                    "Command '{}' accepts at most {} arguments, got {}. Usage: {}",
                    self.name(),
                    max,
                    args.len(),
                    self.usage()
                )));
            }
        }
        
        Ok(())
    }
}

/// Helper trait for creating command handlers with common functionality
pub trait CommandHandlerExt: CommandHandler {
    /// Send a simple text response to the user
    async fn send_response(&self, bot: &Bot, message: &Message, text: &str) -> BotResult<()> {
        bot.send_message(message.chat.id, text)
            .await
            .map_err(crate::error::BotError::Telegram)?;
        Ok(())
    }
    
    /// Send a formatted response with markdown
    async fn send_markdown_response(&self, bot: &Bot, message: &Message, text: &str) -> BotResult<()> {
        bot.send_message(message.chat.id, text)
            .parse_mode(teloxide::types::ParseMode::MarkdownV2)
            .await
            .map_err(crate::error::BotError::Telegram)?;
        Ok(())
    }
    
    /// Reply to the original message
    async fn reply(&self, bot: &Bot, message: &Message, text: &str) -> BotResult<()> {
        bot.send_message(message.chat.id, text)
            .reply_to_message_id(message.id)
            .await
            .map_err(crate::error::BotError::Telegram)?;
        Ok(())
    }
}

// Blanket implementation of CommandHandlerExt for all CommandHandler implementations
impl<T: CommandHandler> CommandHandlerExt for T {}

#[cfg(test)]
pub mod tests {
    use super::*;
    
    /// Mock command handler for testing
    pub struct MockCommandHandler {
        name: String,
        description: String,
        usage: String,
        requires_args: bool,
        min_args: usize,
        max_args: Option<usize>,
    }
    
    impl MockCommandHandler {
        pub fn new() -> Self {
            Self {
                name: "mock".to_string(),
                description: "A mock command for testing".to_string(),
                usage: "/mock - A test command".to_string(),
                requires_args: false,
                min_args: 0,
                max_args: None,
            }
        }
        
        pub fn with_name(mut self, name: &str) -> Self {
            self.name = name.to_string();
            self
        }
        
        pub fn with_description(mut self, description: &str) -> Self {
            self.description = description.to_string();
            self
        }
        
        pub fn with_usage(mut self, usage: &str) -> Self {
            self.usage = usage.to_string();
            self
        }
        
        pub fn with_args_requirement(mut self, requires_args: bool, min_args: usize, max_args: Option<usize>) -> Self {
            self.requires_args = requires_args;
            self.min_args = min_args;
            self.max_args = max_args;
            self
        }
    }
    
    #[async_trait]
    impl CommandHandler for MockCommandHandler {
        async fn handle(&self, _bot: &Bot, _message: &Message, _args: Vec<String>) -> BotResult<()> {
            // Mock implementation - just return Ok
            Ok(())
        }
        
        fn description(&self) -> &str {
            &self.description
        }
        
        fn usage(&self) -> &str {
            &self.usage
        }
        
        fn name(&self) -> &str {
            &self.name
        }
        
        fn requires_args(&self) -> bool {
            self.requires_args
        }
        
        fn min_args(&self) -> usize {
            self.min_args
        }
        
        fn max_args(&self) -> Option<usize> {
            self.max_args
        }
    }
    
    // Note: Creating test messages is complex due to teloxide's structure
    // For property-based tests, we'll use simpler mock structures
    
    #[test]
    fn test_mock_command_handler() {
        let handler = MockCommandHandler::new()
            .with_name("test")
            .with_description("Test command")
            .with_usage("/test - A test command")
            .with_args_requirement(true, 1, Some(3));
        
        assert_eq!(handler.name(), "test");
        assert_eq!(handler.description(), "Test command");
        assert_eq!(handler.usage(), "/test - A test command");
        assert!(handler.requires_args());
        assert_eq!(handler.min_args(), 1);
        assert_eq!(handler.max_args(), Some(3));
    }
    
    #[test]
    fn test_argument_validation() {
        let handler = MockCommandHandler::new()
            .with_args_requirement(true, 2, Some(4));
        
        // Test too few arguments
        let result = handler.validate_args(&["one".to_string()]);
        assert!(result.is_err());
        
        // Test valid number of arguments
        let result = handler.validate_args(&["one".to_string(), "two".to_string()]);
        assert!(result.is_ok());
        
        let result = handler.validate_args(&["one".to_string(), "two".to_string(), "three".to_string()]);
        assert!(result.is_ok());
        
        // Test too many arguments
        let result = handler.validate_args(&[
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
        ]);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_no_args_validation() {
        let handler = MockCommandHandler::new(); // Default: no args required
        
        // Should accept any number of arguments
        assert!(handler.validate_args(&[]).is_ok());
        assert!(handler.validate_args(&["one".to_string()]).is_ok());
        assert!(handler.validate_args(&["one".to_string(), "two".to_string()]).is_ok());
    }
}