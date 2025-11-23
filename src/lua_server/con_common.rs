use mlua::Table;

use server_emulator_macro::log_impl_calls;

use crate::mt5_apiserver::{
    IMTConCommon,
    IMTConCommon__bindgen_vtable
};

use crate::interfaces::con_common::MT5ConCommon;

use crate::lua_server::{
    lua_object::LuaObject,
    lua_object::LuaConstructible,
    lua_handler::LuaHandler
};

use crate::vtable_impl;

use std::sync::Arc;

pub struct ConCommon {
    lua_impl:               Table,
    lua:                    Arc<LuaHandler>,
    name:                   Vec<u16>,
    name_full:              Vec<u16>,
    owner:                  Vec<u16>,
    owner_id:               Vec<u16>,
    owner_host:             Vec<u16>,
    owner_email:            Vec<u16>,
    product:                Vec<u16>,
    account_url:            Vec<u16>,
    account_deposit_url:    Vec<u16>,
    account_withdrawal_url: Vec<u16>
}

impl ConCommon {
    const VTABLE: IMTConCommon__bindgen_vtable = vtable_impl::con_common::new();
}

impl LuaObject for ConCommon {
    fn lua_impl(&self) -> Table {
        self.lua_impl.clone()
    }

    fn lua_handler(&self) -> &LuaHandler {
        &self.lua
    }
}

impl LuaConstructible for ConCommon {
    type MTType = IMTConCommon;

    fn new(lua: Arc<LuaHandler>, lua_impl: Table) -> Self {
        let mut result = Self {
            lua_impl,
            lua,
            name:                   Vec::new(),
            name_full:              Vec::new(),
            owner:                  Vec::new(),
            owner_id:               Vec::new(),
            owner_host:             Vec::new(),
            owner_email:            Vec::new(),
            product:                Vec::new(),
            account_url:            Vec::new(),
            account_deposit_url:    Vec::new(),
            account_withdrawal_url: Vec::new()
        };

        result.name                   = result.call_str("name");
        result.name_full              = result.call_str("name_full");
        result.owner                  = result.call_str("owner");
        result.owner_id               = result.call_str("owner_id");
        result.owner_host             = result.call_str("owner_host");
        result.owner_email            = result.call_str("owner_email");
        result.product                = result.call_str("product");
        result.account_url            = result.call_str("account_url");
        result.account_deposit_url    = result.call_str("account_deposit_url");
        result.account_withdrawal_url = result.call_str("account_withdrawal_url");

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
impl MT5ConCommon for ConCommon {
    fn release(&mut self) {
        self.free();
    }

    fn name(&self) -> *const u16 {
        self.name.as_ptr()
    }

    fn name_full(&self) -> *const u16 {
        self.name_full.as_ptr()
    }

    fn owner(&self) -> *const u16 {
        self.owner.as_ptr()
    }

    fn owner_id(&self) -> *const u16 {
        self.owner_id.as_ptr()
    }

    fn owner_host(&self) -> *const u16 {
        self.owner_host.as_ptr()
    }

    fn owner_email(&self) -> *const u16 {
        self.owner_email.as_ptr()
    }

    fn product(&self) -> *const u16 {
        self.product.as_ptr()
    }

    fn account_url(&self) -> *const u16 {
        self.account_url.as_ptr()
    }

    fn account_deposit_url(&self) -> *const u16 {
        self.account_deposit_url.as_ptr()
    }

    fn account_withdrawal_url(&self) -> *const u16 {
        self.account_withdrawal_url.as_ptr()
    }
}
