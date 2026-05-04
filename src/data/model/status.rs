use uuid::Uuid;

#[derive(Debug, serde::Serialize, Hash)]
pub struct Status {
    #[serde(rename = "playerId")]
    pub player_id: Uuid,
    pub status: String,
}
