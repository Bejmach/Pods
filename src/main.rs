use std::time::Duration;

use clap::Parser;

use crate::{cli::Command, daemon::Daemon};


mod cli;
mod daemon;
mod data;
mod db;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let cmd = Command::parse();

    match cmd {
        Command::Enable => Daemon::run().await?,
        Command::Add(notif) => {
            db::add_notification(notif.app_name, notif.app_icon.unwrap_or(String::new()), notif.summary, notif.body).await?;
        },
        Command::Get(flags) => {
            let notes = db::get_recent(flags.since.unwrap_or(10)).await?;
            println!("{}", serde_json::to_string(&notes)?);
        }
        Command::Listen(flags) => {
            loop{
                let notes = db::get_recent(flags.since.unwrap_or(10)).await?;
                println!("{}", serde_json::to_string(&notes)?);
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
        Command::Clear => db::clear().await?,
    }

    Ok(())
}
