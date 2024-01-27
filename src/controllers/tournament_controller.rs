use rocket::http::Status;
use rocket::{response::status, serde::json::Json, State};
use serde_json::json;
use sqlx::{MySql, Pool};

use crate::models::tournament::TournamentEditDTO;
use crate::{
    jwt_auth_handler::UserToken,
    models::{
        tournament::{Tournament, TournamentRegisterDTO},
        user::User,
    },
    responses::{CustomResponse, ErrorResponse, HTTPException, HTTPSuccessResponse},
};

#[post("/", format = "json", data = "<new_tournament_info>")]
pub async fn create_tournament(
    new_tournament_info: Json<TournamentRegisterDTO>,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    let new_tournament_id =
        Tournament::create_tournament(user_id, new_tournament_info.0, &db_pool).await?;

    Ok(HTTPSuccessResponse::Created(CustomResponse {
        message: String::from("Tournament created successfully!"),
        data: serde_json::to_value(json!({
            "id": new_tournament_id
        }))
        .unwrap(),
    }))
}

#[delete("/<tournament_id>")]
pub async fn delete_tournament(
    tournament_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<status::Custom<&str>, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    Tournament::delete_tournament(user_id, tournament_id, &db_pool).await?;

    Ok(status::Custom(Status::NoContent, ""))
}

#[put("/<tournament_id>", format = "json", data = "<edit_data>")]
pub async fn edit_tournament(
    tournament_id: u64,
    token: Result<UserToken, ErrorResponse>,
    edit_data: Json<TournamentEditDTO>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    Tournament::edit_tournament(user_id, tournament_id, edit_data.0, &db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Tournament edited successfully!"),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[get("/<tournament_id>")]
pub async fn get_tournament(
    tournament_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id: Option<u64> = User::get_user_id_by_token(token).ok();
    let tournament_information =
        Tournament::get_tournament_information_by_id(user_id, tournament_id, &db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Tournament information here"),
        data: serde_json::to_value(tournament_information).unwrap(),
    }))
}
