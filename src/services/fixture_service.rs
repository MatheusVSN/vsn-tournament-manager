
use sqlx::{MySql, Pool};

use crate::{
    constants::MYSQL_CUSTOM_ERROR,
    models::{
        fixture::{EditFixtureDTO, Fixture, FixtureDataDTO, FixtureObject},
        league::League,
        team::{Team, TeamInfoDTO},
    },
    responses::HTTPException,
};

use super::league_service::get_teams_from_league;

pub type FixturesList = Vec<Vec<FixtureObject>>;

/// Responsible to generate the league fixtures
///
/// # Arguments
/// * `&mut Vec<Team>` - A mutable reference of a vector of teams
///
/// # Returns
/// * `Vec<Vec<FixtureObject>>` - A vector containing all rounds, each rounds have a vector of games
fn berger_table(teams: &mut Vec<Team>) -> FixturesList {
    let dummy_team = Team {
        id: 0,
        name: String::new(),
        tournament_id: 0,
    };

    if teams.len() % 2 != 0 {
        teams.push(dummy_team);
    }

    let teams_quantity = teams.len();
    let number_of_rounds = teams_quantity - 1;
    let games_per_round = teams_quantity / 2;

    let mut column_a = (&teams[0..games_per_round]).to_vec();
    let mut column_b = (&teams[games_per_round..]).to_vec();

    let mut current_game_week = 1;
    let mut fixtures: Vec<Vec<FixtureObject>> = vec![];

    for _ in 0..number_of_rounds {
        let mut game_week: Vec<FixtureObject> = vec![];

        for index in 0..games_per_round {
            let home_team = &column_a[index];
            let away_team = &column_b[index];

            // Making sure both of these teams aren't "dummy"
            if home_team.id != 0 && away_team.id != 0 {
                game_week.push(FixtureObject {
                    home_team_id: home_team.id,
                    away_team_id: away_team.id,
                    home_score: 0,
                    away_score: 0,
                    played: false,
                    round: current_game_week,
                });
            }
        }
        fixtures.push(game_week);
        current_game_week += 1;

        // Rotating the teams
        let first_from_column_b = column_b.remove(0);
        column_a.insert(1, first_from_column_b);
        let last_element_from_column_a = column_a.pop().unwrap();
        column_b.push(last_element_from_column_a);
    }

    return fixtures;
}

impl Fixture {
    /// Responsible to generate the league fixtures
    ///
    /// # Arguments
    /// * `user_id` - The user id
    /// * `tournament_id` - The tournament id
    /// * `league_id` - The league id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(())` - If the fixtures were generated successfully
    ///
    /// # Errors
    /// * `HTTPException::BadRequest` - If there's not enough teams or the user doesn't have permission to generate the fixtures
    /// * `HTTPException::Internal` - If the database query fails
    pub async fn generate_league_fixtures(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let mut teams = get_teams_from_league(league_id, tournament_id, user_id, db_pool).await?;
        if teams.len() < 2 {
            return Err(HTTPException::BadRequest(String::from(
                "Not enough teams or permission to generate fixtures",
            )));
        }

        let existing_fixtures =
            League::get_league_fixtures(user_id, league_id, tournament_id, db_pool).await?;
        if existing_fixtures.len() > 0 {
            return Err(HTTPException::BadRequest(String::from(
                "Cannot generate a fixture as it may conflict with existing ones",
            )));
        }

        let generated_fixtures = berger_table(&mut teams);

        // Initializing the transaction
        // We'll use this because if one of the query fails we can cancel the whole operation
        let mut transaction = db_pool.begin().await.or_else(|_error| {
            Err(HTTPException::Internal(String::from(
                "Failed initializing the database transaction",
            )))
        })?;

        // Saving all the game rounds from each fixture/game week
        for game_week in generated_fixtures {
            for fixture in game_week {
                let _a = sqlx::query!(
                    "
                CALL generate_fixture(?, ?, ?, ?, ?)
                ",
                    fixture.home_team_id,
                    fixture.away_team_id,
                    league_id,
                    fixture.round,
                    user_id
                )
                .execute(&mut *transaction)
                .await.or_else(|exception| {
                    if let Some(database_error) = exception.as_database_error() {
                        if let Some(code) = database_error.code() {
                            if code == MYSQL_CUSTOM_ERROR {
                                let message=database_error.message();
                                return Err(HTTPException::BadRequest(message.to_string()));
                            }
                        }
                    }

                    Err(HTTPException::Internal(String::from("Failed while creating a new fixture. The operation have been cancelled and will not be saved")))
                })?;
            }
        }

        transaction.commit().await.or_else(|_error| {
            Err(HTTPException::Internal(String::from(
                "Something failed while saving all fixtures. Please try again later",
            )))
        })?;

        Ok(())
    }

