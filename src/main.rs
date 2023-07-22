#![feature(lazy_cell, async_closure, iter_array_chunks)]
#![allow(dead_code)]

#[cfg(test)]
mod tests;

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

use std::sync::{Arc, LazyLock, RwLock};

use command::{Command, Manager};
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

#[derive(PartialEq, Clone)]
enum State {
    Running,
    Shutdown,
}

#[derive(Clone)]
struct Torimies {
    pub delivery: Arc<DashMap<i32, Box<dyn Delivery + Send + Sync>>>,
    pub command: Arc<DashMap<String, Box<dyn Command + Send + Sync>>>,
    pub command_manager: Arc<DashMap<String, Box<dyn Manager + Send + Sync>>>,
    pub database: Database,
    pub itemhistorystorage: crate::itemhistory::ItemHistoryStorage,
    pub state: Arc<RwLock<State>>,
}

// False positive
#[allow(clippy::needless_pass_by_ref_mut)]
async fn update_loop(man: &mut Torimies) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(*UPDATE_INTERVAL));
    loop {
        // Exiting after recieved signal depends on
        // 1) the ongoing update
        // 2) the following UPDATE_INTERVAL-tick
        interval.tick().await;
        let state = man.state.read().unwrap().clone();
        if state == State::Shutdown {
            break;
        }

        if let Err(e) = man.update_all_vahtis().await {
            error!("Error while updating: {}", e);
        }
    }

    info!("Update loop exited")
}

async fn command_loop(man: &Torimies) {
    join_all(
        man.command
            .iter_mut()
            .map(async move |mut c| c.start().await.ok()),
    )
    .await;
    info!("Command loop exited")
}

async fn ctrl_c_handler(man: &Torimies) {
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to register ctrl+c handler");
    info!("Recieved ctrl+c");
    info!("Setting State to State::Shutdown");
    *man.state.write().unwrap() = State::Shutdown;
    join_all(
        man.command_manager
            .iter()
            .map(async move |c| c.shutdown().await),
    )
    .await;
    info!("Ctrl+c handler exited");
}

impl Torimies {
    pub fn new(db: Database) -> Self {
        Self {
            delivery: Arc::new(DashMap::new()),
            command: Arc::new(DashMap::new()),
            command_manager: Arc::new(DashMap::new()),
            database: db,
            itemhistorystorage: Arc::new(DashMap::new()),
            state: Arc::new(RwLock::new(State::Running)),
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
        self.command_manager
            .insert(name.to_string(), commander.manager());
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

    #[cfg(feature = "telegram-delivery")]
    {
        let tg = crate::delivery::telegram::Telegram::init()
            .await
            .expect("Telegram delivery initialization failed");

        the_man.register_deliverer(crate::delivery::telegram::ID, tg)
    }

    #[cfg(feature = "telegram-command")]
    {
        let tg = crate::command::telegram::Telegram::init(&the_man.database.clone())
            .await
            .expect("Telegram commmand initialization failed");

        the_man.register_commander(crate::command::telegram::NAME, tg);
    }

    let the_man2 = the_man.clone();
    let the_man3 = the_man.clone();

    let update = update_loop(&mut the_man);
    let command = command_loop(&the_man2);
    let ctrl_c = ctrl_c_handler(&the_man3);

    futures::join!(update, command, ctrl_c);
}
