mod help;
mod poistavahti;
mod start;
mod vahti;

use async_trait::async_trait;
use teloxide::adaptors::throttle::Limits;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::command::Command;
use crate::database::Database;
use crate::error::Error;

pub const NAME: &str = "telegram";

pub struct Telegram {
    pub bot: Bot,
    pub db: Database,
}

impl Telegram {
    pub async fn init(db: &Database) -> Result<Self, Error> {
        let token =
            std::env::var("TELOXIDE_TOKEN").expect("Expected TELOXIDE_TOKEN in the environment");

        let bot = Bot::new(token);

        Ok(Self {
            bot,
            db: db.clone(),
        })
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands")]
enum TelegramCommand {
    #[command(description = "Display start message")]
    Start,
    #[command(description = "Display help message")]
    Help,
    #[command(description = "Add new vahti with `/vahti [url]`")]
    Vahti(String),
    #[command(description = "Remove a vahti with `/poistavahti [url]`")]
    PoistaVahti(String),
}

async fn handle(bot: Bot, msg: Message, cmd: TelegramCommand, db: Database) -> ResponseResult<()> {
    let response = match cmd {
        TelegramCommand::Vahti(v) => vahti::run(msg.clone(), v, db).await,
        TelegramCommand::PoistaVahti(v) => poistavahti::run(msg.clone(), v, db).await,
        TelegramCommand::Help => help::run().await,
        TelegramCommand::Start => start::run().await,
    }
    .unwrap_or(String::from(
        "Ran into an unhandled error while processing the command",
    ));

    bot.throttle(Limits::default())
        .send_message(msg.chat.id, response)
        .disable_web_page_preview(true)
        .await?;
    Ok(())
}

#[async_trait]
impl Command for Telegram {
    async fn start(&mut self) -> Result<(), Error> {
        let handler = Update::filter_message().branch(
            dptree::entry()
                .filter_command::<TelegramCommand>()
                .endpoint(handle),
        );

        Dispatcher::builder(self.bot.clone(), handler)
            .dependencies(dptree::deps![self.db.clone()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
        Ok(())
    }

    async fn destroy(&mut self) {
        let _ = self.bot.log_out();
    }
}
