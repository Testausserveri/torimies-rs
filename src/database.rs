use crate::vahti::Vahti;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;

pub struct Database {
    database: sqlx::SqlitePool,
}

impl TypeMapKey for Database {
    type Value = Arc<Database>;
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
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        let time = chrono::Local::now().timestamp();
        info!("Lisätään Vahti `{}` käyttäjälle {}", url, userid);
        sqlx::query!(
            "INSERT INTO Vahdit (url, user_id, last_updated) VALUES (?, ?, ?)",
            url,
            userid,
            time
        )
        .execute(&self.database)
        .await
    }
    pub async fn fetch_vahti_entries_by_url(&self, url: &str) -> Result<Vec<Vahti>, sqlx::Error> {
        info!("Haetaan Vahdit {}...", url);
        sqlx::query_as!(Vahti, "SELECT * FROM Vahdit WHERE url = ?", url)
            .fetch_all(&self.database)
            .await
    }
    pub async fn fetch_vahti_entries_by_user_id(
        &self,
        userid: i64,
    ) -> Result<Vec<Vahti>, sqlx::Error> {
        info!("Haetaan käyttäjän {} Vahdit...", userid);
        sqlx::query_as!(Vahti, "SELECT * FROM Vahdit WHERE url = ?", userid)
            .fetch_all(&self.database)
            .await
    }
    pub async fn fetch_vahti(&self, url: &str, userid: i64) -> Result<Vahti, sqlx::Error> {
        info!("Haetaan käyttäjän {} Vahti {}...", userid, url);
        sqlx::query_as!(
            Vahti,
            "SELECT * FROM Vahdit WHERE url = ? AND user_id = ?",
            url,
            userid
        )
        .fetch_one(&self.database)
        .await
    }
    pub async fn fetch_all_vahtis(&self) -> Result<Vec<Vahti>, sqlx::Error> {
        info!("Haetaan kaikki Vahdit...");
        sqlx::query_as!(Vahti, "SELECT * FROM Vahdit")
            .fetch_all(&self.database)
            .await
    }
    pub async fn vahti_updated(
        &self,
        vahti: Vahti,
        timestamp: Option<i64>,
    ) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        info!(
            "Vahti päivitetty {} käyttäjälle {}",
            vahti.url, vahti.user_id
        );
        let time = timestamp.unwrap_or(chrono::Local::now().timestamp());
        info!("Newest item {}s ago",chrono::Local::now().timestamp() - time);
        sqlx::query!(
            "UPDATE Vahdit SET last_updated = ? WHERE url = ? AND user_id = ?",
            time,
            vahti.url,
            vahti.user_id,
        )
        .execute(&self.database)
        .await
    }
}
