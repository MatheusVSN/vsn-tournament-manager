use rocket::{
    http::{ContentType, Header, Method},
    local::asynchronous::{Client, LocalResponse},
};

async fn base_request_test<'a>(
    client: &'a Client,
    method: Method,
    authorization_token: &String,
    route: String,
    body: &str,
) -> LocalResponse<'a> {
    let mut request = match method {
        Method::Get => client.get(route),
        Method::Post => client.post(route),
        Method::Put => client.put(route),
        Method::Delete => client.delete(route),
        _ => panic!("Invalid method"),
    };

    request = request
        .header(ContentType::JSON)
        .header(Header::new(
            "Authorization",
            format!("Bearer {}", authorization_token),
        ))
        .body(body);

    request.dispatch().await
}

pub mod league_utilities;
pub mod team_utilities;
pub mod tournament_utilities;
pub mod fixture_utilities;