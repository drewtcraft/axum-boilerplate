use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get_service,
    Router,
};
use tower_http::services::ServeDir;

use crate::apps::user;

pub fn get_routes() -> Router {
    Router::new()
        .merge(user::routes::routes())
        .merge(static_routes())
        .fallback(page_not_found)
}

fn static_routes() -> Router {
    // ex. "BASE_URL/public/js/htmx.min.js"
    Router::new().nest_service("/public", get_service(ServeDir::new("./public")))
}

async fn page_not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<p><strong>404</strong> Page Not Found</p>"),
    )
}
