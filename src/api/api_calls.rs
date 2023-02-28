use crate::{repositories::mongodb_repo::MongoRepo};
use crate::{repositories::request_repo::ReqwestRepo};
use actix_web::{get, web::{Data, Path}, HttpResponse, post};
use log::{info, warn};
use crate::repositories::help_functions::steam_check_valid_app_id;


#[get("/{collection}/items")]
pub async fn get_all_items(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    info!("get_all_items - api_calls.rs");
    let col = path.into_inner();
    if col.is_empty() {
        warn!("No collection provided in path");
        return HttpResponse::BadRequest().body("invalid Collection");
    }
    info!("/{col}/items");
    let items = db.get_all_items(col).await;
    match items {
        Ok(items) => {
            info!("Collection returned, function end");
            HttpResponse::Ok().json(items)
        },
        Err(err) => {
            warn!("An error occured");
            HttpResponse::InternalServerError().body(err.to_string())
        },
    }
}

#[post("/steam/{app_id}")]
pub async fn steam_games(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    info!("steam_games - api_calls.rs");
    let app_id = path.into_inner();
    if app_id.is_empty() {
        warn!("Unsupported game or software");
        return HttpResponse::BadRequest().body("Unsupported game/software");
    }

    let valid_id = steam_check_valid_app_id(&app_id);
    if !valid_id {
        warn!("Unsupported game or software");
        return HttpResponse::BadRequest().body("Unsupported game/software")
    }

    let client = ReqwestRepo::init().await;

    let latest_feed_pub_date = client.steam_app_check_date(&app_id).await;
    let latest_db_pub = db.get_latest_item(&app_id).await;

    if latest_feed_pub_date == latest_db_pub {
        info!("No update to feeds, function end");
        return HttpResponse::NotModified().finish();
    } else {
        let request_result = client.steam_feed(&app_id).await;
        let db_result = db.get_latest_ten_items(&app_id).await;

        let mut counter:usize = 0;
        while counter < db_result.len(){
            if &latest_db_pub < &request_result[counter].2 {
                db.insert_one_feed_item(&app_id, &request_result[counter]).await;
                let _discord_response = client.post_to_discord(&app_id, &request_result[counter]).await;
                //TODO Implement checks for response
            }
            counter += 1;
        }
        info!("Update finished, function end");
        HttpResponse::Ok().json("hello")
    }
}

#[post("/steam_feeds")]
pub async fn update_feeds() -> HttpResponse {
    info!("update_feeds - api_calls.rs");
    let feeds = vec![
        "1091500".to_string(),  //Cyberpunk2077
        "292030".to_string(),   //Witcher3
        "573090".to_string(),   //Stormworks
        "1144200".to_string(),  //ReadyOrNot
        "648800".to_string(),   //Raft
        "990080".to_string(),   //HogwartsLegacy
        "1196310".to_string(),  //FantasyPlaygroundsUnity
    ];

    let client = ReqwestRepo::init().await;

    for app_id in feeds {
        let url: String = format!("http://localhost:4000/steam/{}", app_id);
        info!("URL: {}", url);
        client.post_request(url).await;
    }
    info!("Feeds updated, function end");
    HttpResponse::Ok().json("Feeds updated")
}

#[get("/test")]
pub async fn test() -> HttpResponse {

    HttpResponse::Ok().json("Test complete")
}