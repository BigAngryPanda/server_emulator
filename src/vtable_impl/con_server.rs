use crate::{
    mt5_apiserver,
    conversion
};

use mt5_apiserver::*;

unsafe extern "C" fn IMTConServer_Release(this: *mut IMTConServer) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTConServer>());
}

unsafe extern "C" fn IMTConServer_Assign(this: *mut IMTConServer, param: *const IMTConServer) -> MTAPIRES {
    (*(*this).impl_ptr).assign(&*param)
}

unsafe extern "C" fn IMTConServer_Clear(this: *mut IMTConServer) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTConServer_Type(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).type0()
}

unsafe extern "C" fn IMTConServer_Type1(this: *mut IMTConServer, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type1(type_)
}

unsafe extern "C" fn IMTConServer_Name(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).name()
}

unsafe extern "C" fn IMTConServer_Name1(this: *mut IMTConServer, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).name1(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTConServer_Address1(this: *mut IMTConServer, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).address1(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTConServer_Address(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).address()
}

unsafe extern "C" fn IMTConServer_Id(this: *const IMTConServer) -> UINT64 {
    (*(*this).impl_ptr).id()
}

unsafe extern "C" fn IMTConServer_Id1(this: *mut IMTConServer, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).id1(id)
}

unsafe extern "C" fn IMTConServer_Password(this: *mut IMTConServer, password: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).password(conversion::to_utf16_str(password))
}

unsafe extern "C" fn IMTConServer_PasswordCheck(this: *const IMTConServer, password: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).password_check(conversion::to_utf16_str(password))
}

unsafe extern "C" fn IMTConServer_ServiceTime(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).service_time()
}

unsafe extern "C" fn IMTConServer_ServiceTime1(this: *mut IMTConServer, stime: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).service_time1(stime)
}

unsafe extern "C" fn IMTConServer_AdaptersCurrent(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).adapters_current()
}

unsafe extern "C" fn IMTConServer_AdaptersCurrent1(this: *mut IMTConServer, current: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).adapters_current1(conversion::to_utf16_str(current))
}

unsafe extern "C" fn IMTConServer_AdaptersTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).adapters_total()
}

unsafe extern "C" fn IMTConServer_AdaptersNext(this: *const IMTConServer, pos: UINT) -> LPCWSTR {
    (*(*this).impl_ptr).adapters_next(pos)
}

unsafe extern "C" fn IMTConServer_AddressTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).address_total()
}

unsafe extern "C" fn IMTConServer_AddressNext(this: *const IMTConServer, pos: UINT) -> UINT {
    (*(*this).impl_ptr).address_next(pos)
}

unsafe extern "C" fn IMTConServer_Version(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).version()
}

unsafe extern "C" fn IMTConServer_Build(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).build()
}

unsafe extern "C" fn IMTConServer_BuildDate(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).build_date()
}

unsafe extern "C" fn IMTConServer_LastBootTime(this: *const IMTConServer) -> INT64 {
    (*(*this).impl_ptr).last_boot_time()
}

unsafe extern "C" fn IMTConServer_Connected(this: *const IMTConServer) -> bool {
    (*(*this).impl_ptr).connected()
}

unsafe extern "C" fn IMTConServer_OS(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).os()
}

unsafe extern "C" fn IMTConServer_CPU(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).cpu()
}

unsafe extern "C" fn IMTConServer_CPUTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).cpu_total()
}

unsafe extern "C" fn IMTConServer_CPUUsageMax(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).cpu_usage_max()
}

unsafe extern "C" fn IMTConServer_CPUUsageCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).cpu_usage_critical()
}

unsafe extern "C" fn IMTConServer_MemoryTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).memory_total()
}

unsafe extern "C" fn IMTConServer_MemoryFree(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).memory_free()
}

unsafe extern "C" fn IMTConServer_MemoryFreeMin(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).memory_free_min()
}

unsafe extern "C" fn IMTConServer_MemoryFreeCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).memory_free_critical()
}

unsafe extern "C" fn IMTConServer_HDDTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_total()
}

unsafe extern "C" fn IMTConServer_HDDFree(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_free()
}

unsafe extern "C" fn IMTConServer_HDDFreeCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_free_critical()
}

unsafe extern "C" fn IMTConServer_HDDFragments(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_fragments()
}

unsafe extern "C" fn IMTConServer_HDDFragmentsCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_fragments_critical()
}

unsafe extern "C" fn IMTConServer_HDDSpeedRead(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_speed_read()
}

unsafe extern "C" fn IMTConServer_HDDSpeedReadCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_speed_read_critical()
}

unsafe extern "C" fn IMTConServer_HDDSpeedWrite(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_speed_write()
}

unsafe extern "C" fn IMTConServer_HDDSpeedWriteCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).hdd_speed_write_critical()
}

