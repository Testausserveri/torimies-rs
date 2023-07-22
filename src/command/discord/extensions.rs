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
            .expect("No Database in client storage");
        Ok(db.to_owned())
    }
}

#[async_trait]
impl ClientContextExt for client::Client {
    async fn get_db(&self) -> Result<Database, Error> {
        let data = self.data.read().await;
        let db = data
            .get::<Database>()
            .expect("No Database in client storage");
        Ok(db.to_owned())
    }
}
