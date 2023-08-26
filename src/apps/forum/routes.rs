use std::sync::Arc;

use axum::routing::get;
use axum::{middleware, Router};

use crate::apps::user::layers::restrict_to_user;
use crate::state::AppState;

use super::handlers::{get_create_thread, post_create_thread, get_thread};

pub fn get_routes(state: Arc<AppState>) -> Router {
    let private_routes = Router::new()
        .layer(middleware::from_fn(restrict_to_user))
        .with_state(state.clone());

    let public_routes = Router::new()
        .route("/threads/new", get(get_create_thread).post(post_create_thread))
        .route("/threads/:id", get(get_thread))
        .with_state(state);

    private_routes.merge(public_routes)
}
