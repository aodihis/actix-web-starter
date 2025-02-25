use crate::config::config::Config;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn get_pool(config: &Config) -> Result<PgPool, sqlx::Error> {
    let connection_string = format!("postgres://{}:{}@{}:{}/{}", config.db_username, config.db_password,
                                    config.db_host, config.db_password, config.db_name);
    let pool = PgPoolOptions::new().max_connections(10).min_connections(5).connect(&connection_string).await;
    pool
}