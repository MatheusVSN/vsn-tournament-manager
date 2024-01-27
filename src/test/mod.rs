use async_recursion::async_recursion;
use rocket::http::{ContentType, Status};
use rocket::local::asynchronous::Client;
use serde::Deserialize;

use crate::models::user::{UserLoginDTO, UserSignUpDTO};

#[derive(Debug, Deserialize)]
struct LoginData {
    token: String,
}

#[derive(Debug, Deserialize)]
struct IdData {
    id: u64,
}

#[derive(Debug, Deserialize)]
struct APIResponse<T> {
    #[allow(dead_code)]
    message: String,
    data: T,
}

async fn simulate_login(client: &Client, login_dto: UserLoginDTO) -> String {
    let json_input = serde_json::to_string(&login_dto).unwrap();

    // Making the request to check the login credentials
    let login_response = client
        .post("/authentication/login")
        .header(ContentType::JSON)
        .body(json_input)
        .dispatch()
        .await;

    // Getting the JWT token so we can use to test the authorization
    let response_body: APIResponse<LoginData> = login_response.into_json().await.unwrap();
    let generated_jwt_token = response_body.data.token;

    generated_jwt_token
}

#[async_recursion]
async fn register_and_login(client: &Client) -> String {
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZ123456789";
    let random_string = random_string::generate(20, charset);
    let formatted_fake_email = format!("{}@test.com", random_string);

    let register_dto = UserSignUpDTO {
        email: formatted_fake_email,
        password: String::from("12345678"),
        name: random_string,
    };
    let json_body = serde_json::to_string(&register_dto).unwrap();

    // Making a request to register
    let response = client
        .post("/authentication/register")
        .header(ContentType::JSON)
        .body(json_body)
        .dispatch()
        .await;

    // In case there's a existing user
    if response.status() == Status::Conflict {
        let new_register = register_and_login(client).await;
        return new_register;
    }

    // Making sure the user is created so we can log in
    assert_eq!(response.status(), Status::Created);

    let authentication_token = simulate_login(
        client,
        UserLoginDTO {
            email: register_dto.email,
            password: register_dto.password,
        },
    )
    .await;

    authentication_token
}

mod utilities;

mod index;
mod league_tests;
mod team_tests;
mod tournament_tests;
mod fixture_tests;