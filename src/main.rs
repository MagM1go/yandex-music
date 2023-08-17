mod consts;
mod http;
mod radio_manager;
mod track_manager;

use std::{thread, time};
use tokio;
use discord_rpc_client::Client;
use rand::Rng;
use reqwest::Error;
use serde_json::Value;

use crate::consts::API_BASE;
use crate::consts::DEVICE;
use crate::http::{create_get_request, create_post_request};

fn create_rpc(track_name: &str, track_small_image: &str) {
    let application_id: u64 = 881072092035047494;
    let mut client = Client::new(application_id);

    client.start();
    client.set_activity(|activity| activity
        .state("Кто")
        .assets(|asset| asset.large_image("yamusic"))
    ).expect("Fail idk why");
    thread::sleep(time::Duration::from_secs(10));
    client.clear_activity().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    loop {
        create_rpc("asd", "sad");
        thread::sleep(time::Duration::from_secs(10));
    }
}