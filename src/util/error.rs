use std::fmt;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use serde_json::json;
use crate::util::constants::Constants;

#[derive(Debug)]
pub enum QuidditchPlayersError {
    InternalServerError(Option<String>),
    YouSentABadRequest(Option<String>),
}

impl fmt::Display for QuidditchPlayersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuidditchPlayersError::InternalServerError(msg) => write!(
                f, "{}",
                msg.as_deref().unwrap_or(Constants::INTERNAL_SERVER_ERROR)
            ),
            QuidditchPlayersError::YouSentABadRequest(msg) => write!(
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
            QuidditchPlayersError::InternalServerError(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            QuidditchPlayersError::YouSentABadRequest(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        };

        println!("HERE_ did we get here status {:?}", status);
        let body = Json(json!({"error": message}));
        (status, body).into_response()
    }
}
