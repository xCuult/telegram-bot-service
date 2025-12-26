//! Defining bot commands using teloxide derive macros

use teloxide::utils::command::BotCommands;

/// Commands supported by the bot
#[derive(BotCommands, Clone, Debug)]
#[command(
    rename_rule = "lowercase",
    description = "Available commands:"
)]
pub enum Command {
    /// Standart command for starting the bot
    #[command(description = "Start interaction with the bot")]
    Start,
}

impl Command {
    /// Returns a welcome message for the /start command
    #[inline]
    pub fn start_message() -> &'static str {
        "Hello World"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_message_not_empty() {
        assert!(!Command::start_message().is_empty());
    }

    #[test]
    fn test_command_parsing() {
        // Test parsing the /start command
        let parsed = Command::parse("/start", "test_bot");
        assert!(parsed.is_ok());
    }
}