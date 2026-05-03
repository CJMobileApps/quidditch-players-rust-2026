use std::sync::Arc;
use std::time::Duration;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use axum::routing::get;
use headers::{CacheControl, HeaderMapExt};
use crate::api::player::repository::player_repository::{PlayerRepository, PlayerRepositoryImpl};
use crate::api::player::service::player_service::{PlayerService, PlayerServiceImpl};
use crate::data::model::response_wrapper::ResponseWrapper;
use crate::util::constants::Constants;

pub fn router() -> Router {
    let repository:Arc<dyn PlayerRepository> = Arc::new(PlayerRepositoryImpl {});
    let service: Arc<dyn PlayerService> = Arc::new(PlayerServiceImpl::new(repository));

    Router::new()
        .route("/", get(get_player_handler))
        .with_state(service)
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_player_handler(State(service): State<Arc<dyn PlayerService>>) -> Response {
    match service.get_players() {
        Ok(players) => {
            let mut response = (
                StatusCode::OK,
                Json(ResponseWrapper {
                    data: Some(players),
                    error: None,
                    status_code: Constants::OK_CODE,
                }),
            ).into_response();

            response.headers_mut().typed_insert(
                CacheControl::new().with_max_age(Duration::from_secs(60))
            );

            response
        }
        Err(error) => error.into_response()
    }
}
