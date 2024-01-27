use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::team::TeamInfoDTO;

#[derive(Deserialize, Serialize, Debug)]
pub struct Fixture {
    pub id: u64,
    pub home_team_id: u64,
    pub away_team_id: u64,
    pub league_id: u64,
    pub playing_data: Option<DateTime<Utc>>,
    pub home_score: u8,
    pub away_score: u8,
    pub played: bool,
    pub round: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FixtureObject {
    pub home_team_id: u64,
    pub away_team_id: u64,
    pub home_score: u8,
    pub away_score: u8,
    pub played: bool,
    pub round: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FixtureDataDTO {
    pub id: u64,
    pub home_team: TeamInfoDTO,
    pub away_team: TeamInfoDTO,
    pub home_score: u8,
    pub away_score: u8,
    pub played: bool,
    pub round: u16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct EditFixtureDTO {
    pub home_score: u8,
    pub away_score: u8,
    pub played: bool,
}
