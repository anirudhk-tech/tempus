use sqlx::{SqlitePool, Row};
use chrono::{Utc, DateTime};
use crate::models::timer::Timer;
use crate::ml::load_and_predict;

pub async fn start_timer(pool: &SqlitePool, note: &str) -> Result<i64, sqlx::Error> {
    let start_time = Utc::now().to_rfc3339();
    let category = load_and_predict(note).map_err(|ml_err| sqlx::Error::Protocol(ml_err.to_string().into()))?;

    let result = sqlx::query(
        "INSERT INTO timers (note, category, start_time, end_time) VALUES (?, ?, ?, NULL)",
    )
    .bind(note)
    .bind(&category)
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
        "SELECT id, note, category, start_time, end_time FROM timers"
    )
    .fetch_all(pool)
    .await?;

    let timers = rows.into_iter().map(|r| {
        let id: i64 = r.get("id");
        let note: String = r.get("note");
        let category: String = r.get("category");
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

        Timer::new(id, note, category, start_time, end_time)
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

pub async fn get_day_timers(pool: &SqlitePool, date: &str) -> Result<Vec<Timer>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT id, note, category, start_time, end_time \
         FROM timers WHERE DATE(start_time) = ?",
    )
    .bind(date)
    .fetch_all(pool)
    .await?;

    let timers = rows.into_iter()
        .filter_map(|r| {
            let id:    i64             = r.get("id");
            let note:  String          = r.get("note");
            let cat:   String          = r.get("category");
            let s:     String          = r.get("start_time");
            let e_opt: Option<String>  = r.get("end_time");

            let start = match DateTime::parse_from_rfc3339(&s) {
                Ok(dt) => dt.with_timezone(&Utc),
                Err(err) => {
                    eprintln!("⚠️ skip id={} bad start `{}`: {}", id, s, err);
                    return None;
                }
            };

            let end = match e_opt {
                Some(e) => match DateTime::parse_from_rfc3339(&e) {
                    Ok(dt) => Some(dt.with_timezone(&Utc)),
                    Err(err) => {
                        eprintln!("⚠️ skip id={} bad end `{}`: {}", id, e, err);
                        return None;
                    }
                },
                None => None,
            };

            Some(Timer::new(id, note, cat, start, end))
        })
        .collect();

    Ok(timers)
}