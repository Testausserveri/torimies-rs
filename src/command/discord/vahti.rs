use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use super::extensions::ClientContextExt;
use crate::vahti::new_vahti;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("vahti")
        .description("Luo uusi vahti")
        .create_option(|option| {
            option
                .name("url")
                .description("Hakusivun linkki")
                .required(true)
                .kind(CommandOptionType::String)
        })
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let mut url = String::new();
    for a in &command.data.options {
        match a.name.as_str() {
            "url" => url = String::from(a.value.as_ref().unwrap().as_str().unwrap()),
            _ => unreachable!(),
        }
    }

    info!("New vahti {}", &url);

    let db = ctx.get_db().await.unwrap();

    new_vahti(db, &url, command.user.id.0)
        .await
        .unwrap_or_else(|e| e.to_string())
}
