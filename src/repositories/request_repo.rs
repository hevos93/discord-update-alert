use std::env;
use std::io::{BufRead, BufReader};
use actix_web::HttpResponse;
use actix_web::web::Buf;
use dotenv::dotenv;
use mongodb::bson::DateTime;
use reqwest::Client;
use rss::Channel;
use crate::MongoRepo;

pub struct FeedRepo {
    client: Client
}

impl FeedRepo {
    pub async fn init() -> Self {
        let client = Client::new();

        FeedRepo { client }
    }

    pub async fn steam_check_date (&self, game: &String) -> DateTime {
        let app_id = get_steam_app_id(game);
        let url = format!("https://store.steampowered.com/feeds/news/app/{}", app_id);
        let response = self.client.get(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        // TODO This is the correct way to convert to datetime for Channel::pub_date, this needs to be implemented for the rest of the similar cases.
        let channel = Channel::read_from(&*response).unwrap();
        let item = channel.items[1].clone().pub_date.unwrap();

        let date = DateTime::parse_rfc3339_str(item).unwrap();
        println!("{}", date);

        date
    }

    //TODO Need to fix error handling, return httpresponse if ID is unknown.
    pub async fn steam_feed(&self, game: &String) -> Vec<(String, String, DateTime)> {
        let app_id = get_steam_app_id(game);
        let url = format!("https://store.steampowered.com/feeds/news/app/{}", app_id);

        let response = self.client.get(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();


        let channel = Channel::read_from(&*response).unwrap();

        let mut length_counter: usize = 0;
        let mut channel_content = Vec::new();

        for item in channel.items {
            let title = item.title.unwrap();
            let link = item.link.unwrap();
            let pub_date = item.pub_date as DateTime;
            let content = (title, link, pub_date);
            channel_content.push(content);

            length_counter += 1;
        }
        channel_content
    }
}

fn get_steam_app_id (game: &String) -> String {
    let app_id: String;
    match game.as_str() {
        "witcher3" => {
            app_id = match env::var("WITCHER3_ID") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };
        }
        "cyberpunk2077" => {
            app_id = match env::var("CYBERPUNK2077_ID") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };            }
        "stormworks" => {
            app_id = match env::var("STORMWORKS_ID"){
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };            }
        "ready-or-not" => {
            app_id = match env::var("READY_OR_NOT_ID") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };
        }
        _ => {
            app_id= "0".parse().unwrap();
        }
    };
    app_id
}