use rocket::{http::Status, local::asynchronous::Client};

use crate::rocket;

#[rocket::async_test]
pub async fn test_hello() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client.get("/").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await.unwrap(), "Hello, world!");
}

#[rocket::async_test]
pub async fn test_non_existing_route() {
    let client = Client::tracked(rocket().await).await.unwrap();
    let response = client.get("/i-dont-exist").dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
}
