use crate::mt5_apiserver::IMTRequest;

use server_emulator_macro::log_trait_calls;

use std::ffi::c_void;
use std::os::raw::{
    c_uchar,
    c_uint,
    c_ushort,
    c_longlong,
    c_ulonglong
};
use std::ptr;

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MTRequest {
    fn release(&mut self) { }

    fn assign(&mut self, request: &IMTRequest) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn print(&self, string: &mut [u16; 260usize]) -> *const c_ushort {
        ptr::null()
    }

    fn id(&self) -> c_uint {
        0
    }

    fn login(&self) -> c_ulonglong {
        0
    }

    fn login1(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn group(&self) -> *const c_ushort {
        ptr::null()
    }

    fn symbol(&self) -> *const c_ushort {
        ptr::null()
    }

    fn symbol1(&mut self, symbol: *const c_ushort) -> c_uint {
        0
    }

    fn digits(&self) -> c_uint {
        0
    }

    fn action(&self) -> c_uint {
        0
    }

    fn action1(&mut self, action: c_uint) -> c_uint {
        0
    }

    fn time_expiration(&self) -> c_longlong {
        0
    }

    fn time_expiration1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn type_(&self) -> c_uint {
        0
    }

    fn type1(&mut self, type_: c_uint) -> c_uint {
        0
    }

    fn type_fill(&self) -> c_uint {
        0
    }

    fn type_fill1(&mut self, type_: c_uint) -> c_uint {
        0
    }

    fn type_time(&self) -> c_uint {
        0
    }

    fn type_time1(&mut self, type_: c_uint) -> c_uint {
        0
    }

    fn flags(&self) -> c_ulonglong {
        0
    }

    fn flags1(&mut self, flags: c_ulonglong) -> c_uint {
        0
    }

    fn volume(&self) -> c_ulonglong {
        0
    }

    fn volume1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn order(&self) -> c_ulonglong {
        0
    }

    fn order1(&mut self, order: c_ulonglong) -> c_uint {
        0
    }

    fn order_external_id(&self) -> *const c_ushort {
        ptr::null()
    }

    fn order_external_id1(&mut self, id: *const c_ushort) -> c_uint {
        0
    }

    fn price_order(&self) -> f64 {
        0.0
    }

    fn price_order1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_trigger(&self) -> f64 {
        0.0
    }

    fn price_trigger1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_sl(&self) -> f64 {
        0.0
    }

    fn price_sl1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_tp(&self) -> f64 {
        0.0
    }

    fn price_tp1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_deviation(&self) -> c_ulonglong {
        0
    }

    fn price_deviation1(&mut self, deviation: c_ulonglong) -> c_uint {
        0
    }

    fn price_deviation_top(&self) -> f64 {
        0.0
    }

    fn price_deviation_bottom(&self) -> f64 {
        0.0
    }

    fn comment(&self) -> *const c_ushort {
        ptr::null()
    }

    fn comment1(&mut self, comment: *const c_ushort) -> c_uint {
        0
    }

    fn result_retcode(&self) -> c_uint {
        0
    }

    fn result_dealer(&self) -> c_ulonglong {
        0
    }

    fn result_deal(&self) -> c_ulonglong {
        0
    }

    fn result_order(&self) -> c_ulonglong {
        0
    }

    fn result_volume(&self) -> c_ulonglong {
        0
    }

    fn result_price(&self) -> f64 {
        0.0
    }

    fn result_dealer_bid(&self) -> f64 {
        0.0
    }

    fn result_dealer_ask(&self) -> f64 {
        0.0
    }

    fn result_dealer_last(&self) -> f64 {
        0.0
    }

    fn result_market_bid(&self) -> f64 {
        0.0
    }

    fn result_market_ask(&self) -> f64 {
        0.0
    }

    fn result_market_last(&self) -> f64 {
        0.0
    }

    fn result_comment(&self) -> *const c_ushort {
        ptr::null()
    }

    fn external_account(&self) -> *const c_ushort {
        ptr::null()
    }

    fn external_account1(&mut self, account: *const c_ushort) -> c_uint {
        0
    }

    fn id_client(&self) -> c_uint {
        0
    }

    fn ip(&self) -> *const c_ushort {
        ptr::null()
    }

    fn ip1(&mut self, ip: *const c_ushort) -> c_uint {
        0
    }

    fn source_login(&self) -> c_ulonglong {
        0
    }

    fn source_login1(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn position(&self) -> c_ulonglong {
        0
    }

    fn position1(&mut self, position: c_ulonglong) -> c_uint {
        0
    }

    fn position_by(&self) -> c_ulonglong {
        0
    }

    fn position_by1(&mut self, position: c_ulonglong) -> c_uint {
        0
    }

    fn position_external_id(&self) -> *const c_ushort {
        ptr::null()
    }

    fn position_external_id1(&mut self, id: *const c_ushort) -> c_uint {
        0
    }

    fn position_by_external_id(&self) -> *const c_ushort {
        ptr::null()
    }

    fn position_by_external_id1(&mut self, id: *const c_ushort) -> c_uint {
        0
    }

    fn volume_ext(&self) -> c_ulonglong {
        0
    }

    fn volume_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn result_volume_ext(&self) -> c_ulonglong {
        0
    }

    fn digits_set(&mut self, digits: c_uint) -> c_uint {
        0
    }

    fn api_data_set(&mut self, app_id: c_ushort, id: c_uchar, value: c_longlong) -> c_uint {
        0
    }

    fn api_data_set1(&mut self, app_id: c_ushort, id: c_uchar, value: c_ulonglong) -> c_uint {
        0
    }

    fn api_data_set2(&mut self, app_id: c_ushort, id: c_uchar, value: f64) -> c_uint {
        0
    }

    fn api_data_get(&self, app_id: c_ushort, id: c_uchar, value: *mut c_longlong) -> c_uint {
        0
    }

    fn api_data_get1(&self, app_id: c_ushort, id: c_uchar, value: *mut c_ulonglong) -> c_uint {
        0
    }

    fn api_data_get2(&self, app_id: c_ushort, id: c_uchar, value: *mut f64) -> c_uint {
        0
    }

    fn api_data_update(&mut self, pos: c_uint, app_id: c_ushort, id: c_uchar, value: c_longlong) -> c_uint {
        0
    }

    fn api_data_update1(&mut self, pos: c_uint, app_id: c_ushort, id: c_uchar, value: c_ulonglong) -> c_uint {
        0
    }

    fn api_data_update2(&mut self, pos: c_uint, app_id: c_ushort, id: c_uchar, value: f64) -> c_uint {
        0
    }

    fn api_data_next(&self, pos: c_uint, app_id: *mut c_ushort, id: *mut c_uchar, value: *mut c_longlong) -> c_uint {
        0
    }

    fn api_data_next1(&self, pos: c_uint, app_id: *mut c_ushort, id: *mut c_uchar, value: *mut c_ulonglong) -> c_uint {
        0
    }

    fn api_data_next2(&self, pos: c_uint, app_id: *mut c_ushort, id: *mut c_uchar, value: *mut f64) -> c_uint {
        0
    }

    fn api_data_raw(&self) -> *mut c_void {
        ptr::null_mut()
    }

    fn api_data_raw_max(&self) -> c_uint {
        0
    }

    fn api_data_clear(&mut self, app_id: c_ushort) -> c_uint {
        0
    }

    fn api_data_clear_all(&mut self) -> c_uint {
        0
    }

    fn volume_current(&self) -> c_ulonglong {
        0
    }

    fn volume_current1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_current_ext(&self) -> c_ulonglong {
        0
    }

    fn volume_current_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn symbol_original(&self) -> *const c_ushort {
        ptr::null()
    }

    fn symbol_original1(&mut self, symbol: *const c_ushort) -> c_uint {
        0
    }
}
