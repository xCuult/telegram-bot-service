//! Bot command handlers

use teloxide::{prelude::*, types::Message};
use tracing::{info, instrument};

use super::commands::Command;
use crate::error::BotResult;

/// Processes incoming bot commands
/// 
/// Pattern matching for command routing
#[instrument(
    name = "command_handler",
    skip(bot, msg),
    fields(
        user_id = %msg.from.as_ref().map(|u| u.id.0).unwrap_or(0),
        chat_id = %msg.chat.id,
        command = ?cmd
    )
)]
pub async fn handle_command(bot: Bot, msg: Message, cmd: Command) -> BotResult<()> {
    match cmd {
        Command::Start => handle_start(bot, msg).await,
    }
}

/// Handles the /start command
async fn handle_start(bot: Bot, msg: Message) -> BotResult<()> {
    let user_info = msg
        .from.as_ref()
        .map(|user| {
            format!(
                "{}{}",
                user.first_name,
                user.last_name
                    .as_ref()
                    .map(|ln| format!(" {}", ln))
                    .unwrap_or_default()
            )
        })
        .unwrap_or_else(|| "Unknown".to_string());

    info!(
        user = %user_info,
        "New user started the bot"
    );

    // Send welcome message
    bot.send_message(msg.chat.id, Command::start_message())
        .await?;

    Ok(())
}