use super::{
    utilities::tournament_utilities::{
        create_tournament_request, delete_tournament_request, edit_tournament_request,
        get_tournament_request,
    },
    APIResponse, IdData,
};
use crate::{models::tournament::TournamentInformationData, rocket, test::register_and_login};
use rocket::{
    http::{ContentType, Status},
    local::asynchronous::Client,
};

/// Creating a tournament with a authenticated user
/// should work without an exception
#[rocket::async_test]
async fn creating_a_new_tournament() {
    let client = Client::tracked(rocket().await).await.unwrap();

    // Creating a fake user
    let authorization_token = register_and_login(&client).await;

    // Tournament data
    let request_body = r#"
    {
        "name": "generated_tournament",
        "public": false
    }
    "#;

    // Sending a request to create a new tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), request_body).await;

    // Status should return as Created
    assert_eq!(response.status(), Status::Created);
}

/// Creating a tournament without a authenticated user
/// the tournament should not be created as there's no authenticated user
#[rocket::async_test]
async fn unauthenticated_creating_tournament() {
    let client = Client::tracked(rocket().await).await.unwrap();

    // Tournament data
    let request_body = r#"
    {
        "name": "generated_tournament",
        "public": false
    }
    "#;

    // Sending a request to create a new tournament
    let response = client
        .post("/tournament")
        .header(ContentType::JSON)
        .body(request_body)
        .dispatch()
        .await;

    // Status should return as Unauthorized as there's no token on the Authorization
    assert_eq!(response.status(), Status::Unauthorized);

    // Testing also with a invalid jwt token
    let response = create_tournament_request(
        &client,
        Some(&String::from("totally_valid_jwt_token")),
        request_body,
    )
    .await;

    // Status should return as Unauthorized as the token is invalid
    assert_eq!(response.status(), Status::Unauthorized);
}

/// Creating and editing a tournament
/// This should work as it's the same user who created the tournament that's editing it
#[rocket::async_test]
async fn authenticated_edit_tournament() {
    let client = Client::tracked(rocket().await).await.unwrap();

    // Creating a fake user
    let authorization_token = register_and_login(&client).await;

    // Tournament data
    let request_body = r#"
    {
        "name": "generated_tournament",
        "public": false
    }
    "#;

    // Sending a request to create a new tournament
    let response =
        create_tournament_request(&client, Some(&authorization_token), request_body).await;

    assert_eq!(response.status(), Status::Created);

    // Getting the response body and the new created tournament ID
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Failed to parse the new tournament ID data response ");
    let created_tournament_id = response_body.data.id;

    // Edit data
    let edit_request_body = r#"
    {
        "name": "edited",
        "public": false
    }
    "#;

    // Sending the edit request
    let response = edit_tournament_request(
        &client,
        created_tournament_id,
        Some(&authorization_token),
        edit_request_body,
    )
    .await;

    // Should be OK as the request body is correct and who's editing is the user who created the tournament
    assert_eq!(response.status(), Status::Ok);
}

/// Editing and deleting a tournament which a user does not owns it
/// This should not allow other user to edit or delete a tournament which he doesn't created it
#[rocket::async_test]
async fn unauthorized_tournament_actions() {
    let client = Client::tracked(rocket().await).await.unwrap();

    // Creating fake users and
    let owner_authorization_token = register_and_login(&client).await;
    let other_user_authorization_token = register_and_login(&client).await;

    // Creating a new tournament
    let new_tournament_data = r#"
    {
        "name": "my tournament",
        "public": false
    }
    "#;

    // Sending the creation request
    let response = create_tournament_request(
        &client,
        Some(&owner_authorization_token),
        new_tournament_data,
    )
    .await;

    // Checking if it's the correct status and getting the request body so we can use the id to make the test
    assert_eq!(response.status(), Status::Created);
    let request_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Failed to parse the new tournament ID data response ");
    let tournament_id = request_body.data.id;

    // Trying to edit the tournament with the other user
    let edit_tournament_data = r#"
    {
        "name": "edited",
        "public"": true
    }
    "#;

    // Sending the request to the API
    let response = edit_tournament_request(
        &client,
        tournament_id,
        Some(&other_user_authorization_token),
        edit_tournament_data,
    )
    .await;

    // The response cant be OK as other user is trying to edit the tournament
    assert_ne!(response.status(), Status::Ok, "This should not be ok");

    // Sending the request but now to delete it
    let response = delete_tournament_request(
        &client,
        tournament_id,
        Some(&other_user_authorization_token),
    )
    .await;

    // Response must be bad request as the user does not owns tournament
    assert_eq!(response.status(), Status::BadRequest)
}

/// Makes sure the tournament creation makes sure that all necessary fields exists before doing anything with the data.
#[rocket::async_test]
async fn creating_tournament_missing_field() {
    let client = Client::tracked(rocket().await).await.unwrap();

    let authorization_token = register_and_login(&client).await;

    // Simulating a request with a missing name field
    let missing_name_field = r#"
    {
        "public": true
    }
    "#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), missing_name_field).await;

    // Should not be Created as there's missing fields
    assert!(response.status() != Status::Created);
}

