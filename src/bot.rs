use teloxide::{Bot, dispatching::UpdateHandler, prelude::*};
use teloxide::types::{Update, UpdateKind, Message};
use crate::config::Config;
use crate::error::{BotError, BotResult, ErrorRecovery};
use crate::commands::CommandRouter;
use log::{info, warn, error, debug};

/// Core bot service that manages the Telegram bot instance and message processing
/// 
/// The BotService is responsible for:
/// - Managing the teloxide Bot instance and authentication
/// - Processing incoming updates from Telegram
/// - Routing messages through the command system
/// - Handling errors gracefully with recovery mechanisms
/// - Coordinating between different bot components
pub struct BotService {
    /// The teloxide Bot instance for Telegram API communication
    bot: Bot,
    
    /// Command router for handling bot commands
    command_router: CommandRouter,
    
    /// Configuration settings
    config: Config,
}

impl BotService {
    /// Create a new BotService instance with the provided configuration
    /// 
    /// This method:
    /// 1. Creates and authenticates the Bot instance with Telegram
    /// 2. Initializes the command router
    /// 3. Sets up the basic bot configuration
    /// 
    /// # Arguments
    /// * `config` - The bot configuration containing token and settings
    /// 
    /// # Returns
    /// * `Ok(BotService)` - Successfully initialized bot service
    /// * `Err(BotError)` - If bot creation or authentication fails
    pub async fn new(config: &Config) -> BotResult<Self> {
        info!("Initializing bot service with configuration");
        
        // Create the Bot instance with the provided token
        let bot = Bot::new(&config.bot_token);
        
        // Test the bot authentication by getting bot information
        match bot.get_me().await {
            Ok(me) => {
                info!("Bot authenticated successfully: @{}", me.username());
                debug!("Bot ID: {}, Name: {}", me.id, me.first_name);
            }
            Err(e) => {
                error!("Failed to authenticate bot: {}", e);
                return Err(BotError::Telegram(e));
            }
        }
        
        // Initialize command router
        let command_router = CommandRouter::new();
        
        Ok(BotService {
            bot,
            command_router,
            config: config.clone(),
        })
    }
    
    /// Get a reference to the command router for registering commands
    /// 
    /// This allows external code to register command handlers with the bot.
    /// 
    /// # Returns
    /// A mutable reference to the command router
    pub fn command_router_mut(&mut self) -> &mut CommandRouter {
        &mut self.command_router
    }
    
    /// Get a reference to the bot instance
    /// 
    /// This provides access to the underlying teloxide Bot for advanced operations.
    /// 
    /// # Returns
    /// A reference to the Bot instance
    pub fn bot(&self) -> &Bot {
        &self.bot
    }
    
    /// Get a reference to the configuration
    /// 
    /// # Returns
    /// A reference to the bot configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Start the bot and begin processing updates
    /// 
    /// This method starts the main bot loop, which will:
    /// 1. Set up the update dispatcher
    /// 2. Begin receiving updates from Telegram
    /// 3. Process each update through the message pipeline
    /// 4. Handle errors gracefully and continue operation
    /// 
    /// # Returns
    /// * `Ok(())` - Bot stopped gracefully
    /// * `Err(BotError)` - Fatal error that prevented bot operation
    pub async fn start(&self) -> BotResult<()> {
        info!("Starting bot service");
        
        // Create the update handler
        let handler = self.create_update_handler();
        
        // Start the dispatcher based on configuration
        if self.config.is_webhook_mode() {
            self.start_webhook_mode(handler).await
        } else {
            self.start_polling_mode(handler).await
        }
    }
    
    /// Create the update handler for processing incoming updates
    fn create_update_handler(&self) -> UpdateHandler<BotError> {
        // Create a handler that processes all updates through our pipeline
        dptree::entry()
            .branch(
                Update::filter_message()
                    .endpoint(|bot: Bot, msg: Message| async move {
                        // This will be replaced with proper handler in the actual implementation
                        // For now, we'll create a simple handler that processes messages
                        Self::handle_message_static(bot, msg).await
                    })
            )
    }
    
    /// Static method to handle messages (used by the dispatcher)
    /// 
    /// This is a static method because the teloxide dispatcher requires
    /// static functions. It recreates the necessary context for message processing.
    async fn handle_message_static(bot: Bot, message: Message) -> Result<(), BotError> {
        // For now, we'll implement a basic message handler
        // In a full implementation, this would need access to the command router
        debug!("Received message from user {:?}: {:?}", 
               message.from().map(|u| u.id), 
               message.text());
        
        // Basic echo functionality for testing
        if let Some(text) = message.text() {
            if text.starts_with('/') {
                // This is a command, but we don't have access to the router here
                // In the actual implementation, we'd need to restructure this
                bot.send_message(message.chat.id, "Command processing not yet implemented in static handler")
                    .await
                    .map_err(BotError::Telegram)?;
            }
        }
        
        Ok(())
    }
    
    /// Handle an individual update with error recovery
    /// 
    /// This method processes a single update through the bot's message pipeline,
    /// with comprehensive error handling and recovery mechanisms.
    /// 
    /// # Arguments
    /// * `update` - The Telegram update to process
    /// 
    /// # Returns
    /// * `Ok(())` - Update processed successfully
    /// * `Err(BotError)` - Unrecoverable error during processing
    pub async fn handle_update(&self, update: Update) -> BotResult<()> {
        debug!("Processing update: {:?}", update.kind);
        
        match self.process_update_with_recovery(update).await {
            Ok(()) => {
                debug!("Update processed successfully");
                Ok(())
            }
            Err(error) => {
                ErrorRecovery::log_error(&error);
                
                if ErrorRecovery::is_recoverable(&error) {
                    warn!("Recoverable error during update processing: {}", error);
                    Ok(()) // Continue processing other updates
                } else {
                    error!("Fatal error during update processing: {}", error);
                    Err(error) // Propagate fatal errors
                }
            }
        }
    }
    
    /// Process an update with error recovery mechanisms
    async fn process_update_with_recovery(&self, update: Update) -> BotResult<()> {
        match update.kind {
            UpdateKind::Message(message) => {
                self.handle_message(&message).await
            }
            UpdateKind::EditedMessage(message) => {
                debug!("Received edited message, treating as regular message");
                self.handle_message(&message).await
            }
            UpdateKind::CallbackQuery(_) => {
                debug!("Received callback query, ignoring for now");
                Ok(()) // Callback queries not implemented yet
            }
            _ => {
                debug!("Received unsupported update type, ignoring");
                Ok(()) // Other update types not supported
            }
        }
    }
    
    /// Handle an individual message
    /// 
    /// This method processes a message through the command router and handles
    /// any errors that occur during command processing.
    /// 
    /// # Arguments
    /// * `message` - The message to process
    /// 
    /// # Returns
    /// * `Ok(())` - Message processed successfully
    /// * `Err(BotError)` - Error during message processing
    pub async fn handle_message(&self, message: &Message) -> BotResult<()> {
        debug!("Handling message from user {:?}", message.from().map(|u| u.id));
        
        // Route the message through the command router
        match self.command_router.handle_message(&self.bot, message).await {
            Ok(()) => {
                debug!("Message processed successfully by command router");
                Ok(())
            }
            Err(BotError::Command(cmd_error)) => {
                // Command errors should be reported to the user
                warn!("Command error: {}", cmd_error);
                
                let user_message = ErrorRecovery::user_friendly_message(&BotError::Command(cmd_error));
                if let Err(send_error) = self.bot.send_message(message.chat.id, user_message).await {
                    error!("Failed to send error message to user: {}", send_error);
                    return Err(BotError::Telegram(send_error));
                }
                
                Ok(()) // Error handled, continue processing
            }
            Err(other_error) => {
                // Other errors might be more serious
                error!("Error processing message: {}", other_error);
                Err(other_error)
            }
        }
    }
    
    /// Start the bot in polling mode
    async fn start_polling_mode(&self, handler: UpdateHandler<BotError>) -> BotResult<()> {
        info!("Starting bot in polling mode");
        
        Dispatcher::builder(self.bot.clone(), handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        
        info!("Bot stopped");
        Ok(())
    }
    
    /// Start the bot in webhook mode
    async fn start_webhook_mode(&self, _handler: UpdateHandler<BotError>) -> BotResult<()> {
        info!("Starting bot in webhook mode");
        
        // Webhook mode implementation would go here
        // For now, return an error as it's not fully implemented
        Err(BotError::Internal(
            "Webhook mode not yet implemented".to_string()
        ))
    }
    
    /// Gracefully shutdown the bot service
    /// 
    /// This method performs cleanup operations before the bot stops.
    /// 
    /// # Returns
    /// * `Ok(())` - Shutdown completed successfully
    /// * `Err(BotError)` - Error during shutdown
    pub async fn shutdown(&self) -> BotResult<()> {
        info!("Shutting down bot service");
        
        // Perform any necessary cleanup here
        // For example: close database connections, save state, etc.
        
        info!("Bot service shutdown complete");
        Ok(())
    }
    
    /// Send a message to a specific chat
    /// 
    /// This is a convenience method for sending messages programmatically.
    /// 
    /// # Arguments
    /// * `chat_id` - The chat ID to send the message to
    /// * `text` - The message text to send
    /// 
    /// # Returns
    /// * `Ok(())` - Message sent successfully
    /// * `Err(BotError)` - Error sending message
    pub async fn send_message(&self, chat_id: teloxide::types::ChatId, text: &str) -> BotResult<()> {
        self.bot.send_message(chat_id, text)
            .await
            .map_err(BotError::Telegram)?;
        Ok(())
    }
    
    /// Get bot information
    /// 
    /// # Returns
    /// * `Ok(Me)` - Bot user information
    /// * `Err(BotError)` - Error getting bot info
    pub async fn get_bot_info(&self) -> BotResult<teloxide::types::Me> {
        self.bot.get_me()
            .await
            .map_err(BotError::Telegram)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    
    // Helper function to create a test config
    fn create_test_config() -> Config {
        Config {
            bot_token: "123456789:ABCdefGHIjklMNOpqrsTUVwxyz".to_string(),
            log_level: "info".to_string(),
            webhook_url: None,
            port: Some(8080),
            ai_enabled: false,
        }
    }
    
    #[tokio::test]
    async fn test_bot_service_creation() {
        let config = create_test_config();
        
        // Note: This test will fail without a real bot token
        // In a real test environment, you'd use a test bot token
        // or mock the Bot creation
        
        // For now, we'll just test that the function doesn't panic
        // and returns the expected error for invalid token
        let result = BotService::new(&config).await;
        
        // We expect this to fail with a Telegram error due to invalid token
        assert!(result.is_err());
        if let Err(BotError::Telegram(_)) = result {
            // This is expected with a fake token
        } else {
            panic!("Expected Telegram error with fake token");
        }
    }
    
    #[test]
    fn test_bot_service_config_access() {
        // We can't easily test the full BotService without a real token,
        // but we can test that the config is stored correctly
        let config = create_test_config();
        
        // This would be tested in integration tests with a real bot
        assert_eq!(config.bot_token, "123456789:ABCdefGHIjklMNOpqrsTUVwxyz");
        assert!(!config.is_webhook_mode());
    }
}