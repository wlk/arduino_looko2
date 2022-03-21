use std::collections::HashMap;
use std::sync::Arc;

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
                air_quality: Vec::from([new_air_quality]),
            };

            store.data.write().insert(id.clone(), new_station_data);
        }
        Some(_) => {
            // modify existing entry
            current_station_data.map(|mut c| -> () {
                println!("add record to entry: {:?}", c);
                if c.air_quality.len() > Store::MAX_RECORDS {
                    c.air_quality.remove(0);
                }
                c.air_quality.push(new_air_quality);
                store.data.write().insert(id.clone(), c);
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
