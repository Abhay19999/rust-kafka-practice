use axum::{
    extract::{Path, State},
    Json,
    http::StatusCode,
};

use crate::{
    errors::api_error::ApiError,
    models::{user::User, user_request::CreateUserRequest},
    services::user_service,
    state::{self, app_state::AppState},
};

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, ApiError> {
    let user = user_service::get_user_by_id(&state, id)?;
    Ok(Json(user))
}

pub async fn get_all_users(State(state): State<AppState>,) -> Result<Json<Vec<User>>, ApiError>{
    let users: Vec<User> = user_service::get_all_users(&state);
    Ok(Json((users)))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<Json<User>, ApiError> {
    let user = user_service::create_user(&state, request)?;
    Ok(Json(user))
}

pub async fn delete_user (State(state): State<AppState>, Path(id): Path<u64>,) 
    -> Result<StatusCode, ApiError>{
        user_service::delete_user(&state, id)?;
        Ok(StatusCode::NO_CONTENT)
    }