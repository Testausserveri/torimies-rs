#[cfg(feature = "discord-command")]
pub mod discord;

use async_trait::async_trait;

use crate::error::Error;

/// This is the Command handler trait. It should be implemented for structs that
/// provide methods for the user to interact with Torimies
///
/// The command trait should implement the start function
/// and the destroy function. It should be able to handles the
/// events, calling the appropriate functions on it's own.
#[async_trait]
pub trait Command {
    async fn start(&mut self) -> Result<(), Error>;
    async fn destroy(&mut self);
}
