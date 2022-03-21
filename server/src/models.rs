use serde_derive::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AirQualityData {
    pub pm25: f32,
    pub date: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StationData {
    pub id: String,
    pub air_quality: Vec<AirQualityData>,
}
