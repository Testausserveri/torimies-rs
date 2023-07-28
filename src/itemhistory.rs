use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use dashmap::DashMap;
use serenity::prelude::TypeMapKey;

#[derive(Debug, Clone)]
pub struct ItemHistory {
    // (item_id, site_id, timestamp)
    items: Vec<(i64, i32, i64)>,
}

// (user_id, delivery_method) => ItemHistory
pub type ItemHistoryStorage = Arc<DashMap<(u64, i32), Arc<Mutex<ItemHistory>>>>;

impl TypeMapKey for ItemHistory {
    type Value = Arc<Mutex<ItemHistory>>;
}

impl ItemHistory {
    pub fn new() -> ItemHistory {
        Self { items: vec![] }
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

    pub fn extend(&mut self, other: &Self) {
        let mut map: HashMap<(i64, i32), i64> = other
            .items
            .clone()
            .into_iter()
            .map(|(iid, sid, t)| ((iid, sid), t))
            .collect();
        map.extend(self.items.iter().map(|(iid, sid, t)| ((*iid, *sid), *t)));
        self.items = map.iter().map(|((iid, sid), t)| (*iid, *sid, *t)).collect();
        self.purge_old()
    }
}

impl Default for ItemHistory {
    fn default() -> Self {
        Self::new()
    }
}
