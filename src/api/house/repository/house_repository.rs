use crate::data::model::house::House;
use crate::data::mock_data::MockData;
use crate::util::error::QuidditchPlayersError;

pub trait HouseRepository: Send + Sync {
    fn get_all_houses(&self) -> Result<&[House], QuidditchPlayersError>;
}

pub struct HouseRepositoryImpl {}

impl HouseRepository for HouseRepositoryImpl {
    fn get_all_houses(&self) -> Result<&[House], QuidditchPlayersError> {
        let rand_number = rand::random_range(1..=10_u32);

        match rand_number {
            1..7 => {
                Ok(MockData::get_houses())
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
