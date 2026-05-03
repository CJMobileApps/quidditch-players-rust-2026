use std::sync::Arc;
use crate::api::player::repository::player_repository::PlayerRepository;
use crate::data::model::player::Player;
use crate::util::error::QuidditchPlayersError;

pub trait PlayerService: Send + Sync {
    fn get_players(&self, house_name: Option<String>) -> Result<Vec<Player>, QuidditchPlayersError>;
}

pub struct PlayerServiceImpl {
    player_repository: Arc<dyn PlayerRepository>,
}

impl PlayerServiceImpl {
    pub fn new(player_repository: Arc<dyn PlayerRepository>) -> Self {
        Self { player_repository }
    }
}

impl PlayerService for PlayerServiceImpl {
    fn get_players(&self, house_name: Option<String>) -> Result<Vec<Player>, QuidditchPlayersError> {
        self.player_repository.get_players(house_name)
    }
}
