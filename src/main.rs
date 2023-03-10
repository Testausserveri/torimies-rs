#![feature(once_cell, async_closure, iter_array_chunks)]
#![allow(dead_code)]

#[cfg(test)]
mod tests;

#[cfg(feature = "tori")]
mod itemhistory;
#[cfg(feature = "tori")]
mod tori;

#[cfg(feature = "huutonet")]
mod huutonet;

mod error;
pub mod models;
pub mod schema;

pub mod command;
pub mod database;
pub mod delivery;
mod vahti;

#[macro_use]
extern crate tracing;
#[macro_use]
extern crate diesel;

use std::sync::{Arc, LazyLock};

use command::Command;
use dashmap::DashMap;
use database::Database;
use delivery::Delivery;
use futures::future::join_all;

static UPDATE_INTERVAL: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("UPDATE_INTERVAL")
        .unwrap_or(String::from("120"))
        .parse()
        .expect("Invalid UPDATED_INTERVAL")
});

#[derive(Clone)]
struct Torimies {
    pub delivery: Arc<DashMap<i32, Box<dyn Delivery + Send + Sync>>>,
    pub command: Arc<DashMap<String, Box<dyn Command + Send + Sync>>>,
    pub database: Database,
    #[cfg(feature = "tori")]
    pub itemhistorystorage: crate::itemhistory::ItemHistoryStorage,
}

async fn update_loop(man: &mut Torimies) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(*UPDATE_INTERVAL));
    loop {
        // Update loop
        interval.tick().await;
        if let Err(e) = man.update_all_vahtis().await {
            error!("Error while updating: {}", e);
        }
    }
}

async fn command_loop(man: &mut Torimies) {
    join_all(
        man.command
            .iter_mut()
            .map(async move |mut c| c.start().await.ok()),
    )
    .await;
}

impl Torimies {
    pub fn new(db: Database) -> Self {
        Self {
            delivery: Arc::new(DashMap::new()),
            command: Arc::new(DashMap::new()),
            database: db,
            #[cfg(feature = "tori")]
            itemhistorystorage: Arc::new(DashMap::new()),
        }
    }

    fn register_deliverer<T: Delivery + Send + Sync + 'static>(&mut self, id: i32, deliverer: T) {
        self.delivery.insert(id, Box::new(deliverer));
    }

    fn register_commander<T: Command + Send + Sync + 'static>(
        &mut self,
        name: impl ToString,
        commander: T,
    ) {
        self.command.insert(name.to_string(), Box::new(commander));
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::fmt::init();

    let database = Database::new().await;

    let mut the_man = Torimies::new(database);

    #[cfg(feature = "discord-delivery")]
    {
        let dc = crate::delivery::discord::Discord::init()
            .await
            .expect("Discord delivery initialization failed");

        the_man.register_deliverer(crate::delivery::discord::ID, dc);
    }

    #[cfg(feature = "discord-command")]
    {
        let dc = crate::command::discord::Discord::init(&the_man.database.clone())
            .await
            .expect("Discord commmand initialization failed");

        the_man.register_commander(crate::command::discord::NAME, dc);
    }

    let mut the_man2 = the_man.clone();

    let update = update_loop(&mut the_man);
    let command = command_loop(&mut the_man2);

    futures::join!(update, command);
}
