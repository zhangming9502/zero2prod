use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello {}!", &name)
// }
async fn health_check() -> impl Responder {
    // HttpResponse::Ok().finish()
    HttpResponse::Ok()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
