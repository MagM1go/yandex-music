use std::collections::HashMap;
use std::time;
use rand::Rng;
use reqwest::Error;
use serde_json::Value;

use crate::http::{create_get_request, create_post_request};

async fn fetch_station_list() -> Result<Value, Error> {
    let url = "/rotor/stations/list";
    let response = create_get_request(
        url,
        None,
        Some(vec!(
            ("language", "ru")
        ))
    ).await?;

    Ok(response.json().await?)
}

async fn fetch_station_info() -> Result<Value, Error> {
    let stations_info = fetch_station_list().await?;
    let mut range = rand::thread_rng();
    let stations_data = stations_info.as_object();

    let stations: Vec<&Value> = stations_data
        .unwrap()
        .values()
        .collect();
    let random_number: usize = range.gen_range(1..stations.len());
    let random_station_data: &Value = &stations[random_number][random_number]["station"];

    println!("{}", stations.len());
    Ok(random_station_data.clone())
}

async fn start_radio() -> Result<Value, Error> {
    let timestamp = time::SystemTime::now()
        .duration_since(time::UNIX_EPOCH);
    let frozen_time = (timestamp.unwrap().as_micros() / 1_000_000).to_string();
    let radio_station_info = fetch_station_info().await?;
    let radio_id = &radio_station_info["id"];
    let station_id = format!("{}:{}", radio_id["type"], radio_id["tag"])
        .replace("\"", "");
    let for_from_id = &radio_station_info["idForFrom"];
    println!("{}", radio_station_info);
    let url = format!("/rotor/station/{}/feedback", station_id);
    let radio_type = "radioStarted";

    let mut data = HashMap::new();
    data.insert("type", radio_type);
    data.insert("timestamp", frozen_time.as_str());

    let response = create_post_request(
        &url,
        Some(data),
        Some(vec!(
            ("from", for_from_id.as_str().unwrap())
        ))
    ).await?;

    Ok(response.json().await?)
}
