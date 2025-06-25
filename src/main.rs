mod repl;
mod init;
mod storage;
mod models;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to Tempus! Let's boost your productivity.");
    println!("Type :help for a list of commands.");

    let pool = init::main("tempus.db").await?;
    repl::run(&pool).await?;

    Ok(())
}
