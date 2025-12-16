use async_trait::async_trait;
use teloxide::{Bot, types::Message};
use crate::commands::handler::{CommandHandler, CommandHandlerExt};
use crate::error::BotResult;

/// HelpCommand lists all available commands and their usage information
/// 
/// This command provides users with a comprehensive list of available bot commands,
/// their descriptions, and usage instructions. It serves as the primary discovery
/// mechanism for bot functionality.
pub struct HelpCommand {
    /// Reference to available commands for generating help text
    /// This will be populated by the CommandRouter when the command is registered
    commands: Vec<CommandInfo>,
}

/// Information about a command for help display
#[derive(Clone, Debug)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub usage: String,
}

impl HelpCommand {
    /// Create a new HelpCommand instance
    pub fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }

    /// Create a HelpCommand with a list of available commands
    /// 
    /// This constructor is used by the CommandRouter to provide the help command
    /// with information about all registered commands.
    pub fn with_commands(commands: Vec<CommandInfo>) -> Self {
        Self { commands }
    }

    /// Update the list of available commands
    /// 
    /// This method allows the CommandRouter to update the help command
    /// when new commands are registered.
    pub fn update_commands(&mut self, commands: Vec<CommandInfo>) {
        self.commands = commands;
    }

    /// Generate the help message text
    fn generate_help_message(&self) -> String {
        let mut help_text = String::from(
            "🤖 **Bot Commands Help**\n\n\
            Here are all the commands I understand:\n\n"
        );

        if self.commands.is_empty() {
            help_text.push_str("No commands are currently available.");
        } else {
            // Sort commands alphabetically for consistent display
            let mut sorted_commands = self.commands.clone();
            sorted_commands.sort_by(|a, b| a.name.cmp(&b.name));

            for command in sorted_commands {
                help_text.push_str(&format!(
                    "**{}**\n{}\n`{}`\n\n",
                    command.name.to_uppercase(),
                    command.description,
                    command.usage
                ));
            }

            help_text.push_str(
                "💡 **Tips:**\n\
                • Commands are case-insensitive\n\
                • Use /help anytime to see this message\n\
                • Arguments in <brackets> are required\n\
                • Arguments in [brackets] are optional"
            );
        }

        help_text
    }
}

#[async_trait]
impl CommandHandler for HelpCommand {
    async fn handle(&self, bot: &Bot, message: &Message, args: Vec<String>) -> BotResult<()> {
        // If a specific command is requested, show detailed help for that command
        if let Some(command_name) = args.first() {
            let command_name = command_name.to_lowercase();
            
            if let Some(command_info) = self.commands.iter().find(|cmd| cmd.name == command_name) {
                let detailed_help = format!(
                    "📖 **Help for /{command_name}**\n\n\
                    **Description:** {description}\n\n\
                    **Usage:** `{usage}`\n\n\
                    Use /help to see all available commands.",
                    command_name = command_info.name,
                    description = command_info.description,
                    usage = command_info.usage
                );
                
                self.send_response(bot, message, &detailed_help).await
            } else {
                let error_message = format!(
                    "❌ Unknown command: /{}\n\n\
                    Use /help to see all available commands.",
                    command_name
                );
                
                self.send_response(bot, message, &error_message).await
            }
        } else {
            // Show general help with all commands
            let help_message = self.generate_help_message();
            self.send_response(bot, message, &help_message).await
        }
    }

    fn description(&self) -> &str {
        "Show help information for all commands or a specific command"
    }

    fn usage(&self) -> &str {
        "/help [command] - Show all commands or detailed help for a specific command"
    }

    fn name(&self) -> &str {
        "help"
    }

    fn requires_args(&self) -> bool {
        false
    }

    fn min_args(&self) -> usize {
        0
    }

    fn max_args(&self) -> Option<usize> {
        Some(1) // Help command accepts at most one argument (the command name)
    }
}

impl Default for HelpCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_command_metadata() {
        let command = HelpCommand::new();
        
        assert_eq!(command.name(), "help");
        assert_eq!(command.description(), "Show help information for all commands or a specific command");
        assert_eq!(command.usage(), "/help [command] - Show all commands or detailed help for a specific command");
        assert!(!command.requires_args());
        assert_eq!(command.min_args(), 0);
        assert_eq!(command.max_args(), Some(1));
    }

    #[test]
    fn test_help_command_validation() {
        let command = HelpCommand::new();
        
        // Should accept no arguments
        assert!(command.validate_args(&[]).is_ok());
        
        // Should accept one argument
        assert!(command.validate_args(&["start".to_string()]).is_ok());
        
        // Should reject multiple arguments
        assert!(command.validate_args(&["start".to_string(), "extra".to_string()]).is_err());
    }

    #[test]
    fn test_help_command_with_commands() {
        let commands = vec![
            CommandInfo {
                name: "start".to_string(),
                description: "Start the bot".to_string(),
                usage: "/start - Start the bot".to_string(),
            },
            CommandInfo {
                name: "echo".to_string(),
                description: "Echo a message".to_string(),
                usage: "/echo <message> - Echo the message".to_string(),
            },
        ];

        let command = HelpCommand::with_commands(commands.clone());
        assert_eq!(command.commands.len(), 2);
        assert_eq!(command.commands[0].name, "start");
        assert_eq!(command.commands[1].name, "echo");
    }

    #[test]
    fn test_generate_help_message_empty() {
        let command = HelpCommand::new();
        let help_message = command.generate_help_message();
        
        assert!(help_message.contains("Bot Commands Help"));
        assert!(help_message.contains("No commands are currently available"));
    }

    #[test]
    fn test_generate_help_message_with_commands() {
        let commands = vec![
            CommandInfo {
                name: "start".to_string(),
                description: "Start the bot".to_string(),
                usage: "/start - Start the bot".to_string(),
            },
        ];

        let command = HelpCommand::with_commands(commands);
        let help_message = command.generate_help_message();
        
        assert!(help_message.contains("Bot Commands Help"));
        assert!(help_message.contains("START"));
        assert!(help_message.contains("Start the bot"));
        assert!(help_message.contains("/start - Start the bot"));
        assert!(help_message.contains("Tips:"));
    }

    #[test]
    fn test_update_commands() {
        let mut command = HelpCommand::new();
        assert_eq!(command.commands.len(), 0);

        let commands = vec![
            CommandInfo {
                name: "test".to_string(),
                description: "Test command".to_string(),
                usage: "/test - Test command".to_string(),
            },
        ];

        command.update_commands(commands);
        assert_eq!(command.commands.len(), 1);
        assert_eq!(command.commands[0].name, "test");
    }
}