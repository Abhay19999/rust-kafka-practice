use std::time::Duration;

use rdkafka::{
    config::ClientConfig,
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
};

use crate::{
    events::{processed_user_created_event::ProcessedUserCreatedEvent,
        raw_user_created_event::RawUserCreatedEvent},
    kafka::topics::{USER_CREATED_RAW_TOPIC, USER_CREATED_PROCESSED_TOPIC},
};

pub fn create_producer(bootstrap_servers: &str) -> FutureProducer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Failed to create Kafka producer")
}

pub async fn publish_raw_user_created_event(
    producer: &FutureProducer,
    event: &RawUserCreatedEvent,
) -> Result<(), KafkaError> {
    let payload = serde_json::to_string(event)
        .expect("Failed to serialize RawUserCreatedEvent");

    let key = event.user_id.to_string();

    producer
        .send(
            FutureRecord::to(USER_CREATED_RAW_TOPIC)
                .key(&key)
                .payload(&payload),
            Duration::from_secs(5),
        )
        .await
        .map(|_| ())
        .map_err(|(error, _)| error)
}

pub async fn publish_processed_user_created_event(
    producer: &FutureProducer,
    event: &ProcessedUserCreatedEvent,
) -> Result<(), KafkaError> {
    let payload = serde_json::to_string(event)
        .expect("Failed to serialize ProcessedUserCreatedEvent");

    let key = event.user_id.to_string();

    producer
        .send(
            FutureRecord::to(USER_CREATED_PROCESSED_TOPIC)
                .key(&key)
                .payload(&payload),
            Duration::from_secs(5),
        )
        .await
        .map(|_| ())
        .map_err(|(error, _)| error)
}