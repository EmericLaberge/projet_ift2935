use actix_web::{get, post, web, HttpResponse, Responder};

use crate::{config::get_config, db::connect_to_db, models::Team};



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

    HttpResponse::Ok().json(team_list)
}

// get the users events 
#[get("/user_teams/{id}")]
async fn get_user_teams(id: web::Path<i32>) -> impl Responder {
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
    let query = "SELECT * FROM getTeamsByUserId(@P1)";
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

    let mut team_list: Vec<Team> = Vec::new();
    HttpResponse::Ok().json(team_list)
}
