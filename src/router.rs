use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};

use crate::apps::user;

pub fn get_routes() -> Router {
    Router::new()
        .merge(user::routes::routes())
        // .layer(user::layers::authorized)
        .fallback(page_not_found)
}

async fn page_not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<p><strong>404</strong> Page Not Found</p>"),
    )
}
