use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

use crate::{config::get_config, db::connect_to_db};


#[derive(Deserialize)]
pub struct Credentials {
    username: String,
    password: String,
}
#[post("/login")]
pub async fn login(login: web::Json<Credentials>) -> impl Responder {
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



