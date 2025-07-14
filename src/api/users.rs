use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
struct User { id: Uuid, username: String }

#[derive(Deserialize)]
struct NewUser { username: String }

#[get("/users")]
async fn list_users() -> impl Responder {
    let demo = vec![User { id: Uuid::new_v4(), username: "demo".into() }];
    HttpResponse::Ok().json(demo)
}

#[post("/users")]
async fn create_user(payload: web::Json<NewUser>) -> impl Responder {
    let user = User { id: Uuid::new_v4(), username: payload.username.clone() };
    HttpResponse::Created().json(user)
}

pub fn scope() -> actix_web::Scope {
    web::scope("").service(list_users).service(create_user)
}
