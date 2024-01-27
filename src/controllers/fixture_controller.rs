use rocket::{serde::json::Json, State};
use sqlx::{MySql, Pool};

use crate::{
    jwt_auth_handler::UserToken,
    models::{fixture::{EditFixtureDTO, Fixture}, league::League, user::User},
    responses::{CustomResponse, ErrorResponse, HTTPException, HTTPSuccessResponse},
};

#[get("/<tournament_id>/leagues/<league_id>/fixtures")]
pub async fn get_league_fixtures(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token).unwrap_or(0);
    let fixtures = League::get_league_fixtures(user_id, tournament_id, league_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::new(),
        data: serde_json::to_value(fixtures).unwrap(),
    }))
}

#[delete("/<tournament_id>/leagues/<league_id>/fixtures")]
pub async fn delete_fixtures_from_league(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    Fixture::delete_all_fixtures_from_league(user_id, tournament_id, league_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Successfully deleted fixtures"),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[post("/<tournament_id>/leagues/<league_id>/fixtures")]
pub async fn generate_fixtures(
    tournament_id: u64,
    league_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    Fixture::generate_league_fixtures(user_id, tournament_id, league_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Successfully generated fixtures"),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[get("/<tournament_id>/leagues/<league_id>/fixtures/<fixture_id>")]
pub async fn get_fixture_by_id(
    tournament_id: u64,
    league_id: u64,
    fixture_id: u64,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token).unwrap_or(0);
    let fixture =
        Fixture::get_league_fixture(user_id, tournament_id, league_id, fixture_id, db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::new(),
        data: serde_json::to_value(fixture).unwrap(),
    }))
}

#[put(
    "/<tournament_id>/leagues/<league_id>/fixtures/<fixture_id>",
    data = "<fixture_data>"
)]
pub async fn edit_fixture(
    tournament_id: u64,
    league_id: u64,
    fixture_id: u64,
    fixture_data: Json<EditFixtureDTO>,
    token: Result<UserToken, ErrorResponse>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let user_id = User::get_user_id_by_token(token)?;
    let fixture = Fixture::edit_fixture_by_id(
        user_id,
        tournament_id,
        league_id,
        fixture_id,
        fixture_data.0,
        db_pool,
    )
    .await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: String::from("Successfully edited fixture"),
        data: serde_json::to_value(fixture).unwrap(),
    }))
}
