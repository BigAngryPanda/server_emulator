use mlua::{
    Lua,
    Table,
    UserData,
    UserDataMethods,
    Value
};

use crate::mt5_apiserver::{
    IMTServerPlugin,
    IMTServerAPI,
    MTServerInfo,
    IMTConCommon,
    IMTConPlugin,
    IMTConServer,
    IMTUserSink,
    IMTUser,
    IMTTradeSink,
    IMTConSymbol,
    IMTOrder,
    IMTRequest,
    IMTServerPlugin__bindgen_vtable,
    IMTServerAPI__bindgen_vtable,
    IMTConPlugin__bindgen_vtable,
    IMTConServer__bindgen_vtable
};

use crate::{
    vtable_impl,
};

use crate::lua_server::{
    lua_handler::LuaHandler,
    lua_object::LuaConstructible
};

use crate::interfaces::{
    server::MT5Server,
};

use server_emulator_macro::{
    log_impl_calls
};

use crate::lua_server::{
    con_common::ConCommon,
    con_plugin::ConPlugin,
    con_server::ConServer,
    order::Order,
    user::User,
    symbol::Symbol,
    request::Request,
    trade_sink::TradeSink,
};

use crate::lua_server::lua_object::{
    LuaObject,
    as_lua_value
};

use crate::conversion::{
    to_utf16_str,
    copy_unaligned,
    FromInt64
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

use std::sync::Arc;

pub struct Server {
    lua: Arc<LuaHandler>,
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
            lua: Arc::new(LuaHandler::new(lua)),
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

    fn read_str(&self, name: &str, dst: *mut u16, max_bytes: usize) {
        let utf16_name = self.call_str(name);

        copy_unaligned(
            utf16_name.as_ptr(),
            dst,
            std::cmp::min(utf16_name.len(), max_bytes));
    }

    fn read_int<T: FromInt64>(&self, name: &str, dst: *mut T) {
        match self.call(name) {
            Value::Integer(ver) => {
                unsafe {
                    std::ptr::write_unaligned(dst, FromInt64::from_i64(ver));
                }
            }
            _ => {}
        }
    }
}

impl LuaObject for Server  {
    fn lua_impl(&self) -> Table {
        self.server_impl.clone()
    }

    fn lua_handler(&self) -> &LuaHandler {
        &self.lua
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
    fn about(&mut self, info: &mut MTServerInfo) -> u32 {
        self.read_str("get_platform_name",  &raw mut info.platform_name  as *mut u16, 63);
        self.read_str("get_platform_owner", &raw mut info.platform_owner as *mut u16, 127);

        self.read_int("get_server_version", &raw mut info.server_version);
        self.read_int("get_server_build",   &raw mut info.server_build);
        self.read_int("get_server_type",    &raw mut info.server_type);
        self.read_int("get_server_id",      &raw mut info.server_type);

        self.call_int("about", 1)
    }

    fn license_check(&mut self, license_name: *const u16) -> u32 {
        let name = String::from_utf16_lossy(to_utf16_str(license_name));

        let lua_str = self.lua.create_string(name.as_bytes());

        self.call_int_with_args("license_check", &[Value::String(lua_str)], 1)
    }

    fn common_create(&mut self) -> *mut IMTConCommon {
        match self.lua.call(self.server_impl.clone(), "common_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                ConCommon::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn common_get(&mut self, common: &mut IMTConCommon) -> u32 {
        let args = [as_lua_value::<_, ConCommon>(common.impl_ptr)];

        self.call_int_with_args("common_get", &args, 1)
    }

    fn plugin_create(&mut self) -> *mut IMTConPlugin {
        match self.lua.call(self.server_impl.clone(), "plugin_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                ConPlugin::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn plugin_current(
        &mut self,
        plugin: &mut IMTConPlugin,
    ) -> u32 {
        let args = [as_lua_value::<_, ConPlugin>(plugin.impl_ptr)];

        self.call_int_with_args("plugin_current", &args, 1)
    }

    fn net_server_create(&mut self) -> *mut IMTConServer {
        match self.lua.call(self.server_impl.clone(), "net_server_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                ConServer::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn net_server_get(
        &mut self,
        id: c_ulonglong,
        config: &mut IMTConServer,
    ) -> u32 {
        let args = [
            as_lua_value::<_, ConServer>(config.impl_ptr),
            Value::Integer(id as i64)
        ];

        self.call_int_with_args("net_server_get", &args, 1)
    }

    fn time_current_msc(&mut self) -> c_longlong {
        use std::time::{SystemTime, UNIX_EPOCH};

        //self.lua.call(self.server_impl.clone(), "time_current_msc", MultiValue::new());

        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as c_longlong
    }

    fn user_subscribe(&mut self, sink: &mut IMTUserSink) -> u32 {
        // todo
        0
    }

    fn user_create(&mut self) -> *mut IMTUser {
        match self.lua.call(self.server_impl.clone(), "user_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                User::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn trade_subscribe(&mut self, sink: &mut IMTTradeSink) -> u32 {
        let trade_sink = self.lua.wrap_value(TradeSink::new(self.lua.clone(), sink));

        self.call_int_with_args("trade_subscribe", &[trade_sink], 1)
    }

    fn symbol_create(&mut self) -> *mut IMTConSymbol {
        match self.lua.call(self.server_impl.clone(), "symbol_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                Symbol::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn trade_request_create(&mut self) -> *mut IMTRequest {
        match self.lua.call(self.server_impl.clone(), "trade_request_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                Request::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn order_create(&mut self) -> *mut IMTOrder {
        match self.lua.call(self.server_impl.clone(), "order_create", self.prepare_args(&[])) {
            Value::Table(lua_impl) => {
                Order::alloc(self.lua.clone(), lua_impl)
            }
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn symbol_next(
        &mut self,
        pos: c_uint,
        symbol: &mut IMTConSymbol,
    ) -> u32 {
        let args = [
            as_lua_value::<_, Symbol>(symbol.impl_ptr),
            Value::Integer(pos as i64)
        ];

        self.call_int_with_args("symbol_next", &args, 1)
    }
}
