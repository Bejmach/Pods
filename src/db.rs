use std::{env::home_dir, path::PathBuf};

use anyhow::anyhow;
use chrono::Utc;
use sled::Db;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Notification{
    pub id: u32,
    pub app_name: String,
    pub app_icon: String,
    pub summary: String,
    pub body: String,
    pub timestamp: i64,
}

impl Notification{
    pub fn new(id: u32, app_name: String, app_icon: String, summary: String, body: String) -> Self{
        let timestamp = Utc::now().timestamp();

        Self { id, app_name, app_icon, summary, body, timestamp }
    }
}

pub fn get_home_dir() -> Option<PathBuf>{
    dirs_next::home_dir()
}

pub fn open_db() -> anyhow::Result<Db>{
    if let Some(home_dir) = get_home_dir(){
        let file_path = home_dir.join(".local").join("share").join(".notifdb").to_str().expect("error getting path ~/home/share/.notifdb").to_string();
        return Ok(sled::open(file_path)?);
    }
    Err(anyhow!("No home dir found"))
}

pub async fn add_notification(id: u32, app_name: String, app_icon: String, summary: String, body: String) -> anyhow::Result<()>{
    let db = open_db()?;
    let notif = Notification::new(id, app_name, app_icon, summary, body);
    let key = format!("{}", notif.timestamp);

    db.insert(key.as_bytes(), serde_json::to_vec(&notif)?)?;
    db.flush()?;

    Ok(())
}

pub async fn get_recent(seconds: u64) -> anyhow::Result<Vec<Notification>>{
    let db = open_db()?;
    let now = Utc::now().timestamp();
    let bound = now - seconds as i64;

    let mut notifs: Vec<Notification> = vec![];
    for item in db.iter(){
        let (_, val) = item?;
        let notif: Notification = serde_json::from_slice(&val)?;
        if notif.timestamp >= bound{
            notifs.push(notif);
        }
    }

    Ok(notifs)
}
pub async fn get_all() -> anyhow::Result<Vec<Notification>>{
    let db = open_db()?;
    let mut notifs: Vec<Notification> = vec![];
    for item in db.iter(){
        let (_, val) = item?;
        let notif: Notification = serde_json::from_slice(&val)?;
        notifs.push(notif);
    }

    Ok(notifs)
}
pub async fn remove(id: u32) -> anyhow::Result<()>{
    let db = open_db()?;
    let key = format!("{}", id);
    
    db.remove(key.as_bytes())?;

    Ok(())
}

pub async fn clear() -> anyhow::Result<()>{
    let db = open_db()?;
    db.clear()?;
    Ok(())
}
