#![allow(dead_code)]

mod api;
mod exec;
mod exec_node;
mod gsettings;
mod node;

fn main() -> eyre::Result<()> {
    use std::str::FromStr;
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    let level = std::env::var("AM_LOG").unwrap_or(String::from("warn"));

    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::from_str(&level)?)
        .init();

    tracing::trace!(?level);

    color_eyre::install()?;

    api::main()?;

    Ok(())
}
