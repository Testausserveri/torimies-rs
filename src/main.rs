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

use std::collections::HashMap;
use std::sync::{Arc, LazyLock, Mutex, RwLock};

use command::Command;
use dashmap::DashMap;
use database::Database;
use delivery::Delivery;
use error::Error;
use futures::future::join_all;
use futures::Future;

static UPDATE_INTERVAL: LazyLock<u64> = LazyLock::new(|| {
    std::env::var("UPDATE_INTERVAL")
        .unwrap_or(String::from("120"))
        .parse()
        .expect("Invalid UPDATED_INTERVAL")
});

#[derive(Clone)]
struct Torimies {
    pub delivery: Arc<DashMap<i32, Box<dyn Delivery + Send + Sync>>>,
    pub command: Arc<Mutex<Vec<Box<dyn Command + Send + Sync>>>>,
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
    // FIXME: This Mutex stays locked :(
    let mut cmd = man.command.lock().unwrap();

    let fs: Vec<std::pin::Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>> =
        cmd.iter_mut().map(|c| c.start()).collect();

    join_all(fs).await;
}

impl Torimies {
    pub fn new(db: Database) -> Self {
        Self {
            delivery: Arc::new(DashMap::new()),
            command: Arc::new(Mutex::new(Vec::new())),
            database: db,
            #[cfg(feature = "tori")]
            itemhistorystorage: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn register_deliverer<T: Delivery + Send + Sync + 'static>(&mut self, id: i32, deliverer: T) {
        self.delivery.insert(id, Box::new(deliverer));
    }

    fn register_commander<T: Command + Send + Sync + 'static>(&mut self, commander: T) {
        self.command.lock().unwrap().push(Box::new(commander));
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

        the_man.register_commander(dc);
    }

    let mut the_man2 = the_man.clone();

    let update = update_loop(&mut the_man);
    let command = command_loop(&mut the_man2);

    futures::join!(update, command);
}
