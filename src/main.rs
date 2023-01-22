mod api;
mod models;
mod repositories;

use std::env;
use dotenv::dotenv;

use actix_web::{web::Data, App, HttpResponse, HttpServer, Responder};
use crate::api::api_calls::{get_all_items, test};
use crate::repositories::mongodb_repo::MongoRepo;
use crate::repositories::request_repo::FeedRepo;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    let port = match env::var("API_PORT") {
        Ok(v) => v.to_string(),
        Err(_) => format!("Error loading env variables")
    };
    println!("Starting web socket at port {}", port);
    HttpServer::new(move ||{
        App::new()
            .app_data(db_data.clone())
            .service(get_all_items)
            .service(test)
    })
        .bind(("127.0.0.1", 4000))?
        .run()
        .await
}
