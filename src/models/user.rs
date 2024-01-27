use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct UserSignUpDTO {
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserLoginDTO {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password: String,
    pub name: String,
}
