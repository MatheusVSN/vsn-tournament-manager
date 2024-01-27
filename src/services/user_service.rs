use bcrypt::{hash, DEFAULT_COST, verify};
use sqlx::{MySql, Pool};

use crate::jwt_auth_handler::{self, UserToken};
use crate::models::user::UserLoginDTO;
use crate::models::user::{User, UserSignUpDTO};
use crate::responses::{HTTPException, ErrorResponse};
use email_address::*;

/// Function responsible to create a user when he signs up
///
/// # Arguments
/// * `new_user` - `UserSignUpDTO` struct with represents the new user credentials
/// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
///
/// # Errors
/// * `HTTPException::BadRequest` - if the e-mail is invalid
/// * `HTTPException::Conflict` - the e-mail the user is trying to sign up is already being in use
/// *  Functions with the "?" on the end can throw a `HTTPException` error. For more details you should look onto them
pub async fn create_user(
    new_user: UserSignUpDTO,
    db_pool: &Pool<MySql>,
) -> Result<(), HTTPException> {
    if EmailAddress::is_valid(&new_user.email) == false {
        return Err(HTTPException::BadRequest(String::from(
            "Invalid e-mail address",
        )));
    }

    let existing_user = User::find_user_by_email(&new_user.email, &db_pool).await?;

    if existing_user.is_some() {
        return Err(HTTPException::Conflict(
            "There's a user with the same e-mail registered.".to_string(),
        ));
    }

    User::create_user(new_user, &db_pool).await?;
    Ok(())
}

/// Function responsible to verify the user login credentials. If it's successful it returns a token which contains the user_id
///
/// # Arguments
/// * `user_login` - `UserLoginDTO` struct which represents the user login credentials
/// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
///
/// # Errors
/// * `HTTPException::Internal` - If something wrong happens when trying to generate the token
/// * Functions with the "?" on the end can throw a `HTTPException` error. For more details you should look onto them
pub async fn user_login(
    user_login: UserLoginDTO,
    db_pool: &Pool<MySql>,
) -> Result<String, HTTPException> {
    let user_id = User::authenticate(&user_login, &db_pool).await?;
    let generated_jwt = match jwt_auth_handler::generate_token(user_id) {
        Err(_) => {
            return Err(HTTPException::Internal(String::from(
                "Failed to generate the login authorization. Please try again later",
            )));
        }
        Ok(token) => Ok(token),
    }?;

    Ok(generated_jwt)
}

impl User {
    /// Returns a optional value of a `User`, or `None` if there's no user associated with the given `email` argument
    ///
    /// # Arguments
    /// * `email` - A `&String` value which represents the e-mail that you're looking for
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(Optional<User>)` - A optional value that may contain a user if the e-mail matches
    ///
    /// # Errors
    ///
    /// * `HTTPException::Internal` if the query fails, this may happens when a SQL connection is not successful
    pub async fn find_user_by_email(
        email: &String,
        db_pool: &Pool<MySql>,
    ) -> Result<Option<User>, HTTPException> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM `users` WHERE `email` = (?)",
            &email.to_lowercase()
        )
        .fetch_optional(db_pool)
        .await;

        match user {
            Ok(optional_user) => Ok(optional_user),
            Err(_) => Err(HTTPException::Internal(
                "Something wrong happened while looking for users. Please try again later"
                    .to_string(),
            )),
        }
    }

    /// Responsible to create a user, the function returns a `true` bool value representing the operation is successful
    ///
    /// # Arguments
    /// * `user` - A `UserSignUpDTO` struct that contains the register data
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(true)` If the operation is successful and the user is saved on the database
    ///
    /// # Errors
    ///
    /// * `HTTPException::Internal` if the query fails, this may happens when a SQL connection is not successful
    pub async fn create_user(
        user: UserSignUpDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<bool, HTTPException> {
        let hashed_password = hash(user.password, DEFAULT_COST).unwrap();

        let result = sqlx::query(
            r#"
        INSERT INTO `users` (`email`, `password`, `name`)
        VALUES (?, ?, ?)
        "#,
        )
        .bind(user.email.to_lowercase())
        .bind(hashed_password)
        .bind(user.name)
        .execute(db_pool)
        .await;

        match result {
            Ok(_) => Ok(true),
            Err(_) => Err(HTTPException::Internal(
                "Failed while saving the user. Please try again later".to_string(),
            )),
        }
    }

    /// Responsible to check the client login credentials and returns a u64 integer representing the user id with the matching email and password
    ///
    /// # Arguments
    /// * `user_login` - A `UserLoginDTO` struct that represents the client login credentials
    /// * `db_pool` - A `&Pool<MySql>` reference for the MySQL database connection
    ///
    /// # Returns
    /// * `Ok(u64)` - A `u64` integer that represents the id of the matching user
    ///
    /// # Errors
    ///
    /// * `HTTPException::Internal` if the query fails, this may happens when a SQL connection is not successful
    /// * `HTTPException::BadRequest` if the client password hash does not match with the found user password hash
    /// * `HTTPException::NotFound` if the e-mail the client provided is not used by any user
    pub async fn authenticate(
        user_login: &UserLoginDTO,
        db_pool: &Pool<MySql>,
    ) -> Result<u64, HTTPException> {
        let user = match User::find_user_by_email(&user_login.email, db_pool).await? {
            None => Err(HTTPException::NotFound(String::from(
                "Not found a user with the specified credentials",
            ))),
            Some(user) => Ok(user),
        }?;

        let _match = match verify(&user_login.password, &user.password) {
            Ok(state) => {
                if state == false {
                    Err(HTTPException::BadRequest(String::from(
                        "Passwords does not match",
                    )))
                } else {
                    Ok(state)
                }
            }
            Err(_) => Err(HTTPException::Internal(String::from(
                "Failed to check the credentials. Please try again later",
            ))),
        }?;

        Ok(user.id)
    }

    /// Responsible to get the user id from a authorization token
    ///
    /// # Arguments
    /// `token` - `Result<UserToken, ErrorResponse>` result which represents if the token is valid
    ///
    /// # Errors
    /// `HTTPException::Unauthorized` - The token is invalid
    pub fn get_user_id_by_token(
        token: Result<UserToken, ErrorResponse>,
    ) -> Result<u64, HTTPException> {
        match token {
            Ok(token) => Ok(token.user_id),
            Err(e) => {
                return Err(HTTPException::Unauthorized(e.message));
            }
        }
    }
}
