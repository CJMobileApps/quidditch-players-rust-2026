use std::sync::Arc;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use axum::extract::State;
use crate::api::house::repository::house_repository::{HouseRepository, HouseRepositoryImpl};
use crate::api::house::service::house_service::{HouseService, HouseServiceImpl};

pub fn router() -> Router {
    let repository: Arc<dyn HouseRepository> = Arc::new(HouseRepositoryImpl {});
    let service: Arc<dyn HouseService> = Arc::new(HouseServiceImpl::new(repository));

    Router::new()
        .route("/", get(get_house_handler))
        .with_state(service)
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_house_handler(
    State(service): State<Arc<dyn HouseService>>,
) -> Response {
    
    match service.get_all_houses() {
        Ok(houses) => {
            (StatusCode::OK, Json::from(houses)).into_response()

        }
        Err(error) => error.into_response()
    }
}

//TODO https://claude.ai/chat/83dd58f3-0e6d-4d9b-9f8e-457195d2aecc
//TODO https://claude.ai/chat/577b1133-fe3d-4af8-aa59-d5d200c19d15
