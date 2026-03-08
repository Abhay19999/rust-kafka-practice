mod errors;
mod events;
mod handlers;
mod kafka;
mod models;
mod processors;
mod routes;
mod services;
mod state;



use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use events::raw_user_created_event::RawUserCreatedEvent;
use kafka::{consumer,producer};
use state::app_state::AppState;

#[tokio::main]
async fn main() {
    let kafka_producer = producer::create_producer("localhost:9092");
        let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
        next_id: Arc::new(Mutex::new(1)),
    };

    let sample_event = RawUserCreatedEvent {
        event_id: String::from("evt-001"),
        user_id: 101,
        name: String::from("  Abhay Bhatt  "),
        email: String::from("ABHAY@EXAMPLE.COM"),
        source: String::from("user-api"),
    };

        match producer::publish_raw_user_created_event(&kafka_producer, &sample_event).await {
        Ok(_) => println!("Published sample raw user event to Kafka"),
        Err(error) => println!("Failed to publish sample raw user event: {}", error),
    }

    let kafka_consumer = kafka::consumer::create_consumer("localhost:9092");
    let consumer_side_producer = kafka_producer.clone();
    tokio::spawn(async move {
        if let Err(error) = kafka::consumer::start_raw_user_event_consumer(kafka_consumer, consumer_side_producer).await {
            println!("Kafka consumer error: {}", error);
        }
    });


    let app  = routes::create_router(state);

    let address  = SocketAddr::from(([127,0,0,1], 8080));
    println!("Serve is runnig on {}", address);

    let listner = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listner, app).await.unwrap();
    
}