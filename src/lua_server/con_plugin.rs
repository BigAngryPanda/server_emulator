use mlua::Table;

use std::os::raw::{
    c_uint
};

use server_emulator_macro::log_impl_calls;

use crate::interfaces::con_plugin::MT5ConPlugin;

use crate::lua_server::lua_object::LuaObject;

use crate::mt5_apiserver::{
    IMTConPlugin,
    IMTConPlugin__bindgen_vtable
};

use crate::vtable_impl;

pub struct ConPlugin {
    pub lua_impl: Table
}

#[log_impl_calls]
impl ConPlugin {
    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    const CON_PLUGIN_VTABLE: IMTConPlugin__bindgen_vtable = vtable_impl::con_plugin::new();

    pub fn new(lua_impl: Table) -> ConPlugin {
        ConPlugin {
            lua_impl
        }
    }

    pub fn alloc(lua_impl: Table) -> *mut ConPlugin {
        Box::into_raw(Box::new(ConPlugin::new(lua_impl)))
    }

    pub fn alloc_con_plugin(self_ptr: *mut dyn MT5ConPlugin) -> *mut IMTConPlugin {
        let con_server = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTConPlugin>()) as *mut IMTConPlugin
        };

        unsafe {
            (*con_server).vtable_ = &Self::CON_PLUGIN_VTABLE as *const IMTConPlugin__bindgen_vtable;
            (*con_server).impl_ptr = self_ptr;
        }

        con_server
    }
}

impl LuaObject for ConPlugin {
    fn lua_impl(&self) -> Table {
        self.lua_impl.clone()
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

    fn name1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn name(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }
}
