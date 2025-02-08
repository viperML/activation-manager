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
        lua.create_function(move |lua, input: Table| {
            let node = crate::node::file_from_lua(input).unwrap();
            tx.send(node).unwrap();
            Ok(())
        })?,
    )?;

    load_module(&lua, "am", &module)?;

    let res: LuaValue = lua.load(args.file.as_path()).eval()?;
    println!("{res:?}");

    while let Ok(next) = rx.try_recv() {
        println!("{next:?}");
    }

    Ok(())
}
