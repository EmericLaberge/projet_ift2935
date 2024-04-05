mod config;
mod db;
mod models;
mod test;


use actix_cors::Cors;
use actix_web::{
    delete, get, post, put, web, App, HttpResponse, HttpServer, Responder,
};
use async_std::stream::StreamExt;
use config::get_config;
use db::connect_to_db;
use jsonwebtoken::{encode, DecodingKey, EncodingKey, Header, Validation};
use models::{staff, team, users};
use serde::{Deserialize, Serialize};
use staff::Staff;
use team::Team;
use test::insert_fake_users;
use tiberius::{self};
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use users::User;

use crate::models::partial_user;

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

#[derive(Serialize, Deserialize, derive_new::new, Debug)]
struct userInfos {
    id: Option<i32>,
    email: String,
    address: String,
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
}

fn generate_jwt(user_id: &str, secret: &[u8]) -> String {
    let claims = Claims {
        sub: user_id.to_string(),
    };
    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(secret);
    encode(&header, &claims, &encoding_key).unwrap()
}

fn validate_jwt(token: &str, secret: &[u8]) -> Result<Claims, jsonwebtoken::errors::Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    let decoding_key = DecodingKey::from_secret(secret);
    let token_data = jsonwebtoken::decode::<Claims>(token, &decoding_key, &validation)?;
    Ok(token_data.claims)
}
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[post("/login")]
async fn login(login: web::Json<LoginRequest>) -> impl Responder {
    let config = get_config().await.unwrap();
    let client = connect_to_db(config).await;
    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let query = "SELECT id from Credentials where username = @P1 and password = @P2";
    let result = client
        .query(query, &[&login.username, &login.password])
        .await;
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let first_result = result.into_row().await;
    let first_result = match first_result {
        Ok(first_result) => first_result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    if first_result.is_none() {
        return HttpResponse::Unauthorized().finish();
    }
    let first_result = first_result.unwrap();
    let id: std::option::Option<i32> = first_result.get(0);
    let id = match id {
        Some(id) => id,
        None => {
            return HttpResponse::InternalServerError().finish();
        }
    };
    // return {"id": id}
    let id_serde = serde_json::json!({ "id": id });
    HttpResponse::Ok().json(id_serde)
}

#[derive(Debug, derive_new::new, Deserialize, Serialize)]
pub struct NewUser {
    email: String,
    address: String,
    first_name: String,
    last_name: String,
    username: String,
    password: String,
}

impl NewUser {
    pub fn get_email(&self) -> &str {
        &self.email
    }

    pub fn get_user_name(&self) -> &str {
        &self.username
    }

    pub fn get_password(&self) -> &str {
        &self.password
    }
}

#[post("/create_user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let (query, params) = user.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await;
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    HttpResponse::Ok().body(format!("{:?}", result))
}

#[post("/create_team")]
async fn create_team(team: web::Json<Team>) -> impl Responder {
    // make a query to the database to insert the user

    let config = get_config().await;
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let client = connect_to_db(config).await;
    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let (query, params) = team.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await;
    HttpResponse::Ok().body(format!("{:?}", result))
}

#[get("/users/{id}")]
async fn get_user_by_id(id: web::Path<i32>) -> impl Responder {
    // make a query to the database to get the user with the id
    // return the user as a json response
    let config = get_config().await;
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let client = connect_to_db(config).await;
    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let query = "SELECT * FROM users WHERE id = @P1";
    let id = id.into_inner();
    let result = client.query(query, &[&id]).await;
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let row = result.into_first_result().await;
    let row = match row {
        Ok(row) => row,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let mut user_list: Vec<userInfos> = Vec::new();
    for row in row {
        let user = userInfos::new(
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
    let config = get_config().await;
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let client = connect_to_db(config).await;
    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let query = "SELECT * FROM users";
    let result = client.query(query, &[]).await;
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let row = result.into_first_result().await;
    let row = match row {
        Ok(row) => row,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let mut user_list: Vec<userInfos> = Vec::new();
    for row in row {
        let user = userInfos::new(
            row.get(0),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get::<&str, usize>(2).unwrap().to_owned(),
            row.get::<&str, usize>(3).unwrap().to_owned(),
            row.get::<&str, usize>(4).unwrap().to_owned(),
        );
        user_list.push(user);
    }
    serde_json::to_string(&user_list).unwrap();
    println!("This is the list of all users");

    HttpResponse::Ok().json(user_list)
}

#[get("/teams")]
async fn get_all_teams() -> impl Responder {
    // make a query to the database to get all the teams
    // return the teams as a json response
    let config = get_config().await;
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let client = connect_to_db(config).await;
    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let query = "SELECT * FROM teams";
    let result = client.query(query, &[]).await;
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let row = result.into_first_result().await;
    let row = match row {
        Ok(row) => row,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let mut team_list: Vec<Team> = Vec::new();
    for row in row {
        let team_name = row.get::<&str, usize>(1);
        let team_name = match team_name {
            None => {
                return HttpResponse::InternalServerError().body("Error getting name");
            }
            Some(name) => name.to_owned(),
        };
        let team = Team::new(row.get(0), team_name);
        team_list.push(team);
    }
    println!("This is the list of all teams");

    HttpResponse::Ok().body(format!("{:?}", team_list))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[delete("/users/{id}")]
async fn delete_user(id: web::Path<i32>) -> impl Responder {
    // make a query to the database to get all the teams
    // return the teams as a json response
    let config = get_config().await;
    let config = match config {
        Ok(config) => config,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let client = connect_to_db(config).await;
    let mut client = match client {
        Ok(client) => client,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let query = "DELETE FROM Users WHERE ID=@P1";
    let id = id.into_inner();
    let result = client.query(query, &[&id]).await;
    match result {
        Ok(_) => {
            return HttpResponse::Ok().body("User deleted");
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    }
}

#[put("/users/{id}")]
async fn update_user(id: web::Path<i32>, user: web::Json<partial_user>) -> impl Responder {
    let id = id.into_inner();
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config.clone()).await.unwrap();
    println!("This is the id: {}", id);
    println!("This is the user: {:?}", user);
    let (query, params) = user.to_alter_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await;
    match result {
        Ok(_) => {
            return HttpResponse::Ok().body("User updated");
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    }
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
            .service(create_user)
            .service(create_team)
            .service(login)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("127.0.0.1", 6516))?
    .run()
    .await;

    Ok(())
}
