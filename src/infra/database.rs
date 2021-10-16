use axum::extract::Extension;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub type DbPool = Extension<PgPool>;

pub async fn get_connection_pool(db_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .connect(db_string)
        .await
}
