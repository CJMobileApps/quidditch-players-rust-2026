use crate::api::status::repository::status_repository::{StatusRepository, StatusRepositoryImpl};
use crate::api::status::service::status_service::{StatusService, StatusServiceImpl};
use crate::data::model::response_wrapper::ResponseWrapper;
use crate::util::constants::Constants;
use axum::extract::{Path, Query, State};
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use headers::{CacheControl, HeaderMapExt};
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub fn router() -> Router {
    let repository: Arc<dyn StatusRepository> = Arc::new(StatusRepositoryImpl {});
    let service: Arc<dyn StatusService> = Arc::new(StatusServiceImpl::new(repository));

    Router::new()
        .route("/", get(get_status_by_house_name_handler))
        .route("/{player_id}", get(get_status_by_player_id))
        .with_state(service)
}

#[derive(Deserialize)]
pub struct StatusQuery {
    #[serde(rename = "houseName")]
    pub house_name: Option<String>,
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_status_by_house_name_handler(
    State(service): State<Arc<dyn StatusService>>,
    Query(StatusQuery { house_name }): Query<StatusQuery>,
) -> Response {
    match service.get_status_by_house_name(house_name) {
        Ok(status) => {
            let mut response = (
                StatusCode::OK,
                Json(ResponseWrapper {
                    data: Some(&status),
                    error: None,
                    status_code: Constants::OK_CODE,
                }),
            )
                .into_response();

            response
                .headers_mut()
                .typed_insert(CacheControl::new().with_max_age(Duration::from_secs(1)));

            let mut hasher = DefaultHasher::new();
            status.hash(&mut hasher);
            let etag = format!("{:x}", hasher.finish());

            response
                .headers_mut()
                .insert(header::ETAG, etag.parse().unwrap());

            response
        }
        Err(error) => error.into_response(),
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_status_by_player_id(
    State(service): State<Arc<dyn StatusService>>,
    Path(player_id): Path<Uuid>,
) -> Response {
    match service.get_status_by_player_id(player_id) {
        Ok(status) => {
            let mut response = (
                StatusCode::OK,
                Json(ResponseWrapper {
                    data: Some(&status),
                    error: None,
                    status_code: Constants::OK_CODE,
                }),
            )
                .into_response();

            response
                .headers_mut()
                .typed_insert(CacheControl::new().with_max_age(Duration::from_secs(1)));

            let mut hasher = DefaultHasher::new();
            status.hash(&mut hasher);
            let etag = format!("{:x}", hasher.finish());

            response
                .headers_mut()
                .insert(header::ETAG, etag.parse().unwrap());

            response
        }
        Err(error) => error.into_response(),
    }
}
