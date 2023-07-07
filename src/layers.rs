use crate::templates::ErrorTemplate;
use axum::{
    http::{HeaderMap, HeaderName, HeaderValue, Request},
    middleware::Next,
    response::{Html, IntoResponse, Response},
};
use log::{info, warn};

use crate::{
    context::Context,
    error::{Error, Result},
};

pub async fn title_mapper(res: Response) -> Response {
    let context = res.extensions().get::<Context>();
    if context.is_none() {
        return res;
    }
    let context = context.unwrap().clone();
    if context.is_htmx.is_some() && context.is_htmx.unwrap() && context.page_title.is_some() {
        let mut new_response = Response::from(res);
        new_response.headers_mut().insert(
            HeaderName::from_static("HX-Title"),
            HeaderValue::from_str(&context.page_title.unwrap()).unwrap(),
        );
        new_response
    } else {
        res
    }
}

pub async fn result_mapper(res: Response) -> Response {
    let error = res.extensions().get::<Error>();

    match error {
        Some(err) => warn!("err: {}", err.as_ref()),
        None => info!("no errors"),
    }

    error
        .as_ref()
        .map(|server_error| {
            let (status_code, client_error) = server_error.status_and_client_error();
            let status_code_str = status_code.to_string();
            let html = ErrorTemplate::new(client_error.as_ref(), &status_code_str);
            (status_code, Html(html)).into_response()
        })
        .unwrap_or(res)
}

pub async fn is_htmx<B>(
    header_map: HeaderMap,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response> {
    // HTMX sends this header by default
    if !header_map.contains_key("hx-request") {
        return Ok(next.run(request).await);
    }
    // The header is present, so mutate or create the context value is_htmx
    let context = request.extensions_mut().get_mut::<Context>();
    if let Some(context) = context {
        context.set_is_htmx(true);
        // request.extensions_mut().insert(val)
    } else {
        request
            .extensions_mut()
            .insert(Context::new(None, Some(true)));
    }

    // run executes remaining middleware
    Ok(next.run(request).await)
}
