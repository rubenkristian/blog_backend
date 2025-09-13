use std::sync::Arc;
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, RocksDb};

pub type DbPool = Arc<Surreal<Db>>;

pub async fn init_db() -> DbPool {
    let db = Surreal::new::<RocksDb>("./app_data")
        .await
        .expect("Failed to open RocksDB storage");

    db.use_ns("database")
        .use_db("app")
        .await
        .expect("Failed to select namespace/database");

    Arc::new(db)
}
