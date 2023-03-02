use serenity::{async_trait, client};

use crate::error::Error;
use crate::Database;

#[async_trait]
pub trait ClientContextExt {
    async fn get_db(&self) -> Result<Database, Error>;
}

#[async_trait]
impl ClientContextExt for client::Context {
    async fn get_db(&self) -> Result<Database, Error> {
        let data = self.data.read().await;
        let db = data
            .get::<Database>()
            .ok_or(Error::Discord(String::from("No database in client data")))?;
        Ok(db.to_owned())
    }
}

#[async_trait]
impl ClientContextExt for client::Client {
    async fn get_db(&self) -> Result<Database, Error> {
        let data = self.data.read().await;
        let db = data
            .get::<Database>()
            .ok_or(Error::Discord(String::from("No database in client data")))?;
        Ok(db.to_owned())
    }
}
