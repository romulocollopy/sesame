use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connection_test() -> Result<(i64,), sqlx::Error> {
    let pool = get_connection_pool("postgres://postgres:pass@localhost:15432/test")
        .await
        .unwrap();

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await?;

    Ok(row)
}

pub async fn get_connection_pool(db_string: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(db_string)
        .await
}
