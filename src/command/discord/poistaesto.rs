use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;

use super::extensions::ClientContextExt;
use super::interaction::menu_from_options;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("poistaesto")
        .description("Salli aiemmin estetty myyjä")
}

pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) -> String {
    let db = ctx.get_db().await.unwrap();
    let blacklist = db
        .fetch_user_blacklist(command.user.id.0 as i64)
        .await
        .unwrap();

    let mut blacklist_names = vec![];
    for entry in &blacklist {
        blacklist_names.push(match entry.1 {
            crate::tori::ID => crate::tori::seller::get_seller_name_from_id(entry.0)
                .await
                .unwrap_or(String::from("Unknown Seller")),
            crate::huutonet::ID => crate::huutonet::seller::get_seller_name_from_id(entry.0)
                .await
                .unwrap_or(String::from("Unknown Seller")),
            _ => String::from("Unknown Seller"),
        });
    }

    let options = blacklist_names
        .iter()
        .zip(blacklist.iter().map(|ids| format!("{},{}", ids.0, ids.1)))
        .collect::<Vec<_>>();

    command
        .edit_original_interaction_response(&ctx.http, |message| {
            message.content("Valitse poistettava(t) esto/estot");

            if blacklist.is_empty() {
                message.content("Ei estettyjä myyjiä!")
            } else {
                message.components(|c| menu_from_options(c, "unblock_seller", options))
            }
        })
        .await
        .unwrap();

    String::new()
}
