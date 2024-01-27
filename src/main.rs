use dotenvy::dotenv;
use sqlx::MySqlPool;
use std::env;

use controllers::{
    authentication_controller::{login, register},
    fixture_controller::{
        delete_fixtures_from_league, edit_fixture, generate_fixtures, get_fixture_by_id,
        get_league_fixtures,
    },
    league_controller::{
        create_new_league, delete_league, edit_league, get_league, get_league_standing_table,
        league_add_team, league_remove_team,
    },
    team_controller::{create_team, delete_team, edit_team, get_team},
    tournament_controller::{
        create_tournament, delete_tournament, edit_tournament, get_tournament,
    },
};

#[macro_use]
extern crate rocket;

mod constants;
mod controllers;
mod jwt_auth_handler;
mod models;
mod responses;
mod services;
#[cfg(test)]
mod test;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL missing on env variables");
    let pool = MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    rocket::build()
        .mount("/", routes![index])
        .mount("/authentication", routes![register, login])
        // Tournaments
        .mount(
            "/tournament",
            routes![
                create_tournament,
                delete_tournament,
                edit_tournament,
                get_tournament
            ],
        )
        // Teams
        .mount(
            "/tournament",
            routes![create_team, edit_team, delete_team, get_team],
        )
        // Leagues
        .mount(
            "/tournament",
            routes![
                create_new_league,
                edit_league,
                delete_league,
                get_league,
                league_add_team,
                league_remove_team,
                get_league_standing_table
            ],
        )
        // Fixtures
        .mount(
            "/tournament",
            routes![
                get_league_fixtures,
                generate_fixtures,
                get_fixture_by_id,
                delete_fixtures_from_league,
                edit_fixture
            ],
        )
        .manage::<MySqlPool>(pool)
}
