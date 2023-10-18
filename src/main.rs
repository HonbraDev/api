mod app;
mod config;
mod error;
mod reqwest;
mod youtube;

use eyre::Context;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use tokio::runtime::Runtime;
use tracing::{debug, field, Level};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use self::app::build_app;
use crate::config::Config;

fn main() -> eyre::Result<()> {
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

    let config: Config = Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::raw())
        .extract()
        .context("failed to parse config")?;

    let rt = Runtime::new().context("failed to create tokio runtime")?;

    debug!(addr = field::display(&config.listen_addr), "binding");

    rt.block_on(async move {
        Ok::<(), eyre::Error>(
            axum::Server::try_bind(&config.listen_addr)
                .context("unable to bind to server address")?
                .serve(build_app(config).into_make_service())
                .await
                .context("server encountered a runtime error")?,
        )
    })?;

    Ok(())
}
