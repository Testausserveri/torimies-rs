use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::extensions::ClientContextExt;

use crate::vahti;

#[command]
#[owners_only]
pub async fn update_all_vahtis(ctx: &Context, msg: &Message) -> CommandResult {
    vahti::update_all_vahtis(ctx.clone().get_db().await,&ctx.http).await;
    msg.reply(&ctx.http, "Updated!").await.unwrap();
    Ok(())
}
