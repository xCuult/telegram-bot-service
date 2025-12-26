//! Application configuration module

use crate::error::BotError;
use std::env;

/// Bot configuration structure
/// 
/// Encapsulates all parameters necessary for init and running the bot
#[derive(Debug, Clone)]
pub struct BotConfig {
    /// Telegram bot token
    token: String,

    /// Logging level (Default: info)
    log_level: String,
}

impl BotConfig {
    /// Creates a configuration from environment variables
    /// 
    /// # Environment Variables
    /// 
    /// - TELOXIDE_TOKEN: Telegram bot token
    /// - RUST_LOG: Logging level (Default: info)
    /// 
    /// # Errors
    /// 
    /// Returns `BotError::Config` if `TELOXIDE_TOKEN` is not set or invalid
    pub fn from_env() -> Result<Self, BotError> {
        // Load environment variables from .env file if present
        let _ = dotenvy::dotenv();

        let token = env::var("TELOXIDE_TOKEN")
            .map_err(|_| BotError::Config("TELOXIDE_TOKEN environment variable  not set".into()))?;

        // Validate token format
        Self::validate_token(&token)?;

        let log_level = env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        Ok(Self { token, log_level })
    }

    /// Returns the bot token
    #[inline]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Returns the logging level
    pub fn log_level(&self) -> &str {
        &self.log_level
    }

    /// Validates the format of the Telegram bot token
    fn validate_token(token: &str) -> Result<(), BotError> {
        if !token.contains(':') || token.len() < 20 {
            return Err(BotError::Config(
                "Invalid token format!".into(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_token_format() {
        let result = BotConfig::validate_token("123456789:ABCdefGHIjklMNOpqrsTUVwxyz");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_token_format() {
        let result = BotConfig::validate_token("invalid");
        assert!(result.is_err());
    }
}