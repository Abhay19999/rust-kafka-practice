use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::models::user::User;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<u64, User>>>,
    pub next_id: Arc<Mutex<u64>>,
}