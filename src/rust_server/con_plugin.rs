use std::os::raw::{
    c_uint
};

use server_emulator_macro::log_impl_calls;

use crate::interfaces::con_plugin::MT5ConPlugin;

pub struct ConPlugin {
}

#[log_impl_calls]
impl ConPlugin {
    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    pub fn new() -> ConPlugin {
        ConPlugin { }
    }
}

#[log_impl_calls]
impl MT5ConPlugin for ConPlugin {
    fn name1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn name(&self) -> *const u16 {
        Self::STR_DATA.as_ptr()
    }
}
