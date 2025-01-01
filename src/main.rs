use actix_cors::Cors;
use actix_web::{App, HttpServer};
mod handlers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define an array of allowed origins
    let allowed_origins = vec![
        "http://localhost:3000",
        "https://example.com",
        "https://another-example.com",
    ];

    HttpServer::new(move || {
        let mut cors = Cors::default();
        
        // Dynamically add each origin to the CORS configuration
        for origin in &allowed_origins {
            cors = cors.allowed_origin(origin);
        }

        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
            ])
            .max_age(3600); // Cache preflight response for 1 hour

        App::new()
            .wrap(cors) // Add CORS middleware
            .configure(routes::status_routes::configure) // Use the route configuration
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
