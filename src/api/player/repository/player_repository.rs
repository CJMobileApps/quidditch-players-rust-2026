use crate::data::mock_data::MockData;
use crate::data::model::player::Player;
use crate::util::error::QuidditchPlayersError;

pub trait PlayerRepository: Send + Sync {
    fn get_players(&self, house_name: Option<String>) -> Result<Vec<Player>, QuidditchPlayersError>;
}

pub struct PlayerRepositoryImpl {}

impl PlayerRepository for PlayerRepositoryImpl {
    fn get_players(&self, house_name: Option<String>) -> Result<Vec<Player>, QuidditchPlayersError> {
        let rand_number = rand::random_range(1..=10_u32);

        match rand_number {
            1..7 => {
                match house_name {
                    Some(house_name) => {
                        let quidditch_team = MockData::get_all_quidditch_teams()
                            .iter()
                            .filter(|player| player.house_name.to_string() == house_name)
                            .cloned()
                            .collect::<Vec<Player>>();
                        Ok(quidditch_team)
                    }
                    None => {
                        Ok(MockData::get_all_quidditch_teams().to_vec())
                    }
                }
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
