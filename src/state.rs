use dashmap::DashMap;
use std::sync::Arc;
use teloxide::types::{ChatId, MessageId, UserId};
use uuid::Uuid;
use crate::database::{DatabaseProvider, DatabaseManager, DatabaseConfig};

/// Application state using concurrent data structures
/// 
/// This struct manages all runtime state for the bot using DashMap for thread-safe
/// concurrent access. Based on patterns from production Telegram bots.
#[derive(Clone)]
pub struct AppState {
    /// Maps forwarded message IDs to original user info for reply functionality
    /// Key: MessageId in admin chat, Value: (original user ChatId, original MessageId)
    pub forwarded_messages: Arc<DashMap<MessageId, (ChatId, MessageId)>>,
    
    /// Maps chat IDs to group names for all groups the bot has joined
    /// Key: ChatId, Value: Group name
    pub managed_groups: Arc<DashMap<ChatId, String>>,
    
    /// Maps user IDs to their session information for AI routing
    /// Key: UserId, Value: Session data (conversation context, preferences, etc.)
    pub user_sessions: Arc<DashMap<UserId, UserSession>>,
    
    /// Temporary storage for any pending operations or proposals
    /// Key: Operation ID, Value: Operation data
    pub pending_operations: Arc<DashMap<String, PendingOperation>>,
    
    /// Optional database provider for persistence
    /// If None, all data is kept in memory only
    pub database: Option<Arc<dyn DatabaseProvider>>,
}

/// User session data for AI routing and conversation context
#[derive(Clone, Debug)]
pub struct UserSession {
    pub user_id: UserId,
    pub chat_id: ChatId,
    pub session_id: String,
    pub ai_enabled: bool,
    pub conversation_context: Vec<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_activity: chrono::DateTime<chrono::Utc>,
}

/// Represents a pending operation (like group creation, user invitation, etc.)
#[derive(Clone, Debug)]
pub struct PendingOperation {
    pub id: String,
    pub operation_type: String,
    pub data: serde_json::Value,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

impl AppState {
    /// Create a new application state instance without database
    pub fn new() -> Self {
        Self {
            forwarded_messages: Arc::new(DashMap::new()),
            managed_groups: Arc::new(DashMap::new()),
            user_sessions: Arc::new(DashMap::new()),
            pending_operations: Arc::new(DashMap::new()),
            database: None,
        }
    }
    
    /// Create a new application state instance with database
    pub fn with_database(database: Arc<dyn DatabaseProvider>) -> Self {
        Self {
            forwarded_messages: Arc::new(DashMap::new()),
            managed_groups: Arc::new(DashMap::new()),
            user_sessions: Arc::new(DashMap::new()),
            pending_operations: Arc::new(DashMap::new()),
            database: Some(database),
        }
    }
    
    /// Create application state from configuration
    pub async fn from_config(config: &crate::config::Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let db_config = config.get_database_config();
        
        match db_config {
            DatabaseConfig::None => {
                log::info!("🚫 Database disabled - running in memory-only mode");
                Ok(Self::new())
            }
            _ => {
                log::info!("🗄️ Initializing database...");
                let database = DatabaseManager::create_provider(db_config).await?;
                Ok(Self::with_database(database))
            }
        }
    }

    /// Get or create a user session for AI routing
    pub fn get_or_create_session(&self, user_id: UserId, chat_id: ChatId) -> UserSession {
        if let Some(session) = self.user_sessions.get(&user_id) {
            // Update last activity
            let mut updated_session = session.clone();
            updated_session.last_activity = chrono::Utc::now();
            self.user_sessions.insert(user_id, updated_session.clone());
            updated_session
        } else {
            // Create new session
            let session = UserSession {
                user_id,
                chat_id,
                session_id: Uuid::new_v4().to_string(),
                ai_enabled: false, // Default to false, can be enabled per user
                conversation_context: Vec::new(),
                created_at: chrono::Utc::now(),
                last_activity: chrono::Utc::now(),
            };
            self.user_sessions.insert(user_id, session.clone());
            session
        }
    }

    /// Add a message to user's conversation context for AI processing
    pub fn add_to_conversation_context(&self, user_id: UserId, message: String) {
        if let Some(mut session) = self.user_sessions.get_mut(&user_id) {
            session.conversation_context.push(message);
            
            // Keep only last 10 messages to prevent memory bloat
            if session.conversation_context.len() > 10 {
                session.conversation_context.remove(0);
            }
            
            session.last_activity = chrono::Utc::now();
        }
    }

    /// Store a forwarded message mapping for admin replies
    pub async fn store_forwarded_message(&self, admin_msg_id: MessageId, original_chat_id: ChatId, original_msg_id: MessageId) {
        // Store in memory for fast access
        self.forwarded_messages.insert(admin_msg_id, (original_chat_id, original_msg_id));
        
        // Also persist to database if available
        if let Some(ref db) = self.database {
            if let Err(e) = db.store_forwarded_message(admin_msg_id, original_chat_id, original_msg_id).await {
                log::error!("Failed to persist forwarded message to database: {}", e);
            }
        }
    }

    /// Get original message info from a forwarded message
    pub fn get_original_message(&self, admin_msg_id: MessageId) -> Option<(ChatId, MessageId)> {
        self.forwarded_messages.get(&admin_msg_id).map(|entry| *entry.value())
    }

