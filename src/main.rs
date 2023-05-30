use axum;
use std::net::SocketAddr;

mod apps;
mod error;
mod router;

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    println!("listening on {addr}");

    axum::Server::bind(&addr)
        .serve(router::get_routes().into_make_service())
        .await
        .unwrap();
}
