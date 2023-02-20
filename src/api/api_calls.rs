use crate::{repositories::mongodb_repo::MongoRepo};
use crate::{repositories::request_repo::ReqwestRepo};
use actix_web::{get, web::{Data, Path}, HttpResponse, post};
use crate::repositories::help_functions::steam_check_valid_app_id;


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

#[post("/steam/{app_id}")]
pub async fn steam_games(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let app_id = path.into_inner();
    if app_id.is_empty() {
        return HttpResponse::BadRequest().body("Unsupported game/software");
    }

    let valid_id = steam_check_valid_app_id(&app_id);
    if !valid_id {
        return HttpResponse::BadRequest().body("Unsupported game/software")
    }

    let client = ReqwestRepo::init().await;

    let latest_feed_pub_date = client.steam_app_check_date(&app_id).await;
    let latest_db_pub = db.get_latest_item(&app_id).await;

    if latest_feed_pub_date == latest_db_pub {
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

        HttpResponse::Ok().json("hello")
    }
}

#[post("/steam_feeds")]
pub async fn update_feeds() -> HttpResponse {

    let feeds = vec![
        "1091500".to_string(),  //Cyberpunk2077
        "292030".to_string(),   //Witcher3
        "573090".to_string(),   //Stormworks
        "1144200".to_string(),  //ReadyOrNot
    ];

    let client = ReqwestRepo::init().await;

    for app_id in feeds {
        let url: String = format!("http://localhost:4000/steam/{}", app_id);
        client.post_request(url).await;
    }

    HttpResponse::Ok().json("Feeds updated")
}

#[get("/test")]
pub async fn test() -> HttpResponse {

    HttpResponse::Ok().json("Test complete")
}