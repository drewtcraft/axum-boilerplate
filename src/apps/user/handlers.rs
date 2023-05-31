use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Json;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};

use crate::templates::BaseTemplate;

use super::templates::LogInTemplate;

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
    // Redirect
}

pub async fn get_log_in() -> impl IntoResponse {
    let rendered_login = LogInTemplate().render().unwrap();
    (StatusCode::OK, Html(rendered_login).into_response())
}

pub async fn log_out() -> impl IntoResponse {
    todo!()
}
