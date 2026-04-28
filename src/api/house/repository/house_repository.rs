use crate::data::model::house::{House, HouseName};
use crate::util::error::QuidditchPlayersError;

pub trait HouseRepository: Send + Sync {
    fn get_all_houses(&self) -> Result<Vec<House>, QuidditchPlayersError>;
}

pub struct HouseRepositoryImpl {}

impl HouseRepository for HouseRepositoryImpl {
    fn get_all_houses(&self) -> Result<Vec<House>, QuidditchPlayersError> {
        let rand_number = rand::random_range(1..=10_u32);

        match rand_number {
            1..7 => {
                let house = House {
                    name: HouseName::Gryffindor,
                    image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
                    emoji: "\u{1F981}".to_string(),
                };

                Ok(vec![house])
            }
            9 => {
                Err(QuidditchPlayersError::YouSentABadRequest(None))
            }
            _ => {
                Err(QuidditchPlayersError::InternalServerError(None))
            }
        }
    }
}

// interface HouseService {
// @Throws(ClientException::class, InternalException::class)
// suspend fun getAllHouses(): List<House>
// }