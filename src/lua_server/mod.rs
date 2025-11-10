pub mod server;
pub mod lua_handler;

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

pub fn lua_test() {
    let script_content = std::fs::read_to_string("scripts/test.lua").unwrap();

    let lua = Lua::new();

    lua.globals().set("Server", lua.create_proxy::<server::Server>().unwrap()).unwrap();

    load_module(&lua, "scripts", "server");

    lua.load(&script_content).exec().unwrap();
}
