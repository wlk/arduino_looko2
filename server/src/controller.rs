use std::convert::Infallible;

use chrono::{Duration, Utc};

use models::*;
use store::*;

use crate::looko2_client;
use crate::models;
use crate::store;

#[derive(Clone)]
pub struct Controller {}


impl Controller {
    pub fn new() -> Self {
        Self {}
    }


    pub async fn handle_get_station_data(station_id: String, s: Store) -> Result<impl warp::Reply, Infallible> {
        fn xxx(sd: &StationData) -> bool {
            let earlier = Utc::now() - Duration::seconds(15);//Duration::minutes(5);
            let latest_entry = sd.air_quality.last().map(|x| x.date);

            match latest_entry {
                Some(le) if le.gt(&earlier) => true,
                _ => false
            }
        }

        let current = store::get_all_for_station(&station_id, &s);

        let result: Option<StationData> = match current {
            Some(dd) if xxx(&dd) => {
                Some(dd)
            },
            _ => {
                let looko2_body: Option<String> = looko2_client::latest_get_look_o2_body(&station_id).await;
                println!("looko2 returned: {:?}", looko2_body);
                let current_data: Option<AirQualityData> = looko2_body.and_then(looko2_client::parse_look_o2_body);
                println!("handle_get_station_data.current_data: {:?}", current_data);

                current_data.and_then(|new_air_quality| store::add_record(station_id, new_air_quality, s))
            }
        };


        Ok(warp::reply::json(&result))
    }
}
