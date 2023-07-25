#[cfg(feature = "discord-command")]
pub mod discord;

#[cfg(feature = "telegram-command")]
pub mod telegram;

use async_trait::async_trait;

use crate::error::Error;

/// This is the Command handler trait. It should be implemented for structs that
/// provide methods for the user to interact with Torimies
///
/// The command trait should implement the start function
/// and the manager function, which returns a Manager struct for the commander
/// It should be able to handle the events, calling the appropriate functions on it's own.
#[async_trait]
pub trait Command
where
    Self: Send + Sync,
{
    async fn start(&mut self) -> Result<(), Error>;
    fn manager(&self) -> Box<dyn Manager + Send + Sync>;
}

/// The Manager trait is used for shutting down the corresponding Commander.
#[async_trait]
pub trait Manager
where
    Self: Send + Sync,
{
    async fn shutdown(&self);
}
