use axum::Json;
use axum::{extract::Query, response::IntoResponse};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

const COOKIE_NAME: &'static str = ""; // TODO name this

#[derive(Debug, Deserialize)]
pub struct LogInBody {
    username: String,
    password: String,
}

pub async fn post_log_in(cookies: Cookies, payload: Json<LogInBody>) -> impl IntoResponse {
    let username = &payload.username;
    let password = &payload.password;

    // do async database stuff

    // TODO: generate UID for session
    let session_uid = String::from("");
    cookies.add(Cookie::new(COOKIE_NAME, session_uid));
}

pub async fn get_log_in() -> impl IntoResponse {}

pub async fn log_out() -> impl IntoResponse {
    todo!()
}
