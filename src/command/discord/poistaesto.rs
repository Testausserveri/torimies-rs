use serenity::builder::{CreateCommand, EditInteractionResponse};
use serenity::client::Context;
use serenity::model::application::CommandInteraction;

use super::extensions::ClientContextExt;
use super::interaction::menu_from_options;

pub fn register() -> CreateCommand {
    CreateCommand::new("poistaesto").description("Salli aiemmin estetty myyjä")
}

pub async fn run(ctx: &Context, command: &CommandInteraction) -> String {
    let db = ctx.get_db().await.unwrap();
    let blacklist = db
        .fetch_user_blacklist(u64::from(command.user.id) as i64)
        .await
        .unwrap();

    let mut blacklist_names = vec![];
    for entry in &blacklist {
        blacklist_names.push(match entry.1 {
            #[cfg(feature = "tori")]
            crate::tori::ID => crate::tori::seller::get_seller_name_from_id(entry.0)
                .await
                .unwrap_or(String::from("Unknown Seller")),
            #[cfg(feature = "huutonet")]
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

    let mut edit = EditInteractionResponse::new().content("Valitse poistettava(t) esto/estot");
    if blacklist.is_empty() {
        edit = edit.content("Ei estettyjä myyjiä!");
    } else {
        edit = edit.components(menu_from_options("unblock_seller", options));
    }
    command.edit_response(&ctx.http, edit).await.unwrap();

    String::new()
}
