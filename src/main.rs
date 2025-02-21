//! # Actix Web Starter
//!
//! ## Usage
//!
//! ```bash
//! cargo run
//! ```

mod config;
mod handlers;
mod routes;
mod errors;
mod models;

use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use log::info;
use crate::config::Config;
use crate::handlers::error_handler::{handle_json_error, not_found};
use crate::routes::example_routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::load().expect("Failed to load configuration");

    info!(
        "Listening for incoming connections on {}:{}",
        config.host, config.port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(web::JsonConfig::default().error_handler(handle_json_error))
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .configure(example_routes::init)
            .default_service(web::route().to(not_found))
    })
        .bind((config.host, config.port))?
        .run()
        .await
}
