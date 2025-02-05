use std::path::PathBuf;

use mlua::prelude::*;
use mlua::Table;

#[derive(Debug, clap::Parser)]
struct Args {
    file: PathBuf,
}

pub fn main() -> eyre::Result<()> {
    let args = <Args as clap::Parser>::parse();
    println!("{args:?}");

    let lua = Lua::new();

    let globals = lua.globals();
    let package: Table = globals.get("package").unwrap();
    let loaded: Table = package.get("loaded").unwrap();

    let module = lua.create_table().unwrap();
    loaded.set("am", module).unwrap();

    let res: LuaValue = lua.load(args.file.as_path()).eval().unwrap();
    println!("{res:?}");



    Ok(())
}
