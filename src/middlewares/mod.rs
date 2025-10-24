// Middleware module

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tracing::info;

pub async fn logging_middleware(request: Request, next: Next) -> Response {
    let method = request.method().clone();
    let uri = request.uri().clone();

    info!("Request: {} {}", method, uri);

    let response = next.run(request).await;

    info!("Response: {}", response.status());

    response
}