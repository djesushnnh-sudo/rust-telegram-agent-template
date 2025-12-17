use async_trait::async_trait;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use teloxide::types::{ChatId, UserId, MessageId};
use chrono::Utc;
use log::{info, debug};

use crate::database::provider::{DatabaseProvider, DatabaseStats};
use crate::database::models::{User, ManagedGroup, ForwardedMessage};

/// In-memory database provider for testing and development
/// 
/// This provider stores all data in memory using HashMaps and RwLocks.
/// It's useful for testing, development, or when you don't need persistent storage.
/// All data is lost when the application stops.
pub struct InMemoryProvider {
    users: Arc<RwLock<HashMap<i64, User>>>,
    managed_groups: Arc<RwLock<HashMap<i64, ManagedGroup>>>,
    forwarded_messages: Arc<RwLock<HashMap<i32, ForwardedMessage>>>,
    user_preferences: Arc<RwLock<HashMap<i64, bool>>>,
    next_user_id: Arc<RwLock<i64>>,
}

impl InMemoryProvider {
    /// Create a new in-memory database provider
    pub fn new() -> Self {
        info!("Initializing in-memory database provider");
        
        Self {
            users: Arc::new(RwLock::new(HashMap::new())),
            managed_groups: Arc::new(RwLock::new(HashMap::new())),
            forwarded_messages: Arc::new(RwLock::new(HashMap::new())),
            user_preferences: Arc::new(RwLock::new(HashMap::new())),
            next_user_id: Arc::new(RwLock::new(1)),
        }
    }
}

impl Default for InMemoryProvider {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl DatabaseProvider for InMemoryProvider {
    // =========================================================================
    // USER OPERATIONS
    // =========================================================================
    
    async fn create_user(&self, username: &str, telegram_id: i64) -> Result<()> {
        debug!("Creating/updating user: {} (ID: {})", username, telegram_id);
        
        let mut users = self.users.write().await;
        let mut next_id = self.next_user_id.write().await;
        
        let user = User {
            id: *next_id,
            telegram_id,
            username: username.to_string(),
            created_at: Utc::now().to_rfc3339(),
        };
        
        users.insert(telegram_id, user);
        *next_id += 1;
        
        debug!("User created/updated successfully");
        Ok(())
    }
    
    async fn get_user(&self, telegram_id: i64) -> Result<Option<User>> {
        let users = self.users.read().await;
        Ok(users.get(&telegram_id).cloned())
    }
    
    async fn get_all_users(&self) -> Result<Vec<User>> {
        let users = self.users.read().await;
        let mut user_list: Vec<User> = users.values().cloned().collect();
        
        // Sort by created_at descending (newest first)
        user_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        Ok(user_list)
    }
    
    async fn delete_user(&self, telegram_id: i64) -> Result<bool> {
        let mut users = self.users.write().await;
        Ok(users.remove(&telegram_id).is_some())
    }

    // =========================================================================
    // MANAGED GROUPS OPERATIONS
    // =========================================================================

    async fn store_managed_group(&self, chat_id: ChatId, group_name: &str) -> Result<()> {
        debug!("Storing managed group: {} ({})", group_name, chat_id);
        
        let mut groups = self.managed_groups.write().await;
        let now = Utc::now();
        
        let group = ManagedGroup {
            chat_id,
            group_name: group_name.to_string(),
            created_at: now,
            updated_at: now,
        };
        
        groups.insert(chat_id.0, group);
        Ok(())
    }

    async fn get_managed_groups(&self) -> Result<Vec<ManagedGroup>> {
        let groups = self.managed_groups.read().await;
        let mut group_list: Vec<ManagedGroup> = groups.values().cloned().collect();
        
        // Sort by group name
        group_list.sort_by(|a, b| a.group_name.cmp(&b.group_name));
        
        Ok(group_list)
    }

    async fn remove_managed_group(&self, chat_id: ChatId) -> Result<bool> {
        let mut groups = self.managed_groups.write().await;
        Ok(groups.remove(&chat_id.0).is_some())
    }

    // =========================================================================
    // FORWARDED MESSAGES OPERATIONS
    // =========================================================================

    async fn store_forwarded_message(
        &self,
        admin_msg_id: MessageId,
        original_chat_id: ChatId,
        original_msg_id: MessageId,
    ) -> Result<()> {
        let mut messages = self.forwarded_messages.write().await;
        
        let message = ForwardedMessage {
            admin_msg_id,
            original_chat_id,
            original_msg_id,
            created_at: Utc::now(),
        };
        
        messages.insert(admin_msg_id.0, message);
        Ok(())
    }

    async fn get_original_message(&self, admin_msg_id: MessageId) -> Result<Option<(ChatId, MessageId)>> {
        let messages = self.forwarded_messages.read().await;
        
        if let Some(message) = messages.get(&admin_msg_id.0) {
            Ok(Some((message.original_chat_id, message.original_msg_id)))
        } else {
            Ok(None)
        }
    }

    async fn get_recent_forwarded_messages(&self, limit: i32) -> Result<Vec<ForwardedMessage>> {
        let messages = self.forwarded_messages.read().await;
        let mut message_list: Vec<ForwardedMessage> = messages.values().cloned().collect();
        
        // Sort by created_at descending (newest first)
        message_list.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        // Limit results
        message_list.truncate(limit as usize);
        
        Ok(message_list)
    }

    async fn cleanup_old_forwarded_messages(&self, keep_days: i32) -> Result<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(keep_days as i64);
        let mut messages = self.forwarded_messages.write().await;
        
        let initial_count = messages.len();
        messages.retain(|_, message| message.created_at > cutoff);
        let final_count = messages.len();
        
        let deleted_count = (initial_count - final_count) as u64;
        if deleted_count > 0 {
            info!("Cleaned up {} old forwarded messages", deleted_count);
        }
        
        Ok(deleted_count)
    }

    // =========================================================================
    // USER SESSION OPERATIONS
    // =========================================================================

    async fn set_user_ai_enabled(&self, user_id: UserId, enabled: bool) -> Result<()> {
        let mut preferences = self.user_preferences.write().await;
        preferences.insert(user_id.0 as i64, enabled);
        Ok(())
    }

    async fn get_user_ai_enabled(&self, user_id: UserId) -> Result<bool> {
        let preferences = self.user_preferences.read().await;
        Ok(preferences.get(&(user_id.0 as i64)).copied().unwrap_or(false))
    }

    // =========================================================================
    // UTILITY OPERATIONS
    // =========================================================================

    async fn get_stats(&self) -> Result<DatabaseStats> {
        let users = self.users.read().await;
        let groups = self.managed_groups.read().await;
        let messages = self.forwarded_messages.read().await;
        
        Ok(DatabaseStats {
            users_count: users.len() as u64,
            groups_count: groups.len() as u64,
            forwarded_messages_count: messages.len() as u64,
        })
    }

    async fn health_check(&self) -> Result<()> {
        // In-memory provider is always healthy if it exists
        Ok(())
    }
}