use log::info;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, Sqlite, SqlitePool};
use std::env;

use super::utils::{log_existing_tables, run_database_migrations};

pub async fn connect() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").expect("could not load database url");

    ensure_database(&database_url).await;
    let db_pool = get_database_connection(&database_url).await;
    run_database_migrations(&db_pool).await;

    #[cfg(debug_assertions)]
    log_existing_tables(&db_pool).await;

    db_pool
}

async fn ensure_database(database_url: &String) {
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        info!("existing database not found, creating database now");

        Sqlite::create_database(database_url)
            .await
            .expect("database could not be created and does not exist");
    }

    info!("existing database found");
}

async fn get_database_connection(database_url: &String) -> SqlitePool {
    let db_pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("could not connect to database");

    info!("database connection established");

    db_pool
}
