use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions, SqlitePool};
use anyhow::Result;

pub async fn main(path: &str) -> Result<SqlitePool> {
    let opts = SqliteConnectOptions::new()
        .filename(path)
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            created_at TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )
        "#,
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS timers (
            id         INTEGER PRIMARY KEY,
            note       TEXT NOT NULL,           
            start_time TEXT NOT NULL,    -- ISO8601
            end_time   TEXT              -- NULL until you end
        );"#,
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}