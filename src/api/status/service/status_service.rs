use std::sync::Arc;
use uuid::Uuid;
use crate::api::status::repository::status_repository::{StatusRepository, StatusRepositoryImpl};
use crate::data::model::status::Status;
use crate::util::error::QuidditchPlayersError;

pub trait StatusService: Send + Sync {
    fn get_status_by_house_name(&self, house_name: Option<String>) -> Result<Status, QuidditchPlayersError>;

    fn get_status_by_player_id(&self, player_id: Uuid) -> Result<Status, QuidditchPlayersError>;
}

pub struct StatusServiceImpl {
    status_repository: Arc<dyn StatusRepository>,
}

impl StatusServiceImpl {
    pub fn new(status_repository: Arc<dyn StatusRepository>) -> Self {
        Self { status_repository }
    }
}

impl StatusService for StatusServiceImpl {
    fn get_status_by_house_name(&self, house_name: Option<String>) -> Result<Status, QuidditchPlayersError> {
        self.status_repository.get_status_by_house_name(house_name)
    }

    fn get_status_by_player_id(&self, player_id: Uuid) -> Result<Status, QuidditchPlayersError> {
        self.status_repository.get_status_by_player_id(player_id)
    }
}
