use mlua::{
    Lua,
    Table,
    UserData,
    UserDataMethods,
    MultiValue
};

use crate::mt5_apiserver::{
    IMTServerPlugin,
    IMTServerAPI,
    MTServerInfo,
    IMTConCommon,
    IMTConPlugin,
    IMTConServer,
    IMTServerPlugin__bindgen_vtable,
    IMTServerAPI__bindgen_vtable,
    IMTConPlugin__bindgen_vtable,
    IMTConServer__bindgen_vtable
};

use crate::{
    vtable_impl,
};

use crate::lua_server::lua_handler::LuaHandler;

use crate::interfaces::{
    server::MT5Server,
};

use server_emulator_macro::{
    log_impl_calls
};

use std::os::raw::{
    c_uint,
    c_ulonglong,
    c_longlong
};

use std::sync::atomic::{
    AtomicPtr,
    Ordering::Relaxed
};

pub struct Server {
    lua: LuaHandler,
    server_impl: Table,
    server_api: AtomicPtr<IMTServerAPI>,
    plugin: libloading::Library,
    plugin_vtable: AtomicPtr<IMTServerPlugin>,
}

impl Server {
    const SERVER_VTABLE: IMTServerAPI__bindgen_vtable = vtable_impl::server::new();
    const CON_PLUGIN_VTABLE: IMTConPlugin__bindgen_vtable = vtable_impl::con_plugin::new();
    const CON_SERVER_VTABLE: IMTConServer__bindgen_vtable = vtable_impl::con_server::new();

    pub fn new(lua: Lua, server_impl: Table, path: &str) -> Server {
        let plugin = unsafe {
            libloading::Library::new(path).expect("Failed to get library")
        };

        Server {
            lua: LuaHandler::new(lua),
            server_impl,
            server_api: AtomicPtr::new(std::ptr::null_mut()),
            plugin,
            plugin_vtable: AtomicPtr::new(std::ptr::null_mut())
        }
    }

    pub fn init(&mut self) -> u32 {
        let func: libloading::Symbol<unsafe fn(c_uint, *mut *mut IMTServerPlugin) -> u32> = unsafe {
            self.plugin.get(b"MTServerCreate").unwrap()
        };

        let mut data: *mut IMTServerPlugin = std::ptr::null_mut();

        let ret: u32 = unsafe {
            func(0, &mut data as *mut *mut IMTServerPlugin)
        };

        if ret == 0 {
            self.plugin_vtable.store(data, Relaxed);
        }

        ret
    }

    pub fn start(&mut self) {
        let server_api = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTServerAPI>()) as *mut IMTServerAPI
        };

        unsafe {
            (*server_api).vtable_ = &Self::SERVER_VTABLE as *const IMTServerAPI__bindgen_vtable;
            (*server_api).impl_ptr = self as *mut Server;
        }

        self.server_api.store(server_api, Relaxed);

        unsafe {
            (self.plugin_vtable().IMTServerPlugin_Start)(
                self.plugin_vtable.load(Relaxed),
                self.server_api.load(Relaxed));
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            (self.plugin_vtable().IMTServerPlugin_Stop)(
                self.plugin_vtable.load(Relaxed)
            );
        }
    }

    fn plugin_vtable(&self) -> &IMTServerPlugin__bindgen_vtable {
        unsafe { &(*(*self.plugin_vtable.load(Relaxed)).vtable_) }
    }
}

impl UserData for Server {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("new", |lua, args: (Table, String)| {
            Ok(Server::new(lua.clone(), args.0, args.1.as_str()))
        });

        methods.add_method_mut("init", |_, this, ()| {
            Ok(this.init())
        });

        methods.add_method_mut("start", |_, this, ()| {
            Ok(this.start())
        });

        methods.add_method_mut("stop", |_, this, ()| {
            Ok(this.stop())
        });
    }
}

#[log_impl_calls]
impl MT5Server for Server {
    fn about(&mut self, info: &mut MTServerInfo) -> c_uint {
        0
    }

    fn license_check(&mut self, license_name: &u16) -> c_uint {
        0
    }

    fn common_create(&mut self) -> *mut IMTConCommon {
        self.lua.call(self.server_impl.clone(), "common_create", MultiValue::new());

        std::ptr::null_mut()
    }

    fn common_get(&mut self, common: &mut IMTConCommon) -> c_uint {
        0
    }
/*
    fn plugin_create(&mut self) -> *mut IMTConPlugin {
        unsafe { std::alloc::alloc(std::alloc::Layout::new::<IMTConPlugin>()) as *mut IMTConPlugin }
    }

    fn plugin_current(
        &mut self,
        plugin: &mut IMTConPlugin,
    ) -> c_uint {
        plugin.vtable_ = &Self::CON_PLUGIN_VTABLE as *const IMTConPlugin__bindgen_vtable;
        plugin.impl_ptr = &mut self.con_plugin as *mut con_plugin::ConPlugin;

        0
    }

    fn net_server_create(&mut self) -> *mut IMTConServer {
        let con_server = con_server::ConServer::alloc();

        let mt_con_server = con_server::ConServer::alloc_con_server(con_server);

        mt_con_server
    }

    fn net_server_get(
        &mut self,
        id: c_ulonglong,
        config: &mut IMTConServer,
    ) -> c_uint {
        let _ = unsafe { &mut *(config.impl_ptr as *mut con_server::ConServer) };

        0
    }
*/
    fn time_current_msc(&mut self) -> c_longlong {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as c_longlong
    }
}
