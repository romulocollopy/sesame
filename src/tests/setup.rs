use crate::infra::database;
use crate::webapp::server::app;
use axum::{routing::BoxRoute, Router};

pub async fn build_test_app() -> Router<BoxRoute> {
    let pool = database::get_connection_pool("postgres://postgres:pass@localhost:15432/test")
        .await
        .unwrap();

    app(pool)
}
