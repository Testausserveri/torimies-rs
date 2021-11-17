use crate::Mutex;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

#[derive(Clone)]
pub struct ItemHistory {
    items: Vec<(i64, i64, i64)>,
}

impl TypeMapKey for ItemHistory {
    type Value = Arc<Mutex<ItemHistory>>;
}

impl ItemHistory {
    pub fn new() -> ItemHistory {
        let items = Vec::new();
        Self { items }
    }

    pub fn add_item(&mut self, id: i64, user_id: i64, timestamp: i64) {
        if !self.contains(id, user_id) {
            info!("Adding id: {}, timestamp: {}", id, timestamp);
            self.items.push((id, user_id, timestamp))
        }
    }

    pub fn contains(&self, id: i64, user_id: i64) -> bool {
        self.items.iter().any(|(iid, uid, _)| iid == &id && uid == &user_id)
    }

    pub fn purge_old(&mut self) {
        self.items = self
            .items
            .iter()
            .filter(|(_, _, timestamp)| timestamp > &(chrono::Local::now().timestamp() - 1000))
            .map(|t| t.to_owned())
            .collect();
    }
}
