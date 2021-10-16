pub(crate) mod chassis;
mod infra;
mod settings;
pub(crate) mod webapp;
use infra::database;

#[tokio::main]
async fn main() {
    let db_pool = database::get_connection_pool("postgres://postgres:pass@localhost:15432/test")
        .await
        .unwrap();
    webapp::server::serve(db_pool).await
}

#[cfg(test)]
mod tests;
