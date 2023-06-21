use apps::user::models;
use axum;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, LevelFilter};
use sqlx::{migrate::MigrateDatabase, sqlite::SqlitePoolOptions, FromRow, Row, Sqlite, SqlitePool};
use std::env;
use std::net::SocketAddr;

use crate::state::AppState;

mod apps;
mod context;
mod database;
mod error;
mod layers;
mod mailer;
mod router;
mod state;
mod templates;
mod util;
mod views;

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    info!("Logger initialized.");
    info!("Server starting up!");

    dotenv().expect("secrets file could not be loaded");

    info!(".env file loaded.");

    let db_pool = database::initialize_database().await;

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

    // TEMP
    // let new_user = models::user::CreateUser {
    //     username: String::from("user_zero"),
    //     email: String::from("drewtcraft@gmail.com"),
    //     active: false,
    // };
    // let user_id = models::user::create_user(&db_pool, new_user).await;
    // if let Ok(user_id) = user_id {
    //     println!("created new user! {user_id}");
    // }

    // let user_temp_uid_id = models::user_temp_uid::create_user_sign_up_temp_uid(&db_pool, 1).await;
    // let user_temp_uid_id = user_temp_uid_id.expect("uh oh");
    // println!("created {user_temp_uid_id}");

    let state = AppState::new_arc(db_pool);

    let port = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse::<u16>()
        .unwrap();

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    println!("starting server on {addr}");

    axum::Server::bind(&addr)
        .serve(router::get_routes(state).into_make_service())
        .await
        .expect("could not start axum server");
}
