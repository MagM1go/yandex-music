use reqwest::Error;
use serde_json::Value;

use crate::http::{create_get_request, create_post_request};

async fn fetch_queues() -> Result<String, Error> {
    let response = create_get_request("queues", None, None).await?;

    Ok(response.text().await?)
}

async fn fetch_queue_by_id() -> Result<Value, Error> {
    let queues = fetch_queues().await?;
    let parsed_json = parse_json(queues.as_str());

    let unwrapped_json: Value = parsed_json.unwrap();
    let queue_id = &unwrapped_json["result"]["queues"][0]["id"];
    let response = create_get_request(
        &format!("queues/{}", queue_id),
        None,
        None
    ).await?;
    Ok(response.json().await?)
}

async fn fetch_track_by_queue(queue: Value) -> Result<Value, Error> {
    let result = &queue["result"];
    let context_type = &result["context"]["type"];

    let mut track_id: &str = "";
    match context_type.as_str().unwrap() {
        "various" => track_id = &result["tracks"][0]["trackId"].as_str().unwrap(),
        "my_music" => track_id = &result["tracks"][0]["trackId"].as_str().unwrap(),
        _ => {}
    }

    let response = create_post_request(
        format!("tracks?track-ids={}", track_id).as_mut_str(),
        None,
        None
    ).await?;

    Ok(response.json().await?)
}