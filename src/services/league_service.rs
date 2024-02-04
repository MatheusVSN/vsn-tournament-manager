use sqlx::{MySql, Pool};

use crate::{
    models::{
        fixture::FixtureDataDTO,
        league::{League, LeagueInformationDTO, LeagueRegisterDTO, TeamStandingTable},
        team::{Team, TeamInfoDTO},
    },
    responses::HTTPException,
};

impl League {
    /// Responsible to create a new league for the specified tournament_id
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the id of the user
    /// * `tournament_id` - `u64` integer which represents the id of the tournament which the league will be created
    /// * `league_data` - `LeagueRegisterDTO` struct that represents the new league information data
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Return
    /// * `Ok(u64)` - `u64` integer which represents the id of the new league
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest` -
    /// * * If the tournament does not exist
    /// * * If the user does not own the tournament
    /// * * The name of the league is too long
    pub async fn create_league(
        user_id: u64,
        tournament_id: u64,
        league_data: LeagueRegisterDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<u64, HTTPException> {
        if league_data.name.len() > 20 {
            return Err(HTTPException::BadRequest(String::from(
                "The name of the league is too long",
            )));
        }

        let query = sqlx::query!(
            "
        INSERT INTO `leagues` (`name`, `completed`, `tournament_id`)
        SELECT (?), (?), (?)
        FROM `tournaments` as TournamentRow
        WHERE TournamentRow.id = (?) AND TournamentRow.user_id = (?)
        ",
            league_data.name,
            league_data.completed,
            tournament_id,
            tournament_id,
            user_id
        )
        .execute(db_pool)
        .await;

        let league_id = match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to create a new league. Please try again later",
                )))
            }

            Ok(result) => {
                if result.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to create a new league on the tournament. Please make sure the tournament exists and you have ownership of it")));
                }

                result.last_insert_id()
            }
        };

        Ok(league_id)
    }

    /// This function is responsible to edit a league
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `league_id` - `u64` integer which represents the league id
    /// * `edit_data` - `LeagueRegisterDTO` struct containing the data which will edit the league
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Return
    /// `Ok(())` - Indicates that the operation is successful
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`
    /// * * The tournament or league does not exist
    /// * * The user id does not own the tournament which is related to the league
    /// * * The league name is too long
    pub async fn edit_league(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        edit_data: LeagueRegisterDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        if edit_data.name.len() > 20 {
            return Err(HTTPException::BadRequest(String::from(
                "The name of the league is too long",
            )));
        }

        let query = sqlx::query!(
            "
        UPDATE `leagues` as LeagueRow
        INNER JOIN `tournaments` as TournamentRow
            ON LeagueRow.tournament_id = TournamentRow.id
        SET LeagueRow.name = (?), LeagueRow.completed = (?)
        WHERE TournamentRow.id = (?) AND TournamentRow.user_id = (?) AND LeagueRow.id = (?)
        ",
            edit_data.name,
            edit_data.completed,
            tournament_id,
            user_id,
            league_id
        )
        .execute(db_pool)
        .await;

        match query {
            Err(e) => {
                dbg!(e);
                return Err(HTTPException::Internal(String::from(
                    "Failed to edit the league. Please try again later",
                )));
            }
            Ok(data) => {
                if data.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to edit the league. Make sure the tournament, league exists and that you also own the tournament")));
                }

                data
            }
        };

