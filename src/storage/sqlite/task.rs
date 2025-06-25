use sqlx::{SqlitePool, Row};
use crate::models::task::Task;
use chrono::{DateTime, Utc};
use anyhow::Result;

pub async fn add_task(pool: &SqlitePool, title: &str) -> Result<Task> {
    let now = Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO tasks (title, created_at, completed) VALUES (?, ?, 0)",
    )
    .bind(title)
    .bind(&now)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();

    let created_at = DateTime::parse_from_rfc3339(&now)?
        .with_timezone(&Utc);

    Ok(Task::new(
        id,
        title.to_string(),
        created_at,
        false,
    ))
}

pub async fn complete_task(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query("UPDATE tasks SET completed = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn incomplete_task(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query("UPDATE tasks SET completed = 0 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_tasks(pool: &SqlitePool) -> Result<Vec<Task>> {
    let rows = sqlx::query(
        "SELECT id, title, created_at, completed FROM tasks"
    )
    .fetch_all(pool)
    .await?;

    let tasks = rows.into_iter().map(|r| {
        let id: i64 = r.get("id");
        let title: String = r.get("title");
        let created_at: String = r.get("created_at");
        let completed: bool = r.get("completed");

        let created_at = DateTime::parse_from_rfc3339(&created_at)
            .expect("Failed to parse created_at")
            .with_timezone(&Utc);

        Task::new(id, title, created_at, completed)
    }).collect();

    Ok(tasks)
}

pub async fn edit_task(pool: &SqlitePool, id: i64, new_title: &str) -> Result<()> {
    sqlx::query("UPDATE tasks SET title = ? WHERE id = ?")
        .bind(new_title)
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn delete_task(pool: &SqlitePool, id: i64) -> Result<()> {
    sqlx::query("DELETE FROM tasks WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}

