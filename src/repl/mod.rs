use std::io::{self, Write};
use sqlx::SqlitePool;
use anyhow::Result;

mod help;
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
            ":timers" => { timer::list(pool).await?; },
            ":start" => { timer::start(pool, arg).await?; },
            ":end" => { timer::end(pool).await?; },
            ":delete" => { timer::delete(pool, arg).await?; },
            ":show" => { timer::show_day(pool).await?; },
            "" => continue,
            _ => {
                println!("Unknown command: {}", cmd);
                println!("Type :help for a list of commands.");
            }
        }
    }

    Ok(())
}