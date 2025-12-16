use async_trait::async_trait;
use teloxide::{Bot, types::Message};
use crate::commands::handler::{CommandHandler, CommandHandlerExt};
use crate::error::BotResult;

/// StartCommand provides a welcome message and basic bot introduction
/// 
/// This command is typically the first interaction users have with the bot.
/// It provides a friendly welcome message and basic information about the bot's capabilities.
pub struct StartCommand;

impl StartCommand {
    /// Create a new StartCommand instance
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CommandHandler for StartCommand {
    async fn handle(&self, bot: &Bot, message: &Message, _args: Vec<String>) -> BotResult<()> {
        let welcome_message = format!(
            "🤖 Welcome to the Telegram Bot Template!\n\n\
            Hello {}! I'm a template bot built with Rust and the teloxide library.\n\n\
            🚀 **What I can do:**\n\
            • Respond to commands\n\
            • Echo your messages\n\
            • Show my current status\n\
            • Provide help information\n\n\
            📚 **Getting started:**\n\
            Use /help to see all available commands\n\n\
            This bot serves as a foundation for building more complex Telegram bots. \
            Developers can extend my functionality by adding new command handlers!",
            message.from()
                .map(|user| user.first_name.as_str())
                .unwrap_or("there")
        );

        self.send_response(bot, message, &welcome_message).await
    }

    fn description(&self) -> &str {
        "Start the bot and show welcome message"
    }

    fn usage(&self) -> &str {
        "/start - Display welcome message and bot introduction"
    }

    fn name(&self) -> &str {
        "start"
    }

    fn requires_args(&self) -> bool {
        false
    }

    fn min_args(&self) -> usize {
        0
    }

    fn max_args(&self) -> Option<usize> {
        Some(0) // Start command should not accept any arguments
    }
}

impl Default for StartCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_command_metadata() {
        let command = StartCommand::new();
        
        assert_eq!(command.name(), "start");
        assert_eq!(command.description(), "Start the bot and show welcome message");
        assert_eq!(command.usage(), "/start - Display welcome message and bot introduction");
        assert!(!command.requires_args());
        assert_eq!(command.min_args(), 0);
        assert_eq!(command.max_args(), Some(0));
    }

    #[test]
    fn test_start_command_validation() {
        let command = StartCommand::new();
        
        // Should accept no arguments
        assert!(command.validate_args(&[]).is_ok());
        
        // Should reject arguments
        assert!(command.validate_args(&["arg".to_string()]).is_err());
    }

    #[test]
    fn test_start_command_default() {
        let command = StartCommand::default();
        assert_eq!(command.name(), "start");
    }
}