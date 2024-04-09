use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{config::get_config, db::connect_to_db, models::{partial_user, User}};


#[derive(Serialize, Deserialize, derive_new::new, Debug)]
struct userInfos {
    id: Option<i32>,
    email: String,
    address: String,
    first_name: String,
    last_name: String,
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
    let row = result.into_row().await;
    let row = match row {
        Ok(row) => row,
        Err(e) => {
            return HttpResponse::InternalServerError().body(format!("{:?}", e));
        }
    };

    let mut user: userInfos = userInfos::new(None, "".to_owned(), "".to_owned(), "".to_owned(), "".to_owned());
    let row = row.unwrap();
    user.id = row.get(0);
    user.email = row.get::<&str, usize>(1).unwrap().to_owned();
    user.address = row.get::<&str, usize>(2).unwrap().to_owned();
    user.first_name = row.get::<&str, usize>(3).unwrap().to_owned();
    user.last_name = row.get::<&str, usize>(4).unwrap().to_owned();
    serde_json::to_string(&user).unwrap();
    HttpResponse::Ok().json(user)
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


