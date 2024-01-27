use rocket::http::Status;
use rocket::local::asynchronous::Client;

use crate::{
    models::team::TeamInfoDTO,
    rocket,
    test::{
        register_and_login,
        utilities::team_utilities::{
            create_team_request, delete_team_request, edit_team_request, get_team_request,
        },
        APIResponse, IdData,
    },
};

use super::utilities::tournament_utilities::create_tournament_request;

/// Testing creating a valid team on a tournament
#[rocket::async_test]
async fn create_team_test() {
    let client = Client::tracked(rocket().await).await.unwrap();

    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "test",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a team for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "new team"
    }"#;

    // Creating a team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected team id data");
}

/// Testing if an unauthorized/unauthenticated user can create a team on a tournament that he does not owns it
#[rocket::async_test]
async fn unauthenticated_create_team() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "unauth team test",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a team for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Expected tournament id data");
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "new team"
    }"#;

    // Creating a as unauthenticated user
    let response = create_team_request(&client, None, team_data, tournament_id).await;
    // Should not be created because it's a unauthenticated request
    assert_ne!(response.status(), Status::Created);

    let other_user_authorization_token = register_and_login(&client).await;

    // Creating a as unauthenticated user
    let response = create_team_request(
        &client,
        Some(&other_user_authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    // Should not be created because this user does not created the tournament, meaning he should not have permissions to do it
    assert_ne!(response.status(), Status::Created);
}

/// This should not be allowed because the character limit for the team name is 40
#[rocket::async_test]
async fn more_than_40_chars_create_team() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    // Sending the request  to create the tournament so we can create a team for it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "one two three four five this should be longer than 40 characters"
    }"#;

    // Sending the request to create a team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;

    // Should not be allowed as the name have more than 40 characters
    assert_eq!(response.status(), Status::BadRequest);
}

/// Creating and editing team with a valid authenticated user
#[rocket::async_test]
async fn creating_editing_team_test() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    // Sending the request to create the tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "team name"
    }"#;

    // Sending the request to create the team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    let edit_data = r#"{
        "name": "edited"
    }"#;

    // Sending the request to edit the team
    let response = edit_team_request(
        &client,
        Some(&authorization_token),
        edit_data,
        tournament_id,
        team_id,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
}

// Editing a team as other user which have not created the tournament
#[rocket::async_test]
async fn editing_team_unauthorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    // Sending the request to create the tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "team name"
    }"#;

    // Sending the request to create the team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    let other_authorization_token = register_and_login(&client).await;

    let edit_data = r#"{
        "name": "should not be allowed"
    }"#;

    // Sending the request to edit the team
    let response = edit_team_request(
        &client,
        Some(&other_authorization_token),
        edit_data,
        tournament_id,
        team_id,
    )
    .await;

    // Should not be allowed because the user have not created the tournament
    assert_eq!(response.status(), Status::BadRequest);
}

// Deleting a team with a valid user
#[rocket::async_test]
async fn delete_team_authorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    // Sending the request to create the tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "deleting authorized"
    }"#;

    // Sending the request to create the team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Sending the request to delete the team
    let response =
        delete_team_request(&client, Some(&authorization_token), tournament_id, team_id).await;
    assert_eq!(response.status(), Status::NoContent);
}

// Deleting a team with an unauthorized user
#[rocket::async_test]
async fn delete_team_unauthorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "tournament",
        "public": true
    }"#;

    // Sending the request to create the tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "deleting unauthorized"
    }"#;

    // Sending the request to create the team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Creating other user
    let authorization_token = register_and_login(&client).await;

    // Sending the request to delete the team
    let response =
        delete_team_request(&client, Some(&authorization_token), tournament_id, team_id).await;
    // Should not be allowed as the user did not created the tournament
    assert_eq!(response.status(), Status::BadRequest);
}

// Creating a team and accessing it(authenticated/unauthenticated user) on a public tournament
#[rocket::async_test]
async fn get_team_authorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "team access test",
        "public": true
    }"#;

    // Sending the request to create the tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "team get"
    }"#;

    // Sending the request to create the team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Getting the team as the user who also created it
    let response =
        get_team_request(&client, Some(&authorization_token), tournament_id, team_id).await;
    // Validating if the response and it's body is correct
    assert_eq!(response.status(), Status::Ok);
    let _ = response
        .into_json::<APIResponse<TeamInfoDTO>>()
        .await
        .unwrap();

    // Getting the team as other user, it should return the team later on as the tournament is public
    let response = get_team_request(&client, Some(&String::new()), tournament_id, team_id).await;
    // Validating the response
    assert_eq!(response.status(), Status::Ok);
    let _ = response
        .into_json::<APIResponse<TeamInfoDTO>>()
        .await
        .unwrap();
}

// Creating a private tournament, a team and trying to access it as unauthorized user
#[rocket::async_test]
async fn get_team_unauthorized() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "private tournament",
        "public": false
    }"#;

    // Sending the request to create the tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;
    assert_eq!(response.status(), Status::Created);

    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let tournament_id = response_body.data.id;

    let team_data = r#"{
        "name": "team get"
    }"#;

    // Sending the request to create the team
    let response = create_team_request(
        &client,
        Some(&authorization_token),
        team_data,
        tournament_id,
    )
    .await;
    assert_eq!(response.status(), Status::Created);
    let response_body = response.into_json::<APIResponse<IdData>>().await.unwrap();
    let team_id = response_body.data.id;

    // Getting the team information as a authenticated user and validating if it returns a correct data
    let response =
        get_team_request(&client, Some(&authorization_token), tournament_id, team_id).await;
    assert_eq!(response.status(), Status::Ok);
    let _ = response.into_json::<APIResponse<TeamInfoDTO>>();

    // Testing the case where it's a unauthenticated user
    let authorization_token = register_and_login(&client).await;

    // Should not be allowed as the tournament is private and the user we created do not have permissions to access it
    let response =
        get_team_request(&client, Some(&authorization_token), tournament_id, team_id).await;
    assert_eq!(response.status(), Status::BadRequest);
}
