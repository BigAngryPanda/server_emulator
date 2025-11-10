use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_uint,
    c_int,
    c_ulonglong
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MT5ConPlugin {
    fn release(&mut self) { }

    fn assign(&mut self, param: &IMTConPlugin) -> c_uint {
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

    fn server1(&mut self, server: c_ulonglong) -> c_uint {
        0
    }

    fn server(&self) -> c_ulonglong {
        0
    }

    fn module1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn module(&self) -> *const u16 {
        std::ptr::null()
    }

    fn mode1(&mut self, mode: c_uint) -> c_uint {
        0
    }

    fn mode(&self) -> c_uint {
        0
    }

    fn parameter_add(&mut self, param: &mut IMTConParam) -> c_uint {
        0
    }

    fn parameter_update(
        &mut self,
        pos: c_uint,
        param: &IMTConParam,
    ) -> c_uint {
        0
    }

    fn parameter_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn parameter_clear(&mut self) -> c_uint {
        0
    }

    fn parameter_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn parameter_total(&self) -> c_uint {
        0
    }

    fn parameter_next(
        &self,
        pos: c_uint,
        param: *mut IMTConParam,
    ) -> c_uint {
        0
    }

    fn parameter_get(
        &self,
        name: &[u16],
        param: &mut IMTConParam,
    ) -> c_uint {
        0
    }

    fn flags1(&mut self, flags: c_uint) -> c_uint {
        0
    }

    fn flags(&self) -> c_uint {
        0
    }
}
