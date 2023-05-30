use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn authorized<B>(request: Request<B>, next: Next<B>) -> Result<Response, StatusCode>
where
    B: Send,
{
    let (mut parts, body) = request.into_parts();
    todo!()
}
