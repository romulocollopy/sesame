pub mod ssr {
    use axum::{handler::get, handler::post, routing::BoxRoute, Router};

    pub fn get_routes() -> Router<BoxRoute> {
        Router::new()
            .route("/", get(get_slash))
            .route("/", post(post_slash))
            .boxed()
    }

    async fn get_slash() -> String {
        // `GET /` called
        String::from("Hello, World!")
    }

    async fn post_slash() -> String {
        // `POST /` called
        String::from("Post World, mdf!")
    }
}

pub mod api {
    use crate::chassis::repository::{DbQuery, Repository};
    use crate::infra::database::DbPool;
    use axum::{handler::get, handler::post, routing::BoxRoute, Json, Router};

    pub fn get_routes() -> Router<BoxRoute> {
        Router::new()
            .route("/", get(fuuu))
            .route("/", post(json))
            .boxed()
    }

    async fn fuuu(db_pool: DbPool) -> String {
        let row = Repository::new(db_pool).test_connection().await;
        String::from(format!("{}", row.0))
    }

    async fn json(payload: Json<serde_json::Value>) -> Json<serde_json::Value> {
        Json(serde_json::json!({ "data": payload.0 }))
    }
}
