use std::sync::Arc;

use serenity::{async_trait, client};

use crate::Database;

#[async_trait]
pub trait ClientContextExt {
    async fn get_db(&self) -> Result<Arc<Database>, anyhow::Error>;
}

#[async_trait]
impl ClientContextExt for client::Context {
    async fn get_db(&self) -> Result<Arc<Database>, anyhow::Error> {
        Ok(self
            .data
            .read()
            .await
            .get::<Database>()
            .ok_or(anyhow!("Missing database from client data"))?
            .clone()
            .to_owned())
    }
}

#[async_trait]
impl ClientContextExt for client::Client {
    async fn get_db(&self) -> Result<Arc<Database>, anyhow::Error> {
        Ok(self
            .data
            .read()
            .await
            .get::<Database>()
            .ok_or(anyhow!("Missing database from client data"))?
            .clone())
    }
}
