// This file will define the API for the application, typically this will be end 
// points. 
//

use actix_web::{web, HttpResponse, Responder,App, HttpServer, HttpRequest};

async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(use
}

