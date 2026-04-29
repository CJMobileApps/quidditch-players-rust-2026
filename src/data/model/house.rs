#[derive(Debug, serde::Serialize)]
pub struct House {
    pub name: HouseName,
    pub image_url: String,
    pub emoji: String,
}

#[derive(Debug, serde::Serialize)]
pub enum HouseName {
    Gryffindor,
    Slytherin,
    Ravenclaw,
    Hufflepuff,
}
