use sqlx::{SqlitePool, Row};
use chrono::{Utc, DateTime};
use crate::models::timer::Timer;

pub async fn start_timer(pool: &SqlitePool, note: &str) -> Result<i64, sqlx::Error> {
    let start_time = Utc::now().to_rfc3339();

    let result = sqlx::query(
        "INSERT INTO timers (note, start_time, end_time) VALUES (?, ?, NULL)",
    )
    .bind(note)
    .bind(&start_time)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn end_timer(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let end_time = Utc::now().to_rfc3339();

    sqlx::query(
        r#"
            UPDATE timers
            SET end_time = ?
            WHERE id = (
            SELECT id
            FROM timers
            WHERE end_time IS NULL
            ORDER BY start_time DESC
            LIMIT 1
            )
        "#,
    )
    .bind(&end_time)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_timers(pool: &SqlitePool) -> Result<Vec<Timer>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, note, start_time, end_time FROM timers"
    )
    .fetch_all(pool)
    .await?;

    let timers = rows.into_iter().map(|r| {
        let id: i64 = r.get("id");
        let note: String = r.get("note");
        let start_time_str: String = r.get("start_time");
        let end_time_str: Option<String> = r.get("end_time");

        let start_time = DateTime::parse_from_rfc3339(&start_time_str)
            .expect("Failed to parse start time")
            .with_timezone(&Utc);

        let end_time = end_time_str
            .as_deref()
            .map(|s| {
                DateTime::parse_from_rfc3339(s)
                    .expect("Failed to parse end time")
                    .with_timezone(&Utc)
            });

        Timer::new(id, note, start_time, end_time)
    }).collect();

    Ok(timers)
}

pub async fn delete_timer(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM timers WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}