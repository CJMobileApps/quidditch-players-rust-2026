use std::sync::Arc;
use std::time::Duration;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;
use headers::{CacheControl, HeaderMapExt};
use crate::api::position::repository::position_repository::{PositionRepository, PositionRepositoryImpl};
use crate::api::position::service::position_service::{PositionService, PositionServiceImpl};
use crate::data::model::response_wrapper::ResponseWrapper;
use crate::util::constants::Constants;

pub fn router() -> Router {
    let repository: Arc<dyn PositionRepository> = Arc::new(PositionRepositoryImpl {});
    let service: Arc<dyn PositionService> = Arc::new(PositionServiceImpl::new(repository));
    
    Router::new()
        .route("/", get(get_position_handler))
        .with_state(service)
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_position_handler(State(service): State<Arc<dyn PositionService>>) -> Response {
    match service.get_positions() {
        Ok(houses) => {
            let mut response = (
                StatusCode::OK,
                Json(ResponseWrapper {
                    data: Some(houses),
                    error: None,
                    status_code: Constants::OK_CODE,
                }),
            ).into_response();

            response.headers_mut().typed_insert(
                CacheControl::new().with_max_age(Duration::from_secs(60))
            );

            response
        }
        Err(error) => error.into_response(),
    }
} 
