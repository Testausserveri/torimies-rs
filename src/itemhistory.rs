use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};

use serenity::prelude::TypeMapKey;

#[derive(Debug, Clone)]
pub struct ItemHistory {
    // (item_id, site_id, timestamp)
    items: Vec<(i64, i32, i64)>,
}

// vahti_id => ItemHistory
pub type ItemHistoryStorage = Arc<RwLock<HashMap<i32, Arc<Mutex<ItemHistory>>>>>;

impl TypeMapKey for ItemHistory {
    type Value = Arc<Mutex<ItemHistory>>;
}

impl ItemHistory {
    pub fn new() -> ItemHistory {
        let items = Vec::new();
        Self { items }
    }

    pub fn add_item(&mut self, id: i64, site_id: i32, timestamp: i64) {
        if !self.contains(id, site_id) {
            debug!("Adding id: {},{}, timestamp: {}", id, site_id, timestamp);
            self.items.push((id, site_id, timestamp))
        }
    }

    pub fn contains(&self, id: i64, site_id: i32) -> bool {
        self.items
            .iter()
            .any(|(iid, sid, _)| iid == &id && sid == &site_id)
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

impl Default for ItemHistory {
    fn default() -> Self {
        Self::new()
    }
}
