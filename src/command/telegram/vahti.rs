use teloxide::prelude::*;

use crate::database::Database;
use crate::vahti::new_vahti;

pub async fn run(msg: Message, vahti: String, db: Database) -> ResponseResult<String> {
    if vahti.is_empty() {
        return Ok(String::from("No url provided"));
    }

    Ok(new_vahti(
        db,
        &vahti,
        msg.chat.id.0 as u64,
        crate::delivery::telegram::ID,
    )
    .await
    .unwrap_or_else(|e| e.to_string()))
}
