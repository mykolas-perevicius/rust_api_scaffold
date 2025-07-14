use actix_web::{get, web, HttpResponse, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "UP" }))
}

pub fn scope() -> actix_web::Scope {
    web::scope("").service(health)
}
