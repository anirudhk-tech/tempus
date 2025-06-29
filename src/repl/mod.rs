use std::io::{self, Write};
use sqlx::SqlitePool;
use anyhow::Result;

mod help;
mod task;
mod timer;

pub async fn run(pool: &SqlitePool) -> Result<()> {
    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let cmd = input.trim();

        let mut parts = cmd.splitn(2, ' ');
        let prefix = parts.next().unwrap_or("");
        let arg = parts.next().unwrap_or("").trim();

        match prefix {
            ":help" => { help::main(); },
            ":exit" | ":quit" => {
                println!("Exiting Tempus. Goodbye!");
                break;
            },
            ":tasks" => { task::list(pool).await?; },
            ":add" => { task::add(pool, arg).await?; },
            ":delete" => { task::delete(pool, arg).await?; },
            ":rename" => { 
                let mut parts = arg.splitn(2, ' ');
                let id = parts.next().unwrap_or("").trim();
                let new_title = parts.next().unwrap_or("").trim();

                task::edit(pool, id, new_title).await?; 
            },
            ":complete" => { task::complete(pool, arg).await?; },
            ":reopen" => { task::incomplete(pool, arg).await?; },
            ":timers" => { timer::list(pool).await?; },
            ":start" => { timer::start(pool, arg).await?; },
            ":end" => { timer::end(pool).await?; },
            ":delete_timer" => { timer::delete(pool, arg).await?; }
            "" => continue,
            _ => {
                println!("Unknown command: {}", cmd);
                println!("Type :help for a list of commands.");
            }
        }
    }

    Ok(())
}