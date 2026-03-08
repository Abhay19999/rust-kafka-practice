use axum::{Router, routing::{delete, get, post}};

use crate::{
    handlers::{health_handler, user_handler}, models::user, state::app_state::AppState
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_handler::health))
        .route("/users/{id}", get(user_handler::get_user))
        .route("/users", post(user_handler::create_user))
        .route("/all/users", get(user_handler::get_all_users))
        .route("/user/{id}", delete(user_handler::delete_user))
        .with_state(state)
}



