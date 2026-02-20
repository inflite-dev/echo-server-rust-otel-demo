use axum::body::Body;
use axum::http::{HeaderMap, HeaderValue, Request, StatusCode};
use axum::routing::{get, Router};
use std::net::SocketAddr;

async fn read_echo() -> StatusCode {
    StatusCode::OK
}

async fn write_echo_stream(request: Request<Body>) -> (HeaderMap, Body) {
    (
        HeaderMap::from_iter([(
            axum::http::header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        )]),
        Body::from_stream(request.into_body().into_data_stream()),
    )
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = Router::new().route(
        "/",
        get(read_echo)
            .post(write_echo_stream)
            .put(write_echo_stream),
    );

    let addr = SocketAddr::from(([0, 0, 0, 0], 5000));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    let server = axum::serve(listener, app);

    if let Err(err) = server.await {
        eprintln!("server error: {err}");
    }
    Ok(())
}
