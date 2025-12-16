//! AI Router Module
//! 
//! This module provides a placeholder for AI message processing functionality.
//! It cleanly separates AI logic from Telegram bot logic, making it easy to
//! integrate actual AI services when needed.

pub mod processor;

use anyhow::Result;
use teloxide::types::Message;
use crate::error::BotResult;
pub use processor::{AIProcessor, AIContext, ModelConfig, UserPreferences, ResponseStyle};

/// AI Router for handling message processing and routing decisions
/// 
/// The AIRouter coordinates between message routing decisions and AI processing.
/// It determines whether messages should be processed by AI and handles the
/// routing logic between regular commands and AI-powered responses.
pub struct AIRouter {
    /// Whether AI functionality is enabled
    enabled: bool,
    
    /// AI processor for handling AI-powered message processing
    processor: AIProcessor,
}

impl AIRouter {
    /// Create a new AI router instance
    /// 
    /// # Arguments
    /// * `enabled` - Whether AI functionality is enabled
    /// * `api_key` - Optional API key for AI services
    /// 
    /// # Returns
    /// A new AIRouter instance
    pub fn new(enabled: bool, api_key: Option<String>) -> Self {
        let processor = AIProcessor::new(enabled, api_key);
        Self { 
            enabled,
            processor,
        }
    }
    
    /// Create AI router with custom processor configuration
    /// 
    /// # Arguments
    /// * `enabled` - Whether AI functionality is enabled
    /// * `processor` - Custom AI processor instance
    /// 
    /// # Returns
    /// A new AIRouter instance with custom processor
    pub fn with_processor(enabled: bool, processor: AIProcessor) -> Self {
        Self {
            enabled,
            processor,
        }
    }

    /// Check if a message should be processed by AI
    /// 
    /// This method implements the core routing decision logic.
    /// It determines whether a message should go through AI processing
    /// or be handled by the regular command system.
    /// 
    /// # Arguments
    /// * `message` - The Telegram message to evaluate
    /// 
    /// # Returns
    /// True if the message should be processed by AI
    pub async fn should_process(&self, message: &Message) -> bool {
        if !self.enabled {
            return false;
        }
        
        self.processor.should_process_with_ai(message).await
    }
    
    /// Check if AI functionality is enabled
    /// 
    /// # Returns
    /// True if AI processing is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
    
    /// Check if AI router is ready for processing
    /// 
    /// # Returns
    /// True if AI router is properly configured and ready
    pub fn is_ready(&self) -> bool {
        self.enabled && self.processor.is_ready()
    }

    /// Process a message through AI routing and generate a response
    /// 
    /// This is the main entry point for AI message processing.
    /// It handles the complete flow from message routing decision
    /// through AI processing to response generation.
    /// 
    /// ## AI Service Integration Guide
    /// 
    /// This is the main integration point for AI services. Replace the placeholder
    /// implementation in `AIProcessor::process_message()` with your chosen AI service.
    /// 
    /// ### Popular AI Service Options:
    /// 
    /// #### Cloud-Based Services:
    /// - **OpenAI GPT**: Best for general conversation and text generation
    /// - **Anthropic Claude**: Excellent for reasoning and analysis tasks
    /// - **Google Gemini**: Good for multimodal tasks (text + images)
    /// - **Cohere**: Specialized in enterprise AI applications
    /// 
    /// #### Local/Self-Hosted Options:
    /// - **Ollama**: Easy local LLM deployment (llama2, mistral, etc.)
    /// - **Hugging Face Transformers**: Direct model integration
    /// - **LM Studio**: Local model serving with OpenAI-compatible API
    /// - **vLLM**: High-performance inference server
    /// 
    /// #### Custom Solutions:
    /// - **Custom ML endpoints**: Your own trained models
    /// - **Hybrid approaches**: Combine multiple AI services
    /// - **Specialized models**: Domain-specific AI (medical, legal, etc.)
    /// 
    /// ### Implementation Steps:
    /// 1. Choose your AI service from the options above
    /// 2. Add required dependencies to `Cargo.toml`
    /// 3. Update `AIProcessor::process_message()` with actual API calls
    /// 4. Add necessary configuration fields to `Config`
    /// 5. Update `.env.example` with required API keys
    /// 6. Test with various message types and edge cases
    /// 
    /// # Arguments
    /// * `message` - The Telegram message to process
    /// 
    /// # Returns
    /// * `Ok(Some(String))` - AI-generated response
    /// * `Ok(None)` - Message should not be processed by AI
    /// * `Err(BotError)` - Processing error
    pub async fn process_message(&self, message: &Message) -> BotResult<Option<String>> {
        // First check if this message should be processed by AI
        if !self.should_process(message).await {
            return Ok(None);
        }
        
        // Extract message text
        let message_text = match message.text() {
            Some(text) => text,
            None => return Ok(None), // Non-text messages not supported yet
        };
        
        // Create AI context from the message
        let context = self.processor.create_context_from_message(message);
        
        // Process through AI - this is where the magic happens!
        // The actual AI integration happens in the processor module.
        // See src/ai/processor.rs for implementation details and integration guides.
        match self.processor.process_message(message_text, &context).await {
            Ok(response) => Ok(Some(response)),
            Err(e) => {
                log::error!("AI processing error: {}", e);
                Err(crate::error::BotError::AI(format!("AI processing failed: {}", e)))
            }
        }
    }

    /// Legacy AI routing function for backward compatibility
    /// 
    /// # Arguments
    /// * `message_text` - The text content of the message to process
    /// * `user_id` - The Telegram user ID of the sender
    /// * `session_id` - Optional session identifier for conversation context
    /// 
    /// # Returns
    /// A response from AI processing
    pub async fn ai_router(
        &self,
        message_text: &str,
        user_id: i64,
        session_id: Option<String>,
    ) -> Result<String> {
        // Log the AI processing attempt for debugging
        log::info!(
            "AI router called - User: {}, Session: {:?}, Message: '{}'",
            user_id,
            session_id,
            message_text
        );
        
        if !self.enabled {
            return Ok("AI functionality is currently disabled.".to_string());
        }

        // Create context for legacy interface
        let context = AIContext {
            user_id: teloxide::types::UserId(user_id as u64),
            chat_id: teloxide::types::ChatId(user_id), // Assume private chat for legacy
            session_id,
            message_history: Vec::new(),
            user_preferences: None,
        };
        
        // Process through AI processor
        self.processor.process_message(message_text, &context).await
    }

    /// Alternative routing function name for clarity
    pub async fn route_ai_message(
        &self,
        message_text: &str,
        user_id: i64,
        session_id: Option<String>,
    ) -> Result<String> {
        self.ai_router(message_text, user_id, session_id).await
    }
    
    /// Enable or disable AI functionality
    /// 
    /// # Arguments
    /// * `enabled` - Whether to enable AI processing
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
        self.processor.set_enabled(enabled);
        log::info!("AI router {}", if enabled { "enabled" } else { "disabled" });
    }
    
    /// Get a reference to the AI processor
    /// 
    /// # Returns
    /// Reference to the AI processor for advanced configuration
    pub fn processor(&self) -> &AIProcessor {
        &self.processor
    }
    
    /// Get a mutable reference to the AI processor
    /// 
    /// # Returns
    /// Mutable reference to the AI processor for configuration updates
    pub fn processor_mut(&mut self) -> &mut AIProcessor {
        &mut self.processor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ai_router_disabled() {
        let router = AIRouter::new(false, None);
        let result = router.ai_router("Hello", 12345, None).await.unwrap();
        assert_eq!(result, "AI functionality is currently disabled.");
    }

    #[tokio::test]
    async fn test_ai_router_enabled_no_key() {
        let router = AIRouter::new(true, None);
        let result = router.ai_router("Hello", 12345, Some("session_123".to_string())).await.unwrap();
        assert_eq!(result, "AI service is not properly configured.");
    }
    
    #[tokio::test]
    async fn test_ai_router_enabled_with_key() {
        let router = AIRouter::new(true, Some("test_key".to_string()));
        let result = router.ai_router("Hello", 12345, Some("session_123".to_string())).await.unwrap();
        assert!(result.contains("Hello"));
    }
    
    #[test]
    fn test_is_enabled() {
        let enabled_router = AIRouter::new(true, Some("key".to_string()));
        let disabled_router = AIRouter::new(false, None);
        
        assert!(enabled_router.is_enabled());
        assert!(!disabled_router.is_enabled());
    }
    
    #[test]
    fn test_is_ready() {
        let ready_router = AIRouter::new(true, Some("key".to_string()));
        let not_ready_router = AIRouter::new(true, None);
        let disabled_router = AIRouter::new(false, Some("key".to_string()));
        
        assert!(ready_router.is_ready());
        assert!(!not_ready_router.is_ready());
        assert!(!disabled_router.is_ready());
    }
    
    #[test]
    fn test_set_enabled() {
        let mut router = AIRouter::new(false, Some("key".to_string()));
        assert!(!router.is_enabled());
        
        router.set_enabled(true);
        assert!(router.is_enabled());
        
        router.set_enabled(false);
        assert!(!router.is_enabled());
    }
    
    #[test]
    fn test_processor_access() {
        let router = AIRouter::new(true, Some("key".to_string()));
        
        // Test immutable access
        let processor = router.processor();
        assert!(processor.is_ready());
        
        // Test that we can get a reference without issues
        assert!(router.processor().is_ready());
    }
    
    #[test]
    fn test_processor_mut_access() {
        let mut router = AIRouter::new(true, Some("key".to_string()));
        
        // Test mutable access
        router.processor_mut().set_enabled(false);
        assert!(!router.processor().is_ready()); // Should be false now due to disabled processor
    }
}