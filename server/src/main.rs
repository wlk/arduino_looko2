mod models;
mod looko2_client;
mod controller;
mod store;

use std::net::SocketAddr;

use warp::Filter;
use crate::controller::Controller;
use crate::store::Store;

extern crate chrono;
extern crate chrono_tz;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 14479));

    let store: Store = Store::new();

    let store_filter = warp::any().map(move || store.clone());

    let get_station_data = warp::path!("station" / String)
        .and(store_filter.clone())
        .and_then(|station_id, store| Controller::handle_get_station_data(station_id, store));

    println!("Listening on http://{}", addr);

    warp::serve(get_station_data)
        .run(addr)
        .await;
}
