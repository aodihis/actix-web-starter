use dotenv::Error;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_host: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_name: String,
}
impl Config {
    pub fn load() -> Result<Self, Error> {
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("invalid port value"),
            db_host:env::var("DB_HOST").expect("DB_HOST not set"),
            db_port: env::var("DB_PORT").expect("DB_PORT not set"),
            db_username: env::var("DB_USERNAME").expect("DB_USERNAME not set"),
            db_password: env::var("DB_PASSWORD").expect("DB_PASSWORD not set"),
            db_name: env::var("DB_NAME").expect("DB_NAME not set")
        })
    }
}
