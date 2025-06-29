mod repl;
mod init;
mod storage;
mod models;
mod ml_training;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    println!("Welcome to Tempus! Let's boost your productivity.");
    println!("Type :help for a list of commands.");

    ml_training::train_model()?;

    let pool = init::main("tempus.db").await?;
    repl::run(&pool).await?;

    Ok(())
}
