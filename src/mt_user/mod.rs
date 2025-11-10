use pyo3::prelude::*;

use crate::{
    mt5_apiserver::IMTUser,
    mt5_apiserver::IMTUser__bindgen_vtable,
    interfaces::mt_user::MTUser,
    py_call,
    conversion,
    vtable_impl,
};

use server_emulator_macro::log_impl_calls;

use std::os::raw::{
    c_uint,
    c_ushort,
    c_uchar,
    c_ulong,
    c_ulonglong,
    c_longlong
};

#[pyclass]
pub struct User {
    mt_user_impl: PyObject,
}

// accept py impl
impl User {
    const MT_USER_VTABLE: IMTUser__bindgen_vtable = vtable_impl::mt_user::new();

    pub fn mt_user(&mut self) -> IMTUser {
        IMTUser {
            vtable_: &Self::MT_USER_VTABLE,
            impl_ptr: self,
        }
    }

    fn call_str_to_int<T>(&mut self, name: &str, data: *const u16, default: T) -> T
    where
        for<'a> T: FromPyObject<'a>,
    {
        let wstr = conversion::to_utf16_vec(data);

        let result: T = py_call::call_method(&self.mt_user_impl, name, (&wstr, ), default);

        std::mem::forget(wstr);

        result
    }

    fn call_void_to_str(&self, name: &str) -> *const u16
    {
        let result: Vec<u16> =
            py_call::call_with_no_args(&self.mt_user_impl, name, Vec::new());

        if result.is_empty() {
            std::ptr::null()
        }
        else {
            result.as_ptr()
        }
    }
}

#[pymethods]
impl User {
    #[staticmethod]
    pub fn new(user_impl: PyObject) -> User {
        User {
            mt_user_impl: user_impl,
        }
    }
}

#[log_impl_calls]
impl MTUser for User {
    fn assign(&mut self, user: *const IMTUser) -> c_uint {
        py_call::call_with_no_args(&self.mt_user_impl, "assign", 0)
    }

    fn clear(&mut self) -> c_uint {
        py_call::call_with_no_args(&self.mt_user_impl, "clear", 0)
    }

    fn login(&self) -> c_ulonglong {
        py_call::call_with_no_args(&self.mt_user_impl, "login", 0)
    }

    fn login1(&mut self, login: c_ulonglong) -> c_uint {
        py_call::call_method(&self.mt_user_impl, "login1", (login, ), 0)
    }

    fn group(&self) -> *const u16 {
        let group: Vec<u16> =
            py_call::call_with_no_args(&self.mt_user_impl, "group", Vec::new());

        if group.is_empty() {
            std::ptr::null()
        }
        else {
            group.as_ptr()
        }
    }

    fn group1(&mut self, group: *const u16) -> c_uint {
        self.call_str_to_int("group1", group, 0 as c_uint)
    }

    fn cert_serial_number(&self) -> c_ulonglong {
        py_call::call_with_no_args(&self.mt_user_impl, "cert_serial_number", 0)
    }

    fn rights(&self) -> c_ulonglong {
        py_call::call_with_no_args(&self.mt_user_impl, "rights", 0)
    }

    fn rights1(&mut self, rights: c_ulonglong) -> c_uint {
        py_call::call_with_no_args(&self.mt_user_impl, "rights1", 0)
    }

    fn registration(&self) -> c_longlong {
        py_call::call_with_no_args(&self.mt_user_impl, "registration", 0)
    }

    fn last_access(&self) -> c_longlong {
        py_call::call_with_no_args(&self.mt_user_impl, "last_access", 0)
    }

    fn last_ip(&self, ip: &mut [u16; 260]) -> *const u16 {
        let wstr = conversion::to_utf16_vec(ip.as_ptr());

        let result: Vec<u16> = py_call::call_method(&self.mt_user_impl, "last_ip", (&wstr, ), Vec::new());

        std::mem::forget(wstr);

        if result.is_empty() {
            std::ptr::null()
        }
        else {
            result.as_ptr()
        }
    }

    fn name(&self) -> *const u16 {
        self.call_void_to_str("name")
    }

    fn name1(&mut self, name: *const u16) -> c_uint {
        self.call_str_to_int("name1", name, 0 as c_uint)
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