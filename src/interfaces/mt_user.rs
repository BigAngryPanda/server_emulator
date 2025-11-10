use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_uint,
    c_ushort,
    c_uchar,
    c_ulong,
    c_ulonglong,
    c_longlong
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MTUser {
    fn release(&mut self) { }

    fn assign(&mut self, user: *const IMTUser) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn login(&self) -> c_ulonglong {
        0
    }

    fn login1(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn group(&self) -> *const u16 {
        std::ptr::null()
    }

    fn group1(&mut self, group: *const u16) -> c_uint {
        0
    }

    fn cert_serial_number(&self) -> c_ulonglong {
        0
    }

    fn rights(&self) -> c_ulonglong {
        0
    }

    fn rights1(&mut self, rights: c_ulonglong) -> c_uint {
        0
    }

    fn registration(&self) -> c_longlong {
        0
    }

    fn last_access(&self) -> c_longlong {
        0
    }

    fn last_ip(&self, ip: &mut [u16; 260]) -> *const u16 {
        std::ptr::null()
    }

    fn name(&self) -> *const u16 {
        std::ptr::null()
    }

    fn name1(&mut self, name: *const u16) -> c_uint {
        0
    }

    fn company(&self) -> *const u16 {
        std::ptr::null()
    }

    fn company1(&mut self, id: *const u16) -> c_uint {
        0
    }

    fn account(&self) -> *const u16 {
        std::ptr::null()
    }

    fn account1(&mut self, account: *const u16) -> c_uint {
        0
    }

    fn country(&self) -> *const u16 {
        std::ptr::null()
    }

    fn country1(&mut self, account: *const u16) -> c_uint {
        0
    }

    fn language(&self) -> c_uint {
        0
    }

    fn language1(&mut self, language: c_uint) -> c_uint {
        0
    }

    fn city(&self) -> *const u16 {
        std::ptr::null()
    }

    fn city1(&mut self, city: *const u16) -> c_uint {
        0
    }

    fn state(&self) -> *const u16 {
        std::ptr::null()
    }

    fn state1(&mut self, state: *const u16) -> c_uint {
        0
    }

    fn zipcode(&self) -> *const u16 {
        std::ptr::null()
    }

    fn zipcode1(&mut self, code: *const u16) -> c_uint {
        0
    }

    fn address(&self) -> *const u16 {
        std::ptr::null()
    }

    fn address1(&mut self, code: *const u16) -> c_uint {
        0
    }

    fn phone(&self) -> *const u16 {
        std::ptr::null()
    }

    fn phone1(&mut self, phone: *const u16) -> c_uint {
        0
    }

    fn email(&self) -> *const u16 {
        std::ptr::null()
    }

    fn email1(&mut self, email: *const u16) -> c_uint {
        0
    }

    fn id(&self) -> *const u16 {
        std::ptr::null()
    }

    fn id1(&mut self, email: *const u16) -> c_uint {
        0
    }

    fn status(&self) -> *const u16 {
        std::ptr::null()
    }

    fn status1(&mut self, id: *const u16) -> c_uint {
        0
    }

    fn comment(&self) -> *const u16 {
        std::ptr::null()
    }

    fn comment1(&mut self, comment: *const u16) -> c_uint {
        0
    }

    fn color(&self) -> c_ulong {
        0
    }

    fn color1(&mut self, color: c_ulong) -> c_uint {
        0
    }

    fn phone_password(&self) -> *const u16 {
        std::ptr::null()
    }

    fn phone_password1(&mut self, password: *const u16) -> c_uint {
        0
    }

    fn leverage(&self) -> c_uint {
        0
    }

    fn leverage1(&mut self, leverage: c_uint) -> c_uint {
        0
    }

    fn agent(&self) -> c_ulonglong {
        0
    }

    fn agent1(&mut self, agent: c_ulonglong) -> c_uint {
        0
    }

    fn balance(&self) -> f64 {
        0.0
    }

    fn credit(&self) -> f64 {
        0.0
    }

    fn interest_rate(&self) -> f64 {
        0.0
    }

    fn commission_daily(&self) -> f64 {
        0.0
    }

    fn commission_monthly(&self) -> f64 {
        0.0
    }

    fn commission_agent_daily(&self) -> f64 {
        0.0
    }

    fn commission_agent_monthly(&self) -> f64 {
        0.0
    }

    fn balance_prev_day(&self) -> f64 {
        0.0
    }

    fn balance_prev_month(&self) -> f64 {
        0.0
    }

    fn equity_prev_day(&self) -> f64 {
        0.0
    }

    fn equity_prev_month(&self) -> f64 {
        0.0
    }

    fn api_data_set(
        &mut self,
        app_id: c_ushort,
        id: c_uchar,
        value: c_longlong,
    ) -> c_uint {
        0
    }

    fn api_data_set1(
        &mut self,
        app_id: c_ushort,
        id: c_uchar,
        value: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn api_data_set2(&mut self, app_id: c_ushort, id: c_uchar, value: f64) -> c_uint {
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

    fn api_data_get1(
        &self,
        app_id: c_ushort,
        id: c_uchar,
        value: *mut c_ulonglong,
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

    fn api_data_clear(&mut self, app_id: c_ushort) -> c_uint {
        0
    }

    fn api_data_clear_all(&mut self) -> c_uint {
        0
    }

    fn external_account_add(&mut self, gateway_id: c_ulonglong, account: *const u16) -> c_uint {
        0
    }

    fn external_account_update(
        &mut self,
        pos: c_uint,
        gateway_id: c_ulonglong,
        account: *const u16,
    ) -> c_uint {
        0
    }

    fn external_account_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn external_account_clear(&mut self) -> c_uint {
        0
    }

    fn external_account_total(&self) -> c_uint {
        0
    }

    fn external_account_next(
        &self,
        pos: c_uint,
        gateway_id: *mut c_ulonglong,
        account: &mut [u16; 260],
    ) -> c_uint {
        0
    }

    fn external_account_get(
        &self,
        gateway_id: c_ulonglong,
        account: &mut [u16; 260],
    ) -> c_uint {
        0
    }

    fn last_pass_change(&self) -> c_longlong {
        0
    }

    fn mqid(&self, mqid: &mut [u16; 260]) -> *const u16 {
        std::ptr::null()
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

    fn password_hash(
        &self,
        type_: c_uint,
        password_hash: &mut [u16; 260],
    ) -> c_uint {
        0
    }

    fn lead_campaign(&self) -> *const u16 {
        std::ptr::null()
    }

    fn lead_campaign1(&mut self, lead_campaign: *const u16) -> c_uint {
        0
    }

    fn lead_source(&self) -> *const u16 {
        std::ptr::null()
    }

    fn lead_source1(&mut self, lead_source: *const u16) -> c_uint {
        0
    }

    fn client_id(&self) -> c_ulonglong {
        0
    }

    fn client_id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn first_name(&self) -> *const u16 {
        std::ptr::null()
    }

    fn first_name1(&mut self, first_name: *const u16) -> c_uint {
        0
    }

    fn last_name(&self) -> *const u16 {
        std::ptr::null()
    }

    fn last_name1(&mut self, last_name: *const u16) -> c_uint {
        0
    }

    fn middle_name(&self) -> *const u16 {
        std::ptr::null()
    }

    fn middle_name1(&mut self, middle_name: *const u16) -> c_uint {
        0
    }

    fn registration_set(&mut self, datetime: c_longlong) -> c_uint {
        0
    }

    fn otpsecret(&self) -> *const u16 {
        std::ptr::null()
    }

    fn otpsecret1(&mut self, otp_secret: *const u16) -> c_uint {
        0
    }

    fn limit_orders(&self) -> c_uint {
        0
    }

    fn limit_orders1(&mut self, id: c_uint) -> c_uint {
        0
    }

    fn limit_positions_value(&self) -> f64 {
        0.0
    }

    fn limit_positions_value1(&mut self, value: f64) -> c_uint {
        0
    }
}
