use log::{debug, error, info, warn};
use std::sync::Arc;

use axum::extract::{Extension, Path, Query, State};
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::Form;
use sailfish::TemplateOnce;
use tower_cookies::cookie::time::Duration;
use tower_cookies::{Cookie, Cookies};

use crate::apps::user::models;
use crate::apps::user::templates::{
    EmailLogInTemplate, LogInTemplate, LogOutTemplate, SendInviteTemplate,
};
use crate::context::Context;
use crate::error::{Error, Result};
use crate::mailer::send_email;
use crate::state::AppState;
use crate::traits::{ParamValidator, ToPlainText};
use crate::utils::get_own_url_with;

use super::constants::SESSION_UID_COOKIE;
use super::models::{User, UserTempUid, UserTempUid::TempUidPurpose};
use super::serializers::{
    IdParam, LogInBody, SendInviteBody, SignUpBody, UidParam, UserEditParams, UserListParams,
};
use super::templates::{
    AdminUserEditTemplate, AdminUserListTemplate, EmailInviteTemplate, SignUpTemplate, UserListUser,
};

pub async fn get_sign_up(
    Extension(mut context): Extension<Context>,
    Path(params): Path<UidParam>,
    State(state): State<Arc<AppState>>,
) -> Result<impl IntoResponse> {
    info!("Hit get_sign_up with uid: {}", &params.uid);

    let email =
        UserTempUid::get_user_email_from_uid(&state.db_pool, &params.uid, TempUidPurpose::SignUp)
            .await?;

    info!("Got email from get_sign_up uid lookup: {}", email);

    let rendered = SignUpTemplate::new_render(context.get_is_htmx(), email)?;
    context.page_title = Some(String::from("donkey"));

    Ok(Html(rendered))
}

pub async fn post_sign_up(
    Extension(mut context): Extension<Context>,
    cookies: Cookies,
    Path(params): Path<UidParam>,
    State(state): State<Arc<AppState>>,
    Form(payload): Form<SignUpBody>,
) -> Result<impl IntoResponse> {
    info!(
        "Hit post_sign_up with uid: {}, username: {}",
        &params.uid, &payload.username
    );

    if User::username_exists(&state.db_pool, &payload.username).await? {
        info!(
            "post_sign_up username: {} ALREADY EXISTS!",
            &payload.username
        );
        let html = SignUpTemplate::new_render_error(
            context.get_is_htmx(),
            payload.email,
            Some(payload.username),
            Some("username taken".to_string()),
        )?;
        return Ok(Html(html).into_response());
    }

    // verify email is not taken
    // return error if it is

    let (user_id, email) =
        UserTempUid::validate_user_sign_up_temp_uid(&state.db_pool, &params.uid).await?;

    info!(
        "post_sign_up retrieved user_id: {}, email: {}",
        &user_id, &email
    );

    UserTempUid::delete_user_temp_uid(&state.db_pool, &params.uid).await?;

    info!("post_sign_up deleted uid.");

    // augment the existing user with the username
    User::activate_user(&state.db_pool, user_id, &payload.username).await?;

    info!("post_sign_up activated user.");

    // create a new uid for a session
    let uid = UserTempUid::create_user_session_temp_uid(&state.db_pool, user_id).await?;

    info!("post_sign_up created session uid: {}", uid);

    cookies.add(Cookie::new(SESSION_UID_COOKIE, uid));

    info!("post_sign_up set session cookie.");

    Ok(Redirect::to("/").into_response())
}

