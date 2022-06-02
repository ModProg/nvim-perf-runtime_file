use std::time::{Duration, Instant};

use mlua::prelude::*;
use nvim::{LogLevel, Vim};

#[mlua::lua_module]
fn perf(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "hello",
        lua.create_function(|lua, ()| {
            let vim = Vim::from(lua);
            let now = Instant::now();
            vim.api().nvim_get_runtime_file("/config/*.yml", true);
            vim.api().nvim_get_runtime_file("/config/*.yaml", true);
            vim.api().nvim_get_runtime_file("/config/*.json", true);
            vim.notify(
                &format!("Seperate requests: {}", now.elapsed().as_micros()),
                LogLevel::Error,
                None,
            );
            let now = Instant::now();
            vim.api().nvim_get_runtime_file("/config/*.{yml,yaml,json}", true);
            vim.notify(
                &format!("One request: {}", now.elapsed().as_micros()),
                LogLevel::Error,
                None,
            );
            Ok(())
        })?,
    )?;
    Ok(exports)
}
