use anyhow::Result;
use crate::storage::sqlite::task::{ add_task, complete_task, delete_task, edit_task, get_tasks, incomplete_task };
use sqlx::SqlitePool;
use crossterm::terminal::size as terminal_size;

pub async fn list(pool: &SqlitePool) -> Result<()> {
    let tasks = get_tasks(pool).await?;
    if tasks.is_empty() {
        println!("📝  No tasks found.");
    } else {
        let (cols, _) = terminal_size().unwrap_or((80, 20));
        let width = cols as usize;


        for task in tasks {
            let status = if task.completed { "✓" } else { " " };
            let left = format!("[{}] {}", status, task.title);
            let right = format!("id: {}", task.id);

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

pub async fn add(pool: &SqlitePool, arg: &str) -> Result<()> {
    let title = arg.trim();

    if title.is_empty() {
        println!("Usage: :add <task name>");
        return Ok(());
    }
    
    add_task(pool, title).await?;

    println!("📝  Task added successfully.");
    

    Ok(())
}

pub async fn complete(pool: &SqlitePool, id: &str) -> Result<()> {
    let num_id = id.trim().parse::<i64>();

    if num_id.is_err() {
        println!("Usage: :complete <task id>");
        return Ok(());
    }

    let true_id = num_id.unwrap();

    if true_id <= 0 {
        println!("Couldn't find task with ID: {}", id);
        return Ok(());
    }

    complete_task(pool, true_id).await?;
    println!("📝  Task marked as completed.");

    Ok(())
}

pub async fn incomplete(pool: &SqlitePool, id: &str) -> Result<()> {
    let num_id = id.trim().parse::<i64>();

    if num_id.is_err() {
        println!("Usage: :incomplete <task id>");
        return Ok(());
    }

    let true_id = num_id.unwrap();

    if true_id <= 0 {
        println!("Couldn't find task with ID: {}", id);
        return Ok(());
    }

    incomplete_task(pool, true_id).await?;
    println!("📝  Task marked as incomplete.");

    Ok(())
}

pub async fn edit(pool: &SqlitePool, id: &str, new_title: &str) -> Result<()> {
    let num_id = id.trim().parse::<i64>();

    if num_id.is_err() {
        println!("Usage: :edit <task id> <new title>");
        return Ok(());
    }

    let true_id = num_id.unwrap();

    if true_id <= 0 {
        println!("Couldn't find task with ID: {}", id);
        return Ok(());
    }

    if new_title.trim().is_empty() {
        println!("New title cannot be empty.");
        return Ok(());
    }

    edit_task(pool, true_id, new_title).await?;
    println!("📝  Task updated.");

    Ok(())
}

pub async fn delete(pool: &SqlitePool, id: &str) -> Result<()> {
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

    delete_task(pool, true_id).await?;

    println!("📝  Task deleted successfully.");

    Ok(())
}