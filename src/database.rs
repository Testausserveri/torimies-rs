use std::collections::BTreeMap;
use std::env;

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::sqlite::SqliteConnection;
use serenity::prelude::TypeMapKey;

use crate::models::*;

#[derive(Clone)]
pub struct Database {
    database: Pool<ConnectionManager<SqliteConnection>>,
}

impl TypeMapKey for Database {
    type Value = Database;
}

impl Database {
    pub async fn new() -> Database {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let manager = ConnectionManager::<SqliteConnection>::new(&database_url);
        let database = Pool::builder()
            .build(manager)
            .expect("Failed to creaTe connection pool");

        Self { database }
    }
    pub async fn add_vahti_entry(
        &self,
        arg_url: &str,
        userid: i64,
    ) -> Result<usize, anyhow::Error> {
        let time = chrono::Local::now().timestamp();
        info!("Adding Vahti `{}` for the user {}", arg_url, userid);
        use crate::schema::Vahdit;
        let new_vahti = NewVahti {
            last_updated: time,
            url: arg_url.to_string(),
            user_id: userid,
        };
        Ok(diesel::insert_into(Vahdit::table)
            .values(&new_vahti)
            .execute(&self.database.get()?)?)
    }
    pub async fn remove_vahti_entry(
        &self,
        arg_url: &str,
        userid: i64,
    ) -> Result<usize, anyhow::Error> {
        info!("Removing Vahti `{}` from the user {}", arg_url, userid);
        use crate::schema::Vahdit::dsl::*;
        Ok(
            diesel::delete(Vahdit.filter(url.eq(arg_url).and(user_id.eq(userid))))
                .execute(&self.database.get()?)?,
        )
    }
    pub async fn fetch_vahti_entries_by_url(
        &self,
        arg_url: &str,
    ) -> Result<Vec<Vahti>, anyhow::Error> {
        info!("Fetching Vahtis {}...", arg_url);
        use crate::schema::Vahdit::dsl::*;
        Ok(Vahdit
            .filter(url.eq(arg_url))
            .load::<Vahti>(&self.database.get()?)?)
    }
    pub async fn fetch_vahti_entries_by_user_id(
        &self,
        userid: i64,
    ) -> Result<Vec<Vahti>, anyhow::Error> {
        info!("Fetching the Vahtis of user {}...", userid);
        use crate::schema::Vahdit::dsl::*;
        Ok(Vahdit
            .filter(user_id.eq(userid))
            .load::<Vahti>(&self.database.get()?)?)
    }
    pub async fn fetch_vahti(&self, arg_url: &str, userid: i64) -> Result<Vahti, anyhow::Error> {
        info!("Fetching the user {}'s Vahti {}...", userid, arg_url);
        use crate::schema::Vahdit::dsl::*;
        Ok(Vahdit
            .filter(user_id.eq(userid).and(url.eq(arg_url)))
            .first::<Vahti>(&self.database.get()?)?)
    }
    pub async fn fetch_all_vahtis(&self) -> Result<Vec<Vahti>, anyhow::Error> {
        info!("Fetching all Vahtis...");
        use crate::schema::Vahdit::dsl::*;
        Ok(Vahdit.load::<Vahti>(&self.database.get()?)?)
    }
    pub async fn fetch_all_vahtis_group(
        &self,
    ) -> Result<BTreeMap<String, Vec<i64>>, anyhow::Error> {
        // FIXME: This could be done in sql
        info!("Fetching all vahtis grouping them by url");
        let vahdit = self.fetch_all_vahtis().await?;
        let ret: BTreeMap<String, Vec<i64>> =
            vahdit.into_iter().fold(BTreeMap::new(), |mut acc, v| {
                acc.entry(v.url).or_default().push(v.user_id);
                acc
            });
        Ok(ret)
    }
    pub async fn vahti_updated(
        &self,
        vahti: Vahti,
        timestamp: Option<i64>,
    ) -> Result<usize, anyhow::Error> {
        info!("Vahti {} for the user {}", vahti.url, vahti.user_id);
        use crate::schema::Vahdit::dsl::*;
        let time = timestamp.unwrap_or_else(|| chrono::Local::now().timestamp());
        info!(
            "Newest item {}s ago",
            chrono::Local::now().timestamp() - time
        );
        Ok(
            diesel::update(Vahdit.filter(url.eq(vahti.url).and(user_id.eq(vahti.user_id))))
                .set(last_updated.eq(time))
                .execute(&self.database.get()?)?,
        )
    }
    pub async fn fetch_user_blacklist(&self, userid: i64) -> Result<Vec<i32>, anyhow::Error> {
        info!("Fetching the blacklist for user {}...", userid);
        use crate::schema::Blacklists::dsl::*;
        Ok(Blacklists
            .select(seller_id)
            .load::<i32>(&self.database.get()?)?)
    }
    pub async fn add_seller_to_blacklist(
        &self,
        userid: i64,
        sellerid: i32,
    ) -> Result<usize, anyhow::Error> {
        info!(
            "Adding seller {} to the blacklist of user {}",
            sellerid, userid
        );
        use crate::schema::Blacklists;
        let new_entry = NewBlacklist {
            user_id: userid,
            seller_id: sellerid,
        };
        Ok(diesel::insert_into(Blacklists::table)
            .values(new_entry)
            .execute(&self.database.get()?)?)
    }
    pub async fn remove_seller_from_blacklist(
        &self,
        userid: i64,
        sellerid: i32,
    ) -> Result<usize, anyhow::Error> {
        info!(
            "Removing seller {} from the blacklist of user {}",
            sellerid, userid
        );
        use crate::schema::Blacklists::dsl::*;
        Ok(
            diesel::delete(Blacklists.filter(user_id.eq(userid).and(seller_id.eq(sellerid))))
                .execute(&self.database.get()?)?,
        )
    }
}
