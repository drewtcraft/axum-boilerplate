use log::{debug, error, info, warn};
use std::sync::Arc;

use askama::Template;
use axum::extract::{Extension, Path, State};
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::{Form, Json};
use serde::Deserialize;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::context::Context;
use crate::error::{Error, Result};
use crate::state::AppState;
use crate::templates::BaseTemplate;
use crate::util;

use super::constants::SESSION_UID_COOKIE;
use super::models::{user, user_temp_uid, user_temp_uid::TempUidPurpose};
use super::templates::{LogInTemplate, SignUpTemplate};

#[derive(Clone, Deserialize)]
pub struct SignUpParams {
    pub uid: String,
}

pub async fn get_sign_up(
    Extension(context): Extension<Context>,
    Path(params): Path<SignUpParams>,
    State(state): State<Arc<AppState>>,
) -> Result<Response> {
    debug!("Processing get_sign_up with uid: {}", &params.uid);

    let email =
        user_temp_uid::get_user_email_from_uid(&state.db_pool, &params.uid, TempUidPurpose::SignUp)
            .await?;

    debug!("Got email from get_sign_up uid lookup: {}", email);

    let rendered_sign_up = SignUpTemplate { email }.render().unwrap();
    Ok((StatusCode::OK, Html(rendered_sign_up)).into_response())
}

#[derive(Clone, Deserialize)]
pub struct SignUpBody {
    pub username: String,
}

pub async fn post_sign_up(
    cookies: Cookies,
    Path(params): Path<SignUpParams>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<SignUpBody>,
) -> Result<Response> {
    debug!(
        "Processing post_sign_up with uid: {}, username: {}",
        &params.uid, &payload.username
    );

    // verify the username is available
    //    -- if not, something interesting happens
    if user::username_exists(&state.db_pool, &payload.username).await? {
        debug!(
            "post_sign_up username: {} ALREADY EXISTS!",
            &payload.username
        );
        todo!("return the form with an error I guess?")
    }

    let (user_id, email) =
        user_temp_uid::validate_user_sign_up_temp_uid(&state.db_pool, &params.uid).await?;

    debug!(
        "post_sign_up retrieved user_id: {}, email: {}",
        &user_id, &email, &params.uid
    );

    user_temp_uid::delete_user_temp_uid(&state.db_pool, &params.uid).await?;

    debug!("post_sign_up deleted uid."");

    // augment the existing user with the username
    user::activate_user(&state.db_pool, user_id, &payload.username).await?;

    debug!("post_sign_up activated user.")

    // create a new uid for a session
    let uid = user_temp_uid::create_user_session_temp_uid(&state.db_pool, user_id).await?;

    cookies.add(Cookie::new(SESSION_UID_COOKIE, uid));

    Ok(Redirect::to("/").into_response())
}

pub async fn get_log_in(Extension(context): Extension<Context>) -> Result<Response> {
    let rendered_login = LogInTemplate()
        .render()
        .map_err(|_| Error::TemplateRenderingFailure)?;

    let rendered = util::render(context.is_htmx, rendered_login)?;
    Ok((StatusCode::OK, Html(rendered)).into_response())
}

#[derive(Clone, Debug, Deserialize)]
pub struct LogInBody {
    username_or_email: String,
}

pub async fn post_log_in(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LogInBody>,
) -> Result<Response> {
    let (user_id, email) =
        user::user_by_username_or_email(&state.db_pool, &payload.username_or_email).await?;

    let uid = user_temp_uid::create_user_log_in_temp_uid(&state.db_pool, user_id).await?;

    // render email template using uid

    // send email

    // render "email sent you can close this page" template

    //

    todo!()
}

pub async fn log_out() -> impl IntoResponse {
    todo!()
}

pub async fn get_send_invite(context: Context) -> Result<Response> {
    todo!()
}
