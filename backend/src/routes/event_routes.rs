use crate::config::get_config;
use crate::db::connect_to_db;
use crate::models::Event;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use chrono::{Date, NaiveDate};
use serde::{Deserialize, Serialize};
use tiberius::ToSql;

#[derive(Serialize, Deserialize, Debug, derive_new::new)]
pub struct EventInfo {
    event_id: i32,
    name: String,
    start_date: String,
    end_date: String,
}

#[post("/events")]
async fn create_event(event: web::Json<Event>) -> impl Responder {
    let config = get_config().await.unwrap();
    let mut client = connect_to_db(config).await.unwrap();
    let (query, params) = event.to_insert_query();
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

// get all events
#[get("/events")]
async fn get_all_events() -> impl Responder {
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
    let query = "SELECT * FROM EventsView";
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

    let mut event_list: Vec<Event> = Vec::new();
    for row in row {
        let event = Event::new(
            row.get(0).unwrap(),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get::<&str, usize>(2).unwrap().to_owned(),
            row.get::<&str, usize>(3).unwrap().to_owned(),
        );
        event_list.push(event);
    }

    println!("This is the list of events: {:?}", event_list);

    HttpResponse::Ok().json(event_list)
}

// get the users events
#[get("/user_events/{id}")]
async fn get_user_events(id: web::Path<i32>) -> impl Responder {
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
    let query = "SELECT * FROM getEventsByPlayerId(@P1)";
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

    let event_list: Vec<Event> = Vec::new();

    HttpResponse::Ok().json(event_list)
}

#[get("/events/{id}")]
async fn get_event_by_id(id: web::Path<i32>) -> impl Responder {
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
    let query = "SELECT * FROM Events WHERE id = @P1";
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

    let mut event = EventInfo::new(0, "".to_owned(), "".to_owned(), "".to_owned());
    for row in row {
        event = EventInfo::new(
            row.get(0).unwrap(),
            row.get::<&str, usize>(1).unwrap().to_owned(),
            row.get::<&str, usize>(2).unwrap().to_owned(),
            row.get::<&str, usize>(3).unwrap().to_owned(),
        );
    }

    HttpResponse::Ok().json(event)
}
#[delete("/events/{id}")]
async fn delete_event(id: web::Path<i32>) -> impl Responder {
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

    let query = "DELETE FROM Events WHERE id = @P1";
    let id = id.into_inner();
    let result = client.query(query, &[&id]).await;
    match result {
        Ok(_) => {
            return HttpResponse::Ok().body("Event deleted successfully");
        }
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    }
}
