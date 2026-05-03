use std::fmt;

#[derive(Debug, serde::Serialize)]
pub struct House {
    pub name: HouseName,
    pub image_url: String,
    pub emoji: String,
}

#[derive(Debug, serde::Serialize, Clone)]
pub enum HouseName {
    Gryffindor,
    Slytherin,
    Ravenclaw,
    Hufflepuff,
}

impl fmt::Display for HouseName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HouseName::Gryffindor => write!(f, "GRYFFINDOR"),
            HouseName::Slytherin => write!(f, "SLYTHERIN"),
            HouseName::Ravenclaw => write!(f, "RAVENCLAW"),
            HouseName::Hufflepuff => write!(f, "HUFFLEPUFF"),
        }
    }
}
