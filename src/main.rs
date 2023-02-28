mod api;
mod models;
mod repositories;

use std::env;
use dotenv::dotenv;
use actix_web::{web::Data, App, HttpServer};
use crate::api::api_calls::{get_all_items, steam_games, test, update_feeds};
use crate::repositories::mongodb_repo::MongoRepo;
use log::{info, error};
use pretty_env_logger as logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    logger::init();
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let port = match env::var("API_PORT") {
        Ok(v) => v.to_string(),
        Err(_) => {
            error!("Could not find API port env variable");
            format!("Error loading env variables")
        }
    };

    info!("Starting web socket at port {}", port);
    HttpServer::new(move ||{
        App::new()
            .app_data(db_data.clone())
            .service(get_all_items)
            .service(steam_games)
            .service(update_feeds)
            .service(test)
    })
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}