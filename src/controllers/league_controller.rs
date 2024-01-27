use rocket::http::Status;
use rocket::{response::status, serde::json::Json, State};
use serde_json::json;
use sqlx::{MySql, Pool};

use crate::{
    jwt_auth_handler::UserToken,
    models::{
        league::{League, LeagueRegisterDTO},
        user::User,
    },
    responses::{CustomResponse, ErrorResponse, HTTPException, HTTPSuccessResponse},
};

#[get("/<tournament_id>/leagues/<league_id>")]
pub async fn get_league(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token).unwrap_or(0);
    let league = League::get_league(user_id, tournament_id, league_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::new(),
        data: serde_json::to_value(league).unwrap(),
    }))
}

#[get("/<tournament_id>/leagues/<league_id>/standing-table")]
pub async fn get_league_standing_table(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token).unwrap_or(0);
    let standing_table = League::get_league_standing_table(user_id, tournament_id, league_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::new(),
        data: serde_json::to_value(standing_table).unwrap(),
    }))
}

#[post("/<tournament_id>/leagues", format = "json", data = "<league_data>")]
pub async fn create_new_league(
    tournament_id: u64,
    token: Result<UserToken, ErrorResponse>,
    league_data: Json<LeagueRegisterDTO>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    let new_league = League::create_league(user_id, tournament_id, league_data.0, db_pool).await?;

    Ok(HTTPSuccessResponse::Created(CustomResponse {
        message: String::from("League created successfully"),
        data: serde_json::to_value(json!({
            "id": new_league,
        }))
        .unwrap(),
    }))
}

#[post("/<tournament_id>/leagues/<league_id>/teams/<team_id>")]
pub async fn league_add_team(
    tournament_id: u64,
    league_id: u64,
    team_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    League::add_team_to_league(user_id, tournament_id, league_id, team_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Team added successfully"),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[put(
    "/<tournament_id>/leagues/<league_id>",
    format = "json",
    data = "<edit_data>"
)]
pub async fn edit_league(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    edit_data: Json<LeagueRegisterDTO>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    League::edit_league(user_id, tournament_id, league_id, edit_data.0, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("League edited successfully"),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[delete("/<tournament_id>/leagues/<league_id>")]
pub async fn delete_league(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<status::Custom<&str>, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    League::remove_league(user_id, tournament_id, league_id, db_pool).await?;

    Ok(status::Custom(Status::NoContent, ""))
}

#[delete("/<tournament_id>/leagues/<league_id>/teams/<team_id>")]
pub async fn league_remove_team(
    tournament_id: u64,
    league_id: u64,
    team_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<status::Custom<&str>, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    League::remove_team_from_league(user_id, tournament_id, league_id, team_id, db_pool).await?;

    Ok(status::Custom(Status::NoContent, ""))
}
