mod infra;
mod settings;
pub mod webapp;
use infra::database;

#[tokio::main]
async fn main() {
    let pool = database::get_connection_pool("postgres://postgres:pass@localhost:15432/test")
        .await
        .unwrap();
    webapp::server::serve(pool).await
}

#[cfg(test)]
mod tests;
