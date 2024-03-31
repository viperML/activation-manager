#![allow(dead_code, unused_imports)] // TODO remove

use clap::Parser;
use eyre::{bail, Result};
use petgraph::algo::toposort;
use petgraph::data::DataMap;
use petgraph::prelude::*;
use petgraph::visit::Walker;
use serde::Deserialize;
use std::env;
use std::ffi::OsString;
use std::io::Read;
use std::os::unix::ffi::OsStringExt;
use std::process::{self, Command, Stdio};
use std::{collections::HashMap, fs::File, path::PathBuf, thread};
use tracing::{debug, info};

#[derive(Debug, Parser)]
struct Args {
    manifest: PathBuf,
}

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    root: RootConfig,
    r#static: StaticConfig,
    nodes: HashMap<String, NodeConfig>,
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

#[derive(Debug, Deserialize)]
struct NodeConfig {
    after: Vec<String>,
    before: Vec<String>,
    #[serde(rename = "generatesNodes")]
    generates_nodes: bool,
    command: Option<Vec<String>>,
}

type Node = (String, NodeConfig);

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

    let config: Config = {
        let f = File::open(&args.manifest)?;
        serde_json::from_reader(f)?
    };
    info!("{config:#?}");

    // Setup root dir
    for (location, var) in [
        (&config.root.location, "AM_ROOT"),
        (&config.r#static.location, "AM_STATIC"),
    ] {
        match location {
            LocationConfig::Absolute(p) => {
                debug!("{var}={p:?}");
                env::set_var(var, p)
            },
            LocationConfig::Command(args) => {
                let mut args = args.iter();
                let mut cmd = Command::new(args.next().unwrap());
                cmd.args(args);
                let output = String::from_utf8(cmd.output()?.stdout)?;
                let v = output.trim();
                debug!("{var}={v}");
                env::set_var(var, v);
            }
        };
    }


    let mut g: Graph<_, ()> = DiGraph::new();

    for (n, v) in &config.nodes {
        let my_index = insert(n.to_owned(), &mut g);

        for dep in &v.after {
            let dep_index = insert(dep.to_owned(), &mut g);
            g.add_edge(dep_index, my_index, ());
        }
        for dep in &v.before {
            let dep_index = insert(dep.to_owned(), &mut g);
            g.add_edge(my_index, dep_index, ());
        }
    }

    debug!("{g:#?}");

    println!(
        "{:?}",
        petgraph::dot::Dot::with_config(&g, &[petgraph::dot::Config::EdgeNoLabel])
    );

    for index in petgraph::visit::Topo::new(&g).iter(&g) {
        let node = &g[index];
        let value = &config.nodes[node];
        debug!(?value);

        println!("Activating {}", node);
        if let Some(ref args) = value.command {
            let args = args.to_owned();
            let mut args = args.iter();
            let mut cmd = Command::new(args.next().unwrap());
            cmd.args(args);
            cmd.stdout(Stdio::inherit());
            cmd.stderr(Stdio::inherit());
            let mut child = cmd.spawn()?;
            let exit = child.wait()?;
            if !exit.success() {
                bail!(exit);
            }
        }
    }

    Ok(())
}

fn insert<N: PartialEq, E>(value: N, g: &mut DiGraph<N, E>) -> NodeIndex {
    for idx in g.node_indices() {
        if value == g[idx] {
            return idx;
        }
    }

    // Node not on graph
    g.add_node(value)
}
