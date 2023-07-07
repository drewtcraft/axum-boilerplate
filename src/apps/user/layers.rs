use std::sync::Arc;

use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
    Extension,
};
use log::info;
use tower_cookies::Cookies;

use crate::{context::{ContextUserData, Context}, error::Result, state::AppState};

use super::{constants::SESSION_UID_COOKIE, models::UserTempUid};

pub async fn pull_user_id_from_session_uid<B>(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    info!("cookies: {:?}", cookies);
    let session_uid = cookies
        .get(SESSION_UID_COOKIE)
        .map(|c| c.value().to_string());

    let user_data = if let Some(session_uid) = session_uid {
        info!("pulled session uid in middleware {:?}", &session_uid);

        // TODO think about this: does the cookie need to be unset here??
        let uid_lookup_response =
            UserTempUid::validate_user_session_temp_uid(&state.db_pool, &session_uid).await;

        if let Ok((user_id, email)) = uid_lookup_response {
            info!("pulled user data from session uid");
            Some(ContextUserData::new(user_id, email, session_uid))
        } else {
            info!("could NOT pull user data from session uid");
            None
        }
    } else {
        info!("no session cookie found");
        None
    };

    info!("creating context using {:?}", &user_data);

    let context = Context::new(user_data, None);

    request.extensions_mut().insert(context);

    info!("set context extension");

    // run executes remaining middleware
    Ok(next.run(request).await)
}

pub async fn restrict_to_user<B>(
    Extension(context): Extension<Context>,
    request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    if context.user_data.is_none() {
        // TODO serve a 403 or something
        return Ok(Redirect::to("/").into_response());
    }
    Ok(next.run(request).await)
}
