use crate::infra::database;
use crate::webapp::server::app;
use axum::{extract::Extension, routing::BoxRoute, Router};
use sqlx::PgPool;

pub async fn build_test_app() -> Router<BoxRoute> {
    app(get_connection_pool().await)
}

async fn get_connection_pool() -> PgPool {
    database::get_connection_pool("postgres://postgres:pass@localhost:15432/test")
        .await
        .unwrap()
}

pub async fn get_extension_pool() -> database::DbPool {
    Extension(get_connection_pool().await)
}