pub async fn get_log_in(Extension(context): Extension<Context>) -> Result<impl IntoResponse> {
    info!("Hit get_login.");

    let rendered = LogInTemplate::new_render(context.get_is_htmx())?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn post_log_in(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<LogInBody>,
) -> Result<impl IntoResponse> {
    // if incorrect email, LogInTemplate::new_submit_error(xxx)

    info!(
        "Hit post_log_in with username or email: {}.",
        &payload.username_or_email
    );

    let (user_id, email) =
        User::user_by_username_or_email(&state.db_pool, &payload.username_or_email).await?;

    info!("post_log_in got user_id {} and email {}", &user_id, &email);

    let uid = UserTempUid::create_user_log_in_temp_uid(&state.db_pool, user_id).await?;

    info!("post_log_in generated log in uid");

    let path = format!("/log-in/{}", &uid);
    let log_in_url = get_own_url_with(&path);

    info!("post_log_in generated log in url: {}", &log_in_url);

    let (html, plain_text) = EmailLogInTemplate::new(log_in_url).render_html_and_plain_text()?;

    #[cfg(not(debug_assertions))]
    send_email(
        &email,
        "Here's your one time login for XXXX",
        html,
        plain_text,
    )
    .await?;

    Ok((StatusCode::OK, Html(html)))
}

pub async fn log_out(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
    Extension(context): Extension<Context>,
) -> Result<impl IntoResponse> {
    info!("hit log_out");
    if let Some(user_data) = &context.user_data {
        info!("deleting user session on log_out");
        UserTempUid::delete_user_temp_uid(&state.db_pool, &user_data.session_uid).await?;
    }

    let mut cookie = Cookie::named(SESSION_UID_COOKIE);
    cookie.set_max_age(Duration::seconds(0));
    cookies.add(cookie);

    let rendered = LogOutTemplate::new_render(context.get_is_htmx())?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn get_send_invite(Extension(context): Extension<Context>) -> Result<impl IntoResponse> {
    info!("Hit get_login.");

    // TODO delete expired invites

    let rendered = SendInviteTemplate::new_render(context.get_is_htmx())?;

    Ok((StatusCode::OK, Html(rendered)))
}

pub async fn post_send_invite(
    State(state): State<Arc<AppState>>,
    Form(payload): Form<SendInviteBody>,
) -> Result<impl IntoResponse> {
    let email = payload.email;

    // verify email is not taken
    if User::email_exists(&state.db_pool, &email).await? {
        todo!();
    }

    // create new stub user
    let user_id = User::create_user(&state.db_pool, None, &email, false, 0).await?;

    let uid = UserTempUid::create_user_sign_up_temp_uid(&state.db_pool, user_id).await?;

    let path = format!("/accept-invite/{}", &uid);
    let acceptance_url = get_own_url_with(&path);

    let template = EmailInviteTemplate::new(acceptance_url);

    let plain_text = template.to_plain_text();
    let html = template
        .render_once()
        .map_err(|_| Error::TemplateRenderingFailure)?;

    send_email(&email, "You've been invited to join XXX", html, plain_text).await?;

    Ok((StatusCode::OK, Html("")))
}

pub async fn get_cookie(
    cookies: Cookies,
    State(state): State<Arc<AppState>>,
    Path(params): Path<UidParam>,
) -> Result<impl IntoResponse> {
    info!("get_cookie hit");
    let (user_id, _) =
        UserTempUid::validate_user_log_in_temp_uid(&state.db_pool, &params.uid).await?;
    info!("validated temp uid");

    UserTempUid::delete_user_temp_uid(&state.db_pool, &params.uid)
        .await
        .unwrap_or(()); // ok if this fails

    // create a new uid for a session
    let uid = UserTempUid::create_user_session_temp_uid(&state.db_pool, user_id).await?;

    info!("get_cookie created session uid: {}", &uid);

    // TODO abstract

    let mut cookie = Cookie::new(SESSION_UID_COOKIE, uid);
    cookie.set_path("/");

    cookies.add(cookie);

    info!("get_cookie set session cookie.");

    Ok(Redirect::to("/"))
}

pub async fn list_users(
    State(state): State<Arc<AppState>>,
    Extension(context): Extension<Context>,
    Query(query_params): Query<UserListParams>,
) -> Result<impl IntoResponse> {
    info!("admin list_users hit");
    let (valid, errors) = query_params.validate();
    let user_roles = models::UserRole::list_user_roles(&state.db_pool).await?;
    if !valid {
        info!("admin list_users invalid params");
        let rendered = AdminUserListTemplate::new_render_error(
            context.get_is_htmx(),
            user_roles,
            query_params,
            errors,
        )?;

        return Ok((StatusCode::OK, Html(rendered)));
    }

    let users = User::list_users(&state.db_pool, &query_params).await?;
    info!("admin list_users found some users");

    let rendered =
        AdminUserListTemplate::new_render(context.get_is_htmx(), users, user_roles, query_params)?;

    Ok((StatusCode::OK, Html(rendered)))
}

// TODO this shit hella broken not even retrieving the user, circle back here
// pub async fn admin_get_edit_user(
//     State(state): State<Arc<AppState>>,
//     Extension(context): Extension<Context>,
//     Path(params): Path<IdParam>,
// ) -> Result<impl IntoResponse> {
//     let user = User::get_user(&state.db_pool, params.id).await?;
//     // TODO there should be a more ergonomic way to do this
//     let user_id_str = user.id.to_string();
//     let submit_url = format!("/admin/users/{}", user.id);
//     let user_roles = models::UserRole::list_user_roles(&state.db_pool).await?;
//     let rendered_user_edit = AdminUserEditTemplate::new_render_existing(
//         context.get_is_htmx(),
//         user_roles,
//         &submit_url.as_str(),
//         None,
//     )?;
//     let html = utils::render_template(context.is_htmx, rendered_user_edit)?;
//     Ok(Html(html))
// }

// pub async fn admin_post_edit_user(
//     State(state): State<Arc<AppState>>,
//     Extension(context): Extension<Context>,
//     Path(params): Path<IdParam>,
//     Form(payload): Form<UserEditParams>, // may need to be Json
// ) -> Result<impl IntoResponse> {
//     let (valid, errors) = payload.validate();
//     let user_id_str = params.id.to_string();
//     let submit_url = format!("/admin/users/{}", params.id);
//     let user_roles = models::UserRole::list_user_roles(&state.db_pool).await?;
//     if !valid {
//         let rendered_user_edit = AdminUserEditTemplate::new_render_error(
//             user_roles,
//             Some(user_id_str.as_str()),
//             payload.username.as_ref().map(|s| s.as_ref()),
//             errors.username.as_ref().map(|s| s.as_ref()),
//             Some(payload.email.as_str()),
//             errors.email.as_ref().map(|s| s.as_ref()),
//             Some(payload.active),
//             Some(&(payload.role as usize)),
//             submit_url.as_str(),
//         )?;
//         let html = utils::render_template(context.is_htmx, rendered_user_edit)?;
//         return Ok(Html(html));
//     }

//     // edit user
//     let edited_user = User::edit_user(&state.db_pool, params.id, &payload).await?;
//     // render template with new user & send
//     let success_message = format!("successfully updated user at {}", &edited_user.updated_at);
//     // let success_message = String::from("honk");
//     let rendered_user_edit = AdminUserEditTemplate::new_render_existing(
//         user_roles,
//         &user_id_str,
//         payload.username.as_ref().map(|s| s.as_str()),
//         &payload.email,
//         payload.active,
//         payload.role as usize,
//         submit_url.as_str(),
//         Some(success_message.as_str()),
//     )?;
//     let html = utils::render_template(context.is_htmx, rendered_user_edit)?;

//     Ok(Html(html))
// }
