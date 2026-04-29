use axum::http::StatusCode;

pub struct Constants;

impl Constants {
    pub const INTERNAL_SERVER_ERROR: &'static str = "Internal Server Error";
    pub const YOU_SENT_A_BAD_REQUEST: &'static str = "You sent a bad bad request";

    pub const OK_CODE: i32 = StatusCode::OK.as_u16() as i32;
    pub const INTERNAL_SERVER_ERROR_CODE: i32 = StatusCode::INTERNAL_SERVER_ERROR.as_u16() as i32;
    pub const BAD_REQUEST_CODE: i32 = StatusCode::BAD_REQUEST.as_u16() as i32;
}
