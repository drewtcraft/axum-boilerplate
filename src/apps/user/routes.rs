use std::sync::Arc;

use axum::middleware::AddExtension;
use axum::routing::{get, post};
use axum::{middleware, Extension, Router};

use crate::context::Context;
use crate::state::AppState;

use super::handlers::{
    /* admin_get_edit_user, admin_post_edit_user,*/ get_cookie, get_log_in, get_send_invite,
    get_sign_up, /* list_users, */ log_out, post_log_in, post_send_invite, post_sign_up,
};
use super::layers::{pull_user_id_from_session_uid, restrict_to_user};

pub fn get_routes(state: Arc<AppState>) -> Router {
    let private_routes = Router::new()
        // .route("/admin/users", get(list_users))
        // .route(
        //     "/admin/users/:id",
        //     get(admin_get_edit_user).post(admin_post_edit_user),
        // )
        .route("/log-out", get(log_out))
        .route("/send-invite", get(get_send_invite).post(post_send_invite))
        .layer(middleware::from_fn(restrict_to_user))
        .with_state(state.clone());

    let public_routes = Router::new()
        .route("/log-in/:uid", get(get_cookie))
        .route("/log-in", get(get_log_in).post(post_log_in))
        .route("/sign-up/:uid", get(get_sign_up).post(post_sign_up))
        .with_state(state);

    private_routes.merge(public_routes)
}