unsafe extern "C" fn IMTConServer_ConnectsMax(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).connects_max()
}

unsafe extern "C" fn IMTConServer_ConnectsCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).connects_critical()
}

unsafe extern "C" fn IMTConServer_NetworkMax(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).network_max()
}

unsafe extern "C" fn IMTConServer_NetworkCritical(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).network_critical()
}

unsafe extern "C" fn IMTConServer_TradeServer(this: *mut IMTConServer) -> *mut IMTConServerTrade {
    (*(*this).impl_ptr).trade_server()
}

unsafe extern "C" fn IMTConServer_HistoryServer(this: *mut IMTConServer) -> *mut IMTConServerHistory {
    (*(*this).impl_ptr).history_server()
}

unsafe extern "C" fn IMTConServer_AccessServer(this: *mut IMTConServer) -> *mut IMTConServerAccess {
    (*(*this).impl_ptr).access_server()
}

unsafe extern "C" fn IMTConServer_BackupServer(this: *mut IMTConServer) -> *mut IMTConServerBackup {
    (*(*this).impl_ptr).backup_server()
}

unsafe extern "C" fn IMTConServer_AntiDDoSServer(this: *mut IMTConServer) -> *mut IMTConServerAntiDDoS {
    (*(*this).impl_ptr).anti_ddos_server()
}

unsafe extern "C" fn IMTConServer_ReservedServer1(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void {
    (*(*this).impl_ptr).reserved_server1()
}

unsafe extern "C" fn IMTConServer_ReservedServer2(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void {
    (*(*this).impl_ptr).reserved_server2()
}

unsafe extern "C" fn IMTConServer_ReservedServer3(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void {
    (*(*this).impl_ptr).reserved_server3()
}

unsafe extern "C" fn IMTConServer_ReservedServer4(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void {
    (*(*this).impl_ptr).reserved_server4()
}

unsafe extern "C" fn IMTConServer_PointsAdd(this: *mut IMTConServer, path: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).points_add(conversion::to_utf16_str(path))
}

unsafe extern "C" fn IMTConServer_PointsUpdate(this: *mut IMTConServer, pos: UINT, address: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).points_update(pos, conversion::to_utf16_str(address))
}

unsafe extern "C" fn IMTConServer_PointsDelete(this: *mut IMTConServer, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).points_delete(pos)
}

unsafe extern "C" fn IMTConServer_PointsClear(this: *mut IMTConServer) -> MTAPIRES {
    (*(*this).impl_ptr).points_clear()
}

unsafe extern "C" fn IMTConServer_PointsShift(this: *mut IMTConServer, pos: UINT, shift: INT) -> MTAPIRES {
    (*(*this).impl_ptr).points_shift(pos, shift)
}

unsafe extern "C" fn IMTConServer_PointsTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).points_total()
}

unsafe extern "C" fn IMTConServer_PointsNext(this: *const IMTConServer, pos: UINT) -> LPCWSTR {
    (*(*this).impl_ptr).points_next(pos)
}

unsafe extern "C" fn IMTConServer_BindingsAdd(this: *mut IMTConServer, path: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).bindings_add(conversion::to_utf16_str(path))
}

unsafe extern "C" fn IMTConServer_BindingsUpdate(this: *mut IMTConServer, pos: UINT, address: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).bindings_update(pos, conversion::to_utf16_str(address))
}

unsafe extern "C" fn IMTConServer_BindingsDelete(this: *mut IMTConServer, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).bindings_delete(pos)
}

unsafe extern "C" fn IMTConServer_BindingsClear(this: *mut IMTConServer) -> MTAPIRES {
    (*(*this).impl_ptr).bindings_clear()
}

unsafe extern "C" fn IMTConServer_BindingsShift(this: *mut IMTConServer, pos: UINT, shift: INT) -> MTAPIRES {
    (*(*this).impl_ptr).bindings_shift(pos, shift)
}

unsafe extern "C" fn IMTConServer_BindingsTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).bindings_total()
}

unsafe extern "C" fn IMTConServer_BindingsNext(this: *const IMTConServer, pos: UINT) -> LPCWSTR {
    (*(*this).impl_ptr).bindings_next(pos)
}

unsafe extern "C" fn IMTConServer_FailoverMode(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).failover_mode()
}

unsafe extern "C" fn IMTConServer_FailoverMode1(this: *mut IMTConServer, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).failover_mode1(mode)
}

unsafe extern "C" fn IMTConServer_FailoverTimeout(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).failover_timeout()
}

unsafe extern "C" fn IMTConServer_FailoverTimeout1(this: *mut IMTConServer, timeout: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).failover_timeout1(timeout)
}

unsafe extern "C" fn IMTConServer_ClusterStateTotal(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).cluster_state_total()
}

