use serenity::client::Context;

use crate::extensions::ClientContextExt;

pub async fn blacklist_seller(
    ctx: &Context,
    userid: u64,
    sellerid: i32,
    siteid: i32,
) -> Result<String, anyhow::Error> {
    let db = ctx.get_db().await?;
    let blacklist = db.fetch_user_blacklist(userid.try_into()?).await?;
    if blacklist.contains(&sellerid) {
        info!(
            "Not blacklisting an already-blacklisted seller {} for the user {}",
            sellerid, userid
        );
        return Ok("Myyjä on jo estetty!".to_string());
    }
    match db
        .add_seller_to_blacklist(userid.try_into()?, sellerid, siteid)
        .await
    {
        Ok(_) => Ok(String::from("Myyjä estetty!")),
        Err(_) => bail!("Virhe tapahtui myyjän myyjän estämisen yhteydessä"),
    }
}
