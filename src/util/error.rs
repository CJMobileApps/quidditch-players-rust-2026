use crate::data::model::response_wrapper::{ResponseError, ResponseWrapper, ResponseWrapperTrait};
use crate::util::constants::Constants;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::fmt;

#[derive(Debug)]
pub enum QuidditchPlayersError {
    InternalError(Option<String>),
    ClientError(Option<String>),
}

impl fmt::Display for QuidditchPlayersError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QuidditchPlayersError::InternalError(msg) => write!(
                f,
                "{}",
                msg.as_deref().unwrap_or(Constants::INTERNAL_SERVER_ERROR)
            ),
            QuidditchPlayersError::ClientError(msg) => write!(
                f,
                "{}",
                msg.as_deref().unwrap_or(Constants::YOU_SENT_A_BAD_REQUEST)
            ),
        }
    }
}

impl IntoResponse for QuidditchPlayersError {
    fn into_response(self) -> Response {
        let (status) = match &self {
            QuidditchPlayersError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            QuidditchPlayersError::ClientError(_) => StatusCode::BAD_REQUEST,
        };

        let response_wrapper = match self {
            QuidditchPlayersError::InternalError(_) => self.to_response_wrapper(),
            QuidditchPlayersError::ClientError(_) => self.to_response_wrapper(),
        };

        let body = Json(response_wrapper);
        (status, body).into_response()
    }
}

impl ResponseWrapperTrait for QuidditchPlayersError {
    fn to_response_wrapper(&self) -> ResponseWrapper<()> {
        match self {
            QuidditchPlayersError::InternalError(msg) => ResponseWrapper {
                data: None,
                error: Some(ResponseError {
                    is_error: true,
                    message: Some(self.to_string()),
                }),
                status_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i32,
            },
            QuidditchPlayersError::ClientError(msg) => ResponseWrapper {
                data: None,
                error: Some(ResponseError {
                    is_error: true,
                    message: Some(self.to_string()),
                }),
                status_code: StatusCode::BAD_REQUEST.as_u16() as i32,
            },
        }
    }
}
