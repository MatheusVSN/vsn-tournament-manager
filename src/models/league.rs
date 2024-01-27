use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct League {
    pub id: u64,
    pub tournament_id: u64,
    pub name: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueRegisterDTO {
    pub name: String,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueInformationDTO {
    pub id: u64,
    pub name: String,
    pub tournament_id: u64,
    pub completed: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TeamStandingTable {
    pub team_id: u64,
    pub team_name: String,
    pub total_points: u8,
    pub win: u8,
    pub draw: u8,
    pub loss: u8,
    pub goals_scored: u8,
    pub goals_against: u8,
    pub goal_difference: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LeagueStandingsTable {
    pub standings_table: Vec<TeamStandingTable>
}