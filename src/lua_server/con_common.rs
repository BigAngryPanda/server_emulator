use mlua::{
    Table,
    Value,
    MultiValue
};

use server_emulator_macro::log_impl_calls;

use crate::mt5_apiserver::{
    IMTConCommon,
    IMTConCommon__bindgen_vtable
};

use crate::interfaces::con_common::MT5ConCommon;

use crate::lua_server::lua_object::LuaObject;

use crate::vtable_impl;

use std::os::raw::c_uint;

pub struct ConCommon {
    lua_impl: Table,
}

impl ConCommon {
    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    const CON_COMMON_VTABLE: IMTConCommon__bindgen_vtable = vtable_impl::con_common::new();

    pub fn new(lua_impl: Table) -> ConCommon {
        ConCommon {
            lua_impl
        }
    }

    pub fn alloc(lua_impl: Table) -> *mut ConCommon {
        Box::into_raw(Box::new(ConCommon::new(lua_impl)))
    }

    pub fn alloc_con_server(self_ptr: *mut dyn MT5ConCommon) -> *mut IMTConCommon {
        let con_server = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTConCommon>()) as *mut IMTConCommon
        };

        unsafe {
            (*con_server).vtable_ = &Self::CON_COMMON_VTABLE as *const IMTConCommon__bindgen_vtable;
            (*con_server).impl_ptr = self_ptr;
        }

        con_server
    }

    pub fn as_lua_arg(self_ptr: *mut ConCommon) -> MultiValue {
        let mut result = MultiValue::new();

        result.push_back(Value::Table(unsafe { (*self_ptr).lua_impl.clone() }));

        result
    }
}

impl LuaObject for ConCommon {
    fn lua_impl(&self) -> Table {
        self.lua_impl.clone()
    }
}

#[log_impl_calls]
impl MT5ConCommon for ConCommon {
    fn release(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut ConCommon as *mut u8, std::alloc::Layout::new::<ConCommon>());
        };
    }

    fn assign(&mut self, param: &IMTConCommon) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn name1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn name(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn name_full(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn owner(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn owner_id(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn owner_host(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn owner_email(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn product(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn account_url(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn account_deposit_url(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }

    fn account_withdrawal_url(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }
}