/// Making sure the changes cannot be made to a tournament if the edit body is empty
#[rocket::async_test]
async fn editing_tournament_missing_field() {
    let client = Client::tracked(rocket().await).await.unwrap();

    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"
    {
        "name": "my_tournament",
        "public": true
    }
    "#;
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;

    // Making sure it's created successfully
    assert_eq!(response.status(), Status::Created);

    // Getting the response body and the tournament_id
    let request_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Failed to parse the new tournament ID data response ");
    let tournament_id = request_body.data.id;
    let edit_empty_field = "";

    let response = edit_tournament_request(
        &client,
        tournament_id,
        Some(&authorization_token),
        edit_empty_field,
    )
    .await;

    // Edit fields cannot be empty
    assert_eq!(response.status(), Status::BadRequest);
}

/// Making sure that the tournament can be edited even if it's only 1 field
#[rocket::async_test]
async fn editing_with_missing_field() {
    let client = Client::tracked(rocket().await).await.unwrap();

    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"
    {
        "name": "my_tournament",
        "public": true
    }
    "#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;

    // Making sure it's created successfully
    assert_eq!(response.status(), Status::Created);

    // Getting the response body and the tournament_id
    let request_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Failed to parse the new tournament ID data response ");

    let tournament_id = request_body.data.id;
    let edit_empty_field = r#"
    {
        "name": "edited"
    }
    "#;

    let response = edit_tournament_request(
        &client,
        tournament_id,
        Some(&authorization_token),
        edit_empty_field,
    )
    .await;

    // Should be ok
    // As not every field is required to edit a tournament
    assert_eq!(response.status(), Status::Ok);
}

/// Creating a tournament with a user and accessing it as it
/// We'll also edit and make the public to "true" and make sure it does have the same behavior
#[rocket::async_test]
async fn creating_and_accessing_tournament() {
    let client = Client::tracked(rocket().await).await.unwrap();

    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "my_tournament",
        "public": false
    }"#;

    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;

    assert_eq!(response.status(), Status::Created);

    // Getting the response body and the tournament_id so we can access it
    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Failed to parse the new tournament ID data response ");

    let tournament_id = response_body.data.id;
    let response = get_tournament_request(&client, tournament_id, Some(&authorization_token)).await;

    assert_eq!(response.status(), Status::Ok);

    // Getting the tournament data
    response
        .into_json::<APIResponse<TournamentInformationData>>()
        .await
        .expect("Failed to parse the response to TournamentInformationData");

    // Editing the tournament to public so we can check if we can access it
    let edit_data = r#"{
            "public": true
        }"#;

    let response = edit_tournament_request(
        &client,
        tournament_id,
        Some(&authorization_token),
        edit_data,
    )
    .await;

    assert_eq!(response.status(), Status::Ok);

    // Getting the tournament data and see if we can access it again
    let response = get_tournament_request(&client, tournament_id, Some(&authorization_token)).await;

    // Making sure the response data is correct
    response
        .into_json::<APIResponse<TournamentInformationData>>()
        .await
        .expect("Failed to parse the response to TournamentInformationData");
}

/// Creating a tournament a tournament and accessing with other user and a unauthenticated one
#[rocket::async_test]
async fn accessing_tournament_as_other_user() {
    let client = Client::tracked(rocket().await).await.unwrap();

    let authorization_token = register_and_login(&client).await;

    let tournament_data = r#"{
        "name": "my_tournament",
        "public": true
    }"#;

    // Creating a tournament normally and checking if others users/unauthenticated users can access it
    let response =
        create_tournament_request(&client, Some(&authorization_token), tournament_data).await;

    let response_body = response
        .into_json::<APIResponse<IdData>>()
        .await
        .expect("Failed to parse the new tournament ID data response ");

    let tournament_id = response_body.data.id;

    // Trying to access the public tournament as an unauthenticated user
    let response = get_tournament_request(&client, tournament_id, None).await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<TournamentInformationData>>()
        .await
        .expect("Failed to parse the response to TournamentInformationData");

    let other_user_authorization_token = register_and_login(&client).await;

    // Trying to access the public tournament as an user but that doesn't created it
    let response = get_tournament_request(
        &client,
        tournament_id,
        Some(&other_user_authorization_token),
    )
    .await;
    assert_eq!(response.status(), Status::Ok);
    response
        .into_json::<APIResponse<TournamentInformationData>>()
        .await
        .expect("Failed to parse the response to TournamentInformationData");

    // Editing the tournament to false so we can confirm other users cannot access it
    let edit_data = r#"{
        "public": false
    }"#;

    let response = edit_tournament_request(
        &client,
        tournament_id,
        Some(&authorization_token),
        edit_data,
    )
    .await;
    assert_eq!(response.status(), Status::Ok);

    // Trying to access the private tournament as an unauthenticated user. This should not be allowed
    let response = get_tournament_request(&client, tournament_id, None).await;
    assert_eq!(response.status(), Status::BadRequest);

    // Same thing as above but now with an authenticated user
    let response = get_tournament_request(
        &client,
        tournament_id,
        Some(&other_user_authorization_token),
    )
    .await;
    assert_eq!(response.status(), Status::BadRequest);
}
