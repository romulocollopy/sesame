use crate::infra::database::DbPool;
use async_trait::async_trait;

#[derive(Clone, Debug)]
pub struct Repository {
    db_pool: DbPool,
}

impl Repository {
    pub fn new(db_pool: DbPool) -> Repository {
        Repository { db_pool }
    }
}

#[async_trait]
pub trait DbQuery {
    async fn test_connection(self) -> (i64,);
}

#[async_trait]
impl DbQuery for Repository {
    async fn test_connection(self) -> (i64,) {
        sqlx::query_as("SELECT $1")
            .bind(1_i64)
            .fetch_one(&*self.db_pool)
            .await
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::setup::get_extension_pool;

    #[tokio::test]
    async fn test_connection() {
        let pool = get_extension_pool().await;
        let repo = Repository::new(pool);
        assert_eq!((1_i64,), repo.test_connection().await)
    }
}
