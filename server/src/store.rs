use std::collections::HashMap;

use crate::models::{AirQualityData, StationData};

#[derive(Debug, Clone)]
pub struct Store {
    map: HashMap<String, StationData>,
}

impl Store {
    const MAX_RECORDS: i32 = 50;

    pub fn new() -> Self {Self{map: HashMap::new()}}

    pub fn add_record(&self, id: String, new_air_quality: AirQualityData) -> () {
        let current_station_data: Option<StationData> = self.map.get(&id).cloned();
        current_station_data.map(|mut c| -> () {
            c.air_quality.remove(0);
            c.air_quality.push(new_air_quality);
        }
        );
        //current_air_quality.map(|mut current| current.push(air_quality));

        //self.map.insert(id, air_quality).is_some()
    }

    pub fn get_all(&self, id: String) -> Option<&StationData> {
        self.map.get(&id)
    }
}