use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, Router};

use crate::apps::user::layers::restrict_to_user;
use crate::state::AppState;

use super::handlers::hello_handler;

pub fn get_routes(state: Arc<AppState>) -> Router {
    let private_routes = Router::new()
        .route("/admin/XXXXX", get(hello_handler))
        .layer(middleware::from_fn(restrict_to_user))
        .with_state(state.clone());

    let public_routes = Router::new()
        .route("/XXXXXXX", get(hello_handler))
        .with_state(state);

    private_routes.merge(public_routes)
}
