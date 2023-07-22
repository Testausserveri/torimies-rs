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
/// It should be able to handle the
/// events, calling the appropriate functions on it's own.
#[async_trait]
pub trait Command {
    async fn start(&mut self) -> Result<(), Error>;
}

/// The Manager trait is used for shutting down the corresponding
/// Commander. It is recommended that the Commander-struct implements
/// a way to generate a Manager instance for it.
#[async_trait]
pub trait Manager {
    async fn shutdown(&self);
}
