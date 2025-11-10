use pyo3::prelude::*;

use crate::mt5_apiserver::*;
use crate::mt_user::User;

use std::sync::atomic::{
    AtomicPtr,
    Ordering::Relaxed
};

#[pyclass]
struct UserSink {
    user_sink: AtomicPtr<IMTUserSink>,
}

impl UserSink {
    pub fn new(user_sink: *const IMTUserSink) -> UserSink {
        UserSink {
            user_sink: AtomicPtr::new(user_sink as *mut IMTUserSink)
        }
    }

    fn vtable(&self) -> &IMTUserSink__bindgen_vtable {
        unsafe { &*(*self.user_sink.load(Relaxed)).vtable_ }
    }
}

#[allow(unused_variables)]
#[pymethods]
impl UserSink {
    // accept IMTUser python impl
    // create User struct
    // create IMTUser struct
    // call plugin on_user_add via vtable
    pub fn on_user_add(&mut self, user: &mut User) {
        unsafe {
            (self.vtable().IMTUserSink_OnUserAdd)(self.user_sink.load(Relaxed), &user.mt_user())
        }
    }

    pub fn on_user_update(&mut self, user: &mut User) {

    }

    pub fn on_user_delete(&mut self, user: &mut User) {

    }

    pub fn on_user_clean(&mut self, login: u64) {

    }

    pub fn on_user_login(&mut self, ip: &[u8], user: &User, type_: u32) {

    }

    pub fn on_user_sync(&mut self) {

    }

    pub fn hook_user_add(&mut self, user: &User) -> u32 {
        0
    }

    pub fn hook_user_update(&mut self, prev: &User, user: &User) {

    }

    pub fn hook_user_delete(&mut self, user: &User) {

    }

    pub fn hook_user_login(&mut self, ip: &[u8], user: &User, type_: UINT) {

    }

    pub fn on_user_logout(&mut self, ip: &[u8], user: &User, type_: UINT) {

    }

    pub fn on_user_archive(&mut self, user: &User) {

    }

    pub fn on_user_restore(&mut self, user: &User) {

    }

    pub fn hook_user_archive(&mut self, user: &User) -> u32 {
        0
    }

    pub fn hook_user_login_ext(&mut self, user: &User, online: &User) -> u32 {
        0
    }

    pub fn on_user_login_ext(&mut self, user: &User, online: &User) {

    }

    pub fn on_user_logout_ext(&mut self, user: &User, online: &User) {

    }

    pub fn on_user_add_ext(&mut self, user: &User, master_password: &[u8], investor_password: &[u8]) {

    }

    pub fn on_user_change_password(&mut self, user: &User, password_type: u32, password: &[u8]) {

    }

    pub fn hook_user_add_ext(&mut self, user: &User, master_password: &[u8], investor_password: &[u8]) -> u32 {
        0
    }

    pub fn hook_user_change_password(&mut self, user: &User, password_type: u32, password: &[u8]) -> u32 {
        0
    }
}