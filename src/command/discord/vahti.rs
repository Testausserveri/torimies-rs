use serenity::builder::{CreateCommand, CreateCommandOption};
use serenity::client::Context;
use serenity::model::application::{CommandInteraction, CommandOptionType};

use super::extensions::ClientContextExt;
use crate::vahti::new_vahti;

pub fn register() -> CreateCommand {
    CreateCommand::new("vahti")
        .description("Luo uusi vahti")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "url", "Hakusivun linkki")
                .required(true),
        )
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    let mut url = String::new();
    for a in &command.data.options {
        match a.name.as_str() {
            "url" => url = String::from(a.value.as_str().unwrap()),
            _ => unreachable!(),
        }
    }

    info!("New vahti {}", &url);

    let db = ctx.get_db().await.unwrap();

    new_vahti(
        db,
        &url,
        u64::from(command.user.id),
        crate::delivery::discord::ID,
    )
    .await
    .unwrap_or_else(|e| e.to_string())
}
