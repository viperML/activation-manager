// #![allow(dead_code, unused_imports)] // TODO remove
mod lua;

use clap::{Args, Parser};
use eyre::{bail, Context, Result};
use mlua::prelude::*;
use petgraph::prelude::*;
use tracing_error::ErrorLayer;
use std::{collections::HashMap, fs::File, path::PathBuf};
use tracing::{debug, span};
use mlua::Table;
use tracing::Level;

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
            .with(ErrorLayer::default())
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

        let res: Table = lua.load(self.manifest.as_path())
            .eval()
            .wrap_err("Evaluating manifest")?;

        for (i, pairs) in res.pairs::<LuaValue, LuaValue>().enumerate() {
            let (k,v) = pairs?;
            debug!(?i, ?k, ?v);

            if let LuaValue::Table(t) = v {
                let span = span!(Level::DEBUG, "table", %i);
                let _enter = span.enter();

                // for (j, pairs) in t.pairs::<LuaValue, LuaValue>().enumerate() {
                //     let (k,v) = pairs?;
                //     debug!(?j, ?k, ?v);
                // }

                let v = crate::lua::get_t::<LuaFunction>(&lua, t, None)?;
                debug!(?v);
            }
        }

        Ok(())
    }
}
