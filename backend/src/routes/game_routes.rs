use crate::db::connect_to_db;
use crate::models::Event;
use crate::{config::get_config, models::game::Game};
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use tiberius::ToSql;

#[derive(Serialize, Deserialize, derive_new::new, Debug)]
struct gameInfos {
    id: Option<i32>,
    sport_name: String,
    event_id: i32,
    first_team_id: i32,
    second_team_id: i32,
    game_date: String,
    final_score: String,
}

#[post("/games")]
async fn create_game(game: web::Json<Game>) -> impl Responder {
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let (query, params) = game.to_insert_query();
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

#[get("/games")]
async fn get_games() -> impl Responder {
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let query = "SELECT * FROM GamesView";
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

    let mut game_list: Vec<gameInfos> = Vec::new();
    for row in row {
        let game = gameInfos::new(
            row.get(0),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
            row.get(4).unwrap(),
            row.get::<&str, usize>(5).unwrap().to_owned(),
            row.get::<&str, usize>(6).unwrap().to_owned(),
        );
        game_list.push(game);
    }

    println!("This is the list of Games: {:?}", game_list);

    HttpResponse::Ok().json(game_list)
}

#[get("/games/{id}")]
async fn get_game_by_id(id: web::Path<i32>) -> impl Responder {
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let query = "SELECT * FROM Games WHERE ID=@P1";
    let id = id.into_inner();
    let result = client.query(query, &[&id]).await;
    let result = match result {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };
    let row = result.into_row().await;
    let row = match row {
        Ok(row) => row,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let mut game: gameInfos =
        gameInfos::new(None, "".to_owned(), 0, 0, 0, "".to_owned(), "".to_owned());

    if let Some(row) = row {
        game = gameInfos::new(
            row.get(0),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get(2).unwrap(),
            row.get(3).unwrap(),
            row.get(4).unwrap(),
            row.get::<&str, usize>(5).unwrap().to_owned(),
            row.get::<&str, usize>(6).unwrap().to_owned(),
        );
    }

    HttpResponse::Ok().json(game)
}


#[delete("/games/{id}")]
async fn delete_game(id: web::Path<i32>) -> impl Responder {
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

    let query = "DELETE FROM Games WHERE ID=@P1";
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

// get the users events
#[get("/user_games/{id}")]
async fn get_user_games(id: web::Path<i32>) -> impl Responder {
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
    // utiliser la fonction getEventsByPlayerId presente dans le sql
    let query = "SELECT * FROM getGamesByPlayerId(@P1)";
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

    let game_list: Vec<Game> = Vec::new();

    HttpResponse::Ok().json(game_list)
}
