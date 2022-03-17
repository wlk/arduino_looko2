use std::convert::Infallible;
use std::net::SocketAddr;

use chrono::prelude::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 6543));

    let get_station_data = warp::path!("station" / String)
        .and_then(|station_id| handle_get_station_data(station_id));

    println!("Listening on http://{}", addr);

    warp::serve(get_station_data)
        .run(addr)
        .await;
}

fn current_time_string() -> String {
    Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

async fn handle_get_station_data(station_id: String) -> Result<impl warp::Reply, Infallible> {
    let x: Option<String> = latest_get_look_o2_body(&station_id).await;
    println!("{:?}", x);
    let current_data: Option<AirQualityData> = x.and_then(parse_look_o2_body);
    let mut air_quality: Vec<AirQualityData> = Vec::new();
    current_data.map(|cd| air_quality.insert(0, cd));
    let response = StationData { id: station_id, date: current_time_string(), air_quality };
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

fn capture<'a>(pattern: Regex, capture_name: &'a str, body: &'a str) -> Option<&'a str> {
    pattern.captures(&body).and_then(|c| {
        let value = c.name(capture_name).map(|x| x.as_str());
        println!("{:?}", value);
        value
    })
}

fn parse_look_o2_body(body: String) -> Option<AirQualityData> {
    let capture_name_pm25 = "pm25";
    let capture_name_date = "date";
    let pm25_pattern = Regex::new(r#"<H4>PM2\.5</H4><BR>(?P<pm25>.*) ug"#).unwrap();
    let date_pattern = Regex::new(r#"</h6><h6>Ostatnio widziany: (?P<date>.*)</h6>"#).unwrap();
    if body.len() < 1000 {
        Some(AirQualityData { pm25: -1.0, date: current_time_string() }) // for now not sure how to represent invalid data, so assuming "-1.0" is good enough
    } else {
        let pm25_capture = capture(pm25_pattern, capture_name_pm25, &body);
        let date_capture = capture(date_pattern, capture_name_date, &body);

        let pm25_value = pm25_capture.map(|c| c.parse::<f32>().unwrap_or(0.0));
        let date_value = date_capture.map(|c| c);

        match (pm25_value, date_value) {
            (Some(pm25), Some(date)) => Some(AirQualityData{pm25, date: date.to_string()}),
            _ => None
        }
    }
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