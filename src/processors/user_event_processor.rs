use crate::events::{processed_user_created_event::ProcessedUserCreatedEvent,
     raw_user_created_event::RawUserCreatedEvent};


pub fn process_user_created_event(raw_event: RawUserCreatedEvent) -> ProcessedUserCreatedEvent {

    let normalized_email = raw_event.email.trim().to_lowercase();
    let normalized_name = raw_event.name.trim().to_string();

    
    // Simulate some processing logic
    ProcessedUserCreatedEvent {
        user_id: raw_event.user_id,
        event_id: raw_event.event_id,
        name: normalized_name,
        email: normalized_email,
        source: raw_event.source,
        processed_by: "Rust User Event Processor".to_string(),
        status: "processed".to_string(),
    }
}