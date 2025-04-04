use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RequestError {
    #[error("Request not found")]
    RequestNotFound,
    #[error("Request invalid")]
    RequestInvalid,
    #[error("Request unauthorized")]
    RequestUnauthorized,
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        let status_code = match self {
            RequestError::RequestNotFound => StatusCode::NOT_FOUND,
            RequestError::RequestInvalid => StatusCode::BAD_REQUEST,
            RequestError::RequestUnauthorized => StatusCode::UNAUTHORIZED,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
