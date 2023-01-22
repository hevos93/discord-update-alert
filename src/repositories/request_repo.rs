use std::env;
use std::io::{BufRead, BufReader};
use actix_web::HttpResponse;
use actix_web::web::Buf;
use dotenv::dotenv;
use reqwest::Client;
use rss::Channel;

pub struct FeedRepo {
    client: Client
}

impl FeedRepo {
    pub async fn init() -> Self {
        let client = Client::new();

        FeedRepo { client }
    }

    pub async fn steam_feed(&self, game: String){
        let mut app_id: String;
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
                HttpResponse::BadRequest().json("Unsupported game/software");
                return
            }
        }
        let url = format!("https://store.steampowered.com/feeds/news/app/{}", app_id);

        let response = self.client.get(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        //let reader = Ok(BufRead::new(response)).unwrap();
        //let test = BufReader::new(response);

        let channel = Channel::read_from(&*response).unwrap();
        let channel_items = channel.items;

        let xml_contents: Vec<Vec<String>> = vec![vec![]];

        for item in channel_items {
            
        }

        println!("{:?}", channel_items);

    }
}