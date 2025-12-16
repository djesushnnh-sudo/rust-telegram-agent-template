use teloxide::{Bot, types::Message, prelude::Requester, utils::command::BotCommands, payloads::SendMessageSetters};
use crate::error::{BotError, BotResult};
use crate::state::AppState;
use crate::commands::handler::CommandHandler as HandlerTrait;
use std::sync::Arc;

pub mod handler;
pub mod start;
pub mod help;
pub mod echo;
pub mod status;

pub use start::StartCommand;
pub use help::{HelpCommand, CommandInfo};
pub use echo::EchoCommand;
pub use status::StatusCommand;

/// Bot commands using the BotCommands derive macro for type-safe command handling
/// 
/// This enum defines all available bot commands with their descriptions and argument parsing.
/// Based on the pattern from Dads2Dads for clean command management.
#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum BotCommand {
    #[command(description = "Start the bot and show welcome message")]
    Start,
    
    #[command(description = "Show help information and available commands")]
    Help,
    
    #[command(description = "Echo back your message: /echo <message>")]
    Echo(String),
    
    #[command(description = "Show bot status and health information")]
    Status,
    
    // TODO: Add your custom commands here following the same pattern
    // Examples:
    // #[command(description = "Get weather information: /weather <city>")]
    // Weather(String),
    // 
    // #[command(description = "Set user preferences: /settings <key> <value>")]
    // Settings(String),
}

/// Modern command handler using BotCommands derive macro
/// 
/// This handler processes commands using Teloxide's built-in command parsing
/// and routes them to appropriate handlers with shared state access.
pub struct CommandHandler {
    state: Arc<AppState>,
}

impl CommandHandler {
    /// Create a new command handler with shared state
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
    
    /// Handle a bot command with type-safe parsing
    pub async fn handle_command(&self, bot: Bot, msg: Message, cmd: BotCommand) -> BotResult<()> {
        log::debug!("Processing command: {:?}", cmd);
        
        match cmd {
            BotCommand::Start => {
                self.handle_start(bot, msg).await
            }
            BotCommand::Help => {
                self.handle_help(bot, msg).await
            }
            BotCommand::Echo(text) => {
                self.handle_echo(bot, msg, text).await
            }
            BotCommand::Status => {
                self.handle_status(bot, msg).await
            }
            // TODO: Add handlers for your custom commands here
            // BotCommand::Weather(city) => {
            //     self.handle_weather(bot, msg, city).await
            // }
        }
    }
    
    /// Handle the /start command
    async fn handle_start(&self, bot: Bot, msg: Message) -> BotResult<()> {
        let start_handler = StartCommand::new();
        HandlerTrait::handle(&start_handler, &bot, &msg, vec![]).await
    }

    /// Handle the /help command
    async fn handle_help(&self, bot: Bot, msg: Message) -> BotResult<()> {
        let help_text = format!(
            "🤖 <b>Available Commands:</b>\n\n{}\n\n💡 This bot supports AI routing and message processing.",
            BotCommand::descriptions()
        );
        
        bot.send_message(msg.chat.id, help_text)
            .parse_mode(teloxide::types::ParseMode::Html)
            .await
            .map_err(BotError::Telegram)?;
        
        Ok(())
    }

    /// Handle the /echo command
    async fn handle_echo(&self, bot: Bot, msg: Message, text: String) -> BotResult<()> {
        if text.trim().is_empty() {
            bot.send_message(msg.chat.id, "❌ Please provide a message to echo.\n\nUsage: /echo <message>")
                .await
                .map_err(BotError::Telegram)?;
            return Ok(());
        }

        let echo_handler = EchoCommand::new();
        HandlerTrait::handle(&echo_handler, &bot, &msg, vec![text]).await
    }

    /// Handle the /status command
    async fn handle_status(&self, bot: Bot, msg: Message) -> BotResult<()> {
        let status_handler = StatusCommand::new();
        HandlerTrait::handle(&status_handler, &bot, &msg, vec![]).await
    }
    
    /// Handle regular (non-command) messages for AI routing
    pub async fn handle_message(&self, bot: Bot, msg: Message) -> BotResult<()> {
        // Get user info for session management
        let user_id = match msg.from() {
            Some(user) => user.id,
            None => {
                log::debug!("Message from unknown user, ignoring");
                return Ok(());
            }
        };

        let chat_id = msg.chat.id;
        
        // Get or create user session
        let session = self.state.get_or_create_session(user_id, chat_id);
        
        // Add message to conversation context
        if let Some(text) = msg.text() {
            self.state.add_to_conversation_context(user_id, text.to_string());
        }

        // Route to AI if enabled for this user, otherwise send a helpful message
        if session.ai_enabled {
            // TODO: Route to AI processor
            // This is where you'd integrate with your AI service
            log::debug!("Routing message to AI for user {}", user_id);
            
            bot.send_message(chat_id, "🤖 AI processing is enabled but not yet implemented.\n\nUse /help to see available commands.")
                .await
                .map_err(BotError::Telegram)?;
        } else {
            // Send helpful message for non-AI users
            bot.send_message(chat_id, "👋 Hello! I'm a Telegram bot.\n\nUse /help to see what I can do, or /start to get started.")
                .await
                .map_err(BotError::Telegram)?;
        }

        Ok(())
    }
}

impl Default for CommandHandler {
    fn default() -> Self {
        Self::new(Arc::new(AppState::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bot_commands_descriptions() {
        let descriptions = BotCommand::descriptions().to_string();
        assert!(descriptions.contains("start"));
        assert!(descriptions.contains("help"));
        assert!(descriptions.contains("echo"));
        assert!(descriptions.contains("status"));
    }
    
    #[test]
    fn test_command_handler_creation() {
        let state = Arc::new(AppState::new());
        let _handler = CommandHandler::new(state);
        // Just test that it creates without panicking
        assert!(true);
    }
}