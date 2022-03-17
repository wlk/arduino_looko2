use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AirQualityData {
    pub pm25: f32,
    pub date: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StationData {
    pub id: String,
    pub date: String,
    pub air_quality: Vec<AirQualityData>,
}
