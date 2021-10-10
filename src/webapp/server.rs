use crate::webapp::routes;
use axum::{routing::BoxRoute, AddExtensionLayer, Router};
use sqlx::PgPool;
use std::env;
use std::net::SocketAddr;

#[allow(dead_code)]
pub async fn serve(pool: PgPool) {
    // Attaches routes to the host address
    tracing_subscriber::fmt::init();

    let addr = get_host();
    let routes = app(pool).into_make_service();
    axum::Server::bind(&addr).serve(routes).await.unwrap();

    tracing::debug!("listening on {}", addr);
}

pub fn app(pool: PgPool) -> Router<BoxRoute> {
    Router::new()
        .nest("/", routes::ssr::get_routes())
        .nest("/api", routes::api::get_routes())
        .layer(AddExtensionLayer::new(pool))
        .boxed()
}

fn get_host() -> SocketAddr {
    let host = match env::var("HOST") {
        Ok(val) => val,
        Err(_e) => "0.0.0.0".to_string(),
    };
    let port = match env::var("PORT") {
        Ok(val) => val,
        Err(_e) => "8088".to_string(),
    };
    format!("{}:{}", host, port).parse().unwrap()
}
