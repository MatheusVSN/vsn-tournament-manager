use rocket::local::asynchronous::{Client, LocalResponse};

use super::base_request_test;

pub async fn get_league_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/leagues/{}", tournament_id, league_id);

    base_request_test(
        client,
        rocket::http::Method::Get,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

pub async fn create_league_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    league_data: &str,
    tournament_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/leagues", tournament_id);

    base_request_test(
        client,
        rocket::http::Method::Post,
        authorization_token.unwrap_or(&String::new()),
        route,
        league_data,
    )
    .await
}

pub async fn add_team_to_league_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
    team_id: u64,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/teams/{}",
        tournament_id, league_id, team_id
    );

    base_request_test(
        client,
        rocket::http::Method::Post,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

pub async fn edit_league_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    edit_data: &str,
    tournament_id: u64,
    league_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/leagues/{}", tournament_id, league_id);

    base_request_test(
        client,
        rocket::http::Method::Put,
        authorization_token.unwrap_or(&String::new()),
        route,
        edit_data,
    )
    .await
}

pub async fn delete_league_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/leagues/{}", tournament_id, league_id);

    base_request_test(
        client,
        rocket::http::Method::Delete,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

pub async fn remove_team_from_league_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
    team_id: u64,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/teams/{}",
        tournament_id, league_id, team_id
    );

    base_request_test(
        client,
        rocket::http::Method::Delete,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}



pub async fn get_league_standings_table_request<'a>(client: &'a Client, authorization_token: Option<&String>, tournament_id: u64, league_id: u64) -> LocalResponse<'a> {
    let route = format!("/tournament/{}/leagues/{}/standing-table", tournament_id, league_id);

    base_request_test(
        client,
        rocket::http::Method::Get,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}