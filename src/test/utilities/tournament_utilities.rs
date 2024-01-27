use rocket::local::asynchronous::{Client, LocalResponse};

use super::base_request_test;

/// Responsible to make a request to the API to create a new tournament
///
/// # Arguments
/// * `client` - `rocket::local::asynchronous::client`, Rocket client to make requests
/// * `authorization_token` - `String` reference that represents the user authorization token
/// * `tournament_data` - `str` reference  which will represent a JSON format to create the tournament
pub async fn create_tournament_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_data: &str,
) -> LocalResponse<'a> {
    let route = String::from("/tournament");

    base_request_test(
        client,
        rocket::http::Method::Post,
        authorization_token.unwrap_or(&String::new()),
        route,
        tournament_data,
    )
    .await
}

/// Responsible to make a request to the API to edit the tournament
///
/// # Arguments
/// * `client` - `rocket::local::asynchronous::client`, Rocket client to make requests
/// * `tournament_id` - `u64` integer which represents the tournament id that's getting edited
/// * `authorization_token` - `String` reference that represents the user authorization token
/// * `edit_data` - `str` reference which will represent a JSON format to edit the tournament
pub async fn edit_tournament_request<'a>(
    client: &'a Client,
    tournament_id: u64,
    authorization_token: Option<&String>,
    edit_data: &str,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}", tournament_id);

    base_request_test(
        client,
        rocket::http::Method::Put,
        authorization_token.unwrap_or(&String::new()),
        route,
        edit_data,
    )
    .await
}

/// Responsible to make a request to the API to delete the tournament
///
///# Arguments
/// * `client` - `rocket::local::asynchronous::client`, Rocket client to make requests
/// * `tournament_id` - `u64` integer which represents the tournament id that's getting deleted
/// * `authorization_token` - Optional `String` that represents the user authorization token
pub async fn delete_tournament_request<'a>(
    client: &'a Client,
    tournament_id: u64,
    authorization_token: Option<&String>,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}", tournament_id);

    base_request_test(
        client,
        rocket::http::Method::Delete,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

/// Responsible to make a request to the API to get the tournament information
///
/// # Arguments
/// * `client` - `rocket::local::asynchronous::client`, Rocket client to make requests
/// * `tournament_id` - `u64` integer which represents the tournament id that's getting deleted
/// * `authorization_token` - Optional `String` that represents the user authorization token
pub async fn get_tournament_request<'a>(
    client: &'a Client,
    tournament_id: u64,
    authorization_token: Option<&String>,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}", tournament_id);

    base_request_test(
        client,
        rocket::http::Method::Get,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}
