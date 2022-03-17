use crate::looko2_client;
use crate::models;
use crate::store;

use std::convert::Infallible;
use models::*;
use store::Store;
use chrono::prelude::*;


const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn current_time_string() -> String {
    Utc::now().format(DATE_FORMAT).to_string()
}


pub async fn handle_get_station_data(station_id: String) -> Result<impl warp::Reply, Infallible> {
    let x: Option<String> = looko2_client::latest_get_look_o2_body(&station_id).await;
    println!("looko2 returned: {:?}", x);
    let current_data: Option<AirQualityData> = x.and_then(looko2_client::parse_look_o2_body);
    let mut air_quality: Vec<AirQualityData> = Vec::new();
    current_data.map(|cd| air_quality.insert(0, cd));
    //store::Store
    //store::Store::add_record(station_id, air_quality);
    let response = StationData { id: station_id, date: current_time_string(), air_quality };
    Ok(warp::reply::json(&response))
}