    /// Register a new group
    pub async fn register_group(&self, chat_id: ChatId, group_name: String) {
        log::info!("📝 Registered new group: {} ({})", group_name, chat_id);
        
        // Store in memory for fast access
        self.managed_groups.insert(chat_id, group_name.clone());
        
        // Also persist to database if available
        if let Some(ref db) = self.database {
            if let Err(e) = db.store_managed_group(chat_id, &group_name).await {
                log::error!("Failed to persist managed group to database: {}", e);
            }
        }
    }

    /// Get all managed groups
    pub fn get_managed_groups(&self) -> Vec<(ChatId, String)> {
        self.managed_groups
            .iter()
            .map(|entry| (*entry.key(), entry.value().clone()))
            .collect()
    }

    /// Create a pending operation
    pub fn create_pending_operation(&self, operation_type: String, data: serde_json::Value, expires_in_minutes: i64) -> String {
        let id = Uuid::new_v4().to_string();
        let operation = PendingOperation {
            id: id.clone(),
            operation_type,
            data,
            created_at: chrono::Utc::now(),
            expires_at: chrono::Utc::now() + chrono::Duration::minutes(expires_in_minutes),
        };
        
        self.pending_operations.insert(id.clone(), operation);
        id
    }

    /// Get and remove a pending operation
    pub fn consume_pending_operation(&self, operation_id: &str) -> Option<PendingOperation> {
        self.pending_operations.remove(operation_id).map(|(_, op)| op)
    }

    /// Clean up expired operations and old sessions
    pub fn cleanup_expired_data(&self) {
        let now = chrono::Utc::now();
        
        // Remove expired operations
        self.pending_operations.retain(|_, op| op.expires_at > now);
        
        // Remove sessions inactive for more than 24 hours
        let cutoff = now - chrono::Duration::hours(24);
        self.user_sessions.retain(|_, session| session.last_activity > cutoff);
        
        // Keep only last 1000 forwarded messages to prevent memory bloat
        if self.forwarded_messages.len() > 1000 {
            let keys_to_remove: Vec<_> = self.forwarded_messages
                .iter()
                .take(self.forwarded_messages.len() - 1000)
                .map(|entry| *entry.key())
                .collect();
            
            for key in keys_to_remove {
                self.forwarded_messages.remove(&key);
            }
        }
    }

    /// Load initial data from database
    pub async fn load_from_database(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref db) = self.database {
            log::info!("📥 Loading data from database...");
            
            // Load managed groups
            let groups = db.get_managed_groups().await?;
            for group in groups {
                self.managed_groups.insert(group.chat_id, group.group_name);
            }
            log::debug!("Loaded {} managed groups", self.managed_groups.len());
            
            // Load recent forwarded messages (last 1000 for memory efficiency)
            let forwarded = db.get_recent_forwarded_messages(1000).await?;
            for msg in forwarded {
                self.forwarded_messages.insert(msg.admin_msg_id, (msg.original_chat_id, msg.original_msg_id));
            }
            log::debug!("Loaded {} forwarded message mappings", self.forwarded_messages.len());
            
            log::info!("✅ Database loading complete");
        } else {
            log::info!("📥 No database configured - starting with empty state");
        }
        Ok(())
    }

    /// Save current state to database
    pub async fn save_to_database(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if let Some(ref db) = self.database {
            log::info!("💾 Saving state to database...");
            
            // Note: Individual operations (like storing forwarded messages, registering groups)
            // are saved immediately when they happen, so this method is mainly for cleanup
            // and ensuring consistency on shutdown.
            
            // Perform health check to ensure database is still accessible
            db.health_check().await?;
            
            log::info!("✅ Database saving complete");
        } else {
            log::info!("💾 No database configured - state will be lost on restart");
        }
        Ok(())
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_creation() {
        let state = AppState::new();
        assert_eq!(state.managed_groups.len(), 0);
        assert_eq!(state.user_sessions.len(), 0);
    }

    #[test]
    fn test_session_creation() {
        let state = AppState::new();
        let user_id = UserId(12345);
        let chat_id = ChatId(67890);
        
        let session = state.get_or_create_session(user_id, chat_id);
        assert_eq!(session.user_id, user_id);
        assert_eq!(session.chat_id, chat_id);
        assert!(!session.ai_enabled);
    }

    #[tokio::test]
    async fn test_group_registration() {
        let state = AppState::new();
        let chat_id = ChatId(-1001234567890);
        let group_name = "Test Group".to_string();
        
        state.register_group(chat_id, group_name.clone()).await;
        
        let groups = state.get_managed_groups();
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0], (chat_id, group_name));
    }

    #[test]
    fn test_conversation_context() {
        let state = AppState::new();
        let user_id = UserId(12345);
        let chat_id = ChatId(67890);
        
        // Create session
        state.get_or_create_session(user_id, chat_id);
        
        // Add messages
        state.add_to_conversation_context(user_id, "Hello".to_string());
        state.add_to_conversation_context(user_id, "How are you?".to_string());
        
        let session = state.user_sessions.get(&user_id).unwrap();
        assert_eq!(session.conversation_context.len(), 2);
        assert_eq!(session.conversation_context[0], "Hello");
        assert_eq!(session.conversation_context[1], "How are you?");
    }

    #[test]
    fn test_pending_operations() {
        let state = AppState::new();
        let data = serde_json::json!({"test": "data"});
        
        let op_id = state.create_pending_operation("test_op".to_string(), data.clone(), 60);
        
        let operation = state.consume_pending_operation(&op_id).unwrap();
        assert_eq!(operation.operation_type, "test_op");
        assert_eq!(operation.data, data);
        
        // Should be consumed (removed)
        assert!(state.consume_pending_operation(&op_id).is_none());
    }
}