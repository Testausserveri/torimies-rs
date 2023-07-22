use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

pub async fn run() -> ResponseResult<String> {
    Ok(super::TelegramCommand::descriptions().to_string())
}
