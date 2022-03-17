mod models;
mod looko2_client;
mod controller;
mod store;

use std::net::SocketAddr;

use warp::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 6543));

    //let x = store::Store:://::new();
    //let in_mem_store: store::Store = store::Store::new();

    let get_station_data = warp::path!("station" / String)
        .and_then(|station_id| controller::handle_get_station_data(station_id));

    println!("Listening on http://{}", addr);

    warp::serve(get_station_data)
        .run(addr)
        .await;
}
