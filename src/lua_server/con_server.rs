use mlua::Table;

use server_emulator_macro::log_impl_calls;

use crate::mt5_apiserver::{
    IMTConServer,
    IMTConServer__bindgen_vtable
};

use crate::interfaces::con_server::MT5ConServer;

use crate::vtable_impl;

pub struct ConServer {
    pub lua_impl: Table
}

#[log_impl_calls]
impl ConServer {
    const CON_SERVER_VTABLE: IMTConServer__bindgen_vtable = vtable_impl::con_server::new();

    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    fn new(lua_impl: Table) -> ConServer {
        ConServer {
            lua_impl
        }
    }

    pub fn alloc(lua_impl: Table) -> *mut ConServer {
        Box::into_raw(Box::new(ConServer::new(lua_impl)))
    }

    pub fn alloc_con_server(self_ptr: *mut dyn MT5ConServer) -> *mut IMTConServer {
        let con_server = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTConServer>()) as *mut IMTConServer
        };

        unsafe {
            (*con_server).vtable_ = &Self::CON_SERVER_VTABLE as *const IMTConServer__bindgen_vtable;
            (*con_server).impl_ptr = self_ptr;
        }

        con_server
    }
}

#[log_impl_calls]
impl MT5ConServer for ConServer {
    fn address(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn release(&mut self) {
        unsafe {
            std::alloc::dealloc(self as *mut ConServer as *mut u8, std::alloc::Layout::new::<ConServer>());
            std::ptr::drop_in_place(self);
        };
    }
}
