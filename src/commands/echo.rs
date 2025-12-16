use async_trait::async_trait;
use teloxide::{Bot, types::Message};
use crate::commands::handler::{CommandHandler, CommandHandlerExt};
use crate::error::BotResult;

/// EchoCommand repeats back the provided message text
/// 
/// This command is useful for testing message handling and demonstrating
/// basic bot functionality. It takes user input and sends it back as a response.
pub struct EchoCommand;

impl EchoCommand {
    /// Create a new EchoCommand instance
    pub fn new() -> Self {
        Self
    }

    /// Format the echo response with some visual styling
    fn format_echo_response(&self, original_text: &str) -> String {
        format!("🔄 **Echo:**\n{}", original_text)
    }

    /// Join arguments back into a single message
    fn join_args(&self, args: &[String]) -> String {
        args.join(" ")
    }
}

#[async_trait]
impl CommandHandler for EchoCommand {
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
        // Check if we have any arguments to echo
        if args.is_empty() {
            let help_message = format!(
                "❓ **Echo Command Usage**\n\n\
                Please provide a message to echo.\n\n\
                **Example:** `/echo Hello, World!`\n\
                **Result:** I'll repeat back \"Hello, World!\"\n\n\
                **Usage:** `{}`",
                self.usage()
            );
            
            self.send_response(bot, message, &help_message).await
        } else {
            // Join all arguments back into the original message
            let message_to_echo = self.join_args(&args);
            let echo_response = self.format_echo_response(&message_to_echo);
            
            // Reply to the original message to maintain context
            self.reply(bot, message, &echo_response).await
        }
    }

    fn description(&self) -> &str {
        "Echo back the provided message text"
    }

    fn usage(&self) -> &str {
        "/echo <message> - Repeat back the provided message"
    }

    fn name(&self) -> &str {
        "echo"
    }

    fn requires_args(&self) -> bool {
        true
    }

    fn min_args(&self) -> usize {
        1
    }

    fn max_args(&self) -> Option<usize> {
        None // Echo can accept unlimited arguments (they'll be joined together)
    }
}

impl Default for EchoCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_echo_command_metadata() {
        let command = EchoCommand::new();
        
        assert_eq!(command.name(), "echo");
        assert_eq!(command.description(), "Echo back the provided message text");
        assert_eq!(command.usage(), "/echo <message> - Repeat back the provided message");
        assert!(command.requires_args());
        assert_eq!(command.min_args(), 1);
        assert_eq!(command.max_args(), None);
    }

    #[test]
    fn test_echo_command_validation() {
        let command = EchoCommand::new();
        
        // Should reject no arguments
        assert!(command.validate_args(&[]).is_err());
        
        // Should accept one argument
        assert!(command.validate_args(&["hello".to_string()]).is_ok());
        
        // Should accept multiple arguments
        assert!(command.validate_args(&[
            "hello".to_string(),
            "world".to_string(),
            "test".to_string()
        ]).is_ok());
    }

    #[test]
    fn test_join_args() {
        let command = EchoCommand::new();
        
        let args = vec![
            "hello".to_string(),
            "world".to_string(),
            "test".to_string()
        ];
        
        let result = command.join_args(&args);
        assert_eq!(result, "hello world test");
    }

    #[test]
    fn test_join_args_single() {
        let command = EchoCommand::new();
        
        let args = vec!["hello".to_string()];
        let result = command.join_args(&args);
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_join_args_empty() {
        let command = EchoCommand::new();
        
        let args = vec![];
        let result = command.join_args(&args);
        assert_eq!(result, "");
    }

    #[test]
    fn test_format_echo_response() {
        let command = EchoCommand::new();
        
        let response = command.format_echo_response("Hello, World!");
        assert!(response.contains("🔄"));
        assert!(response.contains("Echo:"));
        assert!(response.contains("Hello, World!"));
    }

    #[test]
    fn test_format_echo_response_empty() {
        let command = EchoCommand::new();
        
        let response = command.format_echo_response("");
        assert!(response.contains("🔄"));
        assert!(response.contains("Echo:"));
    }

    #[test]
    fn test_echo_command_default() {
        let command = EchoCommand::default();
        assert_eq!(command.name(), "echo");
    }
}