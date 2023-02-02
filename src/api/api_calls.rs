use crate::{repositories::mongodb_repo::MongoRepo};
use crate::{repositories::request_repo::ReqwestRepo};
use actix_web::{get, web::{Data, Path}, HttpResponse, post};


#[get("/{collection}/items")]
pub async fn get_all_items(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let col = path.into_inner();
    if col.is_empty() {
        return HttpResponse::BadRequest().body("invalid Collection");
    }
    let items = db.get_all_items(col).await;
    match items {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/steam/{game}")]
pub async fn steam_games(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let game = path.into_inner();
    if game.is_empty() {
        return HttpResponse::BadRequest().body("Unsupported game/software");
    }
    let client = ReqwestRepo::init().await;

    let latest_feed_pub_date = client.steam_check_date(&game).await;
    let latest_db_pub = db.get_latest_item(&game).await;

    if latest_feed_pub_date == latest_db_pub {
        return HttpResponse::NotModified().finish();
    } else {
        let request_result = client.steam_feed(&game).await;
        let db_result = db.get_latest_ten_items(&game).await;

        let mut counter:usize = 0;
        while counter < db_result.len(){
            if &latest_db_pub < &request_result[counter].2 {
                db.insert_one_feed_item(&game, &request_result[counter]).await;
                let _discord_response = client.post_to_discord(&game, &request_result[counter]).await;
                //TODO Implement checks for response
            }
            counter += 1;
        }

        HttpResponse::Ok().json("hello")
    }
}

#[post("/test")]
pub async fn test() -> HttpResponse {
    println!("Test initated");
    HttpResponse::Ok().json("Test completed")
}