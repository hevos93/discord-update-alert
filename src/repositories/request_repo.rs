use crate::repositories::help_functions::steam_date_to_rfc3339;
use crate::models::vault_response::VaultResponse;

use std::env;
use actix_web::HttpResponse;
use mongodb::bson::DateTime;
use reqwest::{Client, Response};
use rss::{Channel, Enclosure};
use serde_json::json;
use log::{info, warn};
use scraper::{Html, Selector};


pub struct ReqwestRepo {
    client: Client
}

impl ReqwestRepo {
    pub async fn init() -> Self {
        info!("init - request_repo.rs");
        let client = Client::new();
        info!("Client returned, function end");
        ReqwestRepo { client }
    }

//RSS FUNCTIONS
    pub async fn steam_app_check_date (&self, app_id: &String) -> DateTime {
    info!("steam_app_check_date - request_repo.rs");
        let url = format!("https://store.steampowered.com/feeds/news/app/{}", app_id);
        let response = self.client.get(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();

        let channel = Channel::read_from(&*response).unwrap();
        let item = channel.items[0].clone().pub_date.unwrap();
        let date = steam_date_to_rfc3339(item);
    info!("Date returned, function end");
        date
    //TODO optimise here with returning the document used in the request aswell. Avoids duplicate requests.
    }

    pub async fn steam_feed(&self, app_id: &String) -> Vec<(String, String, DateTime, String)> {
        info!("steam_feed - request_repo.rs");
       let url = format!("https://store.steampowered.com/feeds/news/app/{}", &app_id);

        let response = self.client.get(url)
            .send()
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();


        let channel = Channel::read_from(&*response).unwrap();

        let mut channel_content = Vec::new();

        let header_url = format!("https://cdn.akamai.steamstatic.com/steam/apps/{}/header.jpg", &app_id);
        let mut enclosure = Enclosure::default();
        enclosure.set_url(header_url);

        for item in channel.items {
            let title = item.title.unwrap();
            let link = item.link.unwrap();
            let date = item.pub_date.unwrap();
            let pub_date = steam_date_to_rfc3339(date);
            let img = item.enclosure.unwrap_or(enclosure.clone()).url;

            let content = (title, link, pub_date, img);
            channel_content.push(content);
        }
        info!("RSS feed content returned, function end");
        channel_content
    }

    //HTML Scraper
    pub async fn get_stock_price(&self, stock_id: &String) -> HttpResponse {
        let html = r#"
            <ul>
                <li id="this one">Foo</li>
                <li>Bar</li>
                <li>Baz</li>
            </ul>
            "#;

        let html_doc = Html::parse_document(html);
        let selector = Selector::parse(r#"input[id="this one"]"#).unwrap();
        let fragment = html_doc.select(&selector).next().unwrap();

        println!("{:?}", fragment);


        HttpResponse::Ok().body("Stonks")
    }






    //POST TO DISCORD FUNCTION
    pub async fn post_to_discord(&self, app_id: &String, insert_item: &(String, String, DateTime, String)) -> Response {
        info!("post_to_discord - request_repo.rs");
        let discord_url = self.get_kv_secret(&app_id).await;
        let title = insert_item.clone().0;
        let link = insert_item.clone().1;
        let image = insert_item.clone().3;


        let body = json!({
            "embeds": [
                {
                    "title": title,
                    "url": link,
                    "color": 3092790,
                    "image": {
                        "url": image
                    }
                },
            ],
        });


        let response = self.client.post(discord_url)
            .json(&body)
            .send()
            .await
            .unwrap();
        info!("News ({title}) posted to discord, function end");
        response
    }

    pub async fn post_request(&self, url: String) {
        info!("post_request - request_repo.rs");
        self.client.post(url).send().await.unwrap();
        info!("Request posted, function end");
    }

    // GET SECRET FROM VAULT
    pub async fn get_kv_secret (&self, game: &str) -> String {
        info!("get_kv_secret - request_repo.rs");
        let vault_url = match env::var("VAULT_URL") {
            Ok(v) => {
                info!("Vault URL env variable loaded");
                v.to_string()
            },
            Err(_) => {
                warn!("Error loading env variables");
                format!("Error loading env variables")
            }
        };
        let vault_token = match env::var("DISCORD_UPDATE_ALERT_VAULT_TOKEN") {
            Ok(v) => {
                info!("Vault token loaded");
                v.to_string()
            },
            Err(_) => {
                warn!("Error loading vault token env variable");
                format!("Error loading env variables")
            }
        };

        let url = format!("{}{}",vault_url,game);

        let response: VaultResponse = self.client.get(url)
            .header("X-Vault-Token", vault_token)
            .send()
            .await
            .unwrap()
            .json()
            .await
            .unwrap();

        info!("Webhook receieved from Vault and returned, function end");
        let webhook = response.data.data.webhook;
        webhook
    }

}