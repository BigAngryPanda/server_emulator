#![allow(non_upper_case_globals, non_camel_case_types, dead_code, non_snake_case)]
mod mt5_apiserver;
mod vtable_impl;
mod conversion;
mod indexed_storage;
mod rust_server;
mod interfaces;

mod lua_server;

#[cfg(feature="rust_server")]
use crate::rust_server::server;

fn input() {
    println!("Input...");

    let stdin = std::io::stdin();
    let mut buffer = String::new();
    stdin.read_line(&mut buffer).unwrap();
}

#[cfg(feature="lua_server")]
fn run_server() {
    lua_server::run();
}

#[cfg(feature="rust_server")]
fn run_server() {
    let mut srv = server::Server::new("");

    input();

    srv.init();

    srv.start();

//  while !srv.trade_sink_exist(0) {
//  }
    input();

    srv.stop();
}

#[cfg(not(any(feature = "lua_server", feature = "rust_server")))]
fn run_server() {

}

fn main() {
    run_server();
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
