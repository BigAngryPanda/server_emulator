use mlua::Table;

use server_emulator_macro::log_impl_calls;

use crate::interfaces::con_plugin::MT5ConPlugin;

use crate::lua_server::{
    lua_object::LuaObject,
    lua_object::LuaConstructible,
    lua_handler::LuaHandler
};

use crate::mt5_apiserver::{
    IMTConPlugin,
    IMTConPlugin__bindgen_vtable
};

use crate::vtable_impl;

use std::sync::Arc;

pub struct ConPlugin {
    lua: Arc<LuaHandler>,
    lua_impl: Table,
    name: Vec<u16>
}

#[log_impl_calls]
impl ConPlugin {
    const VTABLE: IMTConPlugin__bindgen_vtable = vtable_impl::con_plugin::new();
}

impl LuaObject for ConPlugin {
    fn lua_impl(&self) -> Table {
        self.lua_impl.clone()
    }

    fn lua_handler(&self) -> &LuaHandler {
        &self.lua
    }
}

impl LuaConstructible for ConPlugin {
    type MTType = IMTConPlugin;

    fn new(lua: Arc<LuaHandler>, lua_impl: Table) -> Self {
        let mut result = Self {
            lua_impl,
            lua,
            name: Vec::new()
        };

        result.name = result.call_str("name");

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
impl MT5ConPlugin for ConPlugin {
    fn release(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut ConPlugin as *mut u8, std::alloc::Layout::new::<ConPlugin>());
        };
    }

    fn name(&self) -> *const u16 {
        self.name.as_ptr()
    }

    fn server(&self) -> u64 {
        self.call_int("server", 1)
    }
}
