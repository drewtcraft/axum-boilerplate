use axum;
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, FromRow, Row, Sqlite, SqlitePool};
use std::env;
use std::net::SocketAddr;

mod apps;
mod database;
mod error;
mod router;
mod templates;

#[derive(Debug, Clone, FromRow)]
struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub active: bool,
}

#[tokio::main]
async fn main() {
    dotenv().expect("secrets file could not be loaded");

    let db_pool = database::initialize_database().await;

    println!("loaded secrets");

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    let result = sqlx::query(
        "
        INSERT INTO users (
            username,
            email,
            active,
            created_at,
            updated_at
        ) VALUES (
            ?, 
            ?, 
            ?, 
            ?, 
            ?
        );",
    )
    .bind("bobby")
    .execute(&db_pool)
    .await
    .unwrap();

    println!("Query result: {:?}", result);

    let port = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("starting server on {addr}");

    axum::Server::bind(&addr)
        .serve(router::get_routes().into_make_service())
        .await
        .expect("could not start axum server");
}
