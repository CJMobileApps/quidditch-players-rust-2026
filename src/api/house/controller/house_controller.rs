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

// #[cfg_attr(debug_assertions, axum::debug_handler)]
// pub async fn get_house_handler() -> Json<House> {
//     let house = House {
//         name: HouseName::Gryffindor,
//         image_url : "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
//         emoji : "\u{1F981}".to_string()
//     };
//     Json::from(house)
// }

pub enum AppError {
    NotFound(String),
    BadRequest(String),
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg)      => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg)    => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(format!(r#"{{"error": "{}"}}"#, message));
        (status, body).into_response()
    }
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_house_handler(
    State(service): State<Arc<dyn HouseService>>,
) -> Response {
    
    match service.get_all_houses() {
        Ok(houses) => {
            (StatusCode::OK, Json::from(houses)).into_response()

        }
        Err(error) => {
            match error {
                AppError::NotFound(e) => {
                    e.to_string().into_response()
                }
                AppError::BadRequest(e) => {
                    e.to_string().into_response()
                } 
                AppError::InternalError(e) => {
                    e.to_string().into_response()
                }
            }
        }
    }
    
    // let n = rand::random_range(0..=1_u32);
    // 
    // 
    // if n == 0 {
    //     AppError::BadRequest("Some bad request".to_string()).into_response()
    // } else {
    //     let house = House {
    //         name: HouseName::Gryffindor,
    //         image_url : "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
    //         emoji : "\u{1F981}".to_string()
    //     };
    //     
    //     (StatusCode::OK, Json::from(service.get_all_houses())).into_response()
    // }
}

//TODO https://claude.ai/chat/83dd58f3-0e6d-4d9b-9f8e-457195d2aecc
//TODO https://claude.ai/chat/577b1133-fe3d-4af8-aa59-d5d200c19d15
