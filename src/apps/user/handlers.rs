use askama::Template;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Json;
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use super::templates::{LogInTemplate, SignUpTemplate};

const COOKIE_NAME: &'static str = ""; // TODO name this

pub async fn get_sign_up() -> impl IntoResponse {
    let rendered_sign_up = SignUpTemplate().render().unwrap();
    (StatusCode::OK, Html(rendered_sign_up).into_response())
}

pub async fn post_sign_up() -> impl IntoResponse {}

#[derive(Debug, Deserialize)]
pub struct LogInBody {
    email: String,
}

pub async fn get_log_in() -> impl IntoResponse {
    let rendered_login = LogInTemplate().render().unwrap();
    (StatusCode::OK, Html(rendered_login).into_response())
}

pub async fn post_log_in(cookies: Cookies, payload: Json<LogInBody>) -> impl IntoResponse {
    let payload = &payload.email;

    // is it a username or an email?
    // do async database stuff

    // TODO: generate UID for session
    let session_uid = String::from("");
    cookies.add(Cookie::new(COOKIE_NAME, session_uid));
    // Redirect
}

pub async fn log_out() -> impl IntoResponse {
    todo!()
}
