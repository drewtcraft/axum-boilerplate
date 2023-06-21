use std::sync::Arc;

use axum::middleware::AddExtension;
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};

use crate::context::Context;
use crate::state::AppState;

use super::handlers::{get_log_in, get_sign_up, log_out, post_log_in, post_sign_up};
use super::layers::pull_user_id_from_session_uid;

pub fn get_routes(state: Arc<AppState>) -> Router {
    let unauthorized_routes = Router::new()
        .route("/log-in", get(get_log_in).post(post_log_in))
        .route("/sign-up/:uid", get(get_sign_up).post(post_sign_up))
        .with_state(state.clone());

    let authorized_routes = Router::new()
        .route("/log-out", get(log_out))
        .with_state(state);

    authorized_routes.merge(unauthorized_routes)
}
