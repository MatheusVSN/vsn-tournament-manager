use sqlx::{MySql, Pool, QueryBuilder};

use crate::{models::tournament::{Tournament, TournamentRegisterDTO, TournamentEditDTO, TournamentInformationData, LeagueInformationData, TeamInformationData}, responses::HTTPException};

impl Tournament {
    /// Responsible to create a new tournament for a user. If the operation succeeds it'll be returned a `Ok(u64)` value that represents the id of the new created tournament
    ///
    /// # Arguments
    /// * `user_id` - A `u64` integer that represents the user id which is creating a new tournament
    /// * `new_tournament_data` - A `TournamentRegisterDTO` struct that represents the tournament data
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(u64)` - A `u64` integer that represents the id of the new tournament
    ///
    /// # Errors
    ///
    /// * `HTTPException::Internal` if the query fails, this may happens when a SQL connection is not successful
    /// * `HTTPException::BadRequest` if the tournament name is more than 20 characters
    pub async fn create_tournament(
        user_id: u64,
        new_tournament_data: TournamentRegisterDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<u64, HTTPException> {
        if new_tournament_data.name.len() > 20 {
            return Err(HTTPException::BadRequest(String::from(
                "Cannot set the tournament name more than 20 characters",
            )));
        }

        let result = sqlx::query_as!(
            Tournament,
            r#"
        INSERT INTO `tournaments` (`name`, `public`, `user_id`)
        VALUES (?, ?, ?)
        "#,
            &new_tournament_data.name,
            &new_tournament_data.public,
            &user_id
        )
        .execute(db_pool)
        .await;

        match result {
            Ok(result) => Ok(result.last_insert_id()),
            Err(_) => Err(HTTPException::Internal(
                "Failed while creating the tournament. Please try again later".to_string(),
            )),
        }
    }

    /// Responsible to delete a tournament that a user owns.
    /// It's important to know that this function **deletes a tournament that the user owns it**.
    /// So the user must be authenticated to delete it, and he cannot delete other users tournament
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer that represents the id of a user which will remove the tournament
    /// * `tournament_id` - `u64` integer that represents the id of the tournament which will be removed
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(())` - indicate that the operation is successful
    /// # Errors
    ///
    /// * `HTTPException::Internal` if the query fails, this may happens when a SQL connection is not successful
    /// * `HTTPException::BadRequest` if the tournament that the `user_id` is trying to delete does not exists or it's a tournament that other users owns it
    pub async fn delete_tournament(
        user_id: u64,
        tournament_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        let result = sqlx::query_as!(
            Tournament,
            r#"
        DELETE FROM `tournaments`
        WHERE `id` = (?) AND `user_id` = (?)
        "#,
            &tournament_id,
            &user_id
        )
        .execute(db_pool)
        .await;

        match result {
            Ok(response) => {
                if response.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from(
                        "Failed to delete the tournament. Make sure that the tournament does exists",
                    )));
                }
                Ok(())
            }
            Err(_) => Err(HTTPException::BadRequest(String::from(
                "Failed to delete the tournament. Please try again later",
            ))),
        }
    }

    /// Responsible to edit a tournament that the argument `user_id` owns it
    ///
    /// # Arguments
    /// * `user_id` - `u64` integer that represents the user id which is performing the edit action
    /// * `tournament` - `u64` integer that represents the tournament id which is getting modified
    /// * `edit_data` - `TournamentEditDTO` struct that represents the data which will be edited and applied into the tournament
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// If the function executes successfully, it'll return a `Ok(())`
    ///
    /// # Errors
    /// * `HTTPException::Internal` if the query fails, this may happens when a SQL connection is not successful
    /// * `HTTPException::BadRequest` does have multiple errors, they are:
    ///     * The user does not provide any of optional values at `edit_data` argument, meaning all the data is `None` and there's nothing to edit
    ///     * The tournament does not exists
    ///     * User is trying to edit a tournament which he does not owns it
    ///     * The user attempted to set the tournament name more than 20 characters
    pub async fn edit_tournament(
        user_id: u64,
        tournament_id: u64,
        edit_data: TournamentEditDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<(), HTTPException> {
        if edit_data.name.is_none() && edit_data.public.is_none() {
            return Err(HTTPException::BadRequest(String::from(
                "Attempted to edit the tournament, but no data was given",
            )));
        }

        let mut query = QueryBuilder::new("UPDATE `tournaments` SET");

        if let Some(public) = edit_data.public {
            query.push(" `public` = ").push_bind(public);
        }

        if let Some(name) = edit_data.name {
            if name.len() > 20 {
                return Err(HTTPException::BadRequest(String::from(
                    "Cannot set the tournament name more than 20 characters",
                )));
            }

            if edit_data.public.is_some() {
                query.push(", ");
            }
            query.push(" `name` = ").push_bind(name);
        }

        let formatted_query = format!(
            " WHERE `id` = {} AND `user_id` = {}",
            tournament_id, user_id
        );
        query.push(formatted_query);

        let result = query.build().execute(db_pool).await;

        match result {
            Ok(data) => {
                if data.rows_affected() < 1 {
                    return Err(HTTPException::BadRequest(String::from(
                        "Failed to edit the tournament. Please make sure the tournament exists and you own it",
                    )));
                }

                Ok(())
            }
            Err(e) => {
                println!("{}", e);
                return Err(HTTPException::Internal(
                    "Failed while editing the tournament. Please try again later".to_string(),
                ));
            }
        }
    }

    /// Gets the tournament information data. Such as leagues and teams associated with
    ///
    /// # Arguments
    /// * `user_id` - `Option<u64>` value which represents the user_id, if existing
    /// * `tournament_id` - `u64` integer which represents the id of the tournament
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// `Ok(TournamentInformationData)` - `TournamentInformationData` struct with the tournament, leagues and teams information
    ///
    /// # Error
    /// `HTTPException::Internal` - If something wrong happens with the database query
    /// `HTTPException::BadRequest` - If the tournament does not exists or it's private and other user is trying to access it
    pub async fn get_tournament_information_by_id(
        user_id: Option<u64>,
        tournament_id: u64,
        db_pool: &Pool<MySql>,
    ) -> Result<TournamentInformationData, HTTPException> {
        let tournament = sqlx::query!(
            "
        SELECT id, name
        FROM `tournaments`
        WHERE `tournaments`.id = ? AND 
        ((public = FALSE AND user_id = ?) OR
        (public = TRUE))
        ",
            tournament_id,
            user_id
        )
        .fetch_optional(db_pool)
        .await;

        let tournament = match tournament {
            Ok(optional_data) => match optional_data {
                Some(data) => data,
                None => {
                    let formatted_message = format!("Failed to find the tournament with the id of {}. It may not exist or you don't have permission to access it", tournament_id);
                    return Err(HTTPException::BadRequest(formatted_message));
                }
            },
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to search for the tournament. Please try again later",
                )));
            }
        };

        let leagues = sqlx::query!(
            "SELECT id, name, completed
            FROM `leagues`
            WHERE tournament_id = ?",
            tournament_id
        )
        .fetch_all(db_pool)
        .await;

        let leagues = match leagues {
            Ok(data) => data,
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to search the leagues on the tournament. Please try again later",
                )));
            }
        };

        let teams = sqlx::query!(
            "SELECT id, name
            FROM `teams`
            WHERE tournament_id = ?",
            tournament_id
        )
        .fetch_all(db_pool)
        .await;

        let teams = match teams {
            Ok(data) => data,
            Err(_) => {
                return Err(HTTPException::Internal(String::from(
                    "Failed to search the teams on the tournament. Please try again later",
                )));
            }
        };

        Ok(TournamentInformationData {
            id: tournament.id,
            name: tournament.name,
            leagues: leagues
                .into_iter()
                .map(|league| {
                    let completed = if league.completed == 1 { true } else { false };
                    return LeagueInformationData {
                        id: league.id,
                        name: league.name,
                        completed,
                    };
                })
                .collect(),
            teams: teams
                .into_iter()
                .map(|team| TeamInformationData {
                    id: team.id,
                    name: team.name,
                })
                .collect(),
        })
    }
}