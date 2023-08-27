mod server;
mod whitelist;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Serialize;
use std::{
    fs::{create_dir_all, File},
    io::prelude::*,
    io::Result,
};

#[derive(Serialize)]
struct Example {
    hello: String,
}

#[get("/")]
async fn get_servers() -> impl Responder {
    HttpResponse::Ok()
        .insert_header(("content-type", "application/json"))
        .body(serde_json::to_string(&server::get_servers().await).unwrap())
}

#[post("/{server_id}/whitelists")]
async fn set_whitelists(info: web::Path<String>, req_body: String) -> impl Responder {
    let whitelist_values =
        whitelist::get_whitelist_uuids(serde_json::from_str(req_body.as_str()).unwrap()).await;
    let whitelist_json = serde_json::to_string(&whitelist_values).unwrap();

    let dir = format!("srv/daemon-data/{}", info.into_inner());
    create_dir_all(dir.clone()).unwrap();
    let mut file = File::create(format!("{}/whitelist.txt", dir)).unwrap();
    file.write_all(whitelist_json.as_bytes()).unwrap();

    HttpResponse::Ok()
        .insert_header(("content-type", "application/json"))
        .body("OK")
}

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    HttpServer::new(|| App::new().service(get_servers).service(set_whitelists))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
