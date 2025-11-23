use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::db::Notification;

#[derive(Serialize, Deserialize)]
pub struct NotifGroup{
    pub app_name: String,
    pub notifs: Vec<Notification>,
}

pub fn group_to_hashmap(notifs: Vec<Notification>) -> HashMap<String, Vec<Notification>>{
    let mut groups: HashMap<String, Vec<Notification>> = HashMap::new();

    for notif in notifs{
        if let Some(notif_vec) = groups.get_mut(&notif.app_name){
            notif_vec.push(notif);
        }
        else{
            groups.insert(notif.app_name.clone(), vec![notif]);
        }
    }

    groups
}

pub fn group_to_vector(notifs: Vec<Notification>) -> Vec<NotifGroup>{
    let group_hashmap = group_to_hashmap(notifs);

    group_hashmap.iter().map(|(k, v)| NotifGroup{app_name: k.to_string(), notifs: v.clone()}).collect()
}
