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
use tracing::debug;


const LIB_NAME: &str = "activation-manager";

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
    lib.set("debug", lua.create_function(|_, v: Value| {
        debug!("{v:?}");
        Ok(())
    })?)?;

    drop(lib);
    drop(lua_globals);
    drop(lua_package);
    drop(lua_loaded);

    Ok(lua)
}

/// From a table, return the first unnamed value from the table for type T, or by the name provided
pub(crate) fn get_t<'lua, 's, T>(ctx: &'lua Lua, table: Table<'lua>, name: Option<&'s str>) -> mlua::Result<T>
where
    'lua: 's,
    T: FromLua<'lua>,
{
    for pairs in table.pairs::<Value, T>() {
        match pairs {
            Ok((index, value)) => {
                match (name, index) {
                    (None, Value::Integer(_)) => return Ok(value),
                    (Some(name), Value::String(index)) => {
                        let index = index.to_str()?;
                        if *name == *index {
                            return Ok(value)
                        }
                    }
                    _ => continue
                }
            }
            Err(err) => match err {
                LuaError::FromLuaConversionError { .. } => continue,
                other => return Err(other)
            }
        }
    }

    return Err(LuaError::RuntimeError(format!("Couldn't find value of type {} in table", std::any::type_name::<T>())));
}
