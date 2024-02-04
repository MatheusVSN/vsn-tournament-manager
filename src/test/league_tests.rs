use rocket::{http::Status, local::asynchronous::Client};

use crate::{
    models::league::{LeagueRegisterDTO, TeamStandingTable},
    rocket,
    test::{
        utilities::{
            league_utilities::{
                add_team_to_league_request, create_league_request, delete_league_request,
                get_league_request, get_league_standings_table_request,
                remove_team_from_league_request,
            },
            team_utilities::create_team_request,
            tournament_utilities::create_tournament_request,
        },
        APIResponse, IdData,
    },
};

use super::{register_and_login, utilities::league_utilities::edit_league_request};

// Creating a new league test
#[rocket::async_test]
async fn create_league_test() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "league test",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a league for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    // Creating a new league and validating it's response body
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response_body = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response_body.status(), Status::Created);
    response_body
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected league id data");
}

// Creating a new league as an unauthenticated user
#[rocket::async_test]
async fn unauthorized_create_league() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "league test",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a league for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    // Logging in as another user
    let authorization_token = register_and_login(&client).await;
    // Creating a new league and validating it's response body
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response_body = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    // Should not be created because the user who's trying to create the new league does not own the tournament
    assert_ne!(response_body.status(), Status::Created);

    // Testing with a invalid authentication user token
    let response_body = create_league_request(&client, None, league_data, tournament_id).await;
    // Should not be created because the authorization token is invalid
    // And it's also not the user who created the tournament
    assert_ne!(response_body.status(), Status::Created);
}

// Testing the league editing
#[rocket::async_test]
async fn edit_league_authorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "edit league",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a league for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    // Creating a new league and validating it's response body
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response_body = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response_body.status(), Status::Created);

    let response = response_body
        .into_json::<APIResponse<IdData>>()
        .await
        .unwrap();
    let league_id = response.data.id;

    // Editing the league
    let edit_data = r#"{
            "name": "edited",
            "completed": true
        }"#;
    let response = edit_league_request(
        &client,
        Some(&authorization_token),
        edit_data,
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok)
}

// Testing the league editing as an unauthorized user
#[rocket::async_test]
async fn edit_league_unauthorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "edit league",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a league for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    // Creating a new league and validating it's response body
    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response_body = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;
    assert_eq!(response_body.status(), Status::Created);

    let response = response_body
        .into_json::<APIResponse<IdData>>()
        .await
        .unwrap();
    let league_id = response.data.id;

    // Logging in as another user
    let authorization_token = register_and_login(&client).await;
    let edit_data = r#"{
        "name": "edited",
        "completed": true
    }"#;

    let league_response = edit_league_request(
        &client,
        Some(&authorization_token),
        edit_data,
        tournament_id,
        league_id,
    )
    .await;
    // Should not be ok as the user who's doing the action have not created the tournament
    assert_ne!(league_response.status(), Status::Ok);

    // Doing the same thing but without a user
    let league_response =
        edit_league_request(&client, None, edit_data, tournament_id, league_id).await;
    assert_ne!(league_response.status(), Status::Ok);
}

// Testing the delete league as an authorized user
#[rocket::async_test]
async fn delete_league_authorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "delete league",
        "public": false
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response_body = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;

    assert_eq!(response_body.status(), Status::Created);

    let response = response_body
        .into_json::<APIResponse<IdData>>()
        .await
        .unwrap();
    let league_id = response.data.id;

    let response = delete_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::NoContent);
}

// Testing if an unauthorized user can delete the tournament
#[rocket::async_test]
async fn delete_league_unauthorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "delete league",
        "public": false
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    let league_data = r#"{
            "name": "new league",
            "completed": false
        }"#;
    let response_body = create_league_request(
        &client,
        Some(&authorization_token),
        league_data,
        tournament_id,
    )
    .await;

    assert_eq!(response_body.status(), Status::Created);

    let response = response_body
        .into_json::<APIResponse<IdData>>()
        .await
        .unwrap();
    let league_id = response.data.id;

    // Both of these conditions make sure unauthorized users cannot delete the league as they don't have permissions to do it
    // Testing if an unauthorized user can delete
    let response = delete_league_request(&client, None, tournament_id, league_id).await;
    assert_ne!(response.status(), Status::NoContent);

    // Testing if other user can delete the league
    let authorization_token = register_and_login(&client).await;
    let response = delete_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_ne!(response.status(), Status::NoContent);
}

// Accessing public league
#[rocket::async_test]
async fn accessing_public_league() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "access league",
        "public": true
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

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

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // These three verification will cover if everyone can access it
    // They all should, because the tournament is public

    // Verifying if the tournament creator can access
    let response = get_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<LeagueRegisterDTO>>()
        .await
        .unwrap();

    // Verifying if other user can access it
    let authorization_token = register_and_login(&client).await;
    let response = get_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<LeagueRegisterDTO>>()
        .await
        .unwrap();

    // Verifying if a unauthorized user can access
    let response = get_league_request(&client, None, tournament_id, league_id).await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<LeagueRegisterDTO>>()
        .await
        .unwrap();
}

