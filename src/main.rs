#![allow(non_upper_case_globals, non_camel_case_types, dead_code, non_snake_case)]
mod mt5_apiserver;
mod vtable_impl;
mod conversion;
mod indexed_storage;
mod rust_server;
mod interfaces;
mod con_plugin;

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
    lua_server::lua_test();
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
