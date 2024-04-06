
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{config::get_config, db::connect_to_db, models::Player};

// Players routes
#[get("/players")]
async fn get_all_players() -> impl Responder {
    // make a query to the database to get all the players
    // return the players as a json response
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

    let query = "SELECT * FROM Players";
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

    let mut player_list: Vec<Player> = Vec::new();
    for row in row {
        let player = Player::new(
            row.get(0),
            row.get(1).unwrap(),
            row.get(2).unwrap(),
        );
        player_list.push(player);
    }
    serde_json::to_string(&player_list).unwrap();
    return HttpResponse::Ok().json(player_list);
}

#[post("/players")]
async fn create_player(player: web::Json<Player>) -> impl Responder {
    // make a query to the database to insert the player
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
    let (query, params) = player.to_insert_query();
    let params: Vec<&dyn tiberius::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let result = client.execute(query, &params[..]).await;
    HttpResponse::Ok().body(format!("{:?}", result))
}

#[get("/playersTeams/{id}")]
async fn get_teams_by_player_id(id: web::Path<i32>) -> impl Responder {
// 2. Get get the team that the player is in
// 3. Return the team as a json response
    HttpResponse::Ok().body(format!("{:?}", "get_teams_by_player_id"))
}

