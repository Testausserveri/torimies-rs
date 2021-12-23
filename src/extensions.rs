use std::sync::Arc;

use serenity::prelude::*;
use serenity::{async_trait, client};

use crate::{Database, ItemHistory};

#[async_trait]
pub trait ClientContextExt {
    async fn get_db(&self) -> Result<Database, anyhow::Error>;
    async fn get_itemhistory(&self) -> Result<Arc<Mutex<ItemHistory>>, anyhow::Error>;
}

#[async_trait]
impl ClientContextExt for client::Context {
    async fn get_db(&self) -> Result<Database, anyhow::Error> {
        let data = self.data.read().await;
        let db = data
            .get::<Database>()
            .ok_or(anyhow!("Missing database from client data"))?;
        Ok(db.to_owned())
    }
    async fn get_itemhistory(&self) -> Result<Arc<Mutex<ItemHistory>>, anyhow::Error> {
        let data = self.data.read().await;
        let itemhistory = data
            .get::<ItemHistory>()
            .ok_or(anyhow!("Missing database from client data"))?;
        Ok(itemhistory.to_owned())
    }
}

#[async_trait]
impl ClientContextExt for client::Client {
    async fn get_db(&self) -> Result<Database, anyhow::Error> {
        let data = self.data.read().await;
        let db = data
            .get::<Database>()
            .ok_or(anyhow!("Missing database from client data"))?;
        Ok(db.to_owned())
    }
    async fn get_itemhistory(&self) -> Result<Arc<Mutex<ItemHistory>>, anyhow::Error> {
        let data = self.data.read().await;
        let itemhistory = data
            .get::<ItemHistory>()
            .ok_or(anyhow!("Missing database from client data"))?;
        Ok(itemhistory.to_owned())
    }
}
