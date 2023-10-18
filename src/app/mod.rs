mod yt_embed;

use axum::{body::Body, Router};
use http::Request;
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{field, span, Level};

pub fn build_app() -> Router {
    Router::new().nest("/yt-embed", yt_embed::router()).layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                span!(
                    Level::INFO,
                    "http-request",
                    uri = field::display(request.uri()),
                )
            })
            .on_request(DefaultOnRequest::new().level(Level::DEBUG))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    )
}
