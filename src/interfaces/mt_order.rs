use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_uint,
    c_ulonglong,
    c_longlong,
    c_ushort,
    c_uchar
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MTOrder {
    fn release(&mut self) {}

    fn assign(&mut self, order: *const IMTOrder) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn print(&self, string: *mut [u16; 260usize]) -> *const u16 {
        std::ptr::null()
    }

    fn order(&self) -> c_ulonglong {
        0
    }

    fn external_id(&self) -> *const u16 {
        std::ptr::null()
    }

    fn external_id1(&mut self, id: *const u16) -> c_uint {
        0
    }

    fn login(&self) -> c_ulonglong {
        0
    }

    fn login1(&mut self, order: c_ulonglong) -> c_uint {
        0
    }

    fn dealer(&self) -> c_ulonglong {
        0
    }

    fn dealer1(&mut self, dealer: c_ulonglong) -> c_uint {
        0
    }

    fn symbol(&self) -> *const u16 {
        std::ptr::null()
    }

    fn symbol1(&mut self, symbol: *const u16) -> c_uint {
        0
    }

    fn digits(&self) -> c_uint {
        0
    }

    fn digits1(&mut self, digits: c_uint) -> c_uint {
        0
    }

    fn digits_currency(&self) -> c_uint {
        0
    }

    fn digits_currency1(&mut self, digits: c_uint) -> c_uint {
        0
    }

    fn contract_size(&self) -> f64 {
        0.0
    }

    fn contract_size1(&mut self, contract_size: f64) -> c_uint {
        0
    }

    fn state(&self) -> c_uint {
        0
    }

    fn reason(&self) -> c_uint {
        0
    }

    fn time_setup(&self) -> c_longlong {
        0
    }

    fn time_setup1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_expiration(&self) -> c_longlong {
        0
    }

    fn time_expiration1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_done(&self) -> c_longlong {
        0
    }

    fn time_done1(&mut self, time: c_longlong) -> c_uint {
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

    fn price_current(&self) -> f64 {
        0.0
    }

    fn price_current1(&mut self, price: f64) -> c_uint {
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

    fn volume_initial(&self) -> c_ulonglong {
        0
    }

    fn volume_initial1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_current(&self) -> c_ulonglong {
        0
    }

    fn volume_current1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn expert_id(&self) -> c_ulonglong {
        0
    }

    fn expert_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn position_id(&self) -> c_ulonglong {
        0
    }

    fn position_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn comment(&self) -> *const u16 {
        std::ptr::null()
    }

    fn comment1(&mut self, comment: *const u16) -> c_uint {
        0
    }

    fn activation_mode(&self) -> c_uint {
        0
    }

    fn activation_time(&self) -> c_longlong {
        0
    }

    fn activation_price(&self) -> f64 {
        0.0
    }

    fn activation_flags(&self) -> c_uint {
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

    fn api_data_clear(&mut self, app_id: c_ushort) -> c_uint {
        0
    }

    fn api_data_clear_all(&mut self) -> c_uint {
        0
    }

    fn time_setup_msc(&self) -> c_longlong {
        0
    }

    fn time_setup_msc1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_done_msc(&self) -> c_longlong {
        0
    }

    fn time_done_msc1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn activation_mode1(&mut self, mode: c_uint) -> c_uint {
        0
    }

    fn activation_time1(&mut self, atm: c_longlong) -> c_uint {
        0
    }

    fn activation_price1(&mut self, price: f64) -> c_uint {
        0
    }

    fn activation_flags1(&mut self, flags: c_uint) -> c_uint {
        0
    }

    fn rate_margin(&self) -> f64 {
        0.0
    }

    fn rate_margin1(&mut self, rate: f64) -> c_uint {
        0
    }

    fn api_data_update(
        &mut self,
        pos: c_uint,
        app_id: c_ushort,
        id: c_uchar,
        value: c_longlong,
    ) -> c_uint {
        0
    }

    fn api_data_update1(
        &mut self,
        pos: c_uint,
        app_id: c_ushort,
        id: c_uchar,
        value: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn api_data_update2(
        &mut self,
        pos: c_uint,
        app_id: c_ushort,
        id: c_uchar,
        value: f64,
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

    fn api_data_next1(
        &self,
        pos: c_uint,
        app_id: *mut c_ushort,
        id: *mut c_uchar,
        value: *mut c_ulonglong,
    ) -> c_uint {
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

    fn order_set(&mut self, order: c_ulonglong) -> c_uint {
        0
    }

    fn position_by_id(&self) -> c_ulonglong {
        0
    }

    fn position_by_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn modification_flags(&self) -> c_uint {
        0
    }

    fn state_set(&mut self, state: c_uint) -> c_uint {
        0
    }

    fn reason_set(&mut self, reason: c_uint) -> c_uint {
        0
    }

    fn volume_initial_ext(&self) -> c_ulonglong {
        0
    }

    fn volume_initial_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_current_ext(&self) -> c_ulonglong {
        0
    }

    fn volume_current_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }
}
