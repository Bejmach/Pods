use std::time::Duration;

use clap::Parser;

use crate::{cli::Command, daemon::Daemon, helpers::{group_to_map, group_to_vector}};


mod cli;
mod daemon;
mod db;
mod helpers;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let cmd = Command::parse();

    match cmd {
        Command::Enable => Daemon::run().await?,
        Command::Add(notif) => {
            db::add_notification(notif.app_name, notif.app_icon.unwrap_or(String::new()), notif.summary, notif.body).await?;
        },
        Command::Get(flags) => {

            if flags.group_hash{
                let notes = if flags.all{
                    group_to_map(db::get_all().await?)
                }else{
                    group_to_map(db::get_recent(flags.since.unwrap_or(10)).await?)
                };

                println!("{}", serde_json::to_string(&notes)?);
            }
            else if flags.group_vec{
                let notes = if flags.all{
                    group_to_vector(db::get_all().await?)
                }else{
                    group_to_vector(db::get_recent(flags.since.unwrap_or(10)).await?)
                };

                println!("{}", serde_json::to_string(&notes)?);
            }
            else{
                let notes = if flags.all{
                    db::get_all().await?
                }else{
                    db::get_recent(flags.since.unwrap_or(10)).await?
                };
                    
                println!("{}", serde_json::to_string(&notes)?);
            }
        }
        Command::Listen(flags) => {
            loop{
                if flags.group_hash{
                    let notes = if flags.all{
                        group_to_map(db::get_all().await?)
                    }else{
                        group_to_map(db::get_recent(flags.since.unwrap_or(10)).await?)
                    };

                    println!("{}", serde_json::to_string(&notes)?);
                }
                else if flags.group_vec{
                    let notes = if flags.all{
                        group_to_vector(db::get_all().await?)
                    }else{
                        group_to_vector(db::get_recent(flags.since.unwrap_or(10)).await?)
                    };

                    println!("{}", serde_json::to_string(&notes)?);
                }
                else{
                    let notes = if flags.all{
                        db::get_all().await?
                    }else{
                        db::get_recent(flags.since.unwrap_or(10)).await?
                    };
                        
                    println!("{}", serde_json::to_string(&notes)?);
                }

                tokio::time::sleep(Duration::from_secs(1)).await;
            }
        }
        Command::Remove { id, app_name } => {
            if let Some(id) = id{
                db::remove(id).await?;
            }else if let Some(app_name) = app_name{
                db::remove_by_app_name(app_name).await?;
            }
        },
        Command::Clear => db::clear().await?,
        _ => {}
    }

    Ok(())
}
