mod config;
mod db;
mod models;
mod test;
use crate::models::{staff, team, users};
use crate::test::{insert_fake_staff, insert_fake_teams, insert_fake_users};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use config::get_config;
use db::connect_to_db;
use staff::Staff;
use team::Team;
use tiberius;
use users::User;

use tiberius::{Client, Config};
use tokio_util::{compat::Compat, compat::TokioAsyncWriteCompatExt};

#[post("/create_user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    // make a query to the database to insert the user
    // return the user as a json response
    dbg!(&user);
    let config:Config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let (query, params) = user.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await.unwrap();
    HttpResponse::Ok().body(format!("{:?}", result))
}

#[post("/create_team")]
async fn create_team(team: web::Json<Team>) -> impl Responder {
    // make a query to the database to insert the user
    // return the user as a json response
    let config:Config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let (query, params) = team.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await.unwrap();
    HttpResponse::Ok().body(format!("{:?}", result))
}

#[get("/users/{id}")]
async fn get_user_by_id(id: web::Path<i32>) -> impl Responder {
    // make a query to the database to get the user with the id
    // return the user as a json response
    let config: Config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let query = "SELECT * FROM users WHERE id = @P1";
    let id = id.into_inner();
    let result = client.query(query, &[&id]).await.unwrap();
    let row = result.into_first_result().await.unwrap();

    let mut user_list: Vec<User> = Vec::new();
    for row in row {
        let user = User::new(
            row.get(0),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get::<&str, usize>(2).unwrap().to_owned(),
            row.get::<&str, usize>(3).unwrap().to_owned(),
            row.get::<&str, usize>(4).unwrap().to_owned(),
        );
        user_list.push(user);
    }

    HttpResponse::Ok().body(format!("{:?}", user_list))
}

#[get("/users")]
async fn get_all_users() -> impl Responder {
    // make a query to the database to get all the users
    // return the users as a json response
    let config: tiberius::Config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let query = "SELECT * FROM users";
    let result = client.query(query, &[]).await.unwrap();
    let row = result.into_first_result().await.unwrap();

    let mut user_list: Vec<User> = Vec::new();
    for row in row {
        let user = User::new(
            row.get(0),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get::<&str, usize>(2).unwrap().to_owned(),
            row.get::<&str, usize>(3).unwrap().to_owned(),
            row.get::<&str, usize>(4).unwrap().to_owned(),
        );
        user_list.push(user);
    }
    println!("This is the list of all users");

    HttpResponse::Ok().body(format!("{:?}", user_list))
}

#[get("/teams")]
async fn get_all_teams() -> impl Responder {
    // make a query to the database to get all the teams
    // return the teams as a json response
    let config: tiberius::Config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let query = "SELECT * FROM teams";
    let result = client.query(query, &[]).await.unwrap();
    let row = result.into_first_result().await.unwrap();

    let mut team_list: Vec<Team> = Vec::new();
    for row in row {
        let team = Team::new(
            row.get(0),
            row.get::<&str, usize>(1).unwrap().to_owned(),
        );
        team_list.push(team);
    }
    println!("This is the list of all teams");

    HttpResponse::Ok().body(format!("{:?}", team_list))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Try to connect to the database using db module (see db.rs)
    let config: Config = get_config().await?;
    let mut client = connect_to_db(config).await?;
    insert_fake_users(&mut client).await?;

    println!("Successfully read the file");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_user_by_id)
            .service(get_all_users)
            .service(get_all_teams)
            .service(create_user)
            .service(create_team)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
