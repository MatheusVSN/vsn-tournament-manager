use rocket::local::asynchronous::{Client, LocalResponse};

use super::base_request_test;

pub async fn get_league_fixtures_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/fixtures",
        tournament_id, league_id
    );

    base_request_test(
        client,
        rocket::http::Method::Get,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

pub async fn generate_fixtures_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/fixtures",
        tournament_id, league_id
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

pub async fn get_fixture_by_id_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
    fixture_id: u64,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/fixtures/{}",
        tournament_id, league_id, fixture_id
    );

    base_request_test(
        client,
        rocket::http::Method::Get,
        authorization_token.unwrap_or(&String::new()),
        route,
        "",
    )
    .await
}

pub async fn delete_all_fixtures_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/fixtures",
        tournament_id, league_id
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

pub async fn edit_fixture_request<'a>(
    client: &'a Client,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
    fixture_id: u64,
    edit_data: &str,
) -> LocalResponse<'a> {
    let route = format!(
        "/tournament/{}/leagues/{}/fixtures/{}",
        tournament_id, league_id, fixture_id
    );

    base_request_test(
        client,
        rocket::http::Method::Put,
        authorization_token.unwrap_or(&String::new()),
        route,
        edit_data,
    )
    .await
}
