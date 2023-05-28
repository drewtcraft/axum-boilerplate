use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let routes = Router::new().fallback(page_not_found);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

async fn page_not_found() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html("<p><strong>404</strong> Page Not Found</p>"),
    )
}
