use std::env;
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

        let channel = Channel::read_from(&*response).unwrap();
        let item = channel.items[0].clone().pub_date.unwrap();
        let date = steam_date_to_rfc3339(item);
        date
    }

    //TODO Need to fix error handling, return httpresponse if ID is unknown.
    pub async fn steam_feed(&self, game: &String) -> Vec<(String, String, DateTime, String)> {
        let app_id = get_steam_app_id(game);

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
    pub async fn post_to_discord(&self, game: &String, insert_item: &(String, String, DateTime, String)) -> Response {
        println!("Post to discord");
        let discord_url = get_steam_app_webhook(&game);
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

        println!("{:?}", response);
        response
    }
}




//EXTRA FUNCTIONS
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

fn get_steam_app_webhook (game: &String) -> String {
    let app_id: String;
    match game.as_str() {
        "witcher3" => {
            app_id = match env::var("WITCHER3_HOOK") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };
        }
        "cyberpunk2077" => {
            app_id = match env::var("CYBERPUNK2077_HOOK") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };            }
        "stormworks" => {
            app_id = match env::var("STORMWORKS_HOOK"){
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };            }
        "ready-or-not" => {
            app_id = match env::var("READY_OR_NOT_HOOK") {
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

fn steam_date_to_rfc3339(date_string: String) -> DateTime {
    //Wed, 04 Jan 2023 15:20:01 +0000
    let yyyy = date_string.get(12..16).unwrap().to_string();
    let month = date_string.get(8..11).unwrap().to_string();
    let MM = month_to_number(month);
    let dd = date_string.get(5..7).unwrap().to_string();
    let hh = date_string.get(17..19).unwrap().to_string();
    let mm = date_string.get(20..22).unwrap().to_string();
    let ss = date_string.get(23..25).unwrap().to_string();
    //2022-12-22T13:06:22.000+00:00

    let temp_rfc_date = format!("{yyyy}-{MM}-{dd}T{hh}:{mm}:{ss}.000+00:00");
    let rfc_date = DateTime::parse_rfc3339_str(temp_rfc_date).unwrap();
    rfc_date
}

fn month_to_number(month: String) -> String {
    let month_num;

    match month.as_str() {
        "Jan" => {
            month_num = "01".to_string();
        }
        "Feb" => {
            month_num = "02".to_string();
        }
        "Mar" => {
            month_num = "03".to_string();
        }
        "Apr" => {
            month_num = "04".to_string();
        }
        "May" => {
            month_num = "05".to_string();
        }
        "Jun" => {
            month_num = "06".to_string();
        }
        "Jul" => {
            month_num = "07".to_string();
        }
        "Aug" => {
            month_num = "08".to_string();
        }
        "Sep" => {
            month_num = "09".to_string();
        }
        "Oct" => {
            month_num = "10".to_string();
        }
        "Nov" => {
            month_num = "11".to_string();
        }
        "Dec" => {
            month_num = "12".to_string();
        }
        _ => {
            month_num = "00".to_string();
        }
    }
    month_num
}