use crate::routes::{health_check, subscriptions};
use actix_web::dev::Server;
use actix_web::{web, HttpServer};
use std::net::TcpListener;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        actix_web::App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscriptions::subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