        Ok(())
    }

    /// Responsible to delete a league
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` `u64` integer which represents the tournament id
    /// * `league_id - `u64` integer which represents the league id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Return
    /// `Ok(())` - Indicates that the operation was successful
    ///
    /// # Errors
    ///
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`
    /// * * The tournament does not exist
    /// * * The league does not exist
    /// * * The user who's doing the action does not have permission to execute it
    pub async fn remove_league(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let query = sqlx::query!(
            "
        DELETE LeagueRow
        FROM `leagues` as LeagueRow
        INNER JOIN `tournaments` as TournamentRow
            ON LeagueRow.tournament_id = TournamentRow.id
        WHERE LeagueRow.id = (?)
        AND TournamentRow.user_id = (?)
        AND TournamentRow.id = (?)
        ",
            league_id,
            user_id,
            tournament_id
        )
        .execute(db_pool)
        .await;

        match query {
            Err(_) => return Err(HTTPException::Internal(String::from(""))),
            Ok(result) => {
                if result.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("")));
                }

                result
            }
        };

        Ok(())
    }

    /// Function responsible to get the league information
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `league_id` - `u64` integer which represents the league id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// `Ok(LeagueInformationDTO)` - `LeagueInformationDTO` struct containing the league information
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`
    /// * * The tournament does not exist
    /// * * The league does not exist
    /// * * The tournament is private and the user does not own the tournament
    pub async fn get_league(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<LeagueInformationDTO, HTTPException> {
        let query = sqlx::query!(
            "
        SELECT LeagueRow.id, LeagueRow.tournament_id, LeagueRow.completed, LeagueRow.name
        FROM `leagues` as LeagueRow
        INNER JOIN `tournaments` as TournamentRow
            ON TournamentRow.id = LeagueRow.tournament_id
        WHERE LeagueRow.id = (?) AND LeagueRow.tournament_id = (?) AND
        (
            (TournamentRow.public = false AND TournamentRow.user_id = (?))
            OR
            (TournamentRow.public = true)
        )
        ",
            league_id,
            tournament_id,
            user_id
        )
        .fetch_optional(db_pool)
        .await;

        let league_info = match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to get the league. Please try again later",
                )))
            }
            Ok(data) => match data {
                None => {
                    return Err(HTTPException::BadRequest(String::from("Failed to get the league. Please make sure the league, tournament exists and you have permissions to access the tournament")));
                }

                Some(value) => value,
            },
        };

        Ok(LeagueInformationDTO {
            id: league_info.id,
            name: league_info.name,
            tournament_id: league_info.tournament_id,
            completed: league_info.completed == 1,
        })
    }

    /// Function responsible to add a team to the league
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `league_id` - `u64` integer which represents the league id
    /// * `team_id` - `u64` integer which represents the team id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(())` - Indicates that the operation is successful
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`
    /// * * The league or team does not exists
    /// * * The user id does not own the tournament
    /// * * The team is already on the league
    pub async fn add_team_to_league(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        team_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let teams_quantity =
            Self::get_teams_quantity_from_league(user_id, league_id, db_pool).await?;
        if teams_quantity >= 24 {
            return Err(HTTPException::BadRequest(String::from(
                "The league is full",
            )));
        }

        let query = sqlx::query!(
            "
        INSERT INTO `teams_leagues` (`league_id`, `team_id`)
        SELECT (?), (?)
        FROM `tournaments` as TournamentRow
        INNER JOIN `leagues` as LeagueRow
            ON LeagueRow.tournament_id = TournamentRow.id
        WHERE LeagueRow.id = (?) AND LeagueRow.tournament_id = (?) AND TournamentRow.id = (?) AND TournamentRow.user_id = (?)
        ",
        league_id, team_id, league_id, tournament_id, tournament_id, user_id
        )
        .execute(db_pool)
        .await;

        match query {
            Err(error) => {
                let database_error = match error.as_database_error() {
                    Some(e) => e,
                    None => {
                        return Err(HTTPException::Internal(String::from(
                            "Something unknown happened. Please try again later",
                        )));
                    }
                };

                // Handles unique key violation
                // This happens when the user is trying to add a existing team to the league
                if database_error.is_unique_violation() {
                    return Err(HTTPException::BadRequest(String::from(
                        "Operation failed, the team may be already on the league",
                    )));
                }

                return Err(HTTPException::Internal(String::from(
                    "Something unknown happened. Please try again later",
                )));
            }
            Ok(data) => {
                if data.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to add the team to the league. Please make sure you own the tournament and the team/league exists")));
                }

                data
            }
        };

        Ok(())
    }

    /// Function responsible to remove a team from the league
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `league_id` - `u64` integer which represents the league id
    /// * `team_id` - `u64` integer which represents the team id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(())` - Indicates that the operation is successful
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`
    /// * * The league or team does not exists
    /// * * The user id does not own the tournament
    /// * * The team is not on the league
    pub async fn remove_team_from_league(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        team_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let query = sqlx::query!("
        DELETE TeamsLeaguesRow
        FROM `teams_leagues` as TeamsLeaguesRow

        INNER JOIN `leagues` as LeagueRow
            ON LeagueRow.id = TeamsLeaguesRow.league_id
        INNER JOIN `teams` as TeamRow
            ON TeamRow.id = TeamsLeaguesRow.team_id   
        INNER JOIN `tournaments` as TournamentRow
            On TournamentRow.id = LeagueRow.tournament_id 

        WHERE LeagueRow.id = (?) AND TournamentRow.id = (?) AND TournamentRow.user_id = (?) AND TeamRow.id = (?)
        ", league_id, tournament_id, user_id, team_id).execute(db_pool).await;

        match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from("Something unknown happened when deleting a team from the tournament. Please try again later")))
            }
            Ok(result) => {
                if result.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to delete the team from the league. Please make sure the team/league/tournament exists and that you own the tournament")))
                }

                result
            }
        };

        Ok(())
    }

    /// Responsible to get the league fixtures
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `league_id` - `u64` integer which represents the league id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(Vec<Fixture>)` - `Vec<Fixture>` vector containing the fixtures
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest` - If the league does not exist
    pub async fn get_league_fixtures(
        user_id: u64,
        tournament_id: u64,
        league_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<Vec<FixtureDataDTO>, HTTPException> {
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
            WHERE FixtureRow.league_id = (?) AND TournamentRow.id = (?) AND
            (
                (TournamentRow.public = FALSE AND TournamentRow.user_id = (?))
                OR
                (TournamentRow.public = TRUE)
            )
            ORDER BY FixtureRow.round ASC
        ",
            league_id,
            tournament_id,
            user_id
        )
        .fetch_all(db_pool)
        .await;

        let fixtures = match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from("Something wrong happened while getting the league fixtures. Please try again later")))
            }
            Ok(data) => data
        };

        Ok(fixtures
            .into_iter()
            .map(|fixture| FixtureDataDTO {
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
            .collect())
    }

    /// Function responsible to get the league standing table
    ///
    /// # Arguments
    /// * `league_id` - `u64` integer which represents the league id
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(Vec<TeamStandingTable>)` - `Vec<TeamStandingTable>` vector containing the league standing table
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    pub async fn get_league_standing_table(
        league_id: u64,
        user_id: u64,
        tournament_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<Vec<TeamStandingTable>, HTTPException> {
        // This is relatively a big query, so i'm commenting everything out
        // We're basically organizing a league table, so we can see the teams with best performances on the league
        // The teams table contains information about the teams which plays the league
        // The fixtures table contains information about the games, we'll use that as main parameter to define if a team lost or won
        let query = sqlx::query!(
            r#"
            -- Selecting all necessary tem properties, team name and team id
            SELECT TeamRow.id as team_id, TeamRow.name as team_name, 
            -- Selecting results row which tells the team performance and that we can organize/order it later
            ResultRow.total_points, ResultRow.win, ResultRow.draw, ResultRow.loss, ResultRow.goals_scored, ResultRow.goals_against, ResultRow.goal_difference
            -- Getting the teams so we can check it's stats
            FROM `teams` as TeamRow
            -- Making sure that if the tournament is private, only the tournament creator can access it
            INNER JOIN `tournaments` as TournamentRow
                ON (
                    (TournamentRow.public = FALSE AND TournamentRow.user_id = (?))
                    OR
                    (TournamentRow.public = TRUE)
                )
            JOIN (
                -- Selecting the properties to organize the league table
                -- Using CAST() to convert BigDecimals to UNSIGNED 64 BITS INTEGER
                -- We should downgrade the datatype because it's unnecessary big
                SELECT team_id, 
                CAST(SUM(total_points) as UNSIGNED) as total_points,
                CAST(SUM(win) as UNSIGNED) as win,
                CAST(SUM(draw) as UNSIGNED) as draw,
                CAST(SUM(loss) as UNSIGNED)  as loss,
                CAST(SUM(goals_scored) as UNSIGNED) as goals_scored,
                CAST(SUM(goals_against) as UNSIGNED) as goals_against,
                CAST((SUM(goals_scored) - SUM(goals_against)) as SIGNED) as goal_difference
                
                FROM (
                    -- Calculating the results from it's home performance
                        (
                            -- Selecting the home side of the team from the game fixture
                            SELECT FixtureRow.home_team_id as team_id,
                            
                            -- Handles points
                            -- WIN = 3 points
                            -- DRAW = 1 point
                            -- LOSE = 0 point
                            CASE
                                WHEN FixtureRow.played = TRUE AND FixtureRow.home_score > FixtureRow.away_score THEN 3
                                WHEN FixtureRow.played = TRUE AND FixtureRow.home_score = FixtureRow.away_score THEN 1
                                ELSE 0
                            END as total_points,
                                
                            -- Counts win
                            CASE
                                WHEN FixtureRow.played = TRUE AND FixtureRow.home_score > FixtureRow.away_score THEN 1
                                ELSE 0
                            END as win,
                            
                            -- Counts draw
                            CASE 
                                WHEN FixtureRow.played = TRUE AND FixtureRow.home_score = FixtureRow.away_score THEN 1
                                ELSE 0
                            END as draw,
                                
                            -- Counts losses
                            CASE 
                                WHEN FixtureRow.played = TRUE AND FixtureRow.home_score < FixtureRow.away_score THEN 1
                                ELSE 0
                            END as loss,
                                
                            -- This gets all played matches and get how much the team scored
                            -- This is used to calculated the "goals_scored" stat
                            CASE
                                WHEN FixtureRow.played = TRUE THEN FixtureRow.home_score
                                ELSE 0
                            END as goals_scored,
                                
                            -- Same as above, but calculates the "goal_against" stat
                            CASE 
                                WHEN FixtureRow.played = TRUE THEN FixtureRow.away_score
                                ELSE 0
                            END as goals_against
                                
                            -- Getting the fixtures from it's proper league
                            -- A fixture is the main parameter to get the team performance, as it contains informations such as: home_score and away_score
                            -- ^ But we'll only get the fixtures which contains "played" equals to TRUE
                            -- Makes sure the stats is only valid for it's league
                            -- We don't want count the team stats if he plays in more than 1 league
                            FROM `fixtures` as FixtureRow
                                WHERE FixtureRow.league_id = (?)
                        )
                    -- We'll union everything from both cases
                    UNION ALL
                    -- Calculating the team performance from it's away performance
                    -- The operations bellow is basically the same as above, so i'm not commenting that out
                        (
                            SELECT FixtureRow.away_team_id as team_id,
                            
                            CASE
                                WHEN FixtureRow.played = TRUE AND FixtureRow.away_score > FixtureRow.home_score THEN 3
                                WHEN FixtureRow.played = TRUE AND FixtureRow.away_score = FixtureRow.home_score THEN 1
                                ELSE 0
                            END as total_points,
                            
                                -- Counts win
                            CASE
                                WHEN FixtureRow.played = TRUE AND FixtureRow.away_score > FixtureRow.home_score THEN 1
                                ELSE 0
                            END as win,
                            
                            -- Counts draw
                            CASE 
                                WHEN FixtureRow.played = TRUE AND FixtureRow.away_score = FixtureRow.home_score THEN 1
                                ELSE 0
                            END as draw,
                                
                            -- Counts losses
                            CASE 
                                WHEN FixtureRow.played = TRUE AND FixtureRow.away_score < FixtureRow.home_score THEN 1
                                ELSE 0
                            END as loss,
                            
                            CASE
                                WHEN FixtureRow.played = TRUE THEN FixtureRow.away_score
                                ELSE 0
                            END as goals_scored,
                            
                            CASE 
                                WHEN FixtureRow.played = TRUE THEN FixtureRow.home_score
                                ELSE 0
                            END as goals_against
                        
                            FROM `fixtures` as FixtureRow
                                WHERE FixtureRow.league_id = (?)
                        )
                ) as SubQuery -- As we're grouping multiple queries, we'll rename to "SubQuery" so MySql understands this is a group of queries

                -- Grouping the collected stats by the team id
                GROUP BY team_id
            ) as ResultRow ON TeamRow.id = ResultRow.team_id -- Making that the teams stats doesnt repeat
            -- Making sure the tournament is valid
            WHERE TournamentRow.id = (?)
            -- Finally, we're ordering the table accordingly:
            -- Team with most points
            -- Team with most goals difference
            -- Team with most goals scored
            -- Team with less goals against
            ORDER BY ResultRow.total_points DESC, ResultRow.goal_difference DESC, ResultRow.goals_scored DESC, ResultRow.goals_against ASC
        "#, user_id, league_id, league_id, tournament_id
        ).fetch_all(db_pool).await;

        let standing_table = match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from("Something wrong happened while getting the league standing table. Please try again later")));
            }
            Ok(data) => data,
        };

        let standing_table: Vec<TeamStandingTable> = standing_table
            .into_iter()
            .map(|standing| TeamStandingTable {
                team_id: standing.team_id,
                team_name: standing.team_name,
                total_points: standing.total_points.unwrap_or(0) as u8,
                win: standing.win.unwrap_or(0) as u8,
                draw: standing.draw.unwrap_or(0) as u8,
                loss: standing.loss.unwrap_or(0) as u8,
                goals_scored: standing.goals_scored.unwrap_or(0) as u8,
                goals_against: standing.goals_against.unwrap_or(0) as u8,
                goal_difference: standing.goal_difference.unwrap_or(0) as i16,
            })
            .collect();

        Ok(standing_table)
    }

    /// Responsible to get the quantity of teams from the league
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `league_id` - `u64` integer which represents the league id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(i64)` - `i64` integer which represents the quantity of teams from the league(it may returns 0 if the league/tournament does not exists or the user acessing does not have permission to access it)
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    async fn get_teams_quantity_from_league(
        user_id: u64,
        league_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<i64, HTTPException> {
        let query = sqlx::query!(
            r#"
            SELECT COUNT(TeamRow.id) AS team_count FROM `teams` as TeamRow
            INNER JOIN `tournaments` as TournamentRow
                ON TournamentRow.id = TeamRow.tournament_id
            INNER JOIN `leagues` as LeagueRow
                ON LeagueRow.tournament_id = TournamentRow.id
            WHERE LeagueRow.id = (?)
            AND (
                (TournamentRow.public = FALSE AND TournamentRow.user_id = (?)) 
                OR 
                (TournamentRow.public = TRUE)
            )
        "#,
            league_id,
            user_id
        )
        .fetch_all(db_pool)
        .await
        .or_else(|e| {
            Err(HTTPException::Internal(String::from(
                "Failed to get the teams quantity from the league. Please try again later",
            )))
        })?;

        match query.get(0) {
            None => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to get the teams quantity from the league. Please try again later",
                )));
            }
            Some(data) => Ok(data.team_count),
        }
    }
}

pub async fn get_teams_from_league(
    league_id: u64,
    tournament_id: u64,
    user_id: u64,
    db_pool: &Pool<MySql>,
) -> Result<Vec<Team>, HTTPException> {
    let query = sqlx::query!(
        "
        SELECT TeamRow.*
        FROM `teams` as TeamRow
        INNER JOIN `teams_leagues` as TeamLeagueRow
            ON TeamRow.id = TeamLeagueRow.team_id
        
        INNER JOIN `leagues` as LeagueRow
            ON LeagueRow.id = TeamLeagueRow.league_id

        INNER JOIN `tournaments` as TournamentRow
            ON TournamentRow.id = LeagueRow.tournament_id    

        WHERE TeamLeagueRow.league_id = (?) AND TournamentRow.id = (?) AND TournamentRow.user_id = (?)
    ",
        league_id,
        tournament_id,
        user_id
    )
    .fetch_all(db_pool)
    .await;

    let teams = match query {
        Err(_) => {
            return Err(HTTPException::Internal(String::from(
                "Failed to get the teams from the league. Please try again later",
            )))
        }
        Ok(data) => data,
    };

    let teams = teams
        .into_iter()
        .map(|team| Team {
            id: team.id,
            name: team.name,
            tournament_id: team.tournament_id,
        })
        .collect();

    Ok(teams)
}
