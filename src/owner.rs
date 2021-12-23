use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::extensions::ClientContextExt;
use crate::vahti;

#[command]
#[owners_only]
pub async fn update_all_vahtis(ctx: &Context, msg: &Message) -> CommandResult {
    let database = ctx.get_db().await?;
    let itemhistory = ctx.get_itemhistory().await?;
    vahti::update_all_vahtis(database, itemhistory, ctx.http.clone())
        .await
        .unwrap();
    msg.reply(&ctx.http, "Updated!").await?;
    Ok(())
}
