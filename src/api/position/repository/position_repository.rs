use std::collections::HashMap;
use crate::data::mock_data::MockData;
use crate::data::model::position::Position;
use crate::util::error::QuidditchPlayersError;

pub trait PositionRepository: Send + Sync {
    fn get_positions(&self) -> Result<&HashMap<i32, Position>, QuidditchPlayersError>;
}

pub struct PositionRepositoryImpl {}

impl PositionRepository for PositionRepositoryImpl {
    fn get_positions(&self) -> Result<&HashMap<i32, Position>, QuidditchPlayersError> {
        let rand_number = rand::random_range(1..=10_u32);

        match rand_number {
            1..7 => {
                Ok(MockData::get_positions())
            }
            9 => {
                Err(QuidditchPlayersError::ClientError(None))
            }
            _ => {
                Err(QuidditchPlayersError::InternalError(None))
            }
        }
    }
}
