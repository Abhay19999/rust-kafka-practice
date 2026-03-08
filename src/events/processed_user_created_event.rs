use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessedUserCreatedEvent {
    pub user_id: u64,
    pub event_id: String,
    pub name: String,
    pub email: String,
    pub source: String,
    pub processed_by: String,
    pub status: String,
}