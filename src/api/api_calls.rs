use crate::{repositories::mongodb_repo::MongoRepo};

use crate::{repositories::request_repo::FeedRepo};
use actix_web::{
    get,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::Client;

use crate::models::feed_item::FeedItem;


//#[post("/user")]
//pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
//    let data = User {
//        id: None,
//        name: new_user.name.to_owned(),
//        location: new_user.location.to_owned(),
//        title: new_user.title.to_owned(),
//    };
//    let user_detail = db.create_user(data).await;
//    match user_detail {
//        Ok(user) => HttpResponse::Ok().json(user),
//        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//    }
//}

//#[get("/user/{id}")]
//pub async fn get_user(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
//    let id = path.into_inner();
//    if id.is_empty() {
//        return HttpResponse::BadRequest().body("invalid ID");
//    }
//    let user_detail = db.get_user(&id).await;
//    match user_detail {
//        Ok(user) => HttpResponse::Ok().json(user),
//        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
//    }
//}

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
pub async fn test(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let game = path.into_inner();
    if game.is_empty() {
        return HttpResponse::BadRequest().body("Unsupported game/software");
    }

    let client = FeedRepo::init().await;
    client.steam_feed(game).await;

    HttpResponse::Ok().json("Henlo")
}
