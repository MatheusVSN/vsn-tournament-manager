use std::env;

use chrono::Utc;
use jsonwebtoken::{errors::Error, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::request::Outcome;
use rocket::Request;
use serde::{Deserialize, Serialize};

use crate::responses::ErrorResponse;

#[derive(Serialize, Deserialize, Debug)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user_id: u64,
}

const ONE_WEEK: i64 = 60 * 60 * 24 * 7;

/// This function handles the requests. To use it you need to insert the following parameter:
///
/// ```
/// #[get("/some_route")]
/// fn some_route(token: Result<TokenData<UserToken>, ErrorResponse>)
/// ```
///
/// The position does not matter as long the type is `Result<TokenData<UserToken>, ErrorResponse>``
#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserToken {
    type Error = ErrorResponse;

    // Gets the coming request, if the token parameter with the correct type is specified on the route function
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Function below verifies:
        // If there's a "Authorization" on the coming header request
        // There's a "Bearer" string on the value
        // Split the "Bearer " string into 2 values, one being the Bearer and the other value being the token
        // It decodes the token
        // If everything is successful and the token is valid, it returns the token claims and it can be used on the specified token parameter at the route function
        if let Some(auth_header) = request.headers().get_one("Authorization") {
            let auth_string = auth_header.to_string();
            if auth_string.contains("Bearer") {
                let bearer_vector: Vec<_> = auth_string.split("Bearer ").collect();
                let token = bearer_vector[1];
                if let Ok(token_data) = decode_token(token.to_string()) {
                    return Outcome::Success(token_data.claims);
                }
            }
        }

        // Token is invalid
        Outcome::Error((
            Status::Unauthorized,
            ErrorResponse {
                message: String::from("Unauthorized operation"),
            },
        ))
    }
}

/// Responsible to generate a user token with the `user_id` as only argument.
/// We can use this to get the user from the database and use it for authorization depending on the action
///
/// # Arguments
/// * `user_id` - `u64` integer which represents the user id
///
/// # Errors
/// `Error` - If something wrongs happens when trying to encode the token
pub fn generate_token(user_id: u64) -> Result<String, Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET missing on env variables");
    let now = Utc::now().timestamp();

    let payload = UserToken {
        iat: now,
        exp: now + ONE_WEEK,
        user_id,
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )
}

/// Responsible to decode a jwt token
///
/// # Arguments
/// * `token` - `String` value that represents the jwt token
///
/// # Errors
/// * `Error` -  Happens if the token is invalid or something wrong failed while decoding it
pub fn decode_token(token: String) -> Result<TokenData<UserToken>, Error> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET missing on env variables");

    jsonwebtoken::decode(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::default(),
    )
}
