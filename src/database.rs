use crate::Database;
use crate::vahti::Vahti;
use tracing::info;

impl Database {
    pub async fn add_vahti_entry(&self, url: &str, userid: i64) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
        info!("Lisätään Vahti `{}` käyttäjälle {}", url, userid);
        sqlx::query!(
            "INSERT INTO Vahdit (url, user_id) VALUES (?, ?)",
            url,
            userid,
            )
            .execute(&self.database)
            .await
    }
    pub async fn fetch_vahti_entries_by_url(&self, url: &str) -> Result<Vec<Vahti>, sqlx::Error> {
        info!("Haetaan Vahdit {}...", url);
        sqlx::query_as!(
            Vahti,
            "SELECT * FROM Vahdit WHERE url = ?",
            url
            )
            .fetch_all(&self.database)
            .await
    }
    pub async fn fetch_vahti_entries_by_user_id(&self, userid: i64) -> Result<Vec<Vahti>, sqlx::Error> {
        info!("Haetaan käyttäjän {} Vahdit...", userid);
        sqlx::query_as!(
            Vahti,
            "SELECT * FROM Vahdit WHERE url = ?",
            userid
            )
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
        sqlx::query_as!(
            Vahti,
            "SELECT * FROM Vahdit"
            )
            .fetch_all(&self.database)
            .await
    }
}
