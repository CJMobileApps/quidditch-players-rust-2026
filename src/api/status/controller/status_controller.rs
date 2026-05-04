use crate::api::status::repository::status_repository::{StatusRepository, StatusRepositoryImpl};
use axum::{Json, Router};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use std::time::Duration;
use axum::extract::{Query, State};
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use headers::{CacheControl, HeaderMapExt};
use serde::Deserialize;
use crate::api::status::service::status_service::{StatusService, StatusServiceImpl};
use crate::data::model::response_wrapper::ResponseWrapper;
use crate::util::constants::Constants;

pub fn router() -> Router {
    let repository: Arc<dyn StatusRepository> = Arc::new(StatusRepositoryImpl {});
    let service:Arc<dyn StatusService> = Arc::new(StatusServiceImpl::new(repository));

    Router::new()
        .route("/", get(get_status_by_house_name_handler))
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
            ).into_response();

            response.headers_mut().typed_insert(
                CacheControl::new().with_max_age(Duration::from_secs(1))
            );

            let mut hasher = DefaultHasher::new();
            status.hash(&mut hasher);
            let etag = format!("{:x}", hasher.finish());

            response.headers_mut().insert(
                header::ETAG,
                etag.parse().unwrap(),
            );

            response
        }
        Err(error) => error.into_response(),
    }
}
