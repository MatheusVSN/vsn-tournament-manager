use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Team {
    pub id: u64,
    pub name: String,
    pub tournament_id: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamRegisterDTO {
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamInfoDTO {
    pub id: u64,
    pub name: String,
}
