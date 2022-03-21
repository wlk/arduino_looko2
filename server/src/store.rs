use std::collections::HashMap;
use std::sync::Arc;
use chrono::Utc;

use parking_lot::RwLock;

//use crate::date::current_time_string;
use crate::models::{AirQualityData, StationData};

#[derive(Debug, Clone)]
pub struct Store {
    data: Arc<RwLock<HashMap<String, StationData>>>,
}

impl Store {
    const MAX_RECORDS: usize = 50;

    pub fn new() -> Self { Self { data: Arc::new(RwLock::new(HashMap::new())) } }
}

pub fn add_record(id: String, new_air_quality: AirQualityData, store: Store) -> Option<StationData> {
    let current_station_data: Option<StationData> = store.data.read().get(&id).cloned();

    match current_station_data {
        None => {
            // insert new entry
            println!("new record: {:?}", &new_air_quality);
            let new_station_data = StationData {
                id: id.clone(),
                last_request_time: Utc::now(),
                air_quality: Vec::from([new_air_quality]),
            };

            store.data.write().insert(id.clone(), new_station_data);
        }
        Some(_) => {
            // modify existing entry
            current_station_data.map(|mut sd| -> () {
                println!("add record to entry: {:?}", sd);
                if sd.air_quality.len() > Store::MAX_RECORDS {
                    sd.air_quality.remove(0);
                }
                sd.air_quality.push(new_air_quality);
                sd.air_quality.dedup_by(|a, b| a.date == b.date);
                sd.last_request_time = Utc::now();
                store.data.write().insert(id.clone(), sd);
                ()
            }
            );
        }
    }
    get_all_for_station(&id, &store)
}

pub fn get_all_for_station(id: &String, store: &Store) -> Option<StationData> {
    let result = store.data.read().get(id).cloned();

    println!("get_all_for_station: {:?}", result);
    result
}
