#[cfg(feature = "discord-delivery")]
pub mod discord;

#[cfg(feature = "telegram-delivery")]
pub mod telegram;

use std::sync::Arc;

use async_trait::async_trait;
use dashmap::DashMap;

use crate::error::Error;
use crate::vahti::VahtiItem;

/// This is the Delivery trait. It should be implemented for
/// structs that provide a method for Torimies to deliver the
/// items gathered from Vahti::update().
///
/// The deliver method should take in a Vec of VahtiItems, all of which
/// have the same delivery_method and deliver_to fields
#[async_trait]
pub trait Delivery
where
    Self: Send + Sync,
{
    async fn deliver(&self, vs: Vec<VahtiItem>) -> Result<(), Error>;
}

pub async fn perform_delivery(
    delivery: Arc<DashMap<i32, Box<dyn Delivery + Sync + Send>>>,
    vs: Vec<VahtiItem>,
) -> Result<(), Error> {
    if let Some(v) = vs.first() {
        assert!(vs.iter().all(|vc| vc.delivery_method == v.delivery_method));

        return delivery
            .get(&v.delivery_method.expect("bug: impossible"))
            .expect("Missing delivery-method feature")
            .deliver(vs)
            .await;
    }

    Ok(())
}
