#[derive(Queryable, Clone, Debug)]
pub struct DbVahti {
    pub id: i32,
    pub url: String,
    pub user_id: i64,
    pub last_updated: i64,
    pub site_id: i32,
    pub delivery_method: i32,
}

use crate::schema::Vahdit;

#[derive(Insertable)]
#[table_name = "Vahdit"]
pub struct NewVahti {
    pub url: String,
    pub user_id: i64,
    pub last_updated: i64,
    pub site_id: i32,
}

#[derive(Queryable, Clone, Debug)]
pub struct Blacklist {
    pub id: i64,
    pub user_id: i64,
    pub seller_id: i32,
    pub site_id: i32,
}

use crate::schema::Blacklists;

#[derive(Insertable)]
#[table_name = "Blacklists"]
pub struct NewBlacklist {
    pub user_id: i64,
    pub seller_id: i32,
    pub site_id: i32,
}
