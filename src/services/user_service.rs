use std::fmt::format;

use crate::{
    errors::api_error::ApiError,
    models::{user::{self, User}, user_request::CreateUserRequest},
    state::app_state::AppState,
};

pub fn create_user(state: &AppState, request: CreateUserRequest) -> Result<User, ApiError> {
    if request.name.trim().is_empty() {
        return Err(ApiError::BadRequest(String::from("Name must not be empty")));
    }

    if request.email.trim().is_empty() {
        return Err(ApiError::BadRequest(String::from("Email must not be empty")));
    }

    if !request.email.contains('@') {
        return Err(ApiError::BadRequest(String::from("Email format is invalid")));
    }

    let mut next_id = state.next_id.lock().unwrap();
    let id = *next_id;
    *next_id += 1;

    let user = User {
        id,
        name: request.name,
        email: request.email,
    };

    let mut users = state.users.lock().unwrap();
    users.insert(id, user.clone());

    Ok(user)
}

pub fn get_user_by_id(state: &AppState, id: u64) -> Result<User, ApiError> {
    let users = state.users.lock().unwrap();

    match users.get(&id).cloned() {
        Some(user) => Ok(user),
        None => Err(ApiError::NotFound(format!("User with id {} not found", id))),
    }
}

pub fn get_all_users(state: &AppState) -> Vec<User> {
    let users = state.users.lock().unwrap();
    users.values().cloned().collect()
}
pub fn delete_user(state: &AppState, id: u64)-> Result<(), ApiError>{
    let mut users  = state.users.lock().unwrap();
    match users.remove(&id) {
        Some(_) => Ok(()),
        None => Err(ApiError::NotFound(format!("User with this id {} not found", id))),
    }
}