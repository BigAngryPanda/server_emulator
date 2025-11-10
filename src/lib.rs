#![allow(non_upper_case_globals, non_camel_case_types, dead_code, non_snake_case)]
mod mt5_apiserver;
mod vtable_impl;
mod interfaces;
mod indexed_storage;
mod server;
mod plugin;
mod con_common;
mod con_plugin;
mod con_server;
mod conversion;
mod user_sink;
mod mt_user;
mod py_call;

use pyo3::prelude::*;

extern crate server_emulator_macro;

/// A Python module implemented in Rust.
#[pymodule(gil_used = false)]
fn py_server_emulator(m: &Bound<'_, PyModule>) -> PyResult<()> {
    use crate::server::Server;

    m.add_class::<Server>()?;

    Ok(())
}

#[test]
fn assert_types() {
    assert!(std::mem::size_of::<std::os::raw::c_uint>() == std::mem::size_of::<mt5_apiserver::MTAPIRES>());
    assert!(std::mem::size_of::<std::os::raw::c_ulong>() == std::mem::size_of::<mt5_apiserver::COLORREF>());
    assert!(std::mem::size_of::<std::os::raw::c_uint>() == std::mem::size_of::<mt5_apiserver::UINT>());
    assert!(std::mem::size_of::<u16>() == std::mem::size_of::<mt5_apiserver::WCHAR>());
    assert!(std::mem::size_of::<std::os::raw::c_ulonglong>() == std::mem::size_of::<mt5_apiserver::UINT64>());
    assert!(std::mem::size_of::<std::os::raw::c_longlong>() == std::mem::size_of::<mt5_apiserver::INT64>());
    assert!(std::mem::size_of::<[u16; 260usize]>() == std::mem::size_of::<mt5_apiserver::MTAPISTR>());
    assert!(std::mem::size_of::<std::os::raw::c_ulong>() == std::mem::size_of::<mt5_apiserver::ULONG>());
}
