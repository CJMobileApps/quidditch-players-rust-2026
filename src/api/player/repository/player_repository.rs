use crate::data::mock_data::MockData;
use crate::data::model::player::Player;
use crate::util::error::QuidditchPlayersError;

pub trait PlayerRepository: Send + Sync {
    fn get_players(&self) -> Result<&[Player], QuidditchPlayersError>;
}

pub struct PlayerRepositoryImpl {}

impl PlayerRepository for PlayerRepositoryImpl {
    fn get_players(&self) -> Result<&[Player], QuidditchPlayersError> {
        let rand_number = rand::random_range(1..=10_u32);

        match rand_number {
            1..7 => {
                Ok(MockData::get_all_quidditch_teams())
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
