use std::convert::Infallible;
use std::net::SocketAddr;

use chrono::prelude::*;
//use futures::{FutureExt, TryFutureExt};
//use futures::FutureExt;
//use reqwest::Response;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 6543));

    let get_station_data = warp::path!("station" / String)
        .and_then(|station_id| get_station_data(station_id));

    println!("Listening on http://{}", addr);

    warp::serve(get_station_data)
        .run(addr)
        .await;
}

async fn get_station_data(station_id: String) -> Result<impl warp::Reply, Infallible> {
    let current_time: String = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let x: Option<String> = latest_get_look_o2_body(&station_id).await;//.map(|d| println!("{}", d));
    println!("{:?}", x);
    let current_data: Option<AirQualityData> = x.and_then(parse_look_o2_body);
    let mut air_quality: Vec<AirQualityData> = Vec::new();
    current_data.map(|cd| air_quality.insert(0, cd));
    //air_quality.insert(0, current_data);
    let response = StationData { id: station_id, date: current_time, air_quality };
    Ok(warp::reply::json(&response))
}

async fn latest_get_look_o2_body(station_id: &String) -> Option<String> {
    let url: String = format!("https://looko2.com/tracker2.php?search={}", station_id);
    let body = match reqwest::get(url).await {
        Ok(r) if r.status() == reqwest::StatusCode::OK => Some(r.text().await.unwrap_or(String::new())),
        _ => None
    };
    body
}

fn parse_look_o2_body(body: String) -> Option<AirQualityData> {
    Some(AirQualityData { pm25: 25.5, date: "abc".into() })
}

#[derive(Serialize, Deserialize, Debug)]
struct AirQualityData {
    pm25: f32,
    date: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct StationData {
    id: String,
    date: String,
    air_quality: Vec<AirQualityData>,
}