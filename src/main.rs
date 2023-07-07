#![allow(dead_code)]
use axum;
use dotenv::dotenv;
use env_logger::{Builder, Env};
use log::{info, LevelFilter};
use std::env;
use std::net::SocketAddr;

use crate::state::AppState;

mod apps;
mod constants;
mod context;
mod database;
mod error;
mod layers;
mod mailer;
mod router;
mod state;
mod templates;
mod traits;
mod utils;

#[tokio::main]
async fn main() {
    Builder::from_env(Env::default().default_filter_or("debug"))
        .format_timestamp(None)
        .filter_module("sqlx::query", LevelFilter::Off)
        .format_module_path(false)
        .filter(None, LevelFilter::Info)
        .init();

    info!("Logger initialized.");
    info!("Server starting up!");

    dotenv().expect("secrets file could not be loaded");

    info!(".env file loaded.");

    let db_pool = database::connection::connect().await;

    database::seed::run(&db_pool).await;

    let state = AppState::new_arc(db_pool);

    let port = env::var("PORT")
        .unwrap_or(String::from("8080"))
        .parse::<u16>()
        .expect("could not parse port");

    let addr = SocketAddr::from(([127, 0, 0, 1], port));

    info!("starting server on {addr}");

    axum::Server::bind(&addr)
        .serve(router::get_routes(state).into_make_service())
        .await
        .expect("could not start axum server");
}
