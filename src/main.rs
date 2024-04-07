// #![allow(dead_code, unused_imports)] // TODO remove
mod lua;

use clap::{Args, Parser};
use eyre::{bail, Context, Result};
use mlua::prelude::*;
use petgraph::prelude::*;
use std::{collections::HashMap, fs::File, path::PathBuf};
use tracing::debug;

#[derive(Debug, Parser)]
enum AppArgs {
    Activate(ActivateArgs),
}

#[derive(Debug, Args)]
struct ActivateArgs {
    manifest: PathBuf,
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

    let args = AppArgs::parse();

    match args {
        AppArgs::Activate(args) => args.run(),
    }
}

impl ActivateArgs {
    fn run(self) -> eyre::Result<()> {
        let lua = crate::lua::init()?;

        lua.load(self.manifest.as_path())
            .exec()
            .wrap_err("Evaluating manifest")?;

        Ok(())
    }
}
