use mlua::Table;

use server_emulator_macro::log_impl_calls;

use crate::mt5_apiserver::{
    IMTRequest,
    IMTRequest__bindgen_vtable
};

use crate::interfaces::mt_request::MTRequest;

use crate::lua_server::{
    lua_object::LuaObject,
    lua_object::LuaConstructible,
    lua_handler::LuaHandler
};

use crate::vtable_impl;

use std::sync::Arc;

pub struct Request {
    lua_impl: Table,
    lua: Arc<LuaHandler>,
    symbol: Vec<u16>,
    group: Vec<u16>
}

#[log_impl_calls]
impl Request {
    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    const VTABLE: IMTRequest__bindgen_vtable = vtable_impl::mt_request::new();
}

impl LuaObject for Request {
    fn lua_impl(&self) -> Table {
        self.lua_impl.clone()
    }

    fn lua_handler(&self) -> &LuaHandler {
        &self.lua
    }
}

impl LuaConstructible for Request {
    type MTType = IMTRequest;

    fn new(lua: Arc<LuaHandler>, lua_impl: Table) -> Self {
        let mut result = Self {
            lua_impl,
            lua,
            symbol: Vec::new(),
            group: Vec::new()
        };

        result.symbol = result.call_str("symbol");
        result.group  = result.call_str("group");

        result
    }

    fn mt_type(self_ptr: *mut Self, mt_obj: &mut Self::MTType) {
        mt_obj.vtable_ = &Self::VTABLE;
        mt_obj.impl_ptr = self_ptr;
    }

    fn free_by_ptr(self_ptr: *mut Self::MTType) {
        unsafe { (*(*self_ptr).impl_ptr).release(); }
    }
}

#[log_impl_calls]
impl MTRequest for Request {
    fn release(&mut self) {
        self.free();
    }

    fn login(&self) -> u64 {
        self.call_int("login", 0)
    }

    fn group(&self) -> *const u16 {
        self.group.as_ptr()
    }

    fn symbol(&self) -> *const u16 {
        self.symbol.as_ptr()
    }

    fn action(&self) -> u32 {
        self.call_int("action", 0)
    }

    fn type_(&self) -> u32 {
        self.call_int("type", 0)
    }

    fn flags(&self) -> u64 {
        self.call_int("flags", 0)
    }

    fn volume_ext(&self) -> u64 {
        self.call_int("volume_ext", 0)
    }
}