unsafe extern "C" fn IMTConServer_ClusterStateNext(
    this: *const IMTConServer, pos: UINT, state: *mut IMTConClusterState) -> MTAPIRES {
    (*(*this).impl_ptr).cluster_state_next(pos, &mut *state)
}

unsafe extern "C" fn IMTConServer_ClusterStateGet(
    this: *const IMTConServer, id: UINT64, state: *mut IMTConClusterState) -> MTAPIRES {
    (*(*this).impl_ptr).cluster_state_get(id, &mut *state)
}

unsafe extern "C" fn IMTConServer_AddressIPv6(this: *const IMTConServer) -> LPCWSTR {
    (*(*this).impl_ptr).address_ipv6()
}

unsafe extern "C" fn IMTConServer_AddressIPv61(this: *mut IMTConServer, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).address_ipv61(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTConServer_AddressIPv6Total(this: *const IMTConServer) -> UINT {
    (*(*this).impl_ptr).address_ipv6_total()
}

unsafe extern "C" fn IMTConServer_AddressIPv6Next(this: *const IMTConServer, pos: UINT, address: *mut MTAPISTR) -> MTAPIRES {
    (*(*this).impl_ptr).address_ipv6_next(pos, &mut *address)
}

pub const fn new() -> IMTConServer__bindgen_vtable {
     IMTConServer__bindgen_vtable {
         IMTConServer_Release,
         IMTConServer_Assign,
         IMTConServer_Clear,
         IMTConServer_Type,
         IMTConServer_Type1,
         IMTConServer_Name,
         IMTConServer_Name1,
         IMTConServer_Address1,
         IMTConServer_Address,
         IMTConServer_Id,
         IMTConServer_Id1,
         IMTConServer_Password,
         IMTConServer_PasswordCheck,
         IMTConServer_ServiceTime,
         IMTConServer_ServiceTime1,
         IMTConServer_AdaptersCurrent,
         IMTConServer_AdaptersCurrent1,
         IMTConServer_AdaptersTotal,
         IMTConServer_AdaptersNext,
         IMTConServer_AddressTotal,
         IMTConServer_AddressNext,
         IMTConServer_Version,
         IMTConServer_Build,
         IMTConServer_BuildDate,
         IMTConServer_LastBootTime,
         IMTConServer_Connected,
         IMTConServer_OS,
         IMTConServer_CPU,
         IMTConServer_CPUTotal,
         IMTConServer_CPUUsageMax,
         IMTConServer_CPUUsageCritical,
         IMTConServer_MemoryTotal,
         IMTConServer_MemoryFree,
         IMTConServer_MemoryFreeMin,
         IMTConServer_MemoryFreeCritical,
         IMTConServer_HDDTotal,
         IMTConServer_HDDFree,
         IMTConServer_HDDFreeCritical,
         IMTConServer_HDDFragments,
         IMTConServer_HDDFragmentsCritical,
         IMTConServer_HDDSpeedRead,
         IMTConServer_HDDSpeedReadCritical,
         IMTConServer_HDDSpeedWrite,
         IMTConServer_HDDSpeedWriteCritical,
         IMTConServer_ConnectsMax,
         IMTConServer_ConnectsCritical,
         IMTConServer_NetworkMax,
         IMTConServer_NetworkCritical,
         IMTConServer_TradeServer,
         IMTConServer_HistoryServer,
         IMTConServer_AccessServer,
         IMTConServer_BackupServer,
         IMTConServer_AntiDDoSServer,
         IMTConServer_ReservedServer1,
         IMTConServer_ReservedServer2,
         IMTConServer_ReservedServer3,
         IMTConServer_ReservedServer4,
         IMTConServer_PointsAdd,
         IMTConServer_PointsUpdate,
         IMTConServer_PointsDelete,
         IMTConServer_PointsClear,
         IMTConServer_PointsShift,
         IMTConServer_PointsTotal,
         IMTConServer_PointsNext,
         IMTConServer_BindingsAdd,
         IMTConServer_BindingsUpdate,
         IMTConServer_BindingsDelete,
         IMTConServer_BindingsClear,
         IMTConServer_BindingsShift,
         IMTConServer_BindingsTotal,
         IMTConServer_BindingsNext,
         IMTConServer_FailoverMode,
         IMTConServer_FailoverMode1,
         IMTConServer_FailoverTimeout,
         IMTConServer_FailoverTimeout1,
         IMTConServer_ClusterStateTotal,
         IMTConServer_ClusterStateNext,
         IMTConServer_ClusterStateGet,
         IMTConServer_AddressIPv6,
         IMTConServer_AddressIPv61,
         IMTConServer_AddressIPv6Total,
         IMTConServer_AddressIPv6Next,
    }
}
