use actix_web::{middleware::Logger, App, HttpServer};

mod application;
mod infra;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(routes::methods::post)
            .service(routes::config::configuration)
            .service(routes::methods::get)
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
    .await
}
