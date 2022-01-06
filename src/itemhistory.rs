use std::sync::Arc;

use serenity::prelude::TypeMapKey;

use crate::Mutex;

#[derive(Clone)]
pub struct ItemHistory {
    items: Vec<(i64, i64, i32, i64)>,
}

impl TypeMapKey for ItemHistory {
    type Value = Arc<Mutex<ItemHistory>>;
}

impl ItemHistory {
    pub fn new() -> ItemHistory {
        let items = Vec::new();
        Self { items }
    }

    pub fn add_item(&mut self, id: i64, user_id: i64, site_id: i32, timestamp: i64) {
        if !self.contains(id, user_id, site_id) {
            debug!("Adding id: {},{}, timestamp: {}", id, site_id, timestamp);
            self.items.push((id, user_id, site_id, timestamp))
        }
    }

    pub fn contains(&self, id: i64, user_id: i64, site_id: i32) -> bool {
        self.items
            .iter()
            .any(|(iid, uid, sid, _)| iid == &id && uid == &user_id && sid == &site_id)
    }

    pub fn purge_old(&mut self) {
        self.items = self
            .items
            .iter()
            .filter(|(_, _, _, timestamp)| timestamp > &(chrono::Local::now().timestamp() - 1000))
            .map(|t| t.to_owned())
            .collect();
    }
}
