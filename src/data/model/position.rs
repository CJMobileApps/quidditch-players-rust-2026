#[derive(Debug, serde::Serialize)]
pub struct Position {
    #[serde(rename = "positionName")]
    pub position_name: String,
}
