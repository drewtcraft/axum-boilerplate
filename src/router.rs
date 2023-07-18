use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{Html, IntoResponse},
    routing::get_service,
    Router,
};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

use crate::{
    apps::user::{self, layers::pull_user_id_from_session_uid},
    layers,
    state::AppState,
    error::Result,
};

pub fn get_routes(state: Arc<AppState>) -> Router {
    Router::new()
        .merge(user::routes::get_routes(state.clone()))
        .layer(middleware::map_response(layers::result_mapper))
        .layer(middleware::from_fn(layers::is_htmx))
        .layer(middleware::from_fn_with_state(
            state,
            pull_user_id_from_session_uid,
        ))
        .layer(CookieManagerLayer::new()) // required to do anything with cookies
        .merge(static_routes())
        .fallback(page_not_found)
}

fn static_routes() -> Router {
    // ex. "BASE_URL/public/js/htmx.min.js"
    Router::new().nest_service("/public", get_service(ServeDir::new("./public")))
}

async fn page_not_found() -> Result<impl IntoResponse> {
    
    Ok((
        StatusCode::NOT_FOUND,
        Html("<p><strong>404</strong> Page Not Found</p>"),
    ))
}
