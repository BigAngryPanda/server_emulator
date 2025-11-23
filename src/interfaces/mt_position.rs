use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_ushort,
    c_uchar,
    c_uint,
    c_longlong,
    c_ulonglong
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MTPosition {
    fn release(&mut self) { }

    fn assign(&mut self, position: *const IMTPosition) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn print(&self, string: *mut [u16; 260]) -> *const u16 {
        std::ptr::null()
    }

    fn login(&self) -> c_ulonglong {
        0
    }

    fn symbol1(&mut self, symbol: *const u16) -> c_uint {
        0
    }

    fn symbol(&self) -> *const u16 {
        std::ptr::null()
    }

    fn action1(&mut self, action: c_uint) -> c_uint {
        0
    }

    fn action(&self) -> c_uint {
        0
    }

    fn digits1(&mut self, digits: c_uint) -> c_uint {
        0
    }

    fn digits(&self) -> c_uint {
        0
    }

    fn digits_currency1(&mut self, digits: c_uint) -> c_uint {
        0
    }

    fn digits_currency(&self) -> c_uint {
        0
    }

    fn contract_size1(&mut self, contract_size: f64) -> c_uint {
        0
    }

    fn contract_size(&self) -> f64 {
        0.0
    }

    fn time_create1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_create(&self) -> c_longlong {
        0
    }

    fn time_update1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_update(&self) -> c_longlong {
        0
    }

    fn price_open1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_open(&self) -> f64 {
        0.0
    }

    fn price_current1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_current(&self) -> f64 {
        0.0
    }

    fn price_sl1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_sl(&self) -> f64 {
        0.0
    }

    fn price_tp1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_tp(&self) -> f64 {
        0.0
    }

    fn volume1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume(&self) -> c_ulonglong {
        0
    }

    fn profit1(&mut self, profit: f64) -> c_uint {
        0
    }

    fn profit(&self) -> f64 {
        0.0
    }

    fn storage1(&mut self, storage: f64) -> c_uint {
        0
    }

    fn storage(&self) -> f64 {
        0.0
    }

    fn obsolete_value1(&mut self, value: f64) -> c_uint {
        0
    }

    fn obsolete_value(&self) -> f64 {
        0.0
    }

    fn rate_profit1(&mut self, rate: f64) -> c_uint {
        0
    }

    fn rate_profit(&self) -> f64 {
        0.0
    }

    fn rate_margin1(&mut self, rate: f64) -> c_uint {
        0
    }

    fn rate_margin(&self) -> f64 {
        0.0
    }

    fn expert_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn expert_id(&self) -> c_ulonglong {
        0
    }

    fn expert_position_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn expert_position_id(&self) -> c_ulonglong {
        0
    }

    fn comment1(&mut self, comment: *const u16) -> c_uint {
        0
    }

    fn comment(&self) -> *const u16 {
        std::ptr::null()
    }

    fn activation_mode1(&mut self, mode: c_uint) -> c_uint {
        0
    }

    fn activation_mode(&self) -> c_uint {
        0
    }

    fn activation_time1(&mut self, atm: c_longlong) -> c_uint {
        0
    }

    fn activation_time(&self) -> c_longlong {
        0
    }

    fn activation_price1(&mut self, price: f64) -> c_uint {
        0
    }

    fn activation_price(&self) -> f64 {
        0.0
    }

    fn activation_flags1(&mut self, flags: c_uint) -> c_uint {
        0
    }

    fn activation_flags(&self) -> c_uint {
        0
    }

    fn api_data_set2(&mut self, app_id: c_ushort, id: c_uchar, value: f64) -> c_uint {
        0
    }

    fn api_data_set1(&mut self, app_id: c_ushort, id: c_uchar, value: c_ulonglong) -> c_uint {
        0
    }

    fn api_data_set(&mut self, app_id: c_ushort, id: c_uchar, value: c_longlong) -> c_uint {
        0
    }

    fn api_data_get2(&self, app_id: c_ushort, id: c_uchar, value: *mut f64) -> c_uint {
        0
    }

    fn api_data_get1(&self, app_id: c_ushort, id: c_uchar, value: *mut c_ulonglong) -> c_uint {
        0
    }

    fn api_data_get(&self, app_id: c_ushort, id: c_uchar, value: *mut c_longlong) -> c_uint {
        0
    }

    fn api_data_clear(&mut self, app_id: c_ushort) -> c_uint {
        0
    }

    fn api_data_clear_all(&mut self) -> c_uint {
        0
    }

    fn time_create_msc1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_create_msc(&self) -> c_longlong {
        0
    }

    fn time_update_msc1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_update_msc(&self) -> c_longlong {
        0
    }

    fn dealer1(&mut self, dealer: c_ulonglong) -> c_uint {
        0
    }

    fn dealer(&self) -> c_ulonglong {
        0
    }

    fn api_data_update2(&mut self, pos: c_uint, app_id: c_ushort, id: c_uchar, value: f64) -> c_uint {
        0
    }

    fn api_data_update1(&mut self, pos: c_uint, app_id: c_ushort, id: c_uchar, value: c_ulonglong) -> c_uint {
        0
    }

    fn api_data_update(&mut self, pos: c_uint, app_id: c_ushort, id: c_uchar, value: c_longlong) -> c_uint {
        0
    }

    fn api_data_next2(
        &self,
        pos: c_uint,
        app_id: *mut c_ushort,
        id: *mut c_uchar,
        value: *mut f64,
    ) -> c_uint {
        0
    }

    fn api_data_next1(
        &self,
        pos: c_uint,
        app_id: *mut c_ushort,
        id: *mut c_uchar,
        value: *mut c_ulonglong,
    ) -> c_uint {
        0
    }

    fn api_data_next(
        &self,
        pos: c_uint,
        app_id: *mut c_ushort,
        id: *mut c_uchar,
        value: *mut c_longlong,
    ) -> c_uint {
        0
    }

    fn login_set(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn position(&self) -> c_ulonglong {
        0
    }

    fn external_id1(&mut self, id: *const u16) -> c_uint {
        0
    }

    fn external_id(&self) -> *const u16 {
        std::ptr::null()
    }

    fn modification_flags(&self) -> c_uint {
        0
    }

    fn reason(&self) -> c_uint {
        0
    }

    fn volume_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_ext(&self) -> c_ulonglong {
        0
    }

    fn reason_set(&mut self, reason: c_uint) -> c_uint {
        0
    }
}