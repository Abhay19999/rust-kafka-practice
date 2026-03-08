use serde ::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RawUserCreatedEvent {
    pub user_id: u64,
    pub event_id: String,
    pub name: String,
    pub email: String,
    pub source: String,
}