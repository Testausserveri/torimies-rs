use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::extensions::ClientContextExt;
use crate::{vahti, ItemHistory};

#[command]
#[owners_only]
pub async fn update_all_vahtis(ctx: &Context, msg: &Message) -> CommandResult {
    let database = ctx.clone().get_db().await.unwrap();
    let itemhistory = ctx
        .data
        .write()
        .await
        .get_mut::<ItemHistory>()
        .unwrap()
        .clone();
    vahti::update_all_vahtis(database, itemhistory, ctx.http.clone())
        .await
        .unwrap();
    msg.reply(&ctx.http, "Updated!").await.unwrap();
    Ok(())
}
