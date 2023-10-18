mod app;
mod error;
mod reqwest;
mod youtube;

use std::net::SocketAddr;

use eyre::Context;
use tracing::{debug, field, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use self::app::build_app;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let filter = filter::Targets::new()
        .with_target("honbra_api", Level::TRACE)
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_default(Level::INFO);

    let tracing_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(tracing_layer)
        .with(filter)
        .try_init()
        .map_err(eyre::Error::msg)
        .context("failed to initialize tracing subscriber")?;

    let addr: SocketAddr = ([0, 0, 0, 0], 3000).into();

    debug!(addr = field::display(addr), "binding");

    axum::Server::try_bind(&addr)
        .context("unable to bind to server address")?
        .serve(build_app().into_make_service())
        .await
        .context("server encountered a runtime error")?;

    Ok(())
}
