use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, FromRow, Row, Sqlite, SqlitePool};
use std::{env, path::PathBuf};

pub async fn initialize_database() -> SqlitePool {
    let database_url = env::var("DATABASE_URL").unwrap();

    ensure_database(&database_url).await;
    let db_pool = get_database_connection(&database_url).await;
    run_database_migrations(&db_pool).await;
    db_pool
}

async fn ensure_database(database_url: &String) {
    if !Sqlite::database_exists(database_url).await.unwrap_or(false) {
        println!("existing database not found, creating database now");

        Sqlite::create_database(database_url)
            .await
            .expect("database could not be created and does not exist");
    }

    println!("database found");
}

async fn get_database_connection(database_url: &String) -> SqlitePool {
    let db_pool = SqlitePoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await
        .expect("could not connect to database");

    println!("database connection established");

    db_pool
}

async fn run_database_migrations(db_pool: &SqlitePool) {
    println!("running migration checks");

    let migrations_dir = std::path::Path::new("migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations_dir)
        .await
        .unwrap()
        .run(db_pool)
        .await
        .expect("failed to run migration");

    println!("migration: {:?}", migration_results);
}
