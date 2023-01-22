use std::env;
use dotenv::dotenv;

use mongodb::{bson::{doc, extjson::de::Error, oid::ObjectId},
              results::{InsertOneResult},
              Client, Collection, Database};
use serde::{Serialize, Deserialize};

use futures::stream::TryStreamExt;

use crate::models::feed_item::FeedItem;

pub struct MongoRepo {
    db: Database,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URL") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("update-entries");
        MongoRepo { db }
    }
//    pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
//        let new_doc = User {
//            id: None,
//            name: new_user.name,
//            location: new_user.location,
//            title: new_user.title,
//        };
//        let col: Collection<User> = self.db.collection("User");
//        let user = col
//           .insert_one(new_doc, None)
//            .await
//            .ok()
//            .expect("Error creating user");
//        Ok(user)
//    }
//   pub async fn get_user(&self, id: &String) -> Result<User, Error> {
//        let obj_id = ObjectId::parse_str(id).unwrap();
//        let filter = doc! {"_id": obj_id};
//        let col: Collection<User> = self.db.collection("User");
//        let user_detail = col
//            .find_one(filter, None)
//            .await
//            .ok()
//            .expect("Error getting user's detail");
//        Ok(user_detail.unwrap())
//    }

    pub async fn get_all_items(&self, coll: String) -> Result<Vec<FeedItem>, Error> {
        let col: Collection<FeedItem> = self.db.collection(&coll);
        let mut cursors = col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of items");
        let mut items: Vec<FeedItem> = Vec::new();
        while let Some(item) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            items.push(item)
        }
        Ok(items)
    }
}