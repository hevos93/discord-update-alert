use crate::repositories::help_functions::{steam_get_app_webhook, steam_date_to_rfc3339};

use mongodb::bson::DateTime;
use reqwest::{Client, Response};
use rss::{Channel, Enclosure};
use serde_json::json;



pub struct ReqwestRepo {
    client: Client
}

impl ReqwestRepo {
    pub async fn init() -> Self {
        let client = Client::new();

        ReqwestRepo { client }
    }

//GET DATA FUNCTIONS
    pub async fn steam_app_check_date (&self, app_id: &String) -> DateTime {
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
        date
    //TODO optimise here with returning the document used in the request aswell. Avoids duplicate requests.
    }

    pub async fn steam_feed(&self, app_id: &String) -> Vec<(String, String, DateTime, String)> {
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
        channel_content
    }


    //POST TO DISCORD FUNCTION
    pub async fn post_to_discord(&self, app_id: &String, insert_item: &(String, String, DateTime, String)) -> Response {
        let discord_url = steam_get_app_webhook(&app_id);
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
        response
    }

    pub async fn post_request(&self, url: String) {
        self.client.post(url).send().await.unwrap();
    }
}