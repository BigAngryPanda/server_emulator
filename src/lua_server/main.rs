pub mod server;
pub mod lua_handler;

use mlua::{
    Lua,
    Table
};

fn main() {
    let script_content = std::fs::read_to_string("scripts/test.lua").unwrap();

    let lua = Lua::new();

    lua.globals().set("Server", lua.create_proxy::<server::Server>().unwrap()).unwrap();

    {
        let code = std::fs::read_to_string("scripts/server.lua").unwrap();
        let module_loader = lua.load(&code).into_function().unwrap();

        // Register it in package.preload
        let package: Table = lua.globals().get("package").unwrap();
        let preload: Table = package.get("preload").unwrap();
        preload.set("server", module_loader).unwrap();
    }

    lua.load(&script_content).exec().unwrap();
}