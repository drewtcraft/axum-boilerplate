use axum::{
    extract::Query,
    http::{HeaderMap, Request},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    context::Context,
    error::{Error, Result},
};

pub async fn result_mapper(res: Response) -> Response {
    print!("mapping this bitch");
    let error = res.extensions().get::<Error>();
    let error_response = error.as_ref().map(|server_error| {
        let (status_code, client_error) = server_error.status_and_client_error();
        let body = match client_error {
            _ => Json(json!({ "error": client_error.as_ref() })),
        };
        (status_code, body).into_response()
    });
    error_response.unwrap_or(res)
}

pub async fn is_htmx<B>(
    header_map: HeaderMap,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    // HTMX sends this header by default
    if !header_map.contains_key("HX-Request") {
        return Ok(next.run(request).await);
    }

    // The header is present, so mutate or create the context value is_htmx
    let context = request.extensions_mut().get_mut::<Context>();
    if let Some(context) = context {
        context.is_htmx = Some(true);
    } else {
        request
            .extensions_mut()
            .insert(Context::new(None, Some(true)));
    }

    // run executes remaining middleware
    Ok(next.run(request).await)
}
