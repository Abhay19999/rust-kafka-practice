use futures::StreamExt;
use rdkafka::{
    config::ClientConfig,
    consumer::{Consumer, StreamConsumer},
    error::KafkaError,
    message::Message,
    producer::FutureProducer,
};

use crate::{
    events::raw_user_created_event::RawUserCreatedEvent,
    kafka::{
        producer,
        topics::{RUST_USER_PROCESSOR_GROUP, USER_CREATED_RAW_TOPIC},
    },
    processors::user_event_processor,
};

pub fn create_consumer(bootstrap_servers: &str) -> StreamConsumer {
    ClientConfig::new()
        .set("bootstrap.servers", bootstrap_servers)
        .set("group.id", RUST_USER_PROCESSOR_GROUP)
        .set("enable.auto.commit", "true")
        .set("auto.offset.reset", "earliest")
        .create()
        .expect("Failed to create Kafka consumer")
}

pub async fn start_raw_user_event_consumer(
    consumer: StreamConsumer,
    producer: FutureProducer,
) -> Result<(), KafkaError> {
    consumer.subscribe(&[USER_CREATED_RAW_TOPIC])?;

    println!(
        "Rust consumer subscribed to topic: {}",
        USER_CREATED_RAW_TOPIC
    );

    let mut message_stream = consumer.stream();

    while let Some(message_result) = message_stream.next().await {
        match message_result {
            Ok(message) => {
                match message.payload_view::<str>() {
                    Some(Ok(payload)) => {
                        println!("Received raw event payload: {}", payload);

                        match serde_json::from_str::<RawUserCreatedEvent>(payload) {
                            Ok(raw_event) => {
                                let processed_event =
                                    user_event_processor::process_user_created_event(&raw_event);

                                println!(
                                    "Processed event in Rust: {:?}",
                                    processed_event
                                );

                                match producer::publish_processed_user_created_event(
                                    &producer,
                                    &processed_event,
                                )
                                .await
                                {
                                    Ok(_) => {
                                        println!(
                                            "Published processed event to topic: user.created.processed"
                                        );
                                    }
                                    Err(error) => {
                                        println!(
                                            "Failed to publish processed event: {}",
                                            error
                                        );
                                    }
                                }
                            }
                            Err(error) => {
                                println!(
                                    "Failed to deserialize raw event payload: {}",
                                    error
                                );
                            }
                        }
                    }
                    Some(Err(error)) => {
                        println!("Failed to read message payload as UTF-8: {}", error);
                    }
                    None => {
                        println!("Received Kafka message with empty payload");
                    }
                }
            }
            Err(error) => {
                println!("Kafka consumer error: {}", error);
            }
        }
    }

    Ok(())
}