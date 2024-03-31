#![allow(dead_code, unused_imports)] // TODO remove

use clap::Parser;
use eyre::Result;
use serde::Deserialize;
use std::io::Read;
use std::{collections::HashMap, fs::File, path::PathBuf, thread};
use tracing::info;

#[derive(Debug, Parser)]
struct Args {
    manifest: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    root: RootConfig,
    r#static: StaticConfig,
}

#[derive(Debug, Deserialize)]
struct RootConfig {
    location: LocationConfig,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum LocationConfig {
    Absolute(PathBuf),
    Command(Vec<String>),
}

#[derive(Debug, Deserialize)]
struct StaticConfig {
    location: LocationConfig,
    result: PathBuf,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    {
        use tracing_subscriber::{fmt, prelude::*, EnvFilter};
        tracing_subscriber::registry()
            .with(fmt::layer().without_time().with_line_number(true))
            .with(EnvFilter::from_default_env())
            .init();
    }

    let args = Args::parse();

    let parsed: Config = {
        let f = File::open(&args.manifest)?;
        serde_json::from_reader(f)?
    };

    info!("{parsed:#?}");

    Ok(())
}
