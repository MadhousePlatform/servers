use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MinecraftUser {
    uuid: String,
    name: String,
}

pub struct Whitelist {
    minecraft_users: Vec<MinecraftUser>,
}

pub async fn get_whitelist(_whitelist: String) -> Whitelist {
    let mut ids = Vec::new();
    ids.push(MinecraftUser {
        uuid: String::from("test"),
        name: String::from("test"),
    });

    Whitelist {
        minecraft_users: ids,
    }
}

pub async fn get_whitelist_uuids(whitelists: Vec<String>) -> Vec<MinecraftUser> {
    let mut ids = HashMap::new();
    for i in 0..whitelists.len() {
        let current_ids = get_whitelist(whitelists[i].clone()).await.minecraft_users;
        for j in current_ids {
            ids.insert(j.uuid.clone(), j);
        }
    }
    ids.into_values().collect()
}
