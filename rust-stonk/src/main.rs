use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};
mod websocket;
use crate::websocket::handlers;
type Result<T> = std::result::Result<T, Rejection>;
mod datatypes;
use crate::datatypes::client::{Client, Clients};

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    println!("Configuring websocket route");
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .and_then(handlers::ws_handler);
    let routes = ws_route.with(warp::cors().allow_any_origin());
    println!("Starting server");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}