    /// Responsible to get a league fixture
    ///
    /// # Arguments
    /// * `user_id` - The user id
    /// * `tournament_id` - The tournament id
    /// * `league_id` - The league id
    /// * `fixture_id` - The fixture id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(FixtureDataDTO)` - A `FixtureDataDTO` struct containing the fixture data
    ///
    /// # Errors
    /// * `HTTPException::BadRequest` - If the fixture doesn't exist or the user doesn't have permission to access it
    /// * `HTTPException::Internal` - If the database query fails
    pub async fn get_league_fixture(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        fixture_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<FixtureDataDTO, HTTPException> {
        let query = sqlx::query!(
            "
    SELECT FixtureRow.*, HomeTeamRow.name as home_team_name, AwayTeamRow.name as away_team_name
	FROM `fixtures` as FixtureRow
	INNER JOIN `leagues` as LeagueRow
		ON LeagueRow.id = FixtureRow.league_id
	INNER JOIN `tournaments` as TournamentRow
		ON TournamentRow.id = LeagueRow.tournament_id
	INNER JOIN `teams` as HomeTeamRow
		ON HomeTeamRow.id = FixtureRow.home_team_id
	INNER JOIN `teams` as AwayTeamRow
		ON AwayTeamRow.id = FixtureRow.away_team_id
	WHERE FixtureRow.id = (?) AND FixtureRow.league_id = (?) AND TournamentRow.id = (?) AND
	(
		(TournamentRow.public = FALSE AND TournamentRow.user_id = (?))
		OR
		(TournamentRow.public = TRUE)
	)",
            fixture_id,
            league_id,
            tournament_id,
            user_id
        )
        .fetch_optional(db_pool)
        .await
        .or_else(|_error| {
            Err(HTTPException::Internal(String::from(
                "Failed to fetch the league fixture",
            )))
        })?;

        let fixture = match query {
            None => {
                return Err(HTTPException::BadRequest(String::from("Failed to get the fixture. Please make sure it exists and you have permission to access it")));
            }
            Some(data) => data,
        };

        Ok(FixtureDataDTO {
            id: fixture.id,
            home_team: TeamInfoDTO {
                id: fixture.home_team_id,
                name: fixture.home_team_name,
            },
            away_team: TeamInfoDTO {
                id: fixture.away_team_id,
                name: fixture.away_team_name,
            },
            home_score: fixture.home_score,
            away_score: fixture.away_score,
            played: fixture.played == 1,
            round: fixture.round,
        })
    }

    /// Responsible to delete all fixtures from a league
    ///
    /// # Arguments
    /// * `user_id` - The user id
    /// * `tournament_id` - The tournament id
    /// * `league_id` - The league id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(())` - If the fixtures were deleted successfully
    ///
    /// # Errors
    /// * `HTTPException::BadRequest` - If the fixtures couldn't be deleted or the user doesn't have permission to delete them
    /// * `HTTPException::Internal` - If the database query fails
    pub async fn delete_all_fixtures_from_league(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let query = sqlx::query!(
            "
        DELETE FixtureRow.* FROM `fixtures` as FixtureRow
        INNER JOIN `leagues` as LeagueRow
	        ON LeagueRow.id = FixtureRow.league_id
        INNER JOIN `tournaments` as TournamentRow
	        ON LeagueRow.tournament_id = TournamentRow.id
        WHERE TournamentRow.id = (?) AND LeagueRow.id = (?) AND TournamentRow.user_id = (?)
        ",
            tournament_id,
            league_id,
            user_id
        )
        .execute(db_pool)
        .await
        .or_else(|_exception| {
            Err(HTTPException::Internal(String::from(
                "Failed to reset the fixtures",
            )))
        })?;

        if query.rows_affected() < 1 {
            return Err(HTTPException::BadRequest(String::from(
                "Failed to reset the fixtures. Please make sure the tournament/league exists and you have permission to access it",
            )));
        }

        Ok(())
    }

    /// Responsible to edit a fixture
    /// 
    /// # Arguments
    /// * `user_id` - The user id
    /// * `tournament_id` - The tournament id
    /// * `league_id` - The league id
    /// * `fixture_id` - The fixture id
    /// * `edit_data` - A `EditFixtureDTO` struct containing the data to be edited
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    /// 
    /// # Returns
    /// * `Ok(())` - If the fixture was edited successfully
    /// 
    /// # Errors
    /// * `HTTPException::BadRequest` - If the fixture couldn't be edited or the user doesn't have permission to edit it
    /// * `HTTPException::Internal` - If the database query fails
    pub async fn edit_fixture_by_id(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        fixture_id: u64,
        edit_data: EditFixtureDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let query= sqlx::query!("
        UPDATE `fixtures` as FixtureRow

        INNER JOIN `leagues` as LeagueRow
            ON LeagueRow.id = FixtureRow.league_id
        INNER JOIN `tournaments` as TournamentRow
            ON TournamentRow.id = LeagueRow.tournament_id
            
        SET FixtureRow.home_score = (?), FixtureRow.away_score = (?), FixtureRow.played = (?)
        WHERE FixtureRow.id = (?) AND LeagueRow.id = (?) AND TournamentRow.id = (?) AND TournamentRow.user_id = (?)
        ", edit_data.home_score, edit_data.away_score, edit_data.played, fixture_id, league_id, tournament_id, user_id)
        .execute(db_pool)
        .await
        .or_else(|_exception| {
            Err(HTTPException::Internal(String::from(
                "Failed to reset the fixtures",
            )))
        })?;

        if query.rows_affected() < 1 {
            return Err(HTTPException::BadRequest(String::from(
                "Failed to edit the fixture. Please make sure the tournament/league exists and you have permission to access it",
            )));
        }

        Ok(())
    }
}
