use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Tournament {
    pub id: u64,
    pub user_id: u64,
    pub name: String,
    pub public: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TournamentRegisterDTO {
    pub name: String,
    pub public: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TournamentEditDTO {
    pub name: Option<String>,
    pub public: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueInformationData {
    pub id: u64,
    pub name: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamInformationData {
    pub id: u64,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TournamentInformationData {
    pub id: u64,
    pub name: String,
    pub leagues: Vec<LeagueInformationData>,
    pub teams: Vec<TeamInformationData>,
}
