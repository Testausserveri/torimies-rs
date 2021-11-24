use crate::extensions::ClientContextExt;
use crate::vahti;
use crate::ItemHistory;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[owners_only]
pub async fn update_all_vahtis(ctx: &Context, msg: &Message) -> CommandResult {
    let database = ctx.clone().get_db().await;
    let mut itemhistory = ctx
        .data
        .write()
        .await
        .get_mut::<ItemHistory>()
        .unwrap()
        .clone();
    vahti::update_all_vahtis(database, &mut itemhistory, &ctx.http).await.unwrap();
    msg.reply(&ctx.http, "Updated!").await.unwrap();
    Ok(())
}
