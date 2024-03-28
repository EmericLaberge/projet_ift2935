mod config;
mod db;
mod models;
mod test;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,middleware};
use config::get_config;
use db::connect_to_db;
use models::{staff, team, users};
use staff::Staff;
use team::Team;
use test::insert_fake_users;
use actix_cors::Cors;
use tiberius;
use tiberius::{Client, Config};
use tokio::net::TcpStream;
use tokio_util::compat::Compat;
use users::User;

/// This prevent passing the config around like before
/// This is a struct that holds the config and the client
struct AppState {
    config: Config,
    client: Client<Compat<TcpStream>>,
}

impl AppState {
    async fn new() -> Result<Self, anyhow::Error> {
        let mut state = AppState {
            config: get_config().await?,
            client: connect_to_db(get_config().await?).await?,
        };
        Ok(state)
    }
}

#[post("/create_user")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    // make a query to the database to insert the user
    // return the user as a json response
    dbg!(&user);
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();

    let (query, params) = user.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await;
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


#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    // Try to connect to the database using db module (see db.rs)
    let config: Config = get_config().await?;
    let mut client = connect_to_db(config.clone()).await?;
    let app_state = web::Data::new(AppState { config, client });
    println!("Successfully read the file");
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
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}
