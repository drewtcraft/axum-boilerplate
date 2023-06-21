use std::sync::Arc;

use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_cookies::Cookies;

use crate::{
    context::Context,
    error::{Error, Result},
};
use crate::{context::ContextUserData, state::AppState};

use super::{constants::SESSION_UID_COOKIE, models::user_temp_uid};

pub async fn pull_user_id_from_session_uid<B>(
    State(state): State<Arc<AppState>>,
    cookies: Cookies,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    let session_uid = cookies
        .get(SESSION_UID_COOKIE)
        .map(|c| c.value().to_string());

    let user_data = if let Some(session_uid) = session_uid {
        // TODO think about this: does the cookie need to be unset here??
        let (user_id, email) =
            user_temp_uid::validate_user_session_temp_uid(&state.db_pool, &session_uid).await?;

        Some(ContextUserData::new(user_id, email))
    } else {
        None
    };

    let context = Context::new(user_data, None);

    request.extensions_mut().insert(context);

    // run executes remaining middleware
    Ok(next.run(request).await)
}
