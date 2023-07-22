use teloxide::prelude::*;

use crate::database::Database;
use crate::vahti::remove_vahti;

pub async fn run(msg: Message, vahti: String, db: Database) -> ResponseResult<String> {
    if vahti.is_empty() {
        let vahdit = db
            .fetch_vahti_entries_by_user_id(msg.chat.id.0)
            .await
            .unwrap_or(Vec::new())
            .iter()
            .map(|v| v.url.clone())
            .collect::<Vec<_>>();

        if vahdit.is_empty() {
            return Ok(String::from("You have no registered Vahtis"));
        }

        return Ok(
            "Please provide a Vahti url, here are your registered Vahtis\n".to_owned()
                + &vahdit.join("\n"),
        );
    }

    Ok(remove_vahti(db, &vahti, msg.chat.id.0 as u64)
        .await
        .unwrap_or_else(|e| e.to_string()))
}