// Unauthorized league access
#[rocket::async_test]
async fn unauthorized_league_access() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "access league",
        "public": false
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

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

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let league_id = response_body.data.id;

    // These three verification will cover if everyone can access it
    // They all should not, because the tournament is not public

    // Verifying if the tournament creator can access
    let response = get_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<LeagueRegisterDTO>>()
        .await
        .unwrap();

    // Verifying if other user can access it
    let authorization_token = register_and_login(&client).await;
    let response = get_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_ne!(response.status(), Status::Ok);

    // Verifying if a unauthorized/non-logged user can access
    let response = get_league_request(&client, None, tournament_id, league_id).await;
    assert_ne!(response.status(), Status::Ok);
}

// Authorized adding team to league
#[rocket::async_test]
async fn authorized_add_team_to_league_test() {
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

    // Creating a team
    let team_data = r#"{
            "name": "new team"
        }"#;
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    assert_eq!(response.status(), Status::Created);

    // Getting the team id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Adding the team to the league
    let response = add_team_to_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
        team_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
}

// Unauthorized adding team to league
#[rocket::async_test]
async fn unauthorized_add_team_to_league() {
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

    // Creating a team
    let team_data = r#"{
            "name": "new team"
        }"#;
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    assert_eq!(response.status(), Status::Created);

    // Getting the team id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Logging in as another user
    let authorization_token = register_and_login(&client).await;

    // Adding the team to the league
    let response = add_team_to_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
        team_id,
    )
    .await;
    assert_ne!(response.status(), Status::Ok);

    // Testing with a invalid authentication token
    let response =
        add_team_to_league_request(&client, None, tournament_id, league_id, team_id).await;
    assert_ne!(response.status(), Status::Ok);
}

// Testing if a team can be added to a league that does not exist
#[rocket::async_test]
async fn add_team_to_non_existent_league() {
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

    // Creating a team
    let team_data = r#"{
            "name": "new team"
        }"#;
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    assert_eq!(response.status(), Status::Created);

    // Getting the team id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Adding the team to the league
    let response = add_team_to_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        999999999999999999,
        team_id,
    )
    .await;
    assert_ne!(response.status(), Status::Ok);
}

// Creating and removing a team from a league
#[rocket::async_test]
async fn remove_team_from_league_test() {
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

    // Creating a team
    let team_data = r#"{
            "name": "new team"
        }"#;
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    assert_eq!(response.status(), Status::Created);

    // Getting the team id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Adding the team to the league
    let response = add_team_to_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
        team_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Removing the team from the league
    let response = remove_team_from_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
        team_id,
    )
    .await;
    assert_eq!(response.status(), Status::NoContent);
}

// Testing if a unauthorized user can add a team to a league
#[rocket::async_test]
async fn unauthorized_remove_team_from_league_test() {
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

    // Creating a team
    let team_data = r#"{
            "name": "new team"
        }"#;
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    assert_eq!(response.status(), Status::Created);

    // Getting the team id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Logging in as another user
    let authorization_token = register_and_login(&client).await;

    // Seeing if an unauthorized user can remove the team from the league
    let response = remove_team_from_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
        team_id,
    )
    .await;
    assert_ne!(response.status(), Status::NoContent);

    // Testing with a invalid authentication token
    let response =
        remove_team_from_league_request(&client, None, tournament_id, league_id, team_id).await;
    assert_ne!(response.status(), Status::NoContent);
}

// Testing if a team can be removed from another league which is not related to his tournament
#[rocket::async_test]
async fn remove_team_from_non_existent_league_test() {
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

    // Creating a team
    let team_data = r#"{
            "name": "new team"
        }"#;
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    assert_eq!(response.status(), Status::Created);

    // Getting the team id
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Removing the team from the league
    let response = remove_team_from_league_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        999999999999999999,
        team_id,
    )
    .await;
    assert_ne!(response.status(), Status::NoContent);
}

// Creating a tournament and getting it's standing table
// Doesn't really matter if the tournament is public or not
// As the database query logic will return empty if happens an unauthorized access
#[rocket::async_test]
async fn get_league_standing_table_test() {
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
    // Getting the league standing table
    let response = get_league_standings_table_request(
        &client,
        Some(&authorization_token),
        tournament_id,
        league_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<Vec<TeamStandingTable>>>()
        .await
        .unwrap();
}

// Creating more than 24 teams
// Maximum teams a league can have is 24
#[rocket::async_test]
async fn creating_more_than_24_teams() {
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
    // Creating 25 teams
    for count in 0..25 {
        let team_data = r#"{
            "name": "new team"
        }"#;
        let response = create_team_request(
            &client,
            Some(&authorization_token),
            team_data,
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
        let response = add_team_to_league_request(
            &client,
            Some(&authorization_token),
            tournament_id,
            league_id,
            team_id,
        )
        .await;

        if count < 23 {
            assert_eq!(response.status(), Status::Ok);
        } else {
            // Should not be allowed because the league team limit is 24
            assert_ne!(response.status(), Status::Ok);
        }
    }
}
