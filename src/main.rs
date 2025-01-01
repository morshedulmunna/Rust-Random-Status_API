use actix_web::{web, App, HttpServer};
mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/status/random", web::get().to(handlers::get_random_status))
            .route("/status/add", web::post().to(handlers::add_status))
            .route("/status/all", web::get().to(handlers::get_all_statuses))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
