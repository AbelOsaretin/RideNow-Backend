use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn init_db_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| sqlx::Error::Configuration("DATABASE_URL is not set".into()))?;

    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
}
