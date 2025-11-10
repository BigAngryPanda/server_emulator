use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_uint,
    c_longlong
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MT5ConCommon {
    fn release(&mut self) { }

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
        std::ptr::null()
    }

    fn name_full(&self) -> *const u16 {
        std::ptr::null()
    }

    fn owner(&self) -> *const u16 {
        std::ptr::null()
    }

    fn owner_id(&self) -> *const u16 {
        std::ptr::null()
    }

    fn owner_host(&self) -> *const u16 {
        std::ptr::null()
    }

    fn owner_email(&self) -> *const u16 {
        std::ptr::null()
    }

    fn product(&self) -> *const u16 {
        std::ptr::null()
    }

    fn expiration_license(&self) -> c_longlong {
        0
    }

    fn expiration_support(&self) -> c_longlong {
        0
    }

    fn limit_trade_servers(&self) -> c_uint {
        0
    }

    fn limit_web_servers(&self) -> c_uint {
        0
    }

    fn limit_accounts(&self) -> c_uint {
        0
    }

    fn limit_deals(&self) -> c_uint {
        0
    }

    fn limit_groups(&self) -> c_uint {
        0
    }

    fn live_update_mode(&self) -> c_uint {
        0
    }

    fn live_update_mode1(&mut self, mode: c_uint) -> c_uint {
        0
    }

    fn total_users(&self) -> c_uint {
        0
    }

    fn total_users_real(&self) -> c_uint {
        0
    }

    fn total_deals(&self) -> c_uint {
        0
    }

    fn total_orders(&self) -> c_uint {
        0
    }

    fn total_orders_history(&self) -> c_uint {
        0
    }

    fn total_positions(&self) -> c_uint {
        0
    }

    fn limit_symbols(&self) -> c_uint {
        0
    }

    fn account_url(&self) -> *const u16 {
        std::ptr::null()
    }

    fn account_url1(&mut self, url: &[u16]) -> c_uint {
        0
    }

    fn account_deposit_url(&self) -> *const u16 {
        std::ptr::null()
    }

    fn account_deposit_url1(&mut self, url: &[u16]) -> c_uint {
        0
    }

    fn account_withdrawal_url(&self) -> *const u16 {
        std::ptr::null()
    }

    fn account_withdrawal_url1(&mut self, url: &[u16]) -> c_uint {
        0
    }
}
