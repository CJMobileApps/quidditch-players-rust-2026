use rand::seq::IndexedRandom;
use crate::data::mock_data::MockData;
use crate::data::model::player::Player;
use crate::data::model::status::Status;
use crate::util::error::QuidditchPlayersError;

pub trait StatusRepository: Send + Sync {
    fn get_status_by_house_name(&self, house_name: Option<String>) -> Result<Status, QuidditchPlayersError>;
}

pub struct StatusRepositoryImpl {}

impl StatusRepository for StatusRepositoryImpl {
    fn get_status_by_house_name(&self, house_name: Option<String>) -> Result<Status, QuidditchPlayersError> {
        let rand_number = rand::random_range(1..=10_u32);

        match rand_number {
            1..7 => {
                let players = match house_name {
                    Some(house_name) => {
                        let quidditch_team = MockData::get_all_quidditch_teams()
                            .iter()
                            .filter(|player| player.house_name.to_string() == house_name)
                            .cloned()
                            .collect::<Vec<Player>>();
                        quidditch_team
                    }
                    None => {
                        MockData::get_all_quidditch_teams().to_vec()
                    }
                };

                match players.choose(&mut rand::rng()) {
                    Some(player) => {
                        let statuses = MockData::get_statuses(
                            &format!("{} {}", player.first_name, player.last_name),
                            &player.favorite_subject,
                        );
                        let status= statuses.choose(&mut rand::rng()).cloned().unwrap_or_default();

                        Ok(Status {
                            player_id: player.id,
                            status,
                        })
                    }
                    None => Err(QuidditchPlayersError::ClientError(None)),
                }
            }
            9 => Err(QuidditchPlayersError::ClientError(None)),
            _ => Err(QuidditchPlayersError::InternalError(None)),
        }
    }
}
