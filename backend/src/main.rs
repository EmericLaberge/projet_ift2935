mod config;
mod db;
mod models;
mod routes;
mod test;

use actix_cors::Cors;
use actix_web::{delete, get, post, put, web, App, HttpResponse, HttpServer, Responder};
use async_std::stream::StreamExt;
use config::get_config;
use db::connect_to_db;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use models::{staff, team, users, event, game};
use routes::team_routes::create_team;
use routes::game_routes::{create_game, delete_game, get_user_games, update_game};
use routes::{create_event, create_player, create_user, delete_event, delete_user, get_all_events, get_all_players, get_all_teams, get_all_users, get_event_by_id, get_games, get_user_by_id, get_user_events, get_user_teams, login, update_user};
use serde::{Deserialize, Serialize};
use staff::Staff;
use team::Team;
use test::insert_fake_users;
use tiberius::{self};
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use users::User;

use crate::models::{partial_user, Player};

/// This prevent passing the config around like before This is a struct that holds the config and the client
struct AppState {
    config: Config,
    client: Client<Compat<TcpStream>>,
}

impl AppState {
    async fn new() -> Result<Self, anyhow::Error> {
        let state = AppState {
            config: get_config().await?,
            client: connect_to_db(get_config().await?).await?,
        };
        Ok(state)
    }
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
}
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Try to connect to the database using db module (see db.rs)
    let config: Config = get_config().await?;
    let client = connect_to_db(config.clone()).await?;
    let app_state = web::Data::new(AppState { config, client });

    let config3: Config = get_config().await?;
    let mut client3 = connect_to_db(config3.clone()).await?;
    let query = "SELECT COUNT(*) AS [count] FROM Users";
    let result = client3.query(query, &[]).await?;

    let row = result.into_row().await;

    let row = row.unwrap();
    let user_count = row.unwrap().get(0);

    if user_count == Some(0) {
        // create 100 users
        let config2: Config = get_config().await?;
        let mut client2 = connect_to_db(config2.clone()).await?;
        insert_fake_users(&mut client2).await?;
    }

    let _run = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(cors)
            .app_data(app_state.clone())
            .service(get_user_by_id)
            .service(get_all_users)
            .service(get_all_teams)
            .service(get_user_teams)
            .service(create_user)
            .service(create_team)
            .service(login)
            .service(update_user)
            .service(delete_user)
            .service(get_all_players)
            .service(create_player)
            .service(get_user_events)
            .service(create_team) 
            .service(get_all_events)
            .service(get_event_by_id)
            .service(delete_event)
            .service(create_event)
            .service(get_games)
            .service(create_game)
            .service(update_game)
            .service(delete_game)
            .service(get_user_games)

    })
    .bind(("127.0.0.1", 6969))?
    .run()
    .await;

    Ok(())
} 

