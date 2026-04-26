use axum::{Json, Router};
use axum::routing::get;

pub fn router() -> Router {
    Router::new()
        .route("/", get(house_handler))
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn house_handler() -> Json<String> {
    println!("Caller retrieved a vehicle from axum");
    Json::from(String::from("This is a house"))
}
