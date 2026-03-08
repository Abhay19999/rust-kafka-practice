use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json
};

use crate :: models::error_response::ErrorResponse;


pub enum ApiError{
    BadRequest(String),
    NotFound(String),
}

impl IntoResponse for ApiError{
    fn into_response(self) -> Response {
        match self {
                ApiError::BadRequest(message) => {
                let body = Json(ErrorResponse { message });
                (StatusCode::BAD_REQUEST, body).into_response()
            }
                ApiError::NotFound(message) => {
                let body = Json(ErrorResponse { message });
                (StatusCode::NOT_FOUND, body).into_response()
            }
        }
    }
}