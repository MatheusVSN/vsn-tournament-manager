use rocket::http::Status;
use rocket::{response::status, serde::json::Json, State};
use serde_json::json;
use sqlx::{MySql, Pool};

use crate::{
    jwt_auth_handler::UserToken,
    models::{
        team::{Team, TeamRegisterDTO},
        user::User,
    },
    responses::{CustomResponse, ErrorResponse, HTTPException, HTTPSuccessResponse},
};

#[get("/<tournament_id>/teams/<team_id>")]
pub async fn get_team(
    tournament_id: u64,
    team_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token).unwrap_or(0);
    let team_info =
        Team::get_team_by_id_and_tournament_id(user_id, tournament_id, team_id, &db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::new(),
        data: serde_json::to_value(team_info).unwrap(),
    }))
}

#[post("/<tournament_id>/teams", format = "json", data = "<team_data>")]
pub async fn create_team(
    tournament_id: u64,
    team_data: Json<TeamRegisterDTO>,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    let created_team_id = Team::create_team(user_id, tournament_id, team_data.0, &db_pool).await?;

    Ok(HTTPSuccessResponse::Created(CustomResponse {
        message: String::from("Team created successfully"),
        data: serde_json::to_value(json!({
            "id": created_team_id
        }))
        .unwrap(),
    }))
}

#[put(
    "/<tournament_id>/teams/<team_id>",
    format = "json",
    data = "<edit_data>"
)]
pub async fn edit_team(
    tournament_id: u64,
    team_id: u64,
    token: Result<UserToken, ErrorResponse>,
    edit_data: Json<TeamRegisterDTO>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    Team::edit_team(tournament_id, user_id, team_id, edit_data.0, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Team edited successfully!"),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[delete("/<tournament_id>/teams/<team_id>")]
pub async fn delete_team(
    tournament_id: u64,
    team_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<status::Custom<&str>, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    Team::delete_team(tournament_id, user_id, team_id, db_pool).await?;

    Ok(status::Custom(Status::NoContent, ""))
}
