use crate::common::task_config;
use actix_web::rt::Runtime;
use anyhow::Ok;
use autowired::bean;
use std::time::Duration;
use mongodb::{Client, options::ClientOptions};

pub const DB_NAME: &str = "best_todo";

#[bean(option)]
fn build_mongodb_client() -> Option<Client> {
    // let config = task_config();
    // let client = Runtime::new()
    //     .unwrap()
    //     .block_on(Client::with_uri_str(&config.mongodb_uri));
    // log::info!("build mongodb client, uri={}", config.mongodb_uri);
    // client.ok()

    let config = task_config();
    let client = Runtime::new()
        .unwrap()
        .block_on(connect_to_mongo())
        .unwrap();
    log::info!("build mongodb client, uri={}", config.mongodb_uri);
    Some(client)
}

async fn connect_to_mongo() -> Result<Client, anyhow::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017/best_todo").await?;
    client_options.max_pool_size = Some(20);
    client_options.connect_timeout = Some(Duration::from_secs(5));
    let client = Client::with_options(client_options)?;

    // Get a handle to the "best_todo" database.
    let db = client.database("best_todo");

    // Get a handle to the "task" collection.
    let collection = db.collection("task");

    // Insert a document into the "task" collection.
    let document = doc! { "title": "Learn Rust", "is_done": false, "is_important": true, "is_urgent": true };
    collection.insert_one(document, None).await?;

    Ok(client)
}
