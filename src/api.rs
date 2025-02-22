use std::path::PathBuf;
use std::sync::mpsc;

use mlua::prelude::*;
use mlua::LuaSerdeExt;
use mlua::Table;

use serde::Deserialize;
use tracing::debug;
use tracing::trace;

#[derive(Debug, clap::Parser)]
struct Args {
    file: PathBuf,

    #[arg(short = 'n', long)]
    dry: bool,
}

fn load_module<S: AsRef<str>>(lua: &Lua, name: S, module: &Table) -> LuaResult<()> {
    let name = name.as_ref();
    let globals = lua.globals();
    let package: Table = globals.get("package")?;
    let loaded: Table = package.get("loaded")?;
    loaded.set(name, module)?;
    Ok(())
}

pub fn main() -> eyre::Result<()> {
    let args = <Args as clap::Parser>::parse();
    trace!(?args);

    let lua = Lua::new();

    let module = lua.create_table()?;

    let (tx, rx) = mpsc::channel();

    let txx = tx.clone();
    module.set(
        "file",
        lua.create_function(move |_, input: Table| {
            let node = crate::node::file_from_lua(input)?;
            // let id = node.id.clone();
            txx.send(node).unwrap();
            Ok(())
        })?,
    )?;

    module.set(
        "debug",
        lua.create_function(|lua, input: LuaValue| {
            debug!("{input:?}");
            Ok(())
        })?,
    )?;

    let txx = tx.clone();
    module.set(
        "dconf",
        lua.create_function(move |_, input: Table| {
            let node = crate::gsettings::dconf_node(input)?;
            // let id = node.id.clone();
            txx.send(node).unwrap();
            Ok(())
        })?,
    )?;

    let txx = tx.clone();
    module.set(
        "exec",
        lua.create_function(move |_, input: Table| {
            let node = crate::exec_node::lua_to_exec(input)?;
            let id = node.meta.id.clone();
            txx.send(node).unwrap();
            Ok(id)
        })?,
    )?;

    load_module(&lua, "am", &module)?;

    lua.load(args.file.as_path()).exec()?;

    let mut nodes = Vec::new();
    while let Ok(next) = rx.try_recv() {
        nodes.push(next);
    }
    trace!("{nodes:#?}");

    crate::exec::run_graph(&nodes, args.dry)?;

    Ok(())
}
