use std::path::PathBuf;

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

    let f = lua.create_function(|lua, input: Table| {
        let n = crate::node::file_from_lua(&lua, input);
        println!("{n:?}");
        Ok(())
    })?;

    module.set(
        "file",
        f
    )?;

    load_module(&lua, "am", &module)?;

    let res: LuaValue = lua.load(args.file.as_path()).eval()?;
    println!("{res:?}");

    Ok(())
}
