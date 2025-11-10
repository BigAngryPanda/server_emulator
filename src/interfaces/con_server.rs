use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_uint,
    c_int,
    c_ulonglong,
    c_longlong,
    c_void
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MT5ConServer {
    fn release(&mut self) { }

    fn assign(&mut self, param: &IMTConServer) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn type0(&self) -> c_uint {
        0
    }

    fn type1(&mut self, type_: c_uint) -> c_uint {
        0
    }

    fn name(&self) -> *const u16 {
        std::ptr::null()
    }

    fn name1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn address1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn address(&self) -> *const u16 {
        std::ptr::null()
    }

    fn id(&self) -> c_ulonglong {
        0
    }

    fn id1(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn password(&mut self, password: &[u16]) -> c_uint {
        0
    }

    fn password_check(&self, password: &[u16]) -> c_uint {
        0
    }

    fn service_time(&self) -> c_uint {
        0
    }

    fn service_time1(&mut self, stime: c_uint) -> c_uint {
        0
    }

    fn adapters_current(&self) -> *const u16 {
        std::ptr::null()
    }

    fn adapters_current1(&mut self, current: &[u16]) -> c_uint {
        0
    }

    fn adapters_total(&self) -> c_uint {
        0
    }

    fn adapters_next(&self, pos: c_uint) -> *const u16 {
        std::ptr::null()
    }

    fn address_total(&self) -> c_uint {
        0
    }

    fn address_next(&self, pos: c_uint) -> c_uint {
        0
    }

    fn version(&self) -> c_uint {
        0
    }

    fn build(&self) -> c_uint {
        0
    }

    fn build_date(&self) -> *const u16 {
        std::ptr::null()
    }

    fn last_boot_time(&self) -> c_longlong {
        0
    }

    fn connected(&self) -> bool {
        false
    }

    fn os(&self) -> *const u16 {
        std::ptr::null()
    }

    fn cpu(&self) -> *const u16 {
        std::ptr::null()
    }

    fn cpu_total(&self) -> c_uint {
        0
    }

    fn cpu_usage_max(&self) -> c_uint {
        0
    }

    fn cpu_usage_critical(&self) -> c_uint {
        0
    }

    fn memory_total(&self) -> c_uint {
        0
    }

    fn memory_free(&self) -> c_uint {
        0
    }

    fn memory_free_min(&self) -> c_uint {
        0
    }

    fn memory_free_critical(&self) -> c_uint {
        0
    }

    fn hdd_total(&self) -> c_uint {
        0
    }

    fn hdd_free(&self) -> c_uint {
        0
    }

    fn hdd_free_critical(&self) -> c_uint {
        0
    }

    fn hdd_fragments(&self) -> c_uint {
        0
    }

    fn hdd_fragments_critical(&self) -> c_uint {
        0
    }

    fn hdd_speed_read(&self) -> c_uint {
        0
    }

    fn hdd_speed_read_critical(&self) -> c_uint {
        0
    }

    fn hdd_speed_write(&self) -> c_uint {
        0
    }

    fn hdd_speed_write_critical(&self) -> c_uint {
        0
    }

    fn connects_max(&self) -> c_uint {
        0
    }

    fn connects_critical(&self) -> c_uint {
        0
    }

    fn network_max(&self) -> c_uint {
        0
    }

    fn network_critical(&self) -> c_uint {
        0
    }

    fn trade_server(&mut self) -> *mut IMTConServerTrade {
        std::ptr::null_mut()
    }

    fn history_server(&mut self) -> *mut IMTConServerHistory {
        std::ptr::null_mut()
    }

    fn access_server(&mut self) -> *mut IMTConServerAccess {
        std::ptr::null_mut()
    }

    fn backup_server(&mut self) -> *mut IMTConServerBackup {
        std::ptr::null_mut()
    }

    fn anti_ddos_server(&mut self) -> *mut IMTConServerAntiDDoS {
        std::ptr::null_mut()
    }

    fn reserved_server1(&mut self) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn reserved_server2(&mut self) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn reserved_server3(&mut self) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn reserved_server4(&mut self) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn points_add(&mut self, path: &[u16]) -> c_uint {
        0
    }

    fn points_update(&mut self, pos: c_uint, address: &[u16]) -> c_uint {
        0
    }

    fn points_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn points_clear(&mut self) -> c_uint {
        0
    }

    fn points_shift(&mut self, pos: c_uint, shift: c_int) -> c_uint {
        0
    }

    fn points_total(&self) -> c_uint {
        0
    }

    fn points_next(&self, pos: c_uint) -> *const u16 {
        std::ptr::null()
    }

    fn bindings_add(&mut self, path: &[u16]) -> c_uint {
        0
    }

    fn bindings_update(&mut self, pos: c_uint, address: &[u16]) -> c_uint {
        0
    }

    fn bindings_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn bindings_clear(&mut self) -> c_uint {
        0
    }

    fn bindings_shift( &mut self, pos: c_uint, shift: c_int) -> c_uint {
        0
    }

    fn bindings_total(&self) -> c_uint {
        0
    }

    fn bindings_next(&self, pos: c_uint) -> *const u16 {
        std::ptr::null()
    }

    fn failover_mode(&self) -> c_uint {
        0
    }

    fn failover_mode1(&mut self, mode: c_uint) -> c_uint {
        0
    }

    fn failover_timeout(&self) -> c_uint {
        0
    }

    fn failover_timeout1(&mut self, timeout: c_uint) -> c_uint {
        0
    }

    fn cluster_state_total(&self) -> c_uint {
        0
    }

    fn cluster_state_next(&self, pos: c_uint, state: &mut IMTConClusterState) -> c_uint {
        0
    }

    fn cluster_state_get(&self, id: c_ulonglong, state: &mut IMTConClusterState) -> c_uint {
        0
    }

    fn address_ipv6(&self) -> *const u16 {
        std::ptr::null()
    }

    fn address_ipv61(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn address_ipv6_total(&self) -> c_uint {
        0
    }

    fn address_ipv6_next(&self, pos: c_uint, address: &mut [u16; 260]) -> c_uint {
        0
    }
}
