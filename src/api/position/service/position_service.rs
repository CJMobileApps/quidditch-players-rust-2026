use std::collections::HashMap;
use std::sync::Arc;
use crate::api::position::repository::position_repository::PositionRepository;
use crate::data::model::position::Position;
use crate::util::error::QuidditchPlayersError;

pub trait PositionService: Send + Sync {
    fn get_positions(&self) -> Result<&HashMap<i32, Position>, QuidditchPlayersError>;
}

pub struct PositionServiceImpl {
    position_repository: Arc<dyn PositionRepository>,
}

impl PositionServiceImpl {
    pub fn new(position_repository: Arc<dyn PositionRepository>) -> Self {
        Self { position_repository }
    }
}

impl PositionService for PositionServiceImpl {
    fn get_positions(&self) -> Result<&HashMap<i32, Position>, QuidditchPlayersError> {
        self.position_repository.get_positions()
    }
}
