use axum::routing::{get, post};
use axum::{middleware, Router};

use super::handlers::{get_log_in, log_out, post_log_in};
use super::layers::authorized;

pub fn routes() -> Router {
    let unauthorized_routes = Router::new().route("/log-in", get(get_log_in).post(post_log_in));

    let authorized_routes = Router::new()
        .route("/log-out", get(log_out))
        .layer(middleware::from_fn(authorized));

    authorized_routes.merge(unauthorized_routes)
}
