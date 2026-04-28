use crate::api::house::controller::house_controller::AppError;
use crate::data::model::house::{House, HouseName};

pub trait HouseRepository: Send + Sync {
    fn get_all_houses(&self) -> Result<Vec<House>, AppError>;
}

pub struct HouseRepositoryImpl {}

impl HouseRepository for HouseRepositoryImpl {
    fn get_all_houses(&self) -> Result<Vec<House>, AppError> {
        let n = rand::random_range(0..=1_u32);

        if n == 0 {
            Err(AppError::BadRequest("Some bad request".to_string()))
        } else {
            let house = House {
                name: HouseName::Gryffindor,
                image_url: "https://static.wikia.nocookie.net/harrypotter/images/9/98/Gryffindor.jpg/revision/latest".to_string(),
                emoji: "\u{1F981}".to_string(),
            };

            Ok(vec![house])
        }
    }
}

// interface HouseService {
// @Throws(ClientException::class, InternalException::class)
// suspend fun getAllHouses(): List<House>
// }