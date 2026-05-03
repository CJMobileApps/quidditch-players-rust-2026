use uuid::Uuid;
use crate::data::model::house::HouseName;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Player {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub years_played: Vec<i32>,
    pub favorite_subject: String,
    pub position: i32,
    pub image_url: String,
    pub house_name: HouseName
}
