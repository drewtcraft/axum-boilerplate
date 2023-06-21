use std::sync::Arc;

use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: SqlitePool,
}

impl AppState {
    pub fn new_arc(db_pool: SqlitePool) -> Arc<AppState> {
        Arc::new(AppState { db_pool })
    }
}
