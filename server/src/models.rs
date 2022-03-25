use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AirQualityData {
    pub pm25: u8,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StationData {
    pub id: String,
    pub last_request_time: DateTime<Utc>,
    pub air_quality: Vec<AirQualityData>,
}
