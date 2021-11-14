use crate::Mutex;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemHistory {
    items: Vec<(i64, i64)>,
}

impl TypeMapKey for ItemHistory {
    type Value = Arc<Mutex<ItemHistory>>;
}

impl ItemHistory {
    pub fn new() -> ItemHistory {
        let items = Vec::new();
        Self { items }
    }
    pub fn add_item(&mut self, id: i64, timestamp: i64) {
        if !self.contains(id) {
            info!("Adding id: {}, timestamp: {}", id, timestamp);
            self.items.push((id, timestamp))
        }
    }
    pub fn contains(&self, id: i64) -> bool {
        self.items.iter().any(|(iid, _)| iid == &id)
    }

    pub fn purge_old(&mut self) {
        self.items = self
            .items
            .iter()
            .filter(|(_, timestamp)| timestamp > &(chrono::Local::now().timestamp() - 600))
            .map(|t| t.to_owned())
            .collect();
    }
}
