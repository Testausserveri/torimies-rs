use async_trait::async_trait;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;

use crate::command::Command;
use crate::database::Database;
use crate::error::Error;
use crate::vahti::new_vahti;

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
    #[command(description = "Display help message")]
    Help,
    #[command(description = "Add a new Vahti")]
    Vahti(String),
}

async fn handle(bot: Bot, msg: Message, cmd: TelegramCommand, db: Database) -> ResponseResult<()> {
    match cmd {
        TelegramCommand::Help => println!("Help command"),
        TelegramCommand::Vahti(v) => {
            println!("Add vahti {} for user {}", msg.chat.id, &v);
            bot.send_message(
                ChatId((msg.chat.id.0 as u64) as i64),
                new_vahti(db, &v, msg.chat.id.0 as u64, crate::delivery::telegram::ID)
                    .await
                    .unwrap(),
            )
            .await?;
        }
    }

    Ok(())
}

#[async_trait]
impl Command for Telegram {
    async fn start(&mut self) -> Result<(), Error> {
        // use dispatcher
        //TelegramCommand::repl(self.bot.clone(), handle).await;
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
