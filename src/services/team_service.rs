use sqlx::{MySql, Pool};

use crate::{
    models::team::{Team, TeamInfoDTO, TeamRegisterDTO},
    responses::HTTPException,
};

impl Team {
    /// Responsible to create a team on the tournament which is specified by `tournament_id`
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id of the tournament owner/creator
    /// * `tournamentT_id` - `u64` integer which represents the id of the tournament which the team will be created for
    /// * `team_data` - `TeamRegisterDTO` struct which represents the new team information data
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(u64)` - `u64` integer which represents the created team
    ///
    /// # Errors
    /// * `HTTPException::BadRequest`:
    /// * * The team name is more than 40 characters
    /// * * The user who's trying to add the team is not the tournament owner
    ///
    /// * `HTTPException::Internal` - If the database query fails
    pub async fn create_team(
        user_id: u64,
        tournament_id: u64,
        team_data: TeamRegisterDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<u64, HTTPException> {
        if team_data.name.len() > 40 {
            return Err(HTTPException::BadRequest(String::from(
                "A team name cannot exceed 40 characters",
            )));
        }

        let query = sqlx::query!(
            "
        INSERT INTO `teams` (`name`, `tournament_id`)
        SELECT (?), (?)
        FROM `tournaments`
        WHERE id = (?) AND user_id = (?)",
            team_data.name,
            tournament_id,
            tournament_id,
            user_id
        )
        .execute(db_pool)
        .await;

        let team_id = match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to create a new team. Please try again later",
                )));
            }
            Ok(result) => {
                if result.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to create a new team to the tournament. Please make sure the tournament exists or you own it")));
                }

                Ok(result.last_insert_id())
            }
        }?;

        Ok(team_id)
    }

    /// Responsible to get a team information by it's `id` and `tournament_id`
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the user id
    /// * `tournament_id` - `u64` integer which represents the tournament id
    /// * `team_id` - `u64` integer which represents the team id
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(TeamInfoDTO)` - `TeamInfoDTO` which contains the found team information
    ///
    /// # Errors:
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`:
    /// * * The tournament is private and a user who does not have permissions tries to access it
    /// * * The team does not exist
    /// * * The tournament does not exist
    pub async fn get_team_by_id_and_tournament_id(
        user_id: u64,
        tournament_id: u64,
        team_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<TeamInfoDTO, HTTPException> {
        let query = sqlx::query!(
            "
            SELECT TeamRow.id, TeamRow.name, TeamRow.tournament_id 
            FROM `teams` as TeamRow
            INNER JOIN `tournaments` as TournamentRow
                ON TeamRow.tournament_id = TournamentRow.id
            WHERE TeamRow.id = (?) AND TeamRow.tournament_id = (?) AND
            (
                (TournamentRow.public = FALSE AND TournamentRow.user_id = (?))
                OR (TournamentRow.public = TRUE)
            )
        ",
            team_id,
            tournament_id,
            user_id
        )
        .fetch_optional(db_pool)
        .await;

        let team_info = match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Operation failed while looking for the team. Please try again later",
                )))
            }
            Ok(data) => match data {
                Some(info) => info,
                None => {
                    return Err(HTTPException::BadRequest(format!("Not found a team with the id of {}. Please make sure the team, tournament exists and you have permissions to access the tournament", team_id)));
                }
            },
        };

        Ok(TeamInfoDTO {
            id: team_info.id,
            name: team_info.name,
        })
    }

    /// Responsible to edit the team by it's `id`, `tournament_id` and `user_id`
    ///
    /// # Arguments
    /// * `tournament_id` - `u64` integer which represents the id of the tournament that team is in
    /// * `user_id` - `u64` integer which represents the user id that owns the tournament
    /// * `team_id` - `u64` integer which represents the team id that will be modified
    /// * `edit_data` - `TeamRegisterDTO` struct containing the edit information
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// `Ok(())` - Indicating that the operation is successful
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`:
    /// * * The team name contains more than 40 characters
    /// * * The `tournament_id` or `team_id` does not exist
    /// * * The `user_id` does not owns the tournament
    pub async fn edit_team(
        tournament_id: u64,
        user_id: u64,
        team_id: u64,
        edit_data: TeamRegisterDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        if edit_data.name.len() > 40 {
            return Err(HTTPException::BadRequest(String::from(
                "A team name cannot exceed 40 characters",
            )));
        }

        let query = sqlx::query!(
            "
        UPDATE `teams`
        INNER JOIN `tournaments`
            ON `teams`.tournament_id = `tournaments`.id
        SET `teams`.name = (?)
        WHERE `tournaments`.id = (?) AND `tournaments`.user_id = (?) AND `teams`.id = (?)
        ",
            edit_data.name,
            tournament_id,
            user_id,
            team_id
        )
        .execute(db_pool)
        .await;

        match query {
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to edit the team. Please try again later",
                )));
            }
            Ok(result) => {
                if result.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to edit the team. Please make sure the team, tournament exists and that you also own the tournament")));
                }
            }
        }

        Ok(())
    }

    /// Responsible to edit the team by it's `id` and `user_id`
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer which represents the id of the user doing the action. The user id must be equal to the tournament user_id row when deleting the team
    /// * `team_id` - `u64` integer which represents the team id that will be deleted
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(())` - Representing the action was successful
    ///
    /// # Errors
    /// * `HTTPException::Internal` - If the database query fails
    /// * `HTTPException::BadRequest`:
    pub async fn delete_team(
        tournament_id: u64,
        user_id: u64,
        team_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let query = sqlx::query!(
            "
        DELETE TeamRow
        FROM `teams`as TeamRow
        INNER JOIN `tournaments` as TournamentRow
        WHERE TeamRow.id = (?)
        AND TeamRow.tournament_id = (?)
        AND TournamentRow.user_id = (?)
        ",
            team_id,
            tournament_id,
            user_id
        )
        .execute(db_pool)
        .await;

        match query {
            Ok(result) => {
                if result.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from("Failed to delete the team. Make sure the team exists and you have ownership permissions of the tournament")));
                }
            }
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to delete the team on the database. Please try again later",
                )));
            }
        };

        Ok(())
    }
}
