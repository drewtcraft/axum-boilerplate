use axum;
use dotenv::dotenv;
use sqlx::{migrate::MigrateDatabase, FromRow, Sqlite, SqlitePool};
use std::env;
use std::net::SocketAddr;

mod apps;
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

    let database_url = env::var("DATABASE_URL").unwrap();
    if !Sqlite::database_exists(&database_url)
        .await
        .unwrap_or(false)
    {
        Sqlite::create_database(&database_url)
            .await
            .expect("database could not be created and does not exist");
    }
    println!("database url {}", &database_url);

    let db = SqlitePool::connect(&database_url)
        .await
        .expect("could not connect to database");

    println!("looks like we got a db connection");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(router::get_routes().into_make_service())
        .await
        .unwrap();
}
