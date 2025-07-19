use std::collections::HashMap;

use crate::models::timer::Timer;
use crate::storage::sqlite::timer::{delete_timer, end_timer, get_day_timers, get_timers, start_timer};
use crossterm::terminal::size as terminal_size;
use chrono::{Local};

pub async fn start(pool: &sqlx::SqlitePool, note: &str) -> Result<i64, sqlx::Error> {
    let result = start_timer(pool, note).await?;
    println!("⏱  Timer started.");
    Ok(result)
}

pub async fn end(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    end_timer(pool).await?;
    println!("⏱  Timer ended.");

    Ok(())
}

pub async fn list(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let timers: Vec<Timer> = get_timers(pool).await?;

    if timers.is_empty() {
        println!("⏱  No timers found.");
    } else {
        for t in &timers {
            let start_display = t.start_time
                .with_timezone(&Local)
                .format("%H:%M %d/%m/%y")
                .to_string();

            let end_display = t.end_time
                .as_ref()
                .map(|e| e.with_timezone(&Local).format("%H:%M %d/%m/%y").to_string())
                .unwrap_or_else(|| "Not ended".into());
            
            let (cols, _) = terminal_size().unwrap_or((80, 20));
            let width = cols as usize;

            let left = format!(
        "{} ({}) [{} - {}]",
        t.note,      
        t.category,   
        start_display,
        end_display
    );
            let right = format!("id: {}", t.id);

            let padding = width.saturating_sub(left.chars().count() + right.chars().count());

            println!(
                "{}{}{}",
                left,
                " ".repeat(padding),
                right,
            );
        }
    }

    Ok(())
}

pub async fn delete(pool: &sqlx::SqlitePool, id: &str) -> Result<(), sqlx::Error> {
    let num_id = id.trim().parse::<i64>();

    if num_id.is_err() {
        println!("Usage: :delete <task id>");
        return Ok(());
    }

    let true_id = num_id.unwrap();

    if true_id <= 0 {
        println!("Couldn't find task.");
        return Ok(());
    }

    delete_timer(pool, true_id).await?;

    println!("⏱  Timer deleted.");
    Ok(())
}

pub async fn show_day(pool: &sqlx::SqlitePool) -> Result<(), sqlx::Error> {
    let today = Local::now().date_naive();
    let date_str = today.format("%Y-%m-%d").to_string();

    let timers: Vec<Timer> = get_day_timers(pool, &date_str).await?;

    if timers.is_empty() {
        println!("⏱  No timers found for today.");
    } else {
        let mut category_durations: HashMap<String, i64> = HashMap::new(); // minutes

        for timer in &timers {
            if let Some(end_utc) = &timer.end_time {
                let start = timer.start_time.with_timezone(&Local);
                let end = end_utc.with_timezone(&Local);

                let duration = end.signed_duration_since(start).num_minutes();
                *category_durations.entry(timer.category.clone()).or_default() += duration;
            }
        }

        println!("\n🧱 Time Spent Today by Category:\n");

        let max_len = category_durations
            .values()
            .copied()
            .max()
            .unwrap_or(1);

        for (category, minutes) in category_durations {
            let bar_len = (minutes * 20 / max_len).max(1) as usize; // scale to 20 chars
            let bar = "█".repeat(bar_len);
            let hours = minutes / 60;
            let rem_minutes = minutes % 60;
            println!("{:<10} | {:<20} {:>2}h {:>2}m", category, bar, hours, rem_minutes);
        }
    }

    Ok(())
}