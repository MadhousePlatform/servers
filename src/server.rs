use reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Client,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerAttributes {
    name: String,
    uuid: String,
}

#[derive(Deserialize)]
pub struct ServerList {
    attributes: ServerAttributes,
}

#[derive(Deserialize)]
pub struct ServerResponse {
    data: Vec<ServerList>,
}

pub async fn get_servers() -> Vec<ServerAttributes> {
    let client = Client::new();

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", std::env::var("API_KEY").unwrap())
            .parse()
            .unwrap(),
    );

    let response: ServerResponse = client
        .get("https://panel.madhouseminers.com/api/application/servers")
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    response.data.into_iter().map(|d| d.attributes).collect()
}
