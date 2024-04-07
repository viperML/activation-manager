use std::ops::Deref;
use std::sync::Arc;
use std::sync::Mutex;

use crate::Result;
use mlua::prelude::*;
use mlua::Nil;
use mlua::Table;
use mlua::UserData;
use mlua::Value;
use serde::Serialize;

const LIB_NAME: &str = "activation-manager";

type NodesInner = Arc<Mutex<Vec<Node>>>;

#[derive(Debug, Clone, FromLua, Default)]
struct Nodes(NodesInner);

impl Deref for Nodes {
    type Target = NodesInner;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct Node
<'lua>
{
    name: String,
    before: Vec<String>,
    after: Vec<String>,

    // func: LuaFunction<'lua>,
}

impl UserData for Nodes {
    fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_method("add", |ctx, this, input: Table| {
            let mut nodes = this
                .lock()
                .map_err(|_| LuaError::RuntimeError("poisoned mutex".to_owned()))?;

            let new_node = Node {
                name: input.get("name").context("missing input field: name")?,
                after: {
                    match input.get::<_, Table>("after") {
                        Ok(t) => table_to_vec(ctx, t)?,
                        Err(_) => Default::default(),
                    }
                },
                before: {
                    match input.get::<_, Table>("before") {
                        Ok(t) => table_to_vec(ctx, t)?,
                        Err(_) => Default::default(),
                    }
                },
                ..todo!()
            };

            nodes.push(new_node);

            tracing::debug!(?nodes);

            Ok(())
        });
    }
}

pub(crate) fn table_to_vec<'lua, T: FromLua<'lua>>(
    ctx: &'lua Lua,
    t: Table<'lua>,
) -> mlua::Result<Vec<T>> {
    let mut res = Vec::new();
    for val in t.sequence_values() {
        let val = val?;
        let parsed = T::from_lua(val, ctx)?;
        res.push(parsed);
    }
    Ok(res)
}

pub(crate) fn init() -> Result<mlua::Lua> {
    let lua = Lua::new();

    let lib: Table = lua.create_table()?;

    // Need to drop globals
    let lua_globals = lua.globals();
    let lua_package: Table = lua_globals.get("package")?;
    let lua_loaded: Table = lua_package.get("loaded")?;
    lua_loaded.set(LIB_NAME, &lib)?;

    lib.set("version", env!("CARGO_PKG_VERSION"))?;

    // add!(lua, debug);
    lib.set("debug", lua.create_function(debug)?)?;

    lib.set("Nodes", lua.create_function(|_, ()| Ok(Nodes::default()))?)?;

    drop(lib);
    drop(lua_globals);
    drop(lua_package);
    drop(lua_loaded);

    Ok(lua)
}

fn debug<'lua>(ctx: &'lua Lua, message: Value) -> mlua::Result<impl IntoLua<'lua>> {
    tracing::debug!("{message:?}");
    Ok(Nil)
}
