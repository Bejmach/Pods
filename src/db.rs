use std::{path::PathBuf, str::FromStr, sync::atomic::AtomicU32, time::Duration};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::{SqliteConnectOptions, SqlitePoolOptions}, Pool, Sqlite, SqliteConnection, SqlitePool};
use tokio::sync::OnceCell;

static NOTIF_ID: AtomicU32 = AtomicU32::new(0);

#[derive(Serialize, Deserialize, Clone, FromRow)]
pub struct Notification{
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub timestamp: i64,
}

impl Notification{
    pub fn new(id: u32, app_name: String, app_icon: String, summary: String, body: String) -> Self{
        let timestamp = Utc::now().timestamp();

        Self {id, app_name, app_icon, summary, body, timestamp }
    }
}

pub fn get_home_dir() -> Option<PathBuf>{
    dirs_next::home_dir()
}


static DB_CONN: OnceCell<Pool<Sqlite>> = OnceCell::const_new();

pub async fn db_conn() -> Pool<Sqlite>{
    let path = dirs_next::home_dir()
        .unwrap()
        .join(".local/share/pods.db");

    let url = format!("sqlite://{}", path.to_string_lossy());

    DB_CONN.get_or_init(|| async {
        SqlitePoolOptions::new()
            .max_connections(5)
            .idle_timeout(Duration::from_secs(60))
            .acquire_timeout(Duration::from_secs(5))
            .connect_with(
                SqliteConnectOptions::from_str(&url)
                    .unwrap()
                    .create_if_missing(true)
                    .journal_mode(sqlx::sqlite::SqliteJournalMode::Delete),
            )
            .await
            .unwrap()
    })
    .await
    .clone()
}

pub async fn init() -> anyhow::Result<()> {
    let path = dirs_next::home_dir()
        .unwrap()
        .join(".local/share/pods.db");

    let url = format!("sqlite://{}", path.to_string_lossy());
    let pool = SqlitePool::connect(&url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS notifications (
            id INTEGER PRIMARY KEY,
            app_name TEXT,
            app_icon TEXT,
            summary TEXT,
            body TEXT,
            timestamp INTEGER
        );
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query("PRAGMA journal_mode = WAL;")
        .execute(&pool)
        .await?;

    Ok(())
}

pub async fn add_notification(
    app_name: String,
    app_icon: String,
    summary: String,
    body: String,
) -> anyhow::Result<()> {
    let timestamp = Utc::now().timestamp();

    sqlx::query(
        r#"
        INSERT INTO notifications(id, app_name, app_icon, summary, body, timestamp)
        VALUES (?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(NOTIF_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
    .bind(app_name)
    .bind(app_icon)
    .bind(summary)
    .bind(body)
    .bind(timestamp)
    .execute(&db_conn().await)
    .await?;

    Ok(())
}

pub async fn get_recent(seconds: u64) -> anyhow::Result<Vec<Notification>>{
    
    let now = Utc::now().timestamp();
    let bound = now - seconds as i64;

    let notifs = sqlx::query_as::<_, Notification>(
        r#"
        SELECT id, app_name, app_icon, summary, body, timestamp FROM notifications
        WHERE timestamp > ?
        "#
        )
        .bind(bound)
        .fetch_all(&db_conn().await)
        .await?;
        
    Ok(notifs)

}
pub async fn get_all() -> anyhow::Result<Vec<Notification>>{
    let notifs = sqlx::query_as::<_, Notification>(
        r#"
        SELECT id, app_name, app_icon, summary, body, timestamp FROM notifications
        "#
        )
        .fetch_all(&db_conn().await)
        .await?;
        
    Ok(notifs)
}
pub async fn remove(id: u32) -> anyhow::Result<()>{
    sqlx::query(r#"
        DELETE FROM notifications
        WHERE id = ?
    "#)
    .bind(id)
    .execute(&db_conn().await)
    .await?;

    Ok(())
}

pub async fn remove_by_app_name(app_name: String) -> anyhow::Result<()>{
    sqlx::query(r#"
        DELETE FROM notifications
        WHERE app_name = ?
    "#)
    .bind(app_name)
    .execute(&db_conn().await)
    .await?;

    Ok(())
}

pub async fn clear() -> anyhow::Result<()>{
    sqlx::query(r#"
        DELETE FROM notifications
    "#)
    .execute(&db_conn().await)
    .await?;

    Ok(())
}
