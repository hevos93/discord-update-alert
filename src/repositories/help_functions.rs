use mongodb::bson::DateTime;
use std::env;

pub fn steam_check_valid_app_id(app_id: &String) -> bool {
    let valid_id = vec!["292030", "1091500", "573090", "1144200"];
    let mut result: bool = false;

    for id in valid_id {
        if app_id == id {
            result = true }
    };
    result
}

pub fn steam_get_app_webhook (game: &String) -> String {
    let app_id: String;
    match game.as_str() {
        "292030" => {
            app_id = match env::var("WITCHER3_HOOK") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };
        }
        "1091500" => {
            app_id = match env::var("CYBERPUNK2077_HOOK") {
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };            }
        "573090" => {
            app_id = match env::var("STORMWORKS_HOOK"){
                Ok(v) => v.to_string(),
                Err(_) => format!("Error loading env variables")
            };            }
        "1144200" => {
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

pub fn steam_date_to_rfc3339(date_string: String) -> DateTime {
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

pub fn month_to_number(month: String) -> String {
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