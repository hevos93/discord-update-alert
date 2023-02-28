use crate::models::feed_item::FeedItem;

use std::env;
use mongodb::{bson::{doc, extjson::de::Error}, Client, Collection, Database};
use futures::stream::TryStreamExt;
use mongodb::bson::{DateTime, from_document};
use log::{info, warn};



pub struct MongoRepo {
    db: Database,
}

impl MongoRepo {
    pub async fn init() -> Self {
        info!("init - mongodb_repo.rs");
        let uri = match env::var("MONGO_URL") {
            Ok(v) => {
                info!("MongoDB url loaded");
                v.to_string()
            },
            Err(_) => {
                warn!("Error loading MongoDB url env variable");
                format!("Error loading env variable")
            },
        };
        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("discord-rss");
        info!("Database loaded and returned, function end");
        MongoRepo { db }
    }

    pub async fn get_all_items(&self, coll: String) -> Result<Vec<FeedItem>, Error> {
        info!("get_all_items - mongodb_repo.rs");
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
        info!("Items fetched and returned, function end");
        Ok(items)
    }
    // TODO fix error
    pub async fn get_latest_item(&self, coll: &String) -> DateTime {
        info!("get_lates_item - mongodb_repo.rs");
        let doc = {
            [
                doc! {
                    "$sort": doc! {
                        "pub_date": -1
                    }
                },
                doc! {
                    "$limit": 1
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
            let pub_date= results[0].pub_date;
            info!("pub_date fetched and returned, function end");
            pub_date

    }

    pub async fn get_latest_ten_items(&self, coll: &String) -> Vec<FeedItem> {
        info!("get_latest_ten_items - mongodb_repo.rs");
        let doc = {
            [
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
        info!("Results fetched and returned, function end");
        results
    }

    pub async fn insert_one_feed_item(&self, coll: &String, insert_item: &(String, String, DateTime, String)) {
        info!("insert_one_feed_item - mongodb_repo.rs");
        let col: Collection<FeedItem> = self.db.collection(&coll);
        let title = insert_item.clone().0;
        let link  = insert_item.clone().1;
        let pub_date = insert_item.clone().2;
        let img = insert_item.clone().3;

        let new_doc = FeedItem {
            id: None,
            title,
            link,
            pub_date,
            img
        };

        let _response = col.insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error inserting item");
        info!("Item inserted, function end");
    }
}