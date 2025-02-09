use std::path::PathBuf;
use std::sync::mpsc;

use mlua::prelude::*;
use mlua::LuaSerdeExt;
use mlua::Table;

use serde::Deserialize;

#[derive(Debug, clap::Parser)]
struct Args {
    file: PathBuf,
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
    println!("{args:?}");

    let lua = Lua::new();

    let module = lua.create_table()?;

    let (tx, rx) = mpsc::channel();

    module.set(
        "file",
        lua.create_function(move |_, input: Table| {
            let node = crate::node::file_from_lua(input)?;
            let id = node.id.clone();
            tx.send(node).unwrap();
            Ok(id)
        })?,
    )?;

    load_module(&lua, "am", &module)?;

    lua.load(args.file.as_path()).exec()?;

    let mut nodes = Vec::new();
    while let Ok(next) = rx.try_recv() {
        nodes.push(next);
    }
    println!("{nodes:#?}");

    crate::exec::run_graph(&nodes);

    Ok(())
}
