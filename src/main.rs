use std::net::Ipv4Addr;

use actix_web::{middleware::Logger, App, HttpServer};

mod config;
use config::Config;
mod api;
mod image;
mod query;
mod request;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::read();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(api::handle_image)
            .wrap(Logger::default())
    })
    .bind((Ipv4Addr::UNSPECIFIED, config.port))
    .unwrap()
    .workers(config.workers)
    .run()
    .await
}
