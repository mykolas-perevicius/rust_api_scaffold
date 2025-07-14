use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".into());
    println!("🚀  Listening on http://localhost:{port}");
    HttpServer::new(|| App::new().service(health))
        .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
        .run()
        .await
}
