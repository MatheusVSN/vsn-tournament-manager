use rocket::{
    http::Status,
    local::asynchronous::{Client},
};
use serde_json::json;

use crate::{
    models::{
        fixture::{Fixture, FixtureDataDTO},
    },
    rocket,
    test::{
        register_and_login,
        utilities::{
            fixture_utilities::{
                delete_all_fixtures_request, edit_fixture_request, generate_fixtures_request,
                get_fixture_by_id_request, get_league_fixtures_request,
            },
            league_utilities::{add_team_to_league_request, create_league_request},
            team_utilities::create_team_request,
            tournament_utilities::create_tournament_request,
        },
        APIResponse, IdData,
    },
};

async fn create_teams(
    client: &Client,
    quantity: u8,
    authorization_token: Option<&String>,
    tournament_id: u64,
    league_id: u64,
) {
    for number in 0..quantity {
        let team_name = format!("Team {}", number);
        let team_data = json!({
            "name": team_name
        })
        .to_string();

        let response = create_team_request(
            client,
            authorization_token,
            team_data.as_str(),
            tournament_id,
        )
        .await;

        assert_eq!(response.status(), Status::Created);
        let team_id = response
            .into_json::<APIResponse<IdData>>()
            .await
            .expect("Expected team id data")
            .data
            .id;

        // Adding the team to the league
        let _response = add_team_to_league_request(
            &client,
            authorization_token,
            tournament_id,
            league_id,
            team_id,
        )
        .await;
    }
}

// Creating a tournament and getting it's fixtures and validating the response body
#[rocket::async_test]
async fn get_league_fixtures_test() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "adding team",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;
    // Getting the league fixtures
    let response = get_league_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<Vec<Fixture>>>()
        .await
        .unwrap();
}

// Creating a league and generating it's fixtures
#[rocket::async_test]
async fn generating_fixtures() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "adding team",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        4,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Generating the fixtures
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Making sure the fixtures are created
    let response = get_league_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    let response_body = response
        .into_json::<APIResponse<Vec<FixtureDataDTO>>>()
        .await
        .unwrap();

    // Making sure there's fixtures
    assert!(response_body.data.len() > 0);
}

// Generating fixtures as an unauthorized user
#[rocket::async_test]
async fn generating_fixtures_unauthorized_user() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "adding team",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        4,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Trying to generate fixtures as an unauthorized user
    let response = generate_fixtures_request(&client, None, tournament_id, league_id).await;
    assert_ne!(response.status(), Status::Ok);
    // Logging as other user and trying to generate the fixtures
    let authorization_token = register_and_login(&client).await;
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_ne!(response.status(), Status::Ok);
}

// Generating and getting the fixtures
#[rocket::async_test]
async fn generating_and_getting_fixtures_test() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        4,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Making sure the fixtures are created
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Getting the league fixtures
    let response = get_league_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    let response_body = response
        .into_json::<APIResponse<Vec<FixtureDataDTO>>>()
        .await
        .unwrap()
        .data;

    let fixtures_ids: Vec<u64> = response_body
        .into_iter()
        .map(|fixture| fixture.id)
        .collect();

    // Validating each fixture
    for fixture_id in fixtures_ids {
        let response = get_fixture_by_id_request(
            &client,
            Some(&authorization_token),
            tournament_id,
            league_id,
            fixture_id,
        )
        .await;

        assert_eq!(response.status(), Status::Ok);
        response
            .into_json::<APIResponse<FixtureDataDTO>>()
            .await
            .unwrap();
    }
}

// Accessing a fixture from a private tournament as an unauthorized user
#[rocket::async_test]
async fn accessing_private_fixture() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "tournament",
        "public": false
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        2,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Making sure the fixtures are created
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Getting the league fixtures
    let response = get_league_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    let response_body = response
        .into_json::<APIResponse<Vec<FixtureDataDTO>>>()
        .await
        .unwrap()
        .data;

    let fixtures_ids: Vec<u64> = response_body
        .into_iter()
        .map(|fixture| fixture.id)
        .collect();

    // Checking if a unauthorized user can access it
    for fixture_id in &fixtures_ids {
        let response =
            get_fixture_by_id_request(&client, None, tournament_id, league_id, fixture_id.clone())
                .await;

        assert_ne!(response.status(), Status::Ok);
    }

    // Checking if other use can access it
    let authorization_token = register_and_login(&client).await;
    for fixture_id in &fixtures_ids {
        let response = get_fixture_by_id_request(
            &client,
            Some(&authorization_token),
            tournament_id,
            league_id,
            fixture_id.clone(),
        )
        .await;

        assert_ne!(response.status(), Status::Ok);
    }
}

// Deleting all fixtures from a league
#[rocket::async_test]
async fn delete_all_fixtures_from_league() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        2,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Generating fixtures
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Deleting the fixtures
    let response = delete_all_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
}

// Unauthorized deleting all fixtures from a league
#[rocket::async_test]
async fn delete_all_fixtures_from_league_unauthorized_user() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        2,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Generating fixtures
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Deleting the fixtures
    let response = delete_all_fixtures_request(&client, None, tournament_id, league_id).await;
    assert_ne!(response.status(), Status::Ok);

    // Logging as other user and trying to delete the fixtures
    let authorization_token = register_and_login(&client).await;
    let response = delete_all_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_ne!(response.status(), Status::Ok);
}

// Editing fixtures
#[rocket::async_test]
async fn editing_fixtures() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        2,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Generating fixtures
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Getting the league fixtures
    let response = get_league_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    let response_body = response
        .into_json::<APIResponse<Vec<FixtureDataDTO>>>()
        .await
        .unwrap()
        .data;

    let fixtures_ids: Vec<u64> = response_body
        .into_iter()
        .map(|fixture| fixture.id)
        .collect();

    // Editing each fixture
    for fixture_id in fixtures_ids {
        let edit_data = r#"{
            "home_score": 1,
            "away_score": 2,
            "played": true
        }"#;

        let response = edit_fixture_request(
            &client,
            Some(&authorization_token),
            tournament_id,
            league_id,
            fixture_id,
            edit_data,
        )
        .await;

        assert_eq!(response.status(), Status::Ok);
    }
}

// Editing fixtures as an unauthorized user
#[rocket::async_test]
async fn editing_fixtures_unauthorized_user() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;
    // Creating a tournament
    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    // Getting the tournament id
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;
    // Creating a league
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    // Getting the league id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // Creating teams
    create_teams(
        &client,
        2,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;

    // Generating fixtures
    let response = generate_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Getting the league fixtures
    let response = get_league_fixtures_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    let response_body = response
        .into_json::<APIResponse<Vec<FixtureDataDTO>>>()
        .await
        .unwrap()
        .data;

    let fixtures_ids: Vec<u64> = response_body
        .into_iter()
        .map(|fixture| fixture.id)
        .collect();

    // Editing each fixture
    for fixture_id in fixtures_ids {
        let edit_data = r#"{
            "home_score": 1,
            "away_score": 2,
            "played": true
        }"#;

        let response = edit_fixture_request(
            &client,
            None,
            tournament_id,
            league_id,
            fixture_id,
            edit_data,
        )
        .await;

        assert_ne!(response.status(), Status::Ok);

        // Logging as other user and trying to edit the fixtures
        let authorization_token = register_and_login(&client).await;
        let response = edit_fixture_request(
            &client,
            Some(&authorization_token),
            tournament_id,
            league_id,
            fixture_id,
            edit_data,
        )
        .await;

        assert_ne!(response.status(), Status::Ok);
    }
}
