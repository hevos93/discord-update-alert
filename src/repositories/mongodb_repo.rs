use std::env;
use dotenv::dotenv;
use futures::future::ok;

use mongodb::{bson::{doc, extjson::de::Error, oid::ObjectId},
              results::{InsertOneResult},
              Client, Collection, Database};
use serde::{Serialize, Deserialize};

use futures::stream::TryStreamExt;
use futures::StreamExt;
use mongodb::bson::{DateTime, Document, from_bson, from_document};

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
        let db = client.database("discord-rss");
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
    // TODO fix error
    pub async fn get_latest_item(&self, coll: &String) -> DateTime {
        let col: Collection<FeedItem> = self.db.collection(&coll);
        let cursors = col.find_one(Some(doc! {"newest": 1}), None)
            .await
            .unwrap();
        let pub_date = cursors.unwrap().pub_date as DateTime;
        pub_date
    }

    pub async fn get_latest_five_items(&self, coll: &String) -> Vec<FeedItem> {
        let doc = {
            [
                doc! {
                    "$match": doc! {
                        "newest": 0
                    }
                },
                doc! {
                    "$sort": doc! {
                        "pub_date": -1
                    }
                },
                doc! {
                    "$limit": 10
                }
            ]
     };

        let col: Collection<FeedItem> = self.db.collection(&coll);
        let mut cursors = col.aggregate(doc, None)
            .await
            .unwrap();

        let mut results: Vec<FeedItem> = Vec::new();
        let mut feed_item: FeedItem;
        while let Some(item) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            feed_item = from_document(item).unwrap();
            results.push(feed_item)
        }

        results
    }

    pub async fn insert_one_feed_item(&self, coll: &String, insert_item: &(String, String, DateTime)) {
        let col: Collection<FeedItem> = self.db.collection(&coll);
        let title = insert_item.clone().0;
        let link  = insert_item.clone().1;
        let pub_date = insert_item.clone().2;

        let new_doc = FeedItem {
            id: None,
            title,
            link,
            pub_date,
            newest: 0
        };

        let response = col.insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error inserting item");
        println!("{:?}", response);
    }
}