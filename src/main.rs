// #![allow(dead_code, unused_imports)] // TODO remove
mod api;
mod eval;

use eyre::Result;
use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
enum AppArgs {
    Activate(ActivateArgs),
}

#[derive(Debug, clap::Args)]
struct ActivateArgs {
    manifest: PathBuf,
}

fn main() -> Result<()> {
    {
        color_eyre::install()?;
        use tracing_error::ErrorLayer;
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};
        tracing_subscriber::registry()
            .with(fmt::layer().without_time().with_line_number(true))
            .with(EnvFilter::from_default_env())
            .with(ErrorLayer::default())
            .init();
    }

    let args = <AppArgs as clap::Parser>::parse();

    tokio::runtime::Runtime::new()?.block_on(async {
        match args {
            AppArgs::Activate(args) => crate::eval::eval(args.manifest).await,
        }
    })
}
