use axum::response::{IntoResponse, Html};
use log::info;

use crate::error::Result;

pub async fn hello_handler(
) -> Result<impl IntoResponse> {
    info!("Hit a route");

    Ok(Html("todo"))
}
