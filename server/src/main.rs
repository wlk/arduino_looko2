use std::convert::Infallible;
use std::net::SocketAddr;

use warp::Filter;
use serde::{Serialize, Deserialize};

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
    Ok(format!("station_id: {}", station_id))
}

#[derive(Serialize, Deserialize, Debug)]
struct StationData {
    id: String,
    current_time: String,
}