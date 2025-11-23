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
pub trait MT5Deal {
    fn release(&mut self) { }

    fn assign(&mut self, deal: *const IMTDeal) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn print(&self, string: *mut MTAPISTR) -> *const u16 {
        std::ptr::null()
    }

    fn deal(&self) -> c_ulonglong {
        0
    }

    fn external_id1(&mut self, id: *const u16) -> c_uint {
        0
    }

    fn external_id(&self) -> *const u16 {
        std::ptr::null()
    }

    fn login1(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn login(&self) -> c_ulonglong {
        0
    }

    fn dealer1(&mut self, dealer: c_ulonglong) -> c_uint {
        0
    }

    fn dealer(&self) -> c_ulonglong {
        0
    }

    fn order1(&mut self, order: c_ulonglong) -> c_uint {
        0
    }

    fn order(&self) -> c_ulonglong {
        0
    }

    fn action1(&mut self, action: c_uint) -> c_uint {
        0
    }

    fn action(&self) -> c_uint {
        0
    }

    fn entry1(&mut self, entry: c_uint) -> c_uint {
        0
    }

    fn entry(&self) -> c_uint {
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

    fn time1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time(&self) -> c_longlong {
        0
    }

    fn symbol1(&mut self, symbol: *const u16) -> c_uint {
        0
    }

    fn symbol(&self) -> *const u16 {
        std::ptr::null()
    }

    fn price1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price(&self) -> f64 {
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

    fn commission1(&mut self, comm: f64) -> c_uint {
        0
    }

    fn commission(&self) -> f64 {
        0.0
    }

    fn obsolete_value1(&mut self, agent: f64) -> c_uint {
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

    fn position_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn position_id(&self) -> c_ulonglong {
        0
    }

    fn comment1(&mut self, comment: *const u16) -> c_uint {
        0
    }

    fn comment(&self) -> *const u16 {
        std::ptr::null()
    }

    fn api_data_set2(&mut self, app_id: c_ushort, id: c_uchar, value: f64) -> c_uint {
        0
    }

    fn api_data_set1(
        &mut self,
        app_id: c_ushort,
        id: c_uchar,
        value: c_ulonglong
    ) -> c_uint {
        0
    }

    fn api_data_set(
        &mut self,
        app_id: c_ushort,
        id: c_uchar,
        value: c_longlong,
    ) -> c_uint {
        0
    }

    fn api_data_get2(
        &self,
        app_id: c_ushort,
        id: c_uchar,
        value: *mut f64,
    ) -> c_uint {
        0
    }

    fn api_data_get1(
        &self,
        app_id: c_ushort,
        id: c_uchar,
        value: *mut c_ulonglong,
    ) -> c_uint {
        0
    }

    fn api_data_get(
        &self,
        app_id: c_ushort,
        id: c_uchar,
        value: *mut c_longlong,
    ) -> c_uint {
        0
    }

    fn api_data_clear(&mut self, app_id: c_ushort) -> c_uint {
        0
    }

    fn api_data_clear_all(&mut self) -> c_uint {
        0
    }

    fn profit_raw1(&mut self, profit: f64) -> c_uint {
        0
    }

    fn profit_raw(&self) -> f64 {
        0.0
    }

    fn price_position1(&mut self, price: f64) -> c_uint {
        0
    }

    fn price_position(&self) -> f64 {
        0.0
    }

    fn volume_closed1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_closed(&self) -> c_ulonglong {
        0
    }

    fn tick_value1(&mut self, value: f64) -> c_uint {
        0
    }

    fn tick_value(&self) -> f64 {
        0.0
    }

    fn tick_size1(&mut self, size: f64) -> c_uint {
        0
    }

    fn tick_size(&self) -> f64 {
        0.0
    }

    fn flags1(&mut self, flags: c_ulonglong) -> c_uint {
        0
    }

    fn flags(&self) -> c_ulonglong {
        0
    }

    fn time_msc1(&mut self, time: c_longlong) -> c_uint {
        0
    }

    fn time_msc(&self) -> c_longlong {
        0
    }

    fn reason(&self) -> c_uint {
        0
    }

    fn gateway(&self) -> *const u16 {
        std::ptr::null()
    }

    fn price_gateway(&self) -> f64 {
        0.0
    }

    fn api_data_update2(
        &mut self,
        pos: c_uint,
        app_id: c_ushort,
        id: c_uchar,
        value: f64
    ) -> c_uint {
        0
    }

    fn api_data_update1(
        &mut self,
        pos: c_uint,
        app_id: c_ushort,
        id: c_uchar,
        value: c_ulonglong
    ) -> c_uint {
        0
    }

    fn api_data_update(
        &mut self,
        pos: c_uint,
        app_id: c_ushort,
        id: c_uchar,
        value: c_longlong
    ) -> c_uint {
        0
    }

    fn api_data_next2(
        &self,
        pos: c_uint,
        app_id: *mut c_ushort,
        id: *mut c_uchar,
        value: *mut f64
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

    fn deal_set(&mut self, deal: c_ulonglong) -> c_uint {
        0
    }

    fn modification_flags(&self) -> c_uint {
        0
    }

    fn reason_set(&mut self, reason: c_uint) -> c_uint {
        0
    }

    fn gateway_set(&mut self, gateway: *const u16) -> c_uint {
        0
    }

    fn price_gateway_set(&mut self, price_gateway: f64) -> c_uint {
        0
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

    fn volume_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_ext(&self) -> c_ulonglong {
        0
    }

    fn volume_closed_ext1(&mut self, volume: c_ulonglong) -> c_uint {
        0
    }

    fn volume_closed_ext(&self) -> c_ulonglong {
        0
    }

    fn fee1(&mut self, fee: f64) -> c_uint {
        0
    }

    fn fee(&self) -> f64 {
        0.0
    }

    fn value1(&mut self, value: f64) -> c_uint {
        0
    }

    fn value(&self) -> f64 {
        0.0
    }

    fn market_bid(&self) -> f64 {
        0.0
    }

    fn market_ask(&self) -> f64 {
        0.0
    }

    fn market_last(&self) -> f64 {
        0.0
    }

    fn market_bid_set(&mut self, price: f64) -> c_uint {
        0
    }

    fn market_ask_set(&mut self, price: f64) -> c_uint {
        0
    }

    fn market_last_set(&mut self, price: f64) -> c_uint {
        0
    }
}
