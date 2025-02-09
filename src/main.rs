#![allow(dead_code)]

mod api;
mod exec;
mod node;

fn main() -> eyre::Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    tracing_subscriber::registry()
        .with(fmt::layer().without_time())
        .with(EnvFilter::from_env("AM_LOG"))
        .init();

    color_eyre::install()?;

    api::main()?;

    Ok(())
}
