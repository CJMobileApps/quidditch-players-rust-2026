use std::fmt;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::util::constants::Constants;

#[derive(Debug)]
pub enum QuidditchPlayersError {
    InternalError(Option<String>),
    ClientError(Option<String>),
}

impl fmt::Display for QuidditchPlayersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuidditchPlayersError::InternalError(msg) => write!(
                f, "{}",
                msg.as_deref().unwrap_or(Constants::INTERNAL_SERVER_ERROR)
            ),
            QuidditchPlayersError::ClientError(msg) => write!(
                f, "{}",
                msg.as_deref().unwrap_or(Constants::YOU_SENT_A_BAD_REQUEST)
            ),
        }
    }
}

//TODO might move this to own util
impl IntoResponse for QuidditchPlayersError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            QuidditchPlayersError::InternalError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            QuidditchPlayersError::ClientError(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        let body = Json(json!({"error": message}));
        (status, body).into_response()
    }
}
