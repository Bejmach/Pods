use std::sync::atomic::{AtomicU32, Ordering};

use serde::{Deserialize, Serialize};
use zbus::{interface, Connection};

use crate::db;

static NOTIF_ID: AtomicU32 = AtomicU32::new(0);

pub struct Daemon;

impl Daemon{
    pub async fn run() -> zbus::Result<()>{
        let connection = Connection::session().await?;
    
        connection
            .request_name("org.freedesktop.Notifications")
            .await?;

        connection
            .object_server()
            .at("/org/freedesktop/Notifications", Daemon)
            .await?;
        
        println!("pods daemon started on org.freedesktop.Notifications");
        // Keep running until interrupted
        std::future::pending::<()>().await;
        Ok(())
    }
}

#[interface(name = "org.freedesktop.Notifications")]
impl Daemon{
    fn notify(
        &mut self,
        app_name: &str,
        _replaces_id: u32,
        app_icon: &str,
        summary: &str,
        body: &str,
        _actions: Vec<&str>,
        _hints: std::collections::HashMap<&str, zbus::zvariant::Value>,
        _expire_timeout: i32,
    ) -> u32 {
        let id = NOTIF_ID.fetch_add(1, Ordering::Relaxed);

        let app_name = app_name.to_string();
        let app_icon = app_icon.to_string();
        let summary = summary.to_string();
        let body = body.to_string();

        tokio::spawn(async move{
            let _ = db::add_notification(id, app_name, app_icon, summary, body).await;
        });

        id        
    }
    fn get_server_information(&self) -> (String, String, String, String) {
        (
            "Pods Notification Daemon".to_string(), // Name
            "Bejmach".to_string(),                 // Vendor
            "0.1".to_string(),                      // Version
            "1.2".to_string(),                      // Spec version
        )
    }

    /// Required by spec — advertise supported features.
    fn get_capabilities(&self) -> Vec<&str> {
        vec!["body"]
    }

    /// Required by spec — can be a no-op.
    fn close_notification(&self, _id: u32) {
        println!("Notification closed: {}", _id);
    }
} 
