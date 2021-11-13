struct ItemHistory {
    items: Vec<(i64,i64)>
}

impl ItemHistory {
    pub fn check_new(&self, id: i64) -> bool {
        todo!();
    }

    fn purge_old(&mut self) {
        self.items = self.items.iter().filter(|(_,timestamp)| {
            timestamp < &(chrono::Local::now().timestamp() - 600)
        }).map(|t| t.to_owned()).collect();
    }
}
