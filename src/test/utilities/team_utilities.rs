use rocket::local::asynchronous::{Client, LocalResponse};

use super::base_request_test;

pub async fn get_team_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    team_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/teams/{}", tournament_id, team_id);

    base_request_test(
        client,
        rocket::http::Method::Get,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

pub async fn create_team_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    team_data: &str,
    tournament_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/teams", tournament_id);

    base_request_test(
        client,
        rocket::http::Method::Post,
        authorization_token.unwrap_or(&String::new()),
        route,
        team_data,
    )
    .await
}

pub async fn edit_team_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    edit_data: &str,
    tournament_id: u64,
    team_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/teams/{}", tournament_id, team_id);

    base_request_test(
        client,
        rocket::http::Method::Put,
        authorization_token.unwrap_or(&String::new()),
        route,
        edit_data,
    )
    .await
}

pub async fn delete_team_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    team_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/teams/{}", tournament_id, team_id);

    base_request_test(
        client,
        rocket::http::Method::Delete,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}
