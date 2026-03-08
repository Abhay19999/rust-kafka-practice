mod errors;
mod handlers;
mod models;
mod routes;
mod services;
mod state;



use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use state::app_state::AppState;

#[tokio::main]
async fn main() {
        let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
        next_id: Arc::new(Mutex::new(1)),
    };

    let app  = routes::create_router(state);

    let address  = SocketAddr::from(([127,0,0,1], 8080));
    println!("Serve is runnig on {}", address);

    let listner = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listner, app).await.unwrap();
    
}