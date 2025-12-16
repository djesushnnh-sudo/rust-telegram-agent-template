use async_trait::async_trait;
use teloxide::{Bot, types::Message, prelude::Requester};
use crate::commands::handler::{CommandHandler, CommandHandlerExt};
use crate::error::BotResult;
use std::time::{SystemTime, UNIX_EPOCH};

/// StatusCommand shows bot health and operational information
/// 
/// This command provides users and administrators with information about
/// the bot's current status, including uptime, version, and health checks.
pub struct StatusCommand {
    /// Bot startup time for uptime calculation
    startup_time: SystemTime,
    /// Bot version information
    version: String,
}

impl StatusCommand {
    /// Create a new StatusCommand instance
    pub fn new() -> Self {
        Self {
            startup_time: SystemTime::now(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    /// Create a StatusCommand with custom version
    pub fn with_version(version: String) -> Self {
        Self {
            startup_time: SystemTime::now(),
            version,
        }
    }

    /// Calculate bot uptime in a human-readable format
    fn calculate_uptime(&self) -> String {
        match self.startup_time.elapsed() {
            Ok(duration) => {
                let total_seconds = duration.as_secs();
                let days = total_seconds / 86400;
                let hours = (total_seconds % 86400) / 3600;
                let minutes = (total_seconds % 3600) / 60;
                let seconds = total_seconds % 60;

                if days > 0 {
                    format!("{}d {}h {}m {}s", days, hours, minutes, seconds)
                } else if hours > 0 {
                    format!("{}h {}m {}s", hours, minutes, seconds)
                } else if minutes > 0 {
                    format!("{}m {}s", minutes, seconds)
                } else {
                    format!("{}s", seconds)
                }
            }
            Err(_) => "Unknown".to_string(),
        }
    }

    /// Get current timestamp in a readable format
    fn get_current_time(&self) -> String {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(duration) => {
                let timestamp = duration.as_secs();
                // Simple timestamp formatting (you could use chrono for better formatting)
                format!("{}", timestamp)
            }
            Err(_) => "Unknown".to_string(),
        }
    }

    /// Check bot connectivity by attempting to get bot information
    async fn check_bot_connectivity(&self, bot: &Bot) -> (bool, String) {
        match bot.get_me().await {
            Ok(me) => {
                let username = me.username.as_ref().unwrap_or(&me.first_name).clone();
                (true, format!("Connected as @{}", username))
            },
            Err(e) => (false, format!("Connection error: {}", e)),
        }
    }

    /// Generate the complete status message
    async fn generate_status_message(&self, bot: &Bot) -> String {
        let uptime = self.calculate_uptime();
        let current_time = self.get_current_time();
        let (is_connected, connection_status) = self.check_bot_connectivity(bot).await;

        let status_emoji = if is_connected { "🟢" } else { "🔴" };
        let health_status = if is_connected { "Healthy" } else { "Degraded" };

        format!(
            "{} **Bot Status Report**\n\n\
            **🤖 Bot Information:**\n\
            • Name: Telegram Bot Template\n\
            • Version: v{}\n\
            • Status: {}\n\n\
            **⏱️ Runtime Information:**\n\
            • Uptime: {}\n\
            • Current Time: {}\n\n\
            **🔗 Connectivity:**\n\
            • Telegram API: {}\n\n\
            **💾 System Information:**\n\
            • Language: Rust\n\
            • Framework: teloxide\n\
            • Process ID: {}\n\n\
            **📊 Health Check:**\n\
            • Overall Status: {} {}\n\
            • Last Check: Just now\n\n\
            {}",
            status_emoji,
            self.version,
            health_status,
            uptime,
            current_time,
            connection_status,
            std::process::id(),
            status_emoji,
            health_status,
            if is_connected {
                "✅ All systems operational"
            } else {
                "⚠️ Some issues detected - check logs for details"
            }
        )
    }
}

#[async_trait]
impl CommandHandler for StatusCommand {
    async fn handle(&self, bot: &Bot, message: &Message, _args: Vec<String>) -> BotResult<()> {
        // Generate and send the status message
        let status_message = self.generate_status_message(bot).await;
        self.send_response(bot, message, &status_message).await
    }

    fn description(&self) -> &str {
        "Show bot health and operational status information"
    }

    fn usage(&self) -> &str {
        "/status - Display bot health, uptime, and system information"
    }

    fn name(&self) -> &str {
        "status"
    }

    fn requires_args(&self) -> bool {
        false
    }

    fn min_args(&self) -> usize {
        0
    }

    fn max_args(&self) -> Option<usize> {
        Some(0) // Status command should not accept any arguments
    }
}

impl Default for StatusCommand {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_status_command_metadata() {
        let command = StatusCommand::new();
        
        assert_eq!(command.name(), "status");
        assert_eq!(command.description(), "Show bot health and operational status information");
        assert_eq!(command.usage(), "/status - Display bot health, uptime, and system information");
        assert!(!command.requires_args());
        assert_eq!(command.min_args(), 0);
        assert_eq!(command.max_args(), Some(0));
    }

    #[test]
    fn test_status_command_validation() {
        let command = StatusCommand::new();
        
        // Should accept no arguments
        assert!(command.validate_args(&[]).is_ok());
        
        // Should reject arguments
        assert!(command.validate_args(&["arg".to_string()]).is_err());
    }

    #[test]
    fn test_status_command_with_version() {
        let command = StatusCommand::with_version("1.2.3".to_string());
        assert_eq!(command.version, "1.2.3");
    }

    #[test]
    fn test_calculate_uptime_immediate() {
        let command = StatusCommand::new();
        let uptime = command.calculate_uptime();
        
        // Should be very short uptime (seconds)
        assert!(uptime.ends_with('s'));
        assert!(!uptime.contains('m')); // Should not contain minutes for immediate check
    }

    #[test]
    fn test_calculate_uptime_with_delay() {
        let mut command = StatusCommand::new();
        // Simulate some uptime by setting startup time in the past
        command.startup_time = SystemTime::now() - Duration::from_secs(125); // 2 minutes 5 seconds
        
        let uptime = command.calculate_uptime();
        assert!(uptime.contains('m')); // Should contain minutes
        assert!(uptime.contains('s')); // Should contain seconds
    }

    #[test]
    fn test_get_current_time() {
        let command = StatusCommand::new();
        let current_time = command.get_current_time();
        
        // Should be a numeric timestamp
        assert!(!current_time.is_empty());
        assert_ne!(current_time, "Unknown");
        
        // Should be parseable as a number
        assert!(current_time.parse::<u64>().is_ok());
    }

    #[test]
    fn test_status_command_default() {
        let command = StatusCommand::default();
        assert_eq!(command.name(), "status");
        assert_eq!(command.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn test_uptime_formatting_days() {
        let mut command = StatusCommand::new();
        // Simulate 2 days, 3 hours, 4 minutes, 5 seconds
        command.startup_time = SystemTime::now() - Duration::from_secs(2 * 86400 + 3 * 3600 + 4 * 60 + 5);
        
        let uptime = command.calculate_uptime();
        assert!(uptime.contains("2d"));
        assert!(uptime.contains("3h"));
        assert!(uptime.contains("4m"));
        assert!(uptime.contains("5s"));
    }

    #[test]
    fn test_uptime_formatting_hours_only() {
        let mut command = StatusCommand::new();
        // Simulate 2 hours, 30 minutes, 15 seconds (no days)
        command.startup_time = SystemTime::now() - Duration::from_secs(2 * 3600 + 30 * 60 + 15);
        
        let uptime = command.calculate_uptime();
        assert!(!uptime.contains('d')); // Should not contain days
        assert!(uptime.contains("2h"));
        assert!(uptime.contains("30m"));
        assert!(uptime.contains("15s"));
    }

    #[test]
    fn test_uptime_formatting_minutes_only() {
        let mut command = StatusCommand::new();
        // Simulate 5 minutes, 30 seconds (no hours or days)
        command.startup_time = SystemTime::now() - Duration::from_secs(5 * 60 + 30);
        
        let uptime = command.calculate_uptime();
        assert!(!uptime.contains('d')); // Should not contain days
        assert!(!uptime.contains('h')); // Should not contain hours
        assert!(uptime.contains("5m"));
        assert!(uptime.contains("30s"));
    }
}