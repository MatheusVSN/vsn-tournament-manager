use crate::models::user::{UserLoginDTO, UserSignUpDTO};
use crate::responses::{CustomResponse, HTTPException, HTTPSuccessResponse};
use crate::services::user_service::{create_user, user_login};
use rocket::serde::json::Json;
use rocket::State;
use serde_json::json;
use sqlx::{MySql, Pool};

#[post("/register", format = "json", data = "<user>")]
pub async fn register(
    user: Json<UserSignUpDTO>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    create_user(user.0, db_pool).await?;

    Ok(HTTPSuccessResponse::Created(CustomResponse {
        message: "Account created successfully".to_string(),
        data: serde_json::to_value("").unwrap(),
    }))
}

#[post("/login", format = "json", data = "<login>")]
pub async fn login(
    login: Json<UserLoginDTO>,
    db_pool: &State<Pool<MySql>>,
) -> Result<HTTPSuccessResponse, HTTPException> {
    let jwt = user_login(login.0, &db_pool).await?;

    Ok(HTTPSuccessResponse::Ok(CustomResponse {
        message: "Success!".to_string(),
        data: serde_json::to_value(json!({
            "token": jwt,
        }))
        .unwrap(),
    }))
}
