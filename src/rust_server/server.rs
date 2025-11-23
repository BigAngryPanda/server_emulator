use crate::{
    mt5_apiserver::*,
    interfaces::server::MT5Server,
    indexed_storage::IndexedStorage,
};

use crate::{
    vtable_impl,
};

use crate::rust_server::{
    con_server,
    con_common,
    con_plugin,
    user_sink,
    user,
    trade_sink,
    symbol,
    order,
    request
};

use server_emulator_macro::{
    log_impl_calls
};

use std::os::raw::{
    c_longlong,
    c_uint,
    c_ulonglong,
};

pub struct Server {
    plugin: libloading::Library,
    plugin_vtable: *mut IMTServerPlugin,
    server_api: *mut IMTServerAPI,
    con_plugin: con_plugin::ConPlugin,
    mt_user_sinks: IndexedStorage<*mut IMTUserSink>,
    mt_trade_sinks: IndexedStorage<*mut IMTTradeSink>,
}

impl Server {
    fn plugin_vtable(&self) -> &IMTServerPlugin__bindgen_vtable {
        unsafe { &(*(*self.plugin_vtable).vtable_) }
    }
}

impl Server {
    const SERVER_VTABLE: IMTServerAPI__bindgen_vtable = vtable_impl::server::new();
    const CON_PLUGIN_VTABLE: IMTConPlugin__bindgen_vtable = vtable_impl::con_plugin::new();
    const CON_SERVER_VTABLE: IMTConServer__bindgen_vtable = vtable_impl::con_server::new();

    pub fn new(path: &str) -> Server {
        let plugin = unsafe {
            libloading::Library::new(path).expect("Failed to get library")
        };

        Server {
            plugin,
            plugin_vtable: std::ptr::null_mut(),
            server_api: std::ptr::null_mut(),
            con_plugin: con_plugin::ConPlugin::new(),
            mt_user_sinks: IndexedStorage::new(),
            mt_trade_sinks: IndexedStorage::new(),
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
            self.plugin_vtable = data;
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

        self.server_api = server_api;

        unsafe {
            (self.plugin_vtable().IMTServerPlugin_Start)(
                self.plugin_vtable,
                self.server_api);
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            (self.plugin_vtable().IMTServerPlugin_Stop)(
                self.plugin_vtable
            );
        }
    }

    pub fn user_sink_exist(&self, id: usize) -> bool {
        self.mt_user_sinks.contains(id)
    }

    pub fn user_sink(&self, id: usize) -> user_sink::UserSink {
        user_sink::UserSink::new(self.mt_user_sinks[id])
    }

    pub fn trade_sink_exist(&self, id: usize) -> bool {
        self.mt_trade_sinks.contains(id)
    }

    pub fn trade_sink(&self, id: usize) -> trade_sink::TradeSink {
        trade_sink::TradeSink::new(self.mt_trade_sinks[id])
    }
}

#[log_impl_calls]
impl MT5Server for Server {
    fn about(&mut self, info: &mut MTServerInfo) -> c_uint {
        0
    }

    fn license_check(&mut self, license_name: *const u16) -> c_uint {
        0
    }

    fn common_create(&mut self) -> *mut IMTConCommon {
        let con_common = con_common::ConCommon::alloc();

        let mt_con_common = con_common::ConCommon::alloc_con_server(con_common);

        mt_con_common
    }

    fn common_get(&mut self, common: &mut IMTConCommon) -> c_uint {
        0
    }

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

    fn time_current_msc(&mut self) -> c_longlong {
        use std::time::{SystemTime, UNIX_EPOCH};

        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as c_longlong
    }

    fn user_subscribe(&mut self, sink: &mut IMTUserSink) -> c_uint {
        self.mt_user_sinks.insert(sink as *mut IMTUserSink);

        0
    }

    fn user_create(&mut self) -> *mut IMTUser {
        let user = user::User::alloc();

        let mt_user = user::User::alloc_mt_user(user);

        mt_user
    }

    fn trade_subscribe(&mut self, sink: &mut IMTTradeSink) -> c_uint {
        self.mt_trade_sinks.insert(sink as *mut IMTTradeSink);

        0
    }

    fn symbol_create(&mut self) -> *mut IMTConSymbol {
        let symbol = symbol::Symbol::alloc();

        let con_symbol = symbol::Symbol::alloc_mt_con_symbol(symbol);

        con_symbol
    }

    fn trade_request_create(&mut self) -> *mut IMTRequest {
        let request = request::Request::alloc();

        let mt_request = request::Request::alloc_mt_request(request);

        mt_request
    }

    fn order_create(&mut self) -> *mut IMTOrder {
        let order = order::Order::alloc();

        let mt_order = order::Order::alloc_mt_request(order);

        mt_order
    }

    fn symbol_next(
        &mut self,
        pos: c_uint,
        symbol: &mut IMTConSymbol,
    ) -> c_uint {
        1
    }
}
