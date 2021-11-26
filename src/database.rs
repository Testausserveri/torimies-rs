use std::sync::Arc;

use serenity::prelude::TypeMapKey;

use crate::vahti::Vahti;

pub struct Database {
    database: sqlx::SqlitePool,
}

impl TypeMapKey for Database {
    type Value = Arc<Database>;
}

#[derive(Debug)]
pub struct UrlAndUsers {
    url: String,
    users: Vec<i64>,
}

struct UrlAndUsersString {
    url: String,
    users: Option<String>,
}

impl Database {
    pub async fn new() -> Database {
        let database = sqlx::sqlite::SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(
                sqlx::sqlite::SqliteConnectOptions::new()
                    .filename("database.sqlite")
                    .create_if_missing(true),
            )
            .await
            .expect("Couldn't connect to database");
        sqlx::migrate!("./migrations")
            .run(&database)
            .await
            .expect("Couldn't run database migrations");
        Self { database }
    }
    pub async fn add_vahti_entry(
        &self,
        url: &str,
        userid: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, anyhow::Error> {
        let time = chrono::Local::now().timestamp();
        info!("Adding Vahti `{}` for the user {}", url, userid);
        Ok(sqlx::query!(
            "INSERT INTO Vahdit (url, user_id, last_updated) VALUES (?, ?, ?)",
            url,
            userid,
            time
        )
        .execute(&self.database)
        .await?)
    }
    pub async fn remove_vahti_entry(
        &self,
        url: &str,
        userid: i64,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, anyhow::Error> {
        info!("Removing Vahti `{}` from the user {}", url, userid);
        Ok(sqlx::query!(
            "DELETE FROM Vahdit WHERE url = ? AND user_id = ?",
            url,
            userid,
        )
        .execute(&self.database)
        .await?)
    }
    pub async fn fetch_vahti_entries_by_url(&self, url: &str) -> Result<Vec<Vahti>, anyhow::Error> {
        info!("Fetching Vahtis {}...", url);
        Ok(
            sqlx::query_as!(Vahti, "SELECT * FROM Vahdit WHERE url = ?", url)
                .fetch_all(&self.database)
                .await?,
        )
    }
    pub async fn fetch_vahti_entries_by_user_id(
        &self,
        userid: i64,
    ) -> Result<Vec<Vahti>, anyhow::Error> {
        info!("Fetching the Vahtis of user {}...", userid);
        Ok(
            sqlx::query_as!(Vahti, "SELECT * FROM Vahdit WHERE url = ?", userid)
                .fetch_all(&self.database)
                .await?,
        )
    }
    pub async fn fetch_vahti(&self, url: &str, userid: i64) -> Result<Vahti, anyhow::Error> {
        info!("Fetching the user {}'s Vahti {}...", userid, url);
        Ok(sqlx::query_as!(
            Vahti,
            "SELECT * FROM Vahdit WHERE url = ? AND user_id = ?",
            url,
            userid
        )
        .fetch_one(&self.database)
        .await?)
    }
    pub async fn fetch_all_vahtis(&self) -> Result<Vec<Vahti>, anyhow::Error> {
        info!("Fetching all Vahtis...");
        Ok(sqlx::query_as!(Vahti, "SELECT * FROM Vahdit")
            .fetch_all(&self.database)
            .await?)
    }
    pub async fn fetch_all_vahtis_group(&self) -> Result<Vec<UrlAndUsers>, anyhow::Error> {
        info!("Fetching all vahtis grouping them by url");

        let temp = sqlx::query_as!(UrlAndUsersString,
            "SELECT url, GROUP_CONCAT( user_id ) as users FROM Vahdit GROUP BY url")
            .fetch_all(&self.database)
            .await?;
        let mut ret = Vec::new();
        for e in temp {
            let users = e.users.unwrap().split(',').map(|u| u.parse::<i64>().unwrap()).collect();
            ret.push(UrlAndUsers { url: e.url, users } );
        }
        Ok(ret)
    }
    pub async fn vahti_updated(
        &self,
        vahti: Vahti,
        timestamp: Option<i64>,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, anyhow::Error> {
        info!("Vahti {} for the user {}", vahti.url, vahti.user_id);
        let time = timestamp.unwrap_or_else(|| chrono::Local::now().timestamp());
        info!(
            "Newest item {}s ago",
            chrono::Local::now().timestamp() - time
        );
        Ok(sqlx::query!(
            "UPDATE Vahdit SET last_updated = ? WHERE url = ? AND user_id = ?",
            time,
            vahti.url,
            vahti.user_id,
        )
        .execute(&self.database)
        .await?)
    }
}
