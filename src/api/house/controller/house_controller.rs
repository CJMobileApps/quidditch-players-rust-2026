use axum::{Json, Router};
use axum::routing::get;
use data::model::house::House;
use data::model::house::HouseName;
use crate::data;

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_house_handler))
}

#[cfg_attr(debug_assertions, axum::debug_handler)]
pub async fn get_house_handler() -> Json<House> {
    let house = House {
        name: HouseName::Gryffindor,
        image_url : "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
        emoji : "\u{1F981}".to_string()
    };
    Json::from(house)
}
