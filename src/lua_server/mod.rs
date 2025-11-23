pub mod server;
pub mod lua_handler;
pub mod lua_object;
pub mod option_ptr;
pub mod con_common;
pub mod con_plugin;
pub mod con_server;
pub mod trade_sink;
pub mod order;
pub mod user;
pub mod position;
pub mod request;
pub mod symbol;
pub mod group;
pub mod deal;

use mlua::{
    Lua,
    Table
};

fn load_module(lua: &Lua, dir: &str, mod_name: &str) {
    let code = std::fs::read_to_string(std::format!("{}/{}.lua", dir, mod_name)).unwrap();
    let module_loader = lua.load(&code).into_function().unwrap();

    // Register it in package.preload
    let package: Table = lua.globals().get("package").unwrap();
    let preload: Table = package.get("preload").unwrap();
    preload.set(mod_name, module_loader).unwrap();
}

pub fn run() {
    let script_content = std::fs::read_to_string("scripts/test.lua").unwrap();

    let lua = Lua::new();

    lua.globals().set("Server", lua.create_proxy::<server::Server>().unwrap()).unwrap();

/*
    load_module(&lua, "scripts", "server");
    load_module(&lua, "scripts", "con_common");
    load_module(&lua, "scripts", "con_plugin");
    load_module(&lua, "scripts", "con_server");
    load_module(&lua, "scripts", "con_symbol");
    load_module(&lua, "scripts", "con_group");
    load_module(&lua, "scripts", "deal");
    load_module(&lua, "scripts", "order");
    load_module(&lua, "scripts", "position");
    load_module(&lua, "scripts", "request");
*/

    lua.load(&script_content).exec().unwrap();
}
