use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Player {
    pub id: Uuid,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "yearsPlayed")]
    pub years_played: Vec<i32>,
    #[serde(rename = "favoriteSubject")]
    pub favorite_subject: String,
    pub position: i32,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "house")]
    pub house_name: String
}
