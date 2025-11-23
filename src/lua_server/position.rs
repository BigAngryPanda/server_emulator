use mlua::Table;

use server_emulator_macro::log_impl_calls;

use crate::mt5_apiserver::{
    IMTPosition,
    IMTPosition__bindgen_vtable
};

use crate::interfaces::mt_position::MTPosition;

use crate::lua_server::{
    lua_object::LuaObject,
    lua_object::LuaConstructible,
    lua_handler::LuaHandler
};

use crate::vtable_impl;

use std::sync::Arc;

pub struct Position {
    lua_impl: Table,
    lua: Arc<LuaHandler>
}

#[log_impl_calls]
impl Position {
    const VTABLE: IMTPosition__bindgen_vtable = vtable_impl::mt_position::new();
}

impl LuaObject for Position {
    fn lua_impl(&self) -> Table {
        self.lua_impl.clone()
    }

    fn lua_handler(&self) -> &LuaHandler {
        &self.lua
    }
}

impl LuaConstructible for Position {
    type MTType = IMTPosition;

    fn new(lua: Arc<LuaHandler>, lua_impl: Table) -> Self {
        Self {
            lua_impl,
            lua
        }
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
impl MTPosition for Position {
    fn release(&mut self) {
        self.free();
    }
}
