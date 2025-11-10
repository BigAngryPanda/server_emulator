use crate::{
    mt5_apiserver,
    conversion
};

use mt5_apiserver::*;
use std::os::raw::{
    c_void,
    c_int
};

unsafe extern "C" fn IMTServerAPI_About(this: *mut IMTServerAPI, info: *mut MTServerInfo)
    -> MTAPIRES
{
    (*(*this).impl_ptr).about(&mut *info)
}

unsafe extern "C" fn IMTServerAPI_LicenseCheck(this: *mut IMTServerAPI, license_name: LPCWSTR)
    -> MTAPIRES
{
    (*(*this).impl_ptr).license_check(&*license_name)
}

unsafe extern "C" fn IMTServerAPI_Allocate(this: *mut IMTServerAPI, bytes: UINT)
    -> *mut c_void
{
    (*(*this).impl_ptr).allocate(bytes)
}

unsafe extern "C" fn IMTServerAPI_Free(this: *mut IMTServerAPI, ptr: *mut c_void)
{
    (*(*this).impl_ptr).free(ptr);
}

unsafe extern "C" fn IMTServerAPI_ServerRestart(this: *mut IMTServerAPI)
    -> MTAPIRES
{
    (*(*this).impl_ptr).server_restart()
}

unsafe extern "C" fn IMTServerAPI_ServerSubscribe(this: *mut IMTServerAPI, sink: *mut IMTServerSink)
    -> MTAPIRES
{
    (*(*this).impl_ptr).server_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ServerUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTServerSink)
    -> MTAPIRES
{
    (*(*this).impl_ptr).server_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ServerRestartRemote(this: *mut IMTServerAPI, id: UINT64, reason: LPCWSTR)
    -> MTAPIRES
{
    (*(*this).impl_ptr).server_restart_remote(id, conversion::to_utf16_str(reason))
}

unsafe extern "C" fn IMTServerAPI_ServerReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).server_reserved4()
}

unsafe extern "C" fn IMTServerAPI_LoggerOut(this: *mut IMTServerAPI, code: UINT, msg: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).logger_out(code, conversion::to_utf16_str(msg))
}

unsafe extern "C" fn IMTServerAPI_LoggerRequest(
    this: *mut IMTServerAPI,
    mode: UINT,
    type_: UINT,
    from: INT64,
    to: INT64,
    filter: LPCWSTR,
    records: *mut *mut MTLogRecord,
    records_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).logger_request(mode, type_, from, to, conversion::to_utf16_str(filter), &mut (&mut **records), &mut *records_total)
}

unsafe extern "C" fn IMTServerAPI_LoggerFlush(this: *mut IMTServerAPI) {
    (*(*this).impl_ptr).logger_flush();
}

unsafe extern "C" fn IMTServerAPI_LoggerReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).logger_reserved1()
}

unsafe extern "C" fn IMTServerAPI_LoggerReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).logger_reserved2()
}

unsafe extern "C" fn IMTServerAPI_LoggerReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).logger_reserved3()
}

unsafe extern "C" fn IMTServerAPI_LoggerReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).logger_reserved4()
}

unsafe extern "C" fn IMTServerAPI_PluginCreate(this: *mut IMTServerAPI) -> *mut IMTConPlugin {
    (*(*this).impl_ptr).plugin_create()
}

unsafe extern "C" fn IMTServerAPI_PluginModuleCreate(this: *mut IMTServerAPI) -> *mut IMTConPluginModule {
    (*(*this).impl_ptr).plugin_module_create()
}

unsafe extern "C" fn IMTServerAPI_PluginParamCreate(this: *mut IMTServerAPI) -> *mut IMTConParam {
    (*(*this).impl_ptr).plugin_param_create()
}

unsafe extern "C" fn IMTServerAPI_PluginSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConPluginSink) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_PluginUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConPluginSink) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_PluginCurrent(this: *mut IMTServerAPI, plugin: *mut IMTConPlugin) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_current(&mut *plugin)
}

unsafe extern "C" fn IMTServerAPI_PluginAdd(this: *mut IMTServerAPI, plugin: *mut IMTConPlugin) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_add(&mut *plugin)
}

unsafe extern "C" fn IMTServerAPI_PluginDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_PluginDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_PluginShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_PluginTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).plugin_total()
}

unsafe extern "C" fn IMTServerAPI_PluginNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    plugin: *mut IMTConPlugin,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_next(pos, &mut *plugin)
}

unsafe extern "C" fn IMTServerAPI_PluginGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    plugin: *mut IMTConPlugin,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_get(conversion::to_utf16_str(name), &mut *plugin)
}

unsafe extern "C" fn IMTServerAPI_PluginModuleTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).plugin_module_total()
}

unsafe extern "C" fn IMTServerAPI_PluginModuleNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    module: *mut IMTConPluginModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_module_next(pos, &mut *module)
}

unsafe extern "C" fn IMTServerAPI_PluginModuleGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    module: *mut IMTConPluginModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_module_get(conversion::to_utf16_str(name), &mut *module)
}

unsafe extern "C" fn IMTServerAPI_PluginDelete2(this: *mut IMTServerAPI, server: UINT64, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_delete2(server, conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_PluginGet1(
    this: *mut IMTServerAPI,
    server: UINT64,
    name: LPCWSTR,
    plugin: *mut IMTConPlugin,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_get1(server, conversion::to_utf16_str(name), &mut *plugin)
}

unsafe extern "C" fn IMTServerAPI_PluginModuleGet1(
    this: *mut IMTServerAPI,
    server: UINT64,
    name: LPCWSTR,
    module: *mut IMTConPluginModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_module_get1(server, conversion::to_utf16_str(name), &mut *module)
}

unsafe extern "C" fn IMTServerAPI_PluginReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).plugin_reserved4()
}

unsafe extern "C" fn IMTServerAPI_CommonCreate(this: *mut IMTServerAPI) -> *mut IMTConCommon {
    (*(*this).impl_ptr).common_create()
}

unsafe extern "C" fn IMTServerAPI_CommonSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConCommonSink) -> MTAPIRES {
    (*(*this).impl_ptr).common_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_CommonUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConCommonSink) -> MTAPIRES {
    (*(*this).impl_ptr).common_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_CommonGet(this: *mut IMTServerAPI, common: *mut IMTConCommon) -> MTAPIRES {
    (*(*this).impl_ptr).common_get(&mut *common)
}

unsafe extern "C" fn IMTServerAPI_CommonSet(this: *mut IMTServerAPI, common: *const IMTConCommon) -> MTAPIRES {
    (*(*this).impl_ptr).common_set(&*common)
}

unsafe extern "C" fn IMTServerAPI_CommonReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).common_reserved1()
}

unsafe extern "C" fn IMTServerAPI_CommonReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).common_reserved2()
}

unsafe extern "C" fn IMTServerAPI_CommonReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).common_reserved3()
}

unsafe extern "C" fn IMTServerAPI_CommonReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).common_reserved4()
}

unsafe extern "C" fn IMTServerAPI_NetServerCreate(this: *mut IMTServerAPI) -> *mut IMTConServer {
    (*(*this).impl_ptr).net_server_create()
}

unsafe extern "C" fn IMTServerAPI_NetServerRangeCreate(this: *mut IMTServerAPI) -> *mut IMTConServerRange {
    (*(*this).impl_ptr).net_server_range_create()
}

unsafe extern "C" fn IMTServerAPI_NetServerSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConServerSink) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_NetServerUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConServerSink) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_NetServerAdd(this: *mut IMTServerAPI, config: *mut IMTConServer) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_NetServerDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_NetServerShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_NetServerTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).net_server_total()
}

unsafe extern "C" fn IMTServerAPI_NetServerNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    config: *mut IMTConServer,
) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_next(pos, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_NetServerGet(
    this: *mut IMTServerAPI,
    id: UINT64,
    config: *mut IMTConServer,
) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_get(id, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_NetServerAddressRangeCreate(this: *mut IMTServerAPI) -> *mut IMTConAddressRange {
    (*(*this).impl_ptr).net_server_address_range_create()
}

unsafe extern "C" fn IMTServerAPI_NetServerClusterStateCreate(this: *mut IMTServerAPI) -> *mut IMTConClusterState {
    (*(*this).impl_ptr).net_server_cluster_state_create()
}

unsafe extern "C" fn IMTServerAPI_NetServerReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_reserved3()
}

unsafe extern "C" fn IMTServerAPI_NetServerReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).net_server_reserved4()
}

unsafe extern "C" fn IMTServerAPI_TimeCreate(this: *mut IMTServerAPI) -> *mut IMTConTime {
    (*(*this).impl_ptr).time_create()
}

unsafe extern "C" fn IMTServerAPI_TimeSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConTimeSink) -> MTAPIRES {
    (*(*this).impl_ptr).time_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TimeUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConTimeSink) -> MTAPIRES {
    (*(*this).impl_ptr).time_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TimeCurrent(this: *mut IMTServerAPI) -> INT64 {
    (*(*this).impl_ptr).time_current()
}

unsafe extern "C" fn IMTServerAPI_TimeGet(this: *mut IMTServerAPI, config: *mut IMTConTime) -> MTAPIRES {
    (*(*this).impl_ptr).time_get(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_TimeSet(this: *mut IMTServerAPI, config: *const IMTConTime) -> MTAPIRES {
    (*(*this).impl_ptr).time_set(&*config)
}

unsafe extern "C" fn IMTServerAPI_TimeCurrentMsc(this: *mut IMTServerAPI) -> INT64 {
    (*(*this).impl_ptr).time_current_msc()
}

unsafe extern "C" fn IMTServerAPI_TimeReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).time_reserved2()
}

unsafe extern "C" fn IMTServerAPI_TimeReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).time_reserved3()
}

unsafe extern "C" fn IMTServerAPI_TimeReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).time_reserved4()
}

unsafe extern "C" fn IMTServerAPI_HolidayCreate(this: *mut IMTServerAPI) -> *mut IMTConHoliday {
    (*(*this).impl_ptr).holiday_create()
}

unsafe extern "C" fn IMTServerAPI_HolidaySubscribe(this: *mut IMTServerAPI, sink: *mut IMTConHolidaySink) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_HolidayUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConHolidaySink) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_HolidayAdd(this: *mut IMTServerAPI, config: *mut IMTConHoliday) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_HolidayDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_HolidayShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_HolidayTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).holiday_total()
}

unsafe extern "C" fn IMTServerAPI_HolidayNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    config: *mut IMTConHoliday,
) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_next(pos, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_HolidayReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_reserved1()
}

unsafe extern "C" fn IMTServerAPI_HolidayReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_reserved2()
}

unsafe extern "C" fn IMTServerAPI_HolidayReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_reserved3()
}

unsafe extern "C" fn IMTServerAPI_HolidayReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).holiday_reserved4()
}

unsafe extern "C" fn IMTServerAPI_FirewallCreate(this: *mut IMTServerAPI) -> *mut IMTConFirewall {
    (*(*this).impl_ptr).firewall_create()
}

unsafe extern "C" fn IMTServerAPI_FirewallSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConFirewallSink) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_FirewallUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConFirewallSink) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_FirewallAdd(this: *mut IMTServerAPI, config: *mut IMTConFirewall) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_FirewallDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_FirewallShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_FirewallTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).firewall_total()
}

unsafe extern "C" fn IMTServerAPI_FirewallNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    config: *mut IMTConFirewall,
) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_next(pos, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_FirewallReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_reserved1()
}

unsafe extern "C" fn IMTServerAPI_FirewallReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_reserved2()
}

unsafe extern "C" fn IMTServerAPI_FirewallReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_reserved3()
}

unsafe extern "C" fn IMTServerAPI_FirewallReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).firewall_reserved4()
}

unsafe extern "C" fn IMTServerAPI_SymbolCreate(this: *mut IMTServerAPI) -> *mut IMTConSymbol {
    (*(*this).impl_ptr).symbol_create()
}

unsafe extern "C" fn IMTServerAPI_SymbolSessionCreate(this: *mut IMTServerAPI) -> *mut IMTConSymbolSession {
    (*(*this).impl_ptr).symbol_session_create()
}

unsafe extern "C" fn IMTServerAPI_SymbolSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConSymbolSink) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SymbolUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConSymbolSink) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SymbolAdd(this: *mut IMTServerAPI, symbol: *mut IMTConSymbol) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_add(&mut *symbol)
}

unsafe extern "C" fn IMTServerAPI_SymbolDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_SymbolDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_SymbolShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_SymbolTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).symbol_total()
}

unsafe extern "C" fn IMTServerAPI_SymbolNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    symbol: *mut IMTConSymbol,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_next(pos, &mut *symbol)
}

unsafe extern "C" fn IMTServerAPI_SymbolGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    symbol: *mut IMTConSymbol,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_get(conversion::to_utf16_str(name), &mut *symbol)
}

unsafe extern "C" fn IMTServerAPI_SymbolGet1(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    group: *const IMTConGroup,
    symbol: *mut IMTConSymbol,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_get1(conversion::to_utf16_str(name), &*group, &mut *symbol)
}

unsafe extern "C" fn IMTServerAPI_SymbolExist(
    this: *mut IMTServerAPI,
    symbol: *const IMTConSymbol,
    group: *const IMTConGroup,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_exist(&*symbol, &*group)
}

unsafe extern "C" fn IMTServerAPI_SymbolReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_reserved1()
}

unsafe extern "C" fn IMTServerAPI_SymbolReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_reserved2()
}

unsafe extern "C" fn IMTServerAPI_SymbolReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_reserved3()
}

unsafe extern "C" fn IMTServerAPI_SymbolReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_reserved4()
}

unsafe extern "C" fn IMTServerAPI_GroupCreate(this: *mut IMTServerAPI) -> *mut IMTConGroup {
    (*(*this).impl_ptr).group_create()
}

unsafe extern "C" fn IMTServerAPI_GroupSymbolCreate(this: *mut IMTServerAPI) -> *mut IMTConGroupSymbol {
    (*(*this).impl_ptr).group_symbol_create()
}

unsafe extern "C" fn IMTServerAPI_GroupCommissionCreate(this: *mut IMTServerAPI) -> *mut IMTConCommission {
    (*(*this).impl_ptr).group_commission_create()
}

unsafe extern "C" fn IMTServerAPI_GroupTierCreate(this: *mut IMTServerAPI) -> *mut IMTConCommTier {
    (*(*this).impl_ptr).group_tier_create()
}

unsafe extern "C" fn IMTServerAPI_GroupSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConGroupSink) -> MTAPIRES {
    (*(*this).impl_ptr).group_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_GroupUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConGroupSink) -> MTAPIRES {
    (*(*this).impl_ptr).group_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_GroupAdd(this: *mut IMTServerAPI, group: *mut IMTConGroup) -> MTAPIRES {
    (*(*this).impl_ptr).group_add(&mut *group)
}

unsafe extern "C" fn IMTServerAPI_GroupDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).group_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_GroupDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).group_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_GroupShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).group_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_GroupTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).group_total()
}

unsafe extern "C" fn IMTServerAPI_GroupNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    group: *mut IMTConGroup,
) -> MTAPIRES {
    (*(*this).impl_ptr).group_next(pos, &mut *group)
}

unsafe extern "C" fn IMTServerAPI_GroupGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    group: *mut IMTConGroup,
) -> MTAPIRES {
    (*(*this).impl_ptr).group_get(conversion::to_utf16_str(name), &mut *group)
}

unsafe extern "C" fn IMTServerAPI_GroupReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).group_reserved1()
}

unsafe extern "C" fn IMTServerAPI_GroupReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).group_reserved2()
}

unsafe extern "C" fn IMTServerAPI_GroupReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).group_reserved3()
}

unsafe extern "C" fn IMTServerAPI_GroupReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).group_reserved4()
}

unsafe extern "C" fn IMTServerAPI_ManagerCreate(this: *mut IMTServerAPI) -> *mut IMTConManager {
    (*(*this).impl_ptr).manager_create()
}

unsafe extern "C" fn IMTServerAPI_ManagerAccessCreate(this: *mut IMTServerAPI) -> *mut IMTConManagerAccess {
    (*(*this).impl_ptr).manager_access_create()
}

unsafe extern "C" fn IMTServerAPI_ManagerSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConManagerSink) -> MTAPIRES {
    (*(*this).impl_ptr).manager_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ManagerUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConManagerSink) -> MTAPIRES {
    (*(*this).impl_ptr).manager_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ManagerAdd(this: *mut IMTServerAPI, manager: *mut IMTConManager) -> MTAPIRES {
    (*(*this).impl_ptr).manager_add(&mut *manager)
}

unsafe extern "C" fn IMTServerAPI_ManagerDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).manager_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_ManagerShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).manager_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_ManagerTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).manager_total()
}

unsafe extern "C" fn IMTServerAPI_ManagerNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    manager: *mut IMTConManager,
) -> MTAPIRES {
    (*(*this).impl_ptr).manager_next(pos, &mut *manager)
}

unsafe extern "C" fn IMTServerAPI_ManagerGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    manager: *mut IMTConManager,
) -> MTAPIRES {
    (*(*this).impl_ptr).manager_get(login, &mut *manager)
}

unsafe extern "C" fn IMTServerAPI_ManagerReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).manager_reserved1()
}

unsafe extern "C" fn IMTServerAPI_ManagerReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).manager_reserved2()
}

unsafe extern "C" fn IMTServerAPI_ManagerReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).manager_reserved3()
}

unsafe extern "C" fn IMTServerAPI_ManagerReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).manager_reserved4()
}

unsafe extern "C" fn IMTServerAPI_HistorySyncCreate(this: *mut IMTServerAPI) -> *mut IMTConHistorySync {
    (*(*this).impl_ptr).history_sync_create()
}

unsafe extern "C" fn IMTServerAPI_HistorySyncSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConHistorySyncSink) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_HistorySyncUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConHistorySyncSink) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_HistorySyncAdd(this: *mut IMTServerAPI, config: *mut IMTConHistorySync) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_HistorySyncDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_HistorySyncShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_HistorySyncTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).history_sync_total()
}

unsafe extern "C" fn IMTServerAPI_HistorySyncNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    config: *mut IMTConHistorySync,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_next(pos, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_HistorySyncReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_reserved1()
}

unsafe extern "C" fn IMTServerAPI_HistorySyncReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_reserved2()
}

unsafe extern "C" fn IMTServerAPI_HistorySyncReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_reserved3()
}

unsafe extern "C" fn IMTServerAPI_HistorySyncReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_sync_reserved4()
}

unsafe extern "C" fn IMTServerAPI_FeederCreate(this: *mut IMTServerAPI) -> *mut IMTConFeeder {
    (*(*this).impl_ptr).feeder_create()
}

unsafe extern "C" fn IMTServerAPI_FeederModuleCreate(this: *mut IMTServerAPI) -> *mut IMTConFeederModule {
    (*(*this).impl_ptr).feeder_module_create()
}

unsafe extern "C" fn IMTServerAPI_FeederParamCreate(this: *mut IMTServerAPI) -> *mut IMTConParam {
    (*(*this).impl_ptr).feeder_param_create()
}

unsafe extern "C" fn IMTServerAPI_FeederTranslateCreate(this: *mut IMTServerAPI) -> *mut IMTConFeederTranslate {
    (*(*this).impl_ptr).feeder_translate_create()
}

unsafe extern "C" fn IMTServerAPI_FeederSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConFeederSink) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_FeederUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConFeederSink) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_FeederAdd(this: *mut IMTServerAPI, feeder: *mut IMTConFeeder) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_add(&mut *feeder)
}

unsafe extern "C" fn IMTServerAPI_FeederDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_FeederDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_FeederShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_FeederTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).feeder_total()
}

unsafe extern "C" fn IMTServerAPI_FeederNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    feeder: *mut IMTConFeeder,
) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_next(pos, &mut *feeder)
}

unsafe extern "C" fn IMTServerAPI_FeederGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    feeder: *mut IMTConFeeder,
) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_get(conversion::to_utf16_str(name), &mut *feeder)
}

unsafe extern "C" fn IMTServerAPI_FeederModuleTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).feeder_module_total()
}

unsafe extern "C" fn IMTServerAPI_FeederModuleNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    module: *mut IMTConFeederModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_module_next(pos, &mut *module)
}

unsafe extern "C" fn IMTServerAPI_FeederModuleGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    module: *mut IMTConFeederModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_module_get(conversion::to_utf16_str(name), &mut *module)
}

unsafe extern "C" fn IMTServerAPI_FeederRestart(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_restart(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_FeederReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_reserved2()
}

unsafe extern "C" fn IMTServerAPI_FeederReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_reserved3()
}

unsafe extern "C" fn IMTServerAPI_FeederReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).feeder_reserved4()
}

unsafe extern "C" fn IMTServerAPI_GatewayCreate(this: *mut IMTServerAPI) -> *mut IMTConGateway {
    (*(*this).impl_ptr).gateway_create()
}

unsafe extern "C" fn IMTServerAPI_GatewayModuleCreate(this: *mut IMTServerAPI) -> *mut IMTConGatewayModule {
    (*(*this).impl_ptr).gateway_module_create()
}

unsafe extern "C" fn IMTServerAPI_GatewayParamCreate(this: *mut IMTServerAPI) -> *mut IMTConParam {
    (*(*this).impl_ptr).gateway_param_create()
}

unsafe extern "C" fn IMTServerAPI_GatewayTranslateCreate(this: *mut IMTServerAPI) -> *mut IMTConGatewayTranslate {
    (*(*this).impl_ptr).gateway_translate_create()
}

unsafe extern "C" fn IMTServerAPI_GatewaySubscribe(this: *mut IMTServerAPI, sink: *mut IMTConGatewaySink) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_GatewayUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConGatewaySink) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_GatewayAdd(this: *mut IMTServerAPI, gateway: *mut IMTConGateway) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_add(&mut *gateway)
}

unsafe extern "C" fn IMTServerAPI_GatewayDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_GatewayDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_GatewayShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_GatewayTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).gateway_total()
}

unsafe extern "C" fn IMTServerAPI_GatewayNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    gateway: *mut IMTConGateway,
) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_next(pos, &mut *gateway)
}

unsafe extern "C" fn IMTServerAPI_GatewayGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    gateway: *mut IMTConGateway,
) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_get(conversion::to_utf16_str(name), &mut *gateway)
}

unsafe extern "C" fn IMTServerAPI_GatewayModuleTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).gateway_module_total()
}

unsafe extern "C" fn IMTServerAPI_GatewayModuleNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    module: *mut IMTConGatewayModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_module_next(pos, &mut *module)
}

unsafe extern "C" fn IMTServerAPI_GatewayModuleGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    module: *mut IMTConGatewayModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_module_get(conversion::to_utf16_str(name), &mut *module)
}

unsafe extern "C" fn IMTServerAPI_GatewayRestart(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_restart(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_GatewayReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_reserved2()
}

unsafe extern "C" fn IMTServerAPI_GatewayReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_reserved3()
}

unsafe extern "C" fn IMTServerAPI_GatewayReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_reserved4()
}

unsafe extern "C" fn IMTServerAPI_ReportCreate(this: *mut IMTServerAPI) -> *mut IMTConReport {
    (*(*this).impl_ptr).report_create()
}

unsafe extern "C" fn IMTServerAPI_ReportModuleCreate(this: *mut IMTServerAPI) -> *mut IMTConReportModule {
    (*(*this).impl_ptr).report_module_create()
}

unsafe extern "C" fn IMTServerAPI_ReportParamCreate(this: *mut IMTServerAPI) -> *mut IMTConParam {
    (*(*this).impl_ptr).report_param_create()
}

unsafe extern "C" fn IMTServerAPI_ReportSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConReportSink) -> MTAPIRES {
    (*(*this).impl_ptr).report_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ReportUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConReportSink) -> MTAPIRES {
    (*(*this).impl_ptr).report_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ReportAdd(this: *mut IMTServerAPI, report: *mut IMTConReport) -> MTAPIRES {
    (*(*this).impl_ptr).report_add(&mut *report)
}

unsafe extern "C" fn IMTServerAPI_ReportDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).report_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_ReportDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).report_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_ReportShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_ReportTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).report_total()
}

unsafe extern "C" fn IMTServerAPI_ReportNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    report: *mut IMTConReport,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_next(pos, &mut *report)
}

unsafe extern "C" fn IMTServerAPI_ReportGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    report: *mut IMTConReport,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_get(conversion::to_utf16_str(name), &mut *report)
}

unsafe extern "C" fn IMTServerAPI_ReportModuleTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).report_module_total()
}

unsafe extern "C" fn IMTServerAPI_ReportModuleNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    module: *mut IMTConReportModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_module_next(pos, &mut *module)
}

unsafe extern "C" fn IMTServerAPI_ReportModuleGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    module: *mut IMTConReportModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_module_get(conversion::to_utf16_str(name), &mut *module)
}

unsafe extern "C" fn IMTServerAPI_ReportDelete2(this: *mut IMTServerAPI, server: UINT64, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).report_delete2(server, conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_ReportGet1(
    this: *mut IMTServerAPI,
    server: UINT64,
    name: LPCWSTR,
    report: *mut IMTConReport,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_get1(server, conversion::to_utf16_str(name), &mut *report)
}

unsafe extern "C" fn IMTServerAPI_ReportModuleGet1(
    this: *mut IMTServerAPI,
    server: UINT64,
    name: LPCWSTR,
    module: *mut IMTConReportModule,
) -> MTAPIRES {
    (*(*this).impl_ptr).report_module_get1(server, conversion::to_utf16_str(name), &mut *module)
}

unsafe extern "C" fn IMTServerAPI_ReportReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).report_reserved4()
}

unsafe extern "C" fn IMTServerAPI_RouteCreate(this: *mut IMTServerAPI) -> *mut IMTConRoute {
    (*(*this).impl_ptr).route_create()
}

unsafe extern "C" fn IMTServerAPI_RouteConditionCreate(this: *mut IMTServerAPI) -> *mut IMTConCondition {
    (*(*this).impl_ptr).route_condition_create()
}

unsafe extern "C" fn IMTServerAPI_RouteDealerCreate(this: *mut IMTServerAPI) -> *mut IMTConRouteDealer {
    (*(*this).impl_ptr).route_dealer_create()
}

unsafe extern "C" fn IMTServerAPI_RouteSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConRouteSink) -> MTAPIRES {
    (*(*this).impl_ptr).route_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_RouteUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConRouteSink) -> MTAPIRES {
    (*(*this).impl_ptr).route_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_RouteAdd(this: *mut IMTServerAPI, route: *mut IMTConRoute) -> MTAPIRES {
    (*(*this).impl_ptr).route_add(&mut *route)
}

unsafe extern "C" fn IMTServerAPI_RouteDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).route_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_RouteDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).route_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_RouteShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).route_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_RouteTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).route_total()
}

unsafe extern "C" fn IMTServerAPI_RouteNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    route: *mut IMTConRoute,
) -> MTAPIRES {
    (*(*this).impl_ptr).route_next(pos, &mut *route)
}

unsafe extern "C" fn IMTServerAPI_RouteGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    route: *mut IMTConRoute,
) -> MTAPIRES {
    (*(*this).impl_ptr).route_get(conversion::to_utf16_str(name), &mut *route)
}

unsafe extern "C" fn IMTServerAPI_RouteReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).route_reserved1()
}

unsafe extern "C" fn IMTServerAPI_RouteReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).route_reserved2()
}

unsafe extern "C" fn IMTServerAPI_RouteReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).route_reserved3()
}

unsafe extern "C" fn IMTServerAPI_RouteReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).route_reserved4()
}

unsafe extern "C" fn IMTServerAPI_UserCreate(this: *mut IMTServerAPI) -> *mut IMTUser {
    (*(*this).impl_ptr).user_create()
}

unsafe extern "C" fn IMTServerAPI_UserCreateAccount(this: *mut IMTServerAPI) -> *mut IMTAccount {
    (*(*this).impl_ptr).user_create_account()
}

unsafe extern "C" fn IMTServerAPI_UserSubscribe(this: *mut IMTServerAPI, sink: *mut IMTUserSink) -> MTAPIRES {
    (*(*this).impl_ptr).user_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_UserUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTUserSink) -> MTAPIRES {
    (*(*this).impl_ptr).user_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_UserAdd(
    this: *mut IMTServerAPI,
    user: *mut IMTUser,
    master_pass: LPCWSTR,
    investor_pass: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_add(&mut *user, conversion::to_utf16_str(master_pass), conversion::to_utf16_str(investor_pass))
}

unsafe extern "C" fn IMTServerAPI_UserDelete(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).user_delete(login)
}

unsafe extern "C" fn IMTServerAPI_UserUpdate(this: *mut IMTServerAPI, user: *mut IMTUser) -> MTAPIRES {
    (*(*this).impl_ptr).user_update(&mut *user)
}

unsafe extern "C" fn IMTServerAPI_UserTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).user_total()
}

unsafe extern "C" fn IMTServerAPI_UserGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    user: *mut IMTUser,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_get(login, &mut *user)
}

unsafe extern "C" fn IMTServerAPI_UserGroup(
    this: *mut IMTServerAPI,
    login: UINT64,
    group: *mut MTAPISTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_group(login, &mut *group)
}

unsafe extern "C" fn IMTServerAPI_UserLogins(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    logins: *mut *mut UINT64,
    logins_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_logins(conversion::to_utf16_str(group), &mut (&mut **logins), &mut *logins_total)
}

unsafe extern "C" fn IMTServerAPI_UserPasswordCheck(
    this: *mut IMTServerAPI,
    type_: UINT,
    login: UINT64,
    password: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_password_check(type_, login, conversion::to_utf16_str(password))
}

unsafe extern "C" fn IMTServerAPI_UserPasswordChange(
    this: *mut IMTServerAPI,
    type_: UINT,
    login: UINT64,
    password: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_password_change(type_, login, conversion::to_utf16_str(password))
}

unsafe extern "C" fn IMTServerAPI_UserCertDelete(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_delete(login)
}

unsafe extern "C" fn IMTServerAPI_UserCertConfirm(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_confirm(login)
}

unsafe extern "C" fn IMTServerAPI_UserDepositChangeRaw(
    this: *mut IMTServerAPI,
    login: UINT64,
    value: f64,
    type_: UINT,
    comment: LPCWSTR,
    deal_id: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_deposit_change_raw(login, value, type_, conversion::to_utf16_str(comment), &mut *deal_id)
}

unsafe extern "C" fn IMTServerAPI_UserDepositChange(
    this: *mut IMTServerAPI,
    login: UINT64,
    value: f64,
    action: UINT,
    comment: LPCWSTR,
    deal_id: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_deposit_change(login, value, action, conversion::to_utf16_str(comment), &mut *deal_id)
}

unsafe extern "C" fn IMTServerAPI_UserAccountGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    account: *mut IMTAccount,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_account_get(login, &mut *account)
}

unsafe extern "C" fn IMTServerAPI_UserArchive(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).user_archive(login)
}

unsafe extern "C" fn IMTServerAPI_UserArchiveGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    user: *mut IMTUser,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_archive_get(login, &mut *user)
}

unsafe extern "C" fn IMTServerAPI_UserRestore(this: *mut IMTServerAPI, user: *mut IMTUser) -> MTAPIRES {
    (*(*this).impl_ptr).user_restore(&mut *user)
}

unsafe extern "C" fn IMTServerAPI_UserArchiveLogins(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    logins: *mut *mut UINT64,
    logins_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_archive_logins(conversion::to_utf16_str(group), &mut (&mut **logins), &mut *logins_total)
}

unsafe extern "C" fn IMTServerAPI_DealCreate(this: *mut IMTServerAPI) -> *mut IMTDeal {
    (*(*this).impl_ptr).deal_create()
}

unsafe extern "C" fn IMTServerAPI_DealCreateArray(this: *mut IMTServerAPI) -> *mut IMTDealArray {
    (*(*this).impl_ptr).deal_create_array()
}

unsafe extern "C" fn IMTServerAPI_DealSubscribe(this: *mut IMTServerAPI, sink: *mut IMTDealSink) -> MTAPIRES {
    (*(*this).impl_ptr).deal_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DealUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTDealSink) -> MTAPIRES {
    (*(*this).impl_ptr).deal_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DealDelete(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).deal_delete(ticket)
}

unsafe extern "C" fn IMTServerAPI_DealUpdate(this: *mut IMTServerAPI, deal: *mut IMTDeal) -> MTAPIRES {
    (*(*this).impl_ptr).deal_update(&mut *deal)
}

unsafe extern "C" fn IMTServerAPI_DealGet(
    this: *mut IMTServerAPI,
    ticket: UINT64,
    deal: *mut IMTDeal,
) -> MTAPIRES {
    (*(*this).impl_ptr).deal_get(ticket, &mut *deal)
}

unsafe extern "C" fn IMTServerAPI_DealGet1(
    this: *mut IMTServerAPI,
    from: INT64,
    to: INT64,
    login: UINT64,
    deals: *mut IMTDealArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).deal_get1(from, to, login, &mut *deals)
}

unsafe extern "C" fn IMTServerAPI_DealAdd(this: *mut IMTServerAPI, deal: *mut IMTDeal) -> MTAPIRES {
    (*(*this).impl_ptr).deal_add(&mut *deal)
}

unsafe extern "C" fn IMTServerAPI_DealPerform(this: *mut IMTServerAPI, deal: *mut IMTDeal) -> MTAPIRES {
    (*(*this).impl_ptr).deal_perform(&mut *deal)
}

unsafe extern "C" fn IMTServerAPI_DealPerformCloseBy(
    this: *mut IMTServerAPI,
    deal: *mut IMTDeal,
    dealby: *mut IMTDeal,
) -> MTAPIRES {
    (*(*this).impl_ptr).deal_perform_close_by(&mut *deal, &mut *dealby)
}

unsafe extern "C" fn IMTServerAPI_DealDeleteBatch(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    (*(*this).impl_ptr).deal_delete_batch(tickets, tickets_total, &mut *results)
}

unsafe extern "C" fn IMTServerAPI_PositionCreate(this: *mut IMTServerAPI) -> *mut IMTPosition {
    (*(*this).impl_ptr).position_create()
}

unsafe extern "C" fn IMTServerAPI_PositionCreateArray(this: *mut IMTServerAPI) -> *mut IMTPositionArray {
    (*(*this).impl_ptr).position_create_array()
}

unsafe extern "C" fn IMTServerAPI_PositionSubscribe(this: *mut IMTServerAPI, sink: *mut IMTPositionSink) -> MTAPIRES {
    (*(*this).impl_ptr).position_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_PositionUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTPositionSink) -> MTAPIRES {
    (*(*this).impl_ptr).position_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_PositionDelete(this: *mut IMTServerAPI, login: UINT64, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).position_delete(login, conversion::to_utf16_str(symbol))
}

unsafe extern "C" fn IMTServerAPI_PositionUpdate(this: *mut IMTServerAPI, position: *mut IMTPosition) -> MTAPIRES {
    (*(*this).impl_ptr).position_update(&mut *position)
}

unsafe extern "C" fn IMTServerAPI_PositionGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    symbol: LPCWSTR,
    position: *mut IMTPosition,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_get(login, conversion::to_utf16_str(symbol), &mut *position)
}

unsafe extern "C" fn IMTServerAPI_PositionGet1(
    this: *mut IMTServerAPI,
    login: UINT64,
    position: *mut IMTPositionArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_get1(login, &mut *position)
}

unsafe extern "C" fn IMTServerAPI_PositionDeleteByTicket(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).position_delete_by_ticket(ticket)
}

unsafe extern "C" fn IMTServerAPI_PositionGetByTicket(
    this: *mut IMTServerAPI,
    ticket: UINT64,
    position: *mut IMTPosition,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_get_by_ticket(ticket, &mut *position)
}

unsafe extern "C" fn IMTServerAPI_PositionCheck(
    this: *mut IMTServerAPI,
    login: UINT64,
    current: *mut IMTPositionArray,
    invalid: *mut IMTPositionArray,
    missed: *mut IMTPositionArray,
    nonexist: *mut IMTPositionArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_check(login, &mut *current, &mut *invalid, &mut *missed, &mut *nonexist)
}

unsafe extern "C" fn IMTServerAPI_PositionFix(
    this: *mut IMTServerAPI,
    login: UINT64,
    current: *mut IMTPositionArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_fix(login, &mut *current)
}

unsafe extern "C" fn IMTServerAPI_OrderCreate(this: *mut IMTServerAPI) -> *mut IMTOrder {
    (*(*this).impl_ptr).order_create()
}

unsafe extern "C" fn IMTServerAPI_OrderCreateArray(this: *mut IMTServerAPI) -> *mut IMTOrderArray {
    (*(*this).impl_ptr).order_create_array()
}

unsafe extern "C" fn IMTServerAPI_OrderSubscribe(this: *mut IMTServerAPI, sink: *mut IMTOrderSink) -> MTAPIRES {
    (*(*this).impl_ptr).order_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_OrderUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTOrderSink) -> MTAPIRES {
    (*(*this).impl_ptr).order_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_OrderDelete(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).order_delete(ticket)
}

unsafe extern "C" fn IMTServerAPI_OrderUpdate(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).order_update(&mut *order)
}

unsafe extern "C" fn IMTServerAPI_OrderGet(
    this: *mut IMTServerAPI,
    ticket: UINT64,
    order: *mut IMTOrder,
) -> MTAPIRES {
    (*(*this).impl_ptr).order_get(ticket, &mut *order)
}

unsafe extern "C" fn IMTServerAPI_OrderGet1(
    this: *mut IMTServerAPI,
    login: UINT64,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).order_get1(login, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_OrderAdd(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).order_add(&mut *order)
}

unsafe extern "C" fn IMTServerAPI_OrderDeleteBatch(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, tickets_total as usize) };
    (*(*this).impl_ptr).order_delete_batch(&tickets_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_OrderUpdateBatch(
    this: *mut IMTServerAPI,
    orders: *mut IMTOrderArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let count = ((*(*orders).vtable_).IMTOrderArray_Total)(orders);
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, count as usize) };
    (*(*this).impl_ptr).order_update_batch(&mut *orders, results_slice)
}

unsafe extern "C" fn IMTServerAPI_OrderUpdateBatchArray(
    this: *mut IMTServerAPI,
    orders: *mut *mut IMTOrder,
    orders_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let orders_slice = unsafe { std::slice::from_raw_parts_mut(orders as *mut &mut IMTOrder, orders_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, orders_total as usize) };
    (*(*this).impl_ptr).order_update_batch_array(orders_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_HistorySubscribe(this: *mut IMTServerAPI, sink: *mut IMTHistorySink) -> MTAPIRES {
    (*(*this).impl_ptr).history_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_HistoryUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTHistorySink) -> MTAPIRES {
    (*(*this).impl_ptr).history_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_HistoryDelete(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).history_delete(ticket)
}

unsafe extern "C" fn IMTServerAPI_HistoryUpdate(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).history_update(&mut *order)
}

unsafe extern "C" fn IMTServerAPI_HistoryGet(
    this: *mut IMTServerAPI,
    ticket: UINT64,
    order: *mut IMTOrder,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_get(ticket, &mut *order)
}

unsafe extern "C" fn IMTServerAPI_HistoryGet1(
    this: *mut IMTServerAPI,
    from: INT64,
    to: INT64,
    login: UINT64,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_get1(from, to, login, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_HistoryReopen(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).history_reopen(ticket)
}

unsafe extern "C" fn IMTServerAPI_HistoryAdd(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).history_add(&mut *order)
}

unsafe extern "C" fn IMTServerAPI_HistoryDeleteBatch(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, tickets_total as usize) };
    (*(*this).impl_ptr).history_delete_batch(tickets_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_HistoryUpdateBatch(
    this: *mut IMTServerAPI,
    orders: *mut IMTOrderArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let count = ((*(*orders).vtable_).IMTOrderArray_Total)(orders);
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, count as usize) };
    (*(*this).impl_ptr).history_update_batch(&mut *orders, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DailyCreate(this: *mut IMTServerAPI) -> *mut IMTDaily {
    (*(*this).impl_ptr).daily_create()
}

unsafe extern "C" fn IMTServerAPI_DailyCreateArray(this: *mut IMTServerAPI) -> *mut IMTDailyArray {
    (*(*this).impl_ptr).daily_create_array()
}

unsafe extern "C" fn IMTServerAPI_DailySubscribe(this: *mut IMTServerAPI, sink: *mut IMTDailySink) -> MTAPIRES {
    (*(*this).impl_ptr).daily_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DailyUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTDailySink) -> MTAPIRES {
    (*(*this).impl_ptr).daily_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DailyGet(
    this: *mut IMTServerAPI,
    from: INT64,
    to: INT64,
    login: UINT64,
    daily: *mut IMTDailyArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).daily_get(from, to, login, &mut *daily)
}

unsafe extern "C" fn IMTServerAPI_DailyGetLight(
    this: *mut IMTServerAPI,
    from: INT64,
    to: INT64,
    login: UINT64,
    daily: *mut IMTDailyArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).daily_get_light(from, to, login, &mut *daily)
}

unsafe extern "C" fn IMTServerAPI_DailySelectByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    (*(*this).impl_ptr).daily_select_by_group(conversion::to_utf16_str(group), from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_DailySelectByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).daily_select_by_logins(logins_slice, from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_DailyReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).daily_reserved4()
}

unsafe extern "C" fn IMTServerAPI_TickSubscribe(this: *mut IMTServerAPI, sink: *mut IMTTickSink) -> MTAPIRES {
    (*(*this).impl_ptr).tick_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TickUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTTickSink) -> MTAPIRES {
    (*(*this).impl_ptr).tick_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TickAdd(this: *mut IMTServerAPI, tick: *mut MTTick) -> MTAPIRES {
    (*(*this).impl_ptr).tick_add(&mut *tick)
}

unsafe extern "C" fn IMTServerAPI_TickAddStat(
    this: *mut IMTServerAPI,
    tick: *mut MTTick,
    stat: *mut MTTickStat,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_add_stat(&mut *tick, &mut *stat)
}

unsafe extern "C" fn IMTServerAPI_TickLast(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    tick: *mut MTTickShort,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_last(conversion::to_utf16_str(symbol), &mut *tick)
}

unsafe extern "C" fn IMTServerAPI_TickLast1(
    this: *mut IMTServerAPI,
    symbol: *const IMTConSymbol,
    tick: *mut MTTickShort,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_last1(&*symbol, &mut *tick)
}

unsafe extern "C" fn IMTServerAPI_TickStat(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    stat: *mut MTTickStat,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_stat(conversion::to_utf16_str(symbol), &mut *stat)
}

unsafe extern "C" fn IMTServerAPI_TickGet(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    from: INT64,
    to: INT64,
    ticks: *mut *mut MTTickShort,
    ticks_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_get(conversion::to_utf16_str(symbol), from, to, &mut (&mut **ticks), &mut *ticks_total)
}

unsafe extern "C" fn IMTServerAPI_TickGet1(
    this: *mut IMTServerAPI,
    symbol: *const IMTConSymbol,
    from: INT64,
    to: INT64,
    ticks: *mut *mut MTTickShort,
    ticks_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_get1(&*symbol, from, to, &mut (&mut **ticks), &mut *ticks_total)
}

unsafe extern "C" fn IMTServerAPI_TickHistoryGetRaw(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    from: INT64,
    to: INT64,
    ticks: *mut *mut MTTickShort,
    ticks_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_history_get_raw(conversion::to_utf16_str(symbol), from, to, &mut (&mut **ticks), &mut *ticks_total)
}

unsafe extern "C" fn IMTServerAPI_TickHistoryGet(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    from: INT64,
    to: INT64,
    ticks: *mut *mut MTTickShort,
    ticks_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).tick_history_get(conversion::to_utf16_str(symbol), from, to, &mut (&mut **ticks), &mut *ticks_total)
}

unsafe extern "C" fn IMTServerAPI_TickAddBatch(
    this: *mut IMTServerAPI,
    ticks: *mut MTTick,
    ticks_total: UINT,
) -> MTAPIRES {
    let ticks_slice = unsafe { std::slice::from_raw_parts_mut(ticks, ticks_total as usize) };
    (*(*this).impl_ptr).tick_add_batch(ticks_slice)
}

unsafe extern "C" fn IMTServerAPI_TickReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).tick_reserved3()
}

unsafe extern "C" fn IMTServerAPI_TickReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).tick_reserved4()
}

unsafe extern "C" fn IMTServerAPI_MailCreate(this: *mut IMTServerAPI) -> *mut IMTMail {
    (*(*this).impl_ptr).mail_create()
}

unsafe extern "C" fn IMTServerAPI_MailSubscribe(this: *mut IMTServerAPI, sink: *mut IMTMailSink) -> MTAPIRES {
    (*(*this).impl_ptr).mail_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_MailUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTMailSink) -> MTAPIRES {
    (*(*this).impl_ptr).mail_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_MailSend(this: *mut IMTServerAPI, mail: *mut IMTMail) -> MTAPIRES {
    (*(*this).impl_ptr).mail_send(&mut *mail)
}

unsafe extern "C" fn IMTServerAPI_MailReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).mail_reserved1()
}

unsafe extern "C" fn IMTServerAPI_MailReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).mail_reserved2()
}

unsafe extern "C" fn IMTServerAPI_MailReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).mail_reserved3()
}

unsafe extern "C" fn IMTServerAPI_MailReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).mail_reserved4()
}

unsafe extern "C" fn IMTServerAPI_NewsCreate(this: *mut IMTServerAPI) -> *mut IMTNews {
    (*(*this).impl_ptr).news_create()
}

unsafe extern "C" fn IMTServerAPI_NewsSubscribe(this: *mut IMTServerAPI, sink: *mut IMTNewsSink) -> MTAPIRES {
    (*(*this).impl_ptr).news_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_NewsUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTNewsSink) -> MTAPIRES {
    (*(*this).impl_ptr).news_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_NewsSend(this: *mut IMTServerAPI, news: *mut IMTNews) -> MTAPIRES {
    (*(*this).impl_ptr).news_send(&mut *news)
}

unsafe extern "C" fn IMTServerAPI_NewsReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).news_reserved1()
}

unsafe extern "C" fn IMTServerAPI_NewsReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).news_reserved2()
}

unsafe extern "C" fn IMTServerAPI_NewsReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).news_reserved3()
}

unsafe extern "C" fn IMTServerAPI_NewsReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).news_reserved4()
}

unsafe extern "C" fn IMTServerAPI_CustomSubscribe(this: *mut IMTServerAPI, sink: *mut IMTCustomSink) -> MTAPIRES {
    (*(*this).impl_ptr).custom_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_CustomUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTCustomSink) -> MTAPIRES {
    (*(*this).impl_ptr).custom_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_CustomCreateStream(this: *mut IMTServerAPI) -> *mut IMTByteStream {
    (*(*this).impl_ptr).custom_create_stream()
}

unsafe extern "C" fn IMTServerAPI_CustomReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).custom_reserved2()
}

unsafe extern "C" fn IMTServerAPI_CustomReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).custom_reserved3()
}

unsafe extern "C" fn IMTServerAPI_CustomReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).custom_reserved4()
}

unsafe extern "C" fn IMTServerAPI_TradeRequestCreate(this: *mut IMTServerAPI) -> *mut IMTRequest {
    (*(*this).impl_ptr).trade_request_create()
}

unsafe extern "C" fn IMTServerAPI_TradeSubscribe(this: *mut IMTServerAPI, sink: *mut IMTTradeSink) -> MTAPIRES {
    (*(*this).impl_ptr).trade_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TradeUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTTradeSink) -> MTAPIRES {
    (*(*this).impl_ptr).trade_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TradeRequest(this: *mut IMTServerAPI, request: *mut IMTRequest) -> MTAPIRES {
    (*(*this).impl_ptr).trade_request(&mut *request)
}

unsafe extern "C" fn IMTServerAPI_TradeProfit(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    symbol: LPCWSTR,
    type_: UINT,
    volume: UINT64,
    price_open: f64,
    price_close: f64,
    profit: *mut f64,
    profit_rate: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_profit(conversion::to_utf16_str(group), conversion::to_utf16_str(symbol), type_, volume, price_open, price_close, &mut *profit, &mut *profit_rate)
}

unsafe extern "C" fn IMTServerAPI_TradeRateBuy(
    this: *mut IMTServerAPI,
    base: LPCWSTR,
    currency: LPCWSTR,
    rate: *mut f64,
    group: LPCWSTR,
    symbol: LPCWSTR,
    price: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_rate_buy(
        conversion::to_utf16_str(base),
        conversion::to_utf16_str(currency),
        &mut *rate,
        conversion::to_utf16_str(group),
        conversion::to_utf16_str(symbol),
        price)
}

unsafe extern "C" fn IMTServerAPI_TradeRateSell(
    this: *mut IMTServerAPI,
    base: LPCWSTR,
    currency: LPCWSTR,
    rate: *mut f64,
    group: LPCWSTR,
    symbol: LPCWSTR,
    price: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_rate_sell(
        conversion::to_utf16_str(base),
        conversion::to_utf16_str(currency),
        &mut *rate,
        conversion::to_utf16_str(group),
        conversion::to_utf16_str(symbol),
        price)
}

unsafe extern "C" fn IMTServerAPI_TradeMarginCheck(
    this: *mut IMTServerAPI,
    login: UINT64,
    symbol: LPCWSTR,
    type_: UINT,
    volume: UINT64,
    price: f64,
    account_new: *mut IMTAccount,
    account_current: *mut IMTAccount,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_margin_check(login, conversion::to_utf16_str(symbol), type_, volume, price, &mut *account_new, &mut *account_current)
}

unsafe extern "C" fn IMTServerAPI_TradeMarginCheck1(
    this: *mut IMTServerAPI,
    order: *const IMTOrder,
    account_new: *mut IMTAccount,
    account_current: *mut IMTAccount,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_margin_check1(&*order, &mut *account_new, &mut *account_current)
}

unsafe extern "C" fn IMTServerAPI_TradeBalanceCheckObsolete(
    this: *mut IMTServerAPI,
    login: UINT64,
    fixflag: UINT,
    balance_user: *mut f64,
    balance_history: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_balance_check_obsolete(login, fixflag, &mut *balance_user, &mut *balance_history)
}

unsafe extern "C" fn IMTServerAPI_TradeSubscribeEOD(this: *mut IMTServerAPI, sink: *mut IMTEndOfDaySink) -> MTAPIRES {
    (*(*this).impl_ptr).trade_subscribe_eod(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TradeUnsubscribeEOD(this: *mut IMTServerAPI, sink: *mut IMTEndOfDaySink) -> MTAPIRES {
    (*(*this).impl_ptr).trade_unsubscribe_eod(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_TradeBalanceCheck(
    this: *mut IMTServerAPI,
    login: UINT64,
    fixflag: UINT,
    balance_user: *mut f64,
    balance_history: *mut f64,
    credit_user: *mut f64,
    credit_history: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_balance_check(
        login, fixflag, &mut *balance_user, &mut *balance_history, &mut *credit_user, &mut *credit_history)
}

unsafe extern "C" fn IMTServerAPI_TradeAccountSet(
    this: *mut IMTServerAPI,
    user: *const IMTUser,
    account: *const IMTAccount,
    orders: *const IMTOrderArray,
    positions: *const IMTPositionArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_account_set(&*user, &*account, &*orders, &*positions)
}

unsafe extern "C" fn IMTServerAPI_TradeConfirmCreate(this: *mut IMTServerAPI) -> *mut IMTConfirm {
    (*(*this).impl_ptr).trade_confirm_create()
}

unsafe extern "C" fn IMTServerAPI_TradeExecutionCreate(this: *mut IMTServerAPI) -> *mut IMTExecution {
    (*(*this).impl_ptr).trade_execution_create()
}

unsafe extern "C" fn IMTServerAPI_TradeRequestCreateArray(this: *mut IMTServerAPI) -> *mut IMTRequestArray {
    (*(*this).impl_ptr).trade_request_create_array()
}

unsafe extern "C" fn IMTServerAPI_TradeProfitExt(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    symbol: LPCWSTR,
    type_: UINT,
    volume: UINT64,
    price_open: f64,
    price_close: f64,
    profit: *mut f64,
    profit_rate: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_profit_ext(
        conversion::to_utf16_str(group), conversion::to_utf16_str(symbol), type_, volume, price_open, price_close, &mut *profit, &mut *profit_rate)
}

unsafe extern "C" fn IMTServerAPI_BookSubscribe(this: *mut IMTServerAPI, sink: *mut IMTBookSink) -> MTAPIRES {
    (*(*this).impl_ptr).book_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_BookUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTBookSink) -> MTAPIRES {
    (*(*this).impl_ptr).book_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_BookGet(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    book: *mut MTBook,
) -> MTAPIRES {
    (*(*this).impl_ptr).book_get(conversion::to_utf16_str(symbol), &mut *book)
}

unsafe extern "C" fn IMTServerAPI_BookReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).book_reserved1()
}

unsafe extern "C" fn IMTServerAPI_BookReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).book_reserved2()
}

unsafe extern "C" fn IMTServerAPI_BookReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).book_reserved3()
}

unsafe extern "C" fn IMTServerAPI_BookReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).book_reserved4()
}

unsafe extern "C" fn IMTServerAPI_ChartGet(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    from: INT64,
    to: INT64,
    bars: *mut *mut MTChartBar,
    bars_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).chart_get(conversion::to_utf16_str(symbol), from, to, &mut (&mut **bars), &mut *bars_total)
}

unsafe extern "C" fn IMTServerAPI_ChartDelete(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    bars_dates: *const INT64,
    bars_dates_total: UINT,
) -> MTAPIRES {
    let bars_slice = unsafe { std::slice::from_raw_parts(bars_dates, bars_dates_total as usize) };
    (*(*this).impl_ptr).chart_delete(conversion::to_utf16_str(symbol), bars_slice)
}

unsafe extern "C" fn IMTServerAPI_ChartUpdate(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    bars: *const MTChartBar,
    bars_total: UINT,
) -> MTAPIRES {
    let bars_slice = unsafe { std::slice::from_raw_parts(bars, bars_total as usize) };
    (*(*this).impl_ptr).chart_update(conversion::to_utf16_str(symbol), bars_slice)
}

unsafe extern "C" fn IMTServerAPI_ChartSplit(
    this: *mut IMTServerAPI,
    symbol: LPCWSTR,
    new_shares: UINT,
    old_shares: UINT,
    rounding_rule: UINT,
    datetime_from: INT64,
    datetime_to: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).chart_split(conversion::to_utf16_str(symbol), new_shares, old_shares, rounding_rule, datetime_from, datetime_to)
}

unsafe extern "C" fn IMTServerAPI_ChartReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).chart_reserved2()
}
unsafe extern "C" fn IMTServerAPI_ChartReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).chart_reserved3()
}
unsafe extern "C" fn IMTServerAPI_ChartReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).chart_reserved4()
}

unsafe extern "C" fn IMTServerAPI_UserCertCreate(this: *mut IMTServerAPI) -> *mut IMTCertificate {
    (*(*this).impl_ptr).user_cert_create()
}

unsafe extern "C" fn IMTServerAPI_UserCertUpdate(
    this: *mut IMTServerAPI,
    login: UINT64,
    certificate: *const IMTCertificate,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_update(login, &*certificate)
}

unsafe extern "C" fn IMTServerAPI_UserCertGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    certificate: *mut IMTCertificate,
) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_get(login, &mut *certificate)
}

unsafe extern "C" fn IMTServerAPI_UserCertReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_reserved1()
}
unsafe extern "C" fn IMTServerAPI_UserCertReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_reserved2()
}
unsafe extern "C" fn IMTServerAPI_UserCertReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_reserved3()
}
unsafe extern "C" fn IMTServerAPI_UserCertReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).user_cert_reserved4()
}

unsafe extern "C" fn IMTServerAPI_SpreadCreate(this: *mut IMTServerAPI) -> *mut IMTConSpread {
    (*(*this).impl_ptr).spread_create()
}

unsafe extern "C" fn IMTServerAPI_SpreadLegCreate(this: *mut IMTServerAPI) -> *mut IMTConSpreadLeg {
    (*(*this).impl_ptr).spread_leg_create()
}

unsafe extern "C" fn IMTServerAPI_SpreadSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConSpreadSink) -> MTAPIRES {
    (*(*this).impl_ptr).spread_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SpreadUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConSpreadSink) -> MTAPIRES {
    (*(*this).impl_ptr).spread_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SpreadAdd(this: *mut IMTServerAPI, spread: *mut IMTConSpread) -> MTAPIRES {
    (*(*this).impl_ptr).spread_add(&mut *spread)
}

unsafe extern "C" fn IMTServerAPI_SpreadDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).spread_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_SpreadShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).spread_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_SpreadTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).spread_total()
}

unsafe extern "C" fn IMTServerAPI_SpreadNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    spread: *mut IMTConSpread,
) -> MTAPIRES {
    (*(*this).impl_ptr).spread_next(pos, &mut *spread)
}

unsafe extern "C" fn IMTServerAPI_SpreadGet(
    this: *mut IMTServerAPI,
    id: UINT,
    spread: *mut IMTConSpread,
) -> MTAPIRES {
    (*(*this).impl_ptr).spread_get(id, &mut *spread)
}

unsafe extern "C" fn IMTServerAPI_SpreadReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).spread_reserved1()
}
unsafe extern "C" fn IMTServerAPI_SpreadReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).spread_reserved2()
}
unsafe extern "C" fn IMTServerAPI_SpreadReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).spread_reserved3()
}
unsafe extern "C" fn IMTServerAPI_SpreadReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).spread_reserved4()
}
unsafe extern "C" fn IMTServerAPI_OnlineCreate(this: *mut IMTServerAPI) -> *mut IMTOnline {
    (*(*this).impl_ptr).online_create()
}

unsafe extern "C" fn IMTServerAPI_OnlineCreateArray(this: *mut IMTServerAPI) -> *mut IMTOnlineArray {
    (*(*this).impl_ptr).online_create_array()
}

unsafe extern "C" fn IMTServerAPI_OnlineTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).online_total()
}

unsafe extern "C" fn IMTServerAPI_OnlineNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    online: *mut IMTOnline,
) -> MTAPIRES {
    (*(*this).impl_ptr).online_next(pos, &mut *online)
}

unsafe extern "C" fn IMTServerAPI_OnlineGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    online: *mut IMTOnlineArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).online_get(login, &mut *online)
}

unsafe extern "C" fn IMTServerAPI_OnlineDisconnect(this: *mut IMTServerAPI, online: *mut IMTOnline) -> MTAPIRES {
    (*(*this).impl_ptr).online_disconnect(&mut *online)
}

unsafe extern "C" fn IMTServerAPI_OnlineDisconnectBatch(
    this: *mut IMTServerAPI,
    online: *mut IMTOnlineArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    (*(*this).impl_ptr).online_disconnect_batch(&mut *online, &mut *results)
}

unsafe extern "C" fn IMTServerAPI_OnlineDisconnectBatchArray(
    this: *mut IMTServerAPI,
    online: *mut *mut IMTOnline,
    online_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let online_slice = unsafe { std::slice::from_raw_parts_mut(online as *mut &mut IMTOnline, online_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, online_total as usize) };
    (*(*this).impl_ptr).online_disconnect_batch_array(online_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_OnlineReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).online_reserved4()
}

unsafe extern "C" fn IMTServerAPI_NotificationsSend(
    this: *mut IMTServerAPI,
    metaquotes_ids: LPCWSTR,
    message: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).notifications_send(conversion::to_utf16_str(metaquotes_ids), conversion::to_utf16_str(message))
}

unsafe extern "C" fn IMTServerAPI_NotificationsSend1(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    message: LPCWSTR,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).notifications_send1(logins_slice, conversion::to_utf16_str(message))
}

unsafe extern "C" fn IMTServerAPI_NotificationsReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).notifications_reserved1()
}

unsafe extern "C" fn IMTServerAPI_NotificationsReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).notifications_reserved2()
}

unsafe extern "C" fn IMTServerAPI_NotificationsReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).notifications_reserved3()
}

unsafe extern "C" fn IMTServerAPI_NotificationsReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).notifications_reserved4()
}

unsafe extern "C" fn IMTServerAPI_DealUpdateBatch(
    this: *mut IMTServerAPI,
    deals: *mut IMTDealArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let count = ((*(*deals).vtable_).IMTDealArray_Total)(deals);
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, count as usize) };
    (*(*this).impl_ptr).deal_update_batch(&mut *deals, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DealUpdateBatchArray(
    this: *mut IMTServerAPI,
    deals: *mut *mut IMTDeal,
    deals_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let deals_slice = unsafe { std::slice::from_raw_parts_mut(deals as *mut &mut IMTDeal, deals_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, deals_total as usize) };
    (*(*this).impl_ptr).deal_update_batch_array(deals_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DealAddBatch(
    this: *mut IMTServerAPI,
    deals: *mut IMTDealArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let count = ((*(*deals).vtable_).IMTDealArray_Total)(deals);
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, count as usize) };
    (*(*this).impl_ptr).deal_add_batch(&mut *deals, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DealAddBatchArray(
    this: *mut IMTServerAPI,
    deals: *mut *mut IMTDeal,
    deals_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let deals_slice = unsafe { std::slice::from_raw_parts_mut(deals as *mut &mut IMTDeal, deals_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, deals_total as usize) };
    (*(*this).impl_ptr).deal_add_batch_array(deals_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DealPerformBatch(
    this: *mut IMTServerAPI,
    deals: *mut IMTDealArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let count = ((*(*deals).vtable_).IMTDealArray_Total)(deals);
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, count as usize) };
    (*(*this).impl_ptr).deal_perform_batch(&mut *deals, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DealPerformBatchArray(
    this: *mut IMTServerAPI,
    deals: *mut *mut IMTDeal,
    deals_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let deals_slice = unsafe { std::slice::from_raw_parts_mut(deals as *mut &mut IMTDeal, deals_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, deals_total as usize) };
    (*(*this).impl_ptr).deal_perform_batch_array(deals_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_DealSelectByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    (*(*this).impl_ptr).deal_select_by_group(conversion::to_utf16_str(group), from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_DealSelectByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).deal_select_by_logins(logins_slice, from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_DealGetByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    from: INT64,
    to: INT64,
    deals: *mut IMTDealArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).deal_get_by_group(conversion::to_utf16_str(group), from, to, &mut *deals)
}

unsafe extern "C" fn IMTServerAPI_DealGetByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    from: INT64,
    to: INT64,
    deals: *mut IMTDealArray,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).deal_get_by_logins(logins_slice, from, to, &mut *deals)
}

unsafe extern "C" fn IMTServerAPI_OrderAddBatch(
    this: *mut IMTServerAPI,
    orders: *mut IMTOrderArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let count = ((*(*orders).vtable_).IMTOrderArray_Total)(orders);
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, count as usize) };
    (*(*this).impl_ptr).order_add_batch(&mut *orders, results_slice)
}

unsafe extern "C" fn IMTServerAPI_OrderAddBatchArray(
    this: *mut IMTServerAPI,
    orders: *mut *mut IMTOrder,
    orders_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let orders_slice = unsafe { std::slice::from_raw_parts_mut(orders as *mut &mut IMTOrder, orders_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, orders_total as usize) };
    (*(*this).impl_ptr).order_add_batch_array(orders_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_OrderSelectByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    (*(*this).impl_ptr).order_select_by_group(conversion::to_utf16_str(group), &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_OrderSelectByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).order_select_by_logins(logins_slice, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_OrderGetByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).order_get_by_group(conversion::to_utf16_str(group), &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_OrderGetByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).order_get_by_logins(logins_slice, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_HistoryUpdateBatchArray(
    this: *mut IMTServerAPI,
    orders: *mut *mut IMTOrder,
    orders_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let orders_slice = unsafe { std::slice::from_raw_parts_mut(orders as *mut &mut IMTOrder, orders_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, orders_total as usize) };
    (*(*this).impl_ptr).history_update_batch_array(orders_slice, results_slice)
}

unsafe extern "C" fn IMTServerAPI_HistoryAddBatch(
    this: *mut IMTServerAPI,
    orders: *mut IMTOrderArray,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_add_batch(&mut *orders, &mut *results)
}

unsafe extern "C" fn IMTServerAPI_HistoryAddBatchArray(
    this: *mut IMTServerAPI,
    orders: *mut *mut IMTOrder,
    orders_total: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_add_batch_array(&mut (&mut **orders), orders_total, &mut *results)
}

unsafe extern "C" fn IMTServerAPI_HistorySelectByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_select_by_group(conversion::to_utf16_str(group), from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_HistorySelectByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).history_select_by_logins(logins_slice, from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_HistoryGetByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    from: INT64,
    to: INT64,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).history_get_by_group(conversion::to_utf16_str(group), from, to, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_HistoryGetByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    from: INT64,
    to: INT64,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).history_get_by_logins(logins_slice, from, to, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_DealerStart(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    sink: *mut IMTRequestSink,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_start(dealer, &mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DealerStop(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    sink: *mut IMTRequestSink,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_stop(dealer, &mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DealerGet(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    request: *mut IMTRequest,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_get(dealer, &mut *request)
}

unsafe extern "C" fn IMTServerAPI_DealerLock(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    id: UINT,
    request: *mut IMTRequest,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_lock(dealer, id, &mut *request)
}

unsafe extern "C" fn IMTServerAPI_DealerAnswer(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    confirm: *mut IMTConfirm,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_answer(dealer, &mut *confirm)
}

unsafe extern "C" fn IMTServerAPI_DealerRequestTotal(this: *mut IMTServerAPI, dealer: UINT64) -> UINT {
    (*(*this).impl_ptr).dealer_request_total(dealer)
}

unsafe extern "C" fn IMTServerAPI_DealerRequestNext(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    pos: UINT,
    request: *mut IMTRequest,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_request_next(dealer, pos, &mut *request)
}

unsafe extern "C" fn IMTServerAPI_DealerRequestGet(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    id: UINT,
    request: *mut IMTRequest,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_request_get(dealer, id, &mut *request)
}

unsafe extern "C" fn IMTServerAPI_DealerRequestGetAll(
    this: *mut IMTServerAPI,
    dealer: UINT64,
    requests: *mut IMTRequestArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_request_get_all(dealer, &mut *requests)
}

unsafe extern "C" fn IMTServerAPI_DealerExecution(
    this: *mut IMTServerAPI,
    gateway_name: LPCWSTR,
    gateway_type: LPCWSTR,
    execution: *mut IMTExecution,
) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_execution(
        conversion::to_utf16_str(gateway_name),
        conversion::to_utf16_str(gateway_type),
        &mut *execution)
}

unsafe extern "C" fn IMTServerAPI_DealerReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_reserved2()
}

unsafe extern "C" fn IMTServerAPI_DealerReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_reserved3()
}

unsafe extern "C" fn IMTServerAPI_DealerReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dealer_reserved4()
}

unsafe extern "C" fn IMTServerAPI_TradeMarginCheckExt(
    this: *mut IMTServerAPI,
    login: UINT64,
    symbol: LPCWSTR,
    type_: UINT,
    volume: UINT64,
    price: f64,
    account_new: *mut IMTAccount,
    account_current: *mut IMTAccount,
) -> MTAPIRES {
    (*(*this).impl_ptr).trade_margin_check_ext(
        login,
        conversion::to_utf16_str(symbol),
        type_,
        volume,
        price,
        &mut *account_new,
        &mut *account_current)
}

unsafe extern "C" fn IMTServerAPI_TradeReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).trade_reserved1()
}

unsafe extern "C" fn IMTServerAPI_TradeReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).trade_reserved2()
}

unsafe extern "C" fn IMTServerAPI_TradeReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).trade_reserved3()
}

unsafe extern "C" fn IMTServerAPI_TradeReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).trade_reserved4()
}

unsafe extern "C" fn IMTServerAPI_TradeReserved5(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).trade_reserved5()
}

unsafe extern "C" fn IMTServerAPI_TradeReserved6(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).trade_reserved6()
}

unsafe extern "C" fn IMTServerAPI_EmailCreate(this: *mut IMTServerAPI) -> *mut IMTConEmail {
    (*(*this).impl_ptr).email_create()
}

unsafe extern "C" fn IMTServerAPI_EmailSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConEmailSink) -> MTAPIRES {
    (*(*this).impl_ptr).email_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_EmailUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConEmailSink) -> MTAPIRES {
    (*(*this).impl_ptr).email_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_EmailAdd(this: *mut IMTServerAPI, config: *mut IMTConEmail) -> MTAPIRES {
    (*(*this).impl_ptr).email_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_EmailDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).email_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_EmailDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).email_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_EmailShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).email_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_EmailTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).email_total()
}

unsafe extern "C" fn IMTServerAPI_EmailNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    email: *mut IMTConEmail,
) -> MTAPIRES {
    (*(*this).impl_ptr).email_next(pos, &mut *email)
}

unsafe extern "C" fn IMTServerAPI_EmailGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    email: *mut IMTConEmail,
) -> MTAPIRES {
    (*(*this).impl_ptr).email_get(conversion::to_utf16_str(name), &mut *email)
}

unsafe extern "C" fn IMTServerAPI_EmailSend(
    this: *mut IMTServerAPI,
    account: LPCWSTR,
    to: LPCWSTR,
    to_name: LPCWSTR,
    subject: LPCWSTR,
    body: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).email_send(
        conversion::to_utf16_str(account),
        conversion::to_utf16_str(to),
        conversion::to_utf16_str(to_name),
        conversion::to_utf16_str(subject),
        conversion::to_utf16_str(body))
}

unsafe extern "C" fn IMTServerAPI_EmailReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).email_reserved2()
}

unsafe extern "C" fn IMTServerAPI_EmailReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).email_reserved3()
}

unsafe extern "C" fn IMTServerAPI_EmailReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).email_reserved4()
}

unsafe extern "C" fn IMTServerAPI_MessengerCreate(this: *mut IMTServerAPI) -> *mut IMTConMessenger {
    (*(*this).impl_ptr).messenger_create()
}

unsafe extern "C" fn IMTServerAPI_MessengerSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConMessengerSink) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_MessengerUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConMessengerSink) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_MessengerAdd(this: *mut IMTServerAPI, config: *mut IMTConMessenger) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_MessengerDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_MessengerDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_MessengerShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_MessengerTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).messenger_total()
}

unsafe extern "C" fn IMTServerAPI_MessengerNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    messenger: *mut IMTConMessenger,
) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_next(pos, &mut *messenger)
}

unsafe extern "C" fn IMTServerAPI_MessengerGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    messenger: *mut IMTConMessenger,
) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_get(conversion::to_utf16_str(name), &mut *messenger)
}

unsafe extern "C" fn IMTServerAPI_MessengerSend(
    this: *mut IMTServerAPI,
    destination: LPCWSTR,
    group: LPCWSTR,
    sender: LPCWSTR,
    text: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_send(
        conversion::to_utf16_str(destination),
        conversion::to_utf16_str(group),
        conversion::to_utf16_str(sender),
        conversion::to_utf16_str(text))
}

unsafe extern "C" fn IMTServerAPI_MessengerVerifyPhone(this: *mut IMTServerAPI, phone_number: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).messenger_verify_phone(conversion::to_utf16_str(phone_number))
}

unsafe extern "C" fn IMTServerAPI_MessengerCountryCreate(this: *mut IMTServerAPI) -> *mut IMTConMessengerCountry {
    (*(*this).impl_ptr).messenger_country_create()
}

unsafe extern "C" fn IMTServerAPI_MessengerGroupCreate(this: *mut IMTServerAPI) -> *mut IMTConMessengerGroup {
    (*(*this).impl_ptr).messenger_group_create()
}

unsafe extern "C" fn IMTServerAPI_PositionSelectByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_select_by_group(conversion::to_utf16_str(group), from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_PositionSelectByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    from: INT64,
    to: INT64,
    request: *const IMTDatasetRequest,
    dataset: *mut IMTDataset,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).position_select_by_logins(logins_slice, from, to, &*request, &mut *dataset)
}

unsafe extern "C" fn IMTServerAPI_PositionGetByGroup(
    this: *mut IMTServerAPI,
    group: LPCWSTR,
    positions: *mut IMTPositionArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).position_get_by_group(conversion::to_utf16_str(group), &mut *positions)
}

unsafe extern "C" fn IMTServerAPI_PositionGetByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    positions: *mut IMTPositionArray,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).position_get_by_logins(logins_slice, &mut *positions)
}

unsafe extern "C" fn IMTServerAPI_PositionGetByTickets(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    positions: *mut IMTPositionArray,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    (*(*this).impl_ptr).position_get_by_tickets(tickets_slice, &mut *positions)
}

unsafe extern "C" fn IMTServerAPI_PositionSplit(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    adjustments: *const f64,
    new_shares: UINT,
    old_shares: UINT,
    round_rule_price: UINT,
    round_rule_volume: UINT,
    flags: UINT,
    results: *mut MTAPIRES,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    let adjustments_slice = unsafe { std::slice::from_raw_parts(adjustments, tickets_total as usize) };
    let results_slice = unsafe { std::slice::from_raw_parts_mut(results, tickets_total as usize) };
    (*(*this).impl_ptr).position_split(
        tickets_slice, adjustments_slice, new_shares, old_shares, round_rule_price, round_rule_volume, flags, results_slice)
}

unsafe extern "C" fn IMTServerAPI_PositionReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).position_reserved2()
}
unsafe extern "C" fn IMTServerAPI_PositionReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).position_reserved3()
}
unsafe extern "C" fn IMTServerAPI_PositionReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).position_reserved4()
}

unsafe extern "C" fn IMTServerAPI_DatasetRequestCreate(this: *mut IMTServerAPI) -> *mut IMTDatasetRequest {
    (*(*this).impl_ptr).dataset_request_create()
}

unsafe extern "C" fn IMTServerAPI_DatasetCreate(this: *mut IMTServerAPI) -> *mut IMTDataset {
    (*(*this).impl_ptr).dataset_create()
}

unsafe extern "C" fn IMTServerAPI_DatasetReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dataset_reserved1()
}

unsafe extern "C" fn IMTServerAPI_DatasetReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dataset_reserved2()
}

unsafe extern "C" fn IMTServerAPI_DatasetReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dataset_reserved3()
}

unsafe extern "C" fn IMTServerAPI_DatasetReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).dataset_reserved4()
}

unsafe extern "C" fn IMTServerAPI_OrderGetByTickets(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    (*(*this).impl_ptr).order_get_by_tickets(tickets_slice, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_OrderReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).order_reserved1()
}

unsafe extern "C" fn IMTServerAPI_OrderReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).order_reserved2()
}

unsafe extern "C" fn IMTServerAPI_OrderReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).order_reserved3()
}

unsafe extern "C" fn IMTServerAPI_OrderReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).order_reserved4()
}

unsafe extern "C" fn IMTServerAPI_HistoryGetByTickets(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    orders: *mut IMTOrderArray,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    (*(*this).impl_ptr).history_get_by_tickets(tickets_slice, &mut *orders)
}

unsafe extern "C" fn IMTServerAPI_HistoryReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_reserved1()
}

unsafe extern "C" fn IMTServerAPI_HistoryReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_reserved2()
}

unsafe extern "C" fn IMTServerAPI_HistoryReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_reserved3()
}

unsafe extern "C" fn IMTServerAPI_HistoryReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).history_reserved4()
}

unsafe extern "C" fn IMTServerAPI_DealGetByTickets(
    this: *mut IMTServerAPI,
    tickets: *const UINT64,
    tickets_total: UINT,
    deals: *mut IMTDealArray,
) -> MTAPIRES {
    let tickets_slice = unsafe { std::slice::from_raw_parts(tickets, tickets_total as usize) };
    (*(*this).impl_ptr).deal_get_by_tickets(tickets_slice, &mut *deals)
}

unsafe extern "C" fn IMTServerAPI_DealReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).deal_reserved1()
}

unsafe extern "C" fn IMTServerAPI_DealReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).deal_reserved2()
}

unsafe extern "C" fn IMTServerAPI_DealReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).deal_reserved3()
}

unsafe extern "C" fn IMTServerAPI_DealReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).deal_reserved4()
}

unsafe extern "C" fn IMTServerAPI_ClientCreate(this: *mut IMTServerAPI) -> *mut IMTClient {
    (*(*this).impl_ptr).client_create()
}

unsafe extern "C" fn IMTServerAPI_ClientCreateArray(this: *mut IMTServerAPI) -> *mut IMTClientArray {
    (*(*this).impl_ptr).client_create_array()
}

unsafe extern "C" fn IMTServerAPI_ClientSubscribe(this: *mut IMTServerAPI, sink: *mut IMTClientSink) -> MTAPIRES {
    (*(*this).impl_ptr).client_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ClientUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTClientSink) -> MTAPIRES {
    (*(*this).impl_ptr).client_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_ClientAdd(
    this: *mut IMTServerAPI,
    client: *mut IMTClient,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_add(&mut *client, author)
}

unsafe extern "C" fn IMTServerAPI_ClientUpdate(
    this: *mut IMTServerAPI,
    client: *mut IMTClient,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_update(&mut *client, author)
}

unsafe extern "C" fn IMTServerAPI_ClientDelete(
    this: *mut IMTServerAPI,
    client_id: UINT64,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_delete(client_id, author)
}

unsafe extern "C" fn IMTServerAPI_ClientGet(
    this: *mut IMTServerAPI,
    client_id: UINT64,
    client: *mut IMTClient,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_get(client_id, &mut *client)
}

unsafe extern "C" fn IMTServerAPI_ClientGetHistory(
    this: *mut IMTServerAPI,
    client_id: UINT64,
    author: UINT64,
    from: INT64,
    to: INT64,
    history: *mut IMTClientArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_get_history(client_id, author, from, to,  &mut *history)
}

unsafe extern "C" fn IMTServerAPI_ClientIdsAll(
    this: *mut IMTServerAPI,
    ids: *mut *mut UINT64,
    ids_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_ids_all(&mut (&mut **ids), &mut *ids_total)
}

unsafe extern "C" fn IMTServerAPI_ClientIdsByGroup(
    this: *mut IMTServerAPI,
    groups: LPCWSTR,
    ids: *mut *mut UINT64,
    ids_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_ids_by_group(conversion::to_utf16_str(groups), &mut (&mut **ids), &mut *ids_total)
}

unsafe extern "C" fn IMTServerAPI_ClientIdsByManager(
    this: *mut IMTServerAPI,
    manager: UINT64,
    ids: *mut *mut UINT64,
    ids_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_ids_by_manager(manager, &mut (&mut **ids), &mut *ids_total)
}

unsafe extern "C" fn IMTServerAPI_ClientUserAdd(this: *mut IMTServerAPI, client_id: UINT64, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).client_user_add(client_id, login)
}

unsafe extern "C" fn IMTServerAPI_ClientUserDelete(this: *mut IMTServerAPI, client_id: UINT64, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).client_user_delete(client_id, login)
}

unsafe extern "C" fn IMTServerAPI_ClientUserLogins(
    this: *mut IMTServerAPI,
    client_id: UINT64,
    logins: *mut *mut UINT64,
    logins_total: *mut UINT,
) -> MTAPIRES {
    (*(*this).impl_ptr).client_user_logins(client_id, &mut (&mut **logins), &mut *logins_total)
}

unsafe extern "C" fn IMTServerAPI_ClientReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).client_reserved1()
}

unsafe extern "C" fn IMTServerAPI_ClientReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).client_reserved2()
}

unsafe extern "C" fn IMTServerAPI_ClientReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).client_reserved3()
}

unsafe extern "C" fn IMTServerAPI_ClientReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).client_reserved4()
}

unsafe extern "C" fn IMTServerAPI_DocumentCreate(this: *mut IMTServerAPI) -> *mut IMTDocument {
    (*(*this).impl_ptr).document_create()
}

unsafe extern "C" fn IMTServerAPI_DocumentCreateArray(this: *mut IMTServerAPI) -> *mut IMTDocumentArray {
    (*(*this).impl_ptr).document_create_array()
}

unsafe extern "C" fn IMTServerAPI_DocumentSubscribe(this: *mut IMTServerAPI, sink: *mut IMTDocumentSink) -> MTAPIRES {
    (*(*this).impl_ptr).document_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DocumentUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTDocumentSink) -> MTAPIRES {
    (*(*this).impl_ptr).document_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_DocumentAdd(
    this: *mut IMTServerAPI,
    document: *mut IMTDocument,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).document_add(&mut *document, author)
}

unsafe extern "C" fn IMTServerAPI_DocumentUpdate(
    this: *mut IMTServerAPI,
    document: *mut IMTDocument,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).document_update(&mut *document, author)
}

unsafe extern "C" fn IMTServerAPI_DocumentDelete(
    this: *mut IMTServerAPI,
    document_id: UINT64,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).document_delete(document_id, author)
}

unsafe extern "C" fn IMTServerAPI_DocumentGet(
    this: *mut IMTServerAPI,
    document_id: UINT64,
    document: *mut IMTDocument,
) -> MTAPIRES {
    (*(*this).impl_ptr).document_get(document_id, &mut *document)
}

unsafe extern "C" fn IMTServerAPI_DocumentGetByClient(
    this: *mut IMTServerAPI,
    client_id: UINT64,
    position: UINT,
    total: UINT,
    documents: *mut IMTDocumentArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).document_get_by_client(client_id, position, total, &mut *documents)
}

unsafe extern "C" fn IMTServerAPI_DocumentGetHistory(
    this: *mut IMTServerAPI,
    document_id: UINT64,
    author: UINT64,
    from: INT64,
    to: INT64,
    history: *mut IMTDocumentArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).document_get_history(document_id, author, from, to, &mut *history)
}

unsafe extern "C" fn IMTServerAPI_DocumentReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).document_reserved1()
}

unsafe extern "C" fn IMTServerAPI_DocumentReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).document_reserved2()
}

unsafe extern "C" fn IMTServerAPI_DocumentReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).document_reserved3()
}

unsafe extern "C" fn IMTServerAPI_DocumentReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).document_reserved4()
}

unsafe extern "C" fn IMTServerAPI_CommentCreate(this: *mut IMTServerAPI) -> *mut IMTComment {
    (*(*this).impl_ptr).comment_create()
}

unsafe extern "C" fn IMTServerAPI_CommentCreateArray(this: *mut IMTServerAPI) -> *mut IMTCommentArray {
    (*(*this).impl_ptr).comment_create_array()
}

unsafe extern "C" fn IMTServerAPI_CommentSubscribe(this: *mut IMTServerAPI, sink: *mut IMTCommentSink) -> MTAPIRES {
    (*(*this).impl_ptr).comment_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_CommentUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTCommentSink) -> MTAPIRES {
    (*(*this).impl_ptr).comment_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_CommentAdd(
    this: *mut IMTServerAPI,
    comment: *mut IMTComment,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).comment_add(&mut *comment, author)
}

unsafe extern "C" fn IMTServerAPI_CommentUpdate(
    this: *mut IMTServerAPI,
    comment: *mut IMTComment,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).comment_update(&mut *comment, author)
}

unsafe extern "C" fn IMTServerAPI_CommentDelete(
    this: *mut IMTServerAPI,
    comment_id: UINT64,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).comment_delete(comment_id, author)
}

unsafe extern "C" fn IMTServerAPI_CommentGet(
    this: *mut IMTServerAPI,
    comment_id: UINT64,
    comment: *mut IMTComment,
) -> MTAPIRES {
    (*(*this).impl_ptr).comment_get(comment_id, &mut *comment)
}

unsafe extern "C" fn IMTServerAPI_CommentGetByClient(
    this: *mut IMTServerAPI,
    client_id: UINT64,
    position: UINT,
    total: UINT,
    comments: *mut IMTCommentArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).comment_get_by_client(client_id, position, total, &mut *comments)
}

unsafe extern "C" fn IMTServerAPI_CommentGetByDocument(
    this: *mut IMTServerAPI,
    document_id: UINT64,
    position: UINT,
    total: UINT,
    comments: *mut IMTCommentArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).comment_get_by_document(document_id, position, total, &mut *comments)
}

unsafe extern "C" fn IMTServerAPI_CommentReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).comment_reserved1()
}

unsafe extern "C" fn IMTServerAPI_CommentReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).comment_reserved2()
}

unsafe extern "C" fn IMTServerAPI_CommentReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).comment_reserved3()
}

unsafe extern "C" fn IMTServerAPI_CommentReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).comment_reserved4()
}

unsafe extern "C" fn IMTServerAPI_AttachmentCreate(this: *mut IMTServerAPI) -> *mut IMTAttachment {
    (*(*this).impl_ptr).attachment_create()
}

unsafe extern "C" fn IMTServerAPI_AttachmentAdd(
    this: *mut IMTServerAPI,
    attachment: *mut IMTAttachment,
    author: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).attachment_add(&mut *attachment, author)
}

unsafe extern "C" fn IMTServerAPI_AttachmentGet(
    this: *mut IMTServerAPI,
    attachment_id: UINT64,
    attachment: *mut IMTAttachment,
) -> MTAPIRES {
    (*(*this).impl_ptr).attachment_get(attachment_id, &mut *attachment)
}

unsafe extern "C" fn IMTServerAPI_AttachmentReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).attachment_reserved1()
}

unsafe extern "C" fn IMTServerAPI_AttachmentReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).attachment_reserved2()
}

unsafe extern "C" fn IMTServerAPI_AttachmentReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).attachment_reserved3()
}

unsafe extern "C" fn IMTServerAPI_AttachmentReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).attachment_reserved4()
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateUpdate(
    this: *mut IMTServerAPI,
    pfx_certificate: *const std::os::raw::c_void,
    pfx_certificate_size: UINT,
    password: LPCWSTR,
) -> MTAPIRES {
    let pfx_certificate_slice = unsafe { std::slice::from_raw_parts(pfx_certificate, pfx_certificate_size as usize) };
    (*(*this).impl_ptr).tls_certificate_update(pfx_certificate_slice, conversion::to_utf16_str(password))
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateDelete(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_delete(pos)
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).tls_certificate_total()
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    name: *mut MTAPISTR,
    thumbprint: *mut MTAPISTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_next(pos, &mut *name, &mut *thumbprint)
}

unsafe extern "C" fn IMTServerAPI_TLSCertificatePfx(
    this: *mut IMTServerAPI,
    pos: UINT,
    pfx_certificate_size: *mut UINT,
) -> *mut std::os::raw::c_void {
    (*(*this).impl_ptr).tls_certificate_pfx(pos, &mut *pfx_certificate_size)
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_reserved1()
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_reserved2()
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_reserved3()
}

unsafe extern "C" fn IMTServerAPI_TLSCertificateReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).tls_certificate_reserved4()
}

unsafe extern "C" fn IMTServerAPI_AutomationCreate(this: *mut IMTServerAPI) -> *mut IMTConAutomation {
    (*(*this).impl_ptr).automation_create()
}

unsafe extern "C" fn IMTServerAPI_AutomationConditionCreate(this: *mut IMTServerAPI) -> *mut IMTConAutoCondition {
    (*(*this).impl_ptr).automation_condition_create()
}

unsafe extern "C" fn IMTServerAPI_AutomationActionCreate(this: *mut IMTServerAPI) -> *mut IMTConAutoAction {
    (*(*this).impl_ptr).automation_action_create()
}

unsafe extern "C" fn IMTServerAPI_AutomationParamCreate(this: *mut IMTServerAPI) -> *mut IMTConAutoParam {
    (*(*this).impl_ptr).automation_param_create()
}

unsafe extern "C" fn IMTServerAPI_AutomationSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConAutomationSink) -> MTAPIRES {
    (*(*this).impl_ptr).automation_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_AutomationUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConAutomationSink) -> MTAPIRES {
    (*(*this).impl_ptr).automation_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_AutomationAdd(this: *mut IMTServerAPI, config: *mut IMTConAutomation) -> MTAPIRES {
    (*(*this).impl_ptr).automation_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_AutomationDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).automation_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_AutomationDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).automation_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_AutomationShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).automation_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_AutomationTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).automation_total()
}

unsafe extern "C" fn IMTServerAPI_AutomationNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    config: *mut IMTConAutomation,
) -> MTAPIRES {
    (*(*this).impl_ptr).automation_next(pos, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_AutomationGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    config: *mut IMTConAutomation,
) -> MTAPIRES {
    (*(*this).impl_ptr).automation_get(conversion::to_utf16_str(name), &mut *config)
}

unsafe extern "C" fn IMTServerAPI_AutomationTrigger(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    user: *const IMTUser,
    account: *const IMTAccount,
    deal: *const IMTDeal,
    order: *const IMTOrder,
    position: *const IMTPosition,
) -> MTAPIRES {
    (*(*this).impl_ptr).automation_trigger(
        conversion::to_utf16_str(name),
        &*user,
        &*account,
        &*deal,
        &*order,
        &*position)
}

unsafe extern "C" fn IMTServerAPI_AutomationReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).automation_reserved2()
}

unsafe extern "C" fn IMTServerAPI_AutomationReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).automation_reserved3()
}

unsafe extern "C" fn IMTServerAPI_AutomationReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).automation_reserved4()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgCreate(this: *mut IMTServerAPI) -> *mut IMTConSubscription {
    (*(*this).impl_ptr).subscription_cfg_create()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgSymbolCreate(this: *mut IMTServerAPI) -> *mut IMTConSubscriptionSymbol {
    (*(*this).impl_ptr).subscription_cfg_symbol_create()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgNewsCreate(this: *mut IMTServerAPI) -> *mut IMTConSubscriptionNews {
    (*(*this).impl_ptr).subscription_cfg_news_create()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgSubscribe(
    this: *mut IMTServerAPI,
    sink: *mut IMTConSubscriptionSink,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgUnsubscribe(
    this: *mut IMTServerAPI,
    sink: *mut IMTConSubscriptionSink,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgAdd(this: *mut IMTServerAPI, config: *mut IMTConSubscription) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgDeleteByID(this: *mut IMTServerAPI, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_delete_by_id(id)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).subscription_cfg_total()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgNext(
    this: *mut IMTServerAPI,
    pos: UINT,
    config: *mut IMTConSubscription,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_next(pos, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    config: *mut IMTConSubscription,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_get(conversion::to_utf16_str(name), &mut *config)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgGetByID(
    this: *mut IMTServerAPI,
    id: UINT64,
    config: *mut IMTConSubscription,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_get_by_id(id, &mut *config)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_reserved1()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_reserved2()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_reserved3()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCfgReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cfg_reserved4()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCreate(this: *mut IMTServerAPI) -> *mut IMTSubscription {
    (*(*this).impl_ptr).subscription_create()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCreateArray(this: *mut IMTServerAPI) -> *mut IMTSubscriptionArray {
    (*(*this).impl_ptr).subscription_create_array()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionSubscribe(this: *mut IMTServerAPI, sink: *mut IMTSubscriptionSink) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTSubscriptionSink) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionJoin(
    this: *mut IMTServerAPI,
    manager: UINT64,
    login: UINT64,
    subscription: UINT64,
    record: *mut IMTSubscription,
    history: *mut IMTSubscriptionHistory,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_join(manager, login, subscription, &mut *record, &mut *history)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionCancel(
    this: *mut IMTServerAPI,
    manager: UINT64,
    login: UINT64,
    subscription: UINT64,
    record: *mut IMTSubscription,
    history: *mut IMTSubscriptionHistory,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_cancel(manager, login, subscription, &mut *record, &mut *history)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionExist(this: *mut IMTServerAPI, login: UINT64, subscription: UINT64) -> bool {
    (*(*this).impl_ptr).subscription_exist(login, subscription)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionAdd(this: *mut IMTServerAPI, record: *mut IMTSubscription) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_add(&mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionUpdate(this: *mut IMTServerAPI, record: *mut IMTSubscription) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_update(&mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionDelete(this: *mut IMTServerAPI, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_delete(id)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionGet(
    this: *mut IMTServerAPI,
    login: UINT64,
    records: *mut IMTSubscriptionArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_get(login, &mut *records)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionGetBySubscription(
    this: *mut IMTServerAPI,
    login: UINT64,
    subscription: UINT64,
    record: *mut IMTSubscription,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_get_by_subscription(login, subscription, &mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionGetByID(
    this: *mut IMTServerAPI,
    id: UINT64,
    record: *mut IMTSubscription,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_get_by_id(id, &mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionGetByLogins(
    this: *mut IMTServerAPI,
    logins: *const UINT64,
    logins_total: UINT,
    records: *mut IMTSubscriptionArray,
) -> MTAPIRES {
    let logins_slice = unsafe { std::slice::from_raw_parts(logins, logins_total as usize) };
    (*(*this).impl_ptr).subscription_get_by_logins(logins_slice, &mut *records)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_reserved1()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_reserved2()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_reserved3()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_reserved4()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryCreate(this: *mut IMTServerAPI) -> *mut IMTSubscriptionHistory {
    (*(*this).impl_ptr).subscription_history_create()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryCreateArray(this: *mut IMTServerAPI) -> *mut IMTSubscriptionHistoryArray {
    (*(*this).impl_ptr).subscription_history_create_array()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistorySubscribe(
    this: *mut IMTServerAPI,
    sink: *mut IMTSubscriptionHistorySink,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryUnsubscribe(
    this: *mut IMTServerAPI,
    sink: *mut IMTSubscriptionHistorySink,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryAdd(
    this: *mut IMTServerAPI,
    record: *mut IMTSubscriptionHistory,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_add(&mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryUpdate(
    this: *mut IMTServerAPI,
    record: *mut IMTSubscriptionHistory,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_update(&mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryDelete(this: *mut IMTServerAPI, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_delete(id)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryGet(
    this: *mut IMTServerAPI,
    from: INT64,
    to: INT64,
    login: UINT64,
    records: *mut IMTSubscriptionHistoryArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_get(from, to, login, &mut *records)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryGetByID(
    this: *mut IMTServerAPI,
    id: UINT64,
    record: *mut IMTSubscriptionHistory,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_get_by_id(id, &mut *record)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryGetByLogins(
    this: *mut IMTServerAPI,
    from: INT64,
    to: INT64,
    logins: *const UINT64,
    logins_total: UINT,
    records: *mut IMTSubscriptionHistoryArray,
) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_get_by_logins(from, to, logins, logins_total, &mut *records)
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_reserved1()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_reserved2()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_reserved3()
}

unsafe extern "C" fn IMTServerAPI_SubscriptionHistoryReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).subscription_history_reserved4()
}

unsafe extern "C" fn IMTServerAPI_GeoResolveAny(
    this: *mut IMTServerAPI,
    ip_list: LPCWSTR,
    result: LPWSTR,
    result_len: UINT,
    flags: UINT,
) -> MTAPIRES {
    let result_wstr = unsafe { std::slice::from_raw_parts_mut(result, result_len as usize) };
    (*(*this).impl_ptr).geo_resolve_any(conversion::to_utf16_str(ip_list), result_wstr, flags)
}

unsafe extern "C" fn IMTServerAPI_GeoResolveIPv4(
    this: *mut IMTServerAPI,
    ip: ULONG,
    result: LPWSTR,
    result_len: UINT,
    flags: UINT,
) -> MTAPIRES {
    let wstr = unsafe { std::slice::from_raw_parts_mut(result, result_len as usize) };
    (*(*this).impl_ptr).geo_resolve_ipv4(ip, wstr, flags)
}

unsafe extern "C" fn IMTServerAPI_GeoResolveIPv4Bulk(
    this: *mut IMTServerAPI,
    ip_list: *const ULONG,
    ip_list_len: UINT,
    result: LPWSTR,
    result_len: UINT,
    flags: UINT,
) -> MTAPIRES {
    let ip_list_slice = unsafe { std::slice::from_raw_parts(ip_list, ip_list_len as usize) };
    let result_wstr = unsafe { std::slice::from_raw_parts_mut(result, result_len as usize) };
    (*(*this).impl_ptr).geo_resolve_ipv4_bulk(ip_list_slice, result_wstr, flags)
}

unsafe extern "C" fn IMTServerAPI_GeoResolveIPv6(
    this: *mut IMTServerAPI,
    ip: *const IN6_ADDR,
    result: LPWSTR,
    result_len: UINT,
    flags: UINT,
) -> MTAPIRES {
    let wstr = unsafe { std::slice::from_raw_parts_mut(result, result_len as usize) };
    (*(*this).impl_ptr).geo_resolve_ipv6(&*ip, wstr, flags)
}

unsafe extern "C" fn IMTServerAPI_GeoResolveIPv6Bulk(
    this: *mut IMTServerAPI,
    ip_list: *const IN6_ADDR,
    ip_list_len: UINT,
    result: LPWSTR,
    result_len: UINT,
    flags: UINT,
) -> MTAPIRES {
    let ip_list_slice = unsafe { std::slice::from_raw_parts(ip_list, ip_list_len as usize) };
    let result_wstr = unsafe { std::slice::from_raw_parts_mut(result, result_len as usize) };
    (*(*this).impl_ptr).geo_resolve_ipv6_bulk(ip_list_slice, result_wstr, flags)
}

unsafe extern "C" fn IMTServerAPI_GeoResolveReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).geo_resolve_reserved1()
}

unsafe extern "C" fn IMTServerAPI_GeoResolveReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).geo_resolve_reserved2()
}

unsafe extern "C" fn IMTServerAPI_GeoResolveReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).geo_resolve_reserved3()
}

unsafe extern "C" fn IMTServerAPI_GeoResolveReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).geo_resolve_reserved4()
}

unsafe extern "C" fn IMTServerAPI_VPSCreate(this: *mut IMTServerAPI) -> *mut IMTConVPS {
    (*(*this).impl_ptr).vps_create()
}

unsafe extern "C" fn IMTServerAPI_VPSCreateGroup(this: *mut IMTServerAPI) -> *mut IMTConVPSGroup {
    (*(*this).impl_ptr).vps_create_group()
}

unsafe extern "C" fn IMTServerAPI_VPSSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConVPSSink) -> MTAPIRES {
    (*(*this).impl_ptr).vps_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_VPSUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConVPSSink) -> MTAPIRES {
    (*(*this).impl_ptr).vps_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_VPSGet(this: *mut IMTServerAPI, config: *mut IMTConVPS) -> MTAPIRES {
    (*(*this).impl_ptr).vps_get(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_VPSSet(this: *mut IMTServerAPI, config: *const IMTConVPS) -> MTAPIRES {
    (*(*this).impl_ptr).vps_set(&*config)
}

unsafe extern "C" fn IMTServerAPI_VPSReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).vps_reserved1()
}

unsafe extern "C" fn IMTServerAPI_VPSReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).vps_reserved2()
}

unsafe extern "C" fn IMTServerAPI_VPSReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).vps_reserved3()
}

unsafe extern "C" fn IMTServerAPI_VPSReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).vps_reserved4()
}

unsafe extern "C" fn IMTServerAPI_KYCCreate(this: *mut IMTServerAPI) -> *mut IMTConKYC {
    (*(*this).impl_ptr).kyc_create()
}

unsafe extern "C" fn IMTServerAPI_KYCCountryCreate(this: *mut IMTServerAPI) -> *mut IMTConKYCCountry {
    (*(*this).impl_ptr).kyc_country_create()
}

unsafe extern "C" fn IMTServerAPI_KYCGroupCreate(this: *mut IMTServerAPI) -> *mut IMTConKYCGroup {
    (*(*this).impl_ptr).kyc_group_create()
}

unsafe extern "C" fn IMTServerAPI_KYCSubscribe(this: *mut IMTServerAPI, sink: *mut IMTConKYCSink) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_subscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_KYCUnsubscribe(this: *mut IMTServerAPI, sink: *mut IMTConKYCSink) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_unsubscribe(&mut *sink)
}

unsafe extern "C" fn IMTServerAPI_KYCAdd(this: *mut IMTServerAPI, config: *mut IMTConKYC) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_add(&mut *config)
}

unsafe extern "C" fn IMTServerAPI_KYCDelete(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_delete(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTServerAPI_KYCDelete1(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_delete1(pos)
}

unsafe extern "C" fn IMTServerAPI_KYCShift(
    this: *mut IMTServerAPI,
    pos: UINT,
    shift: std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_shift(pos, shift)
}

unsafe extern "C" fn IMTServerAPI_KYCTotal(this: *mut IMTServerAPI) -> UINT {
    (*(*this).impl_ptr).kyc_total()
}

unsafe extern "C" fn IMTServerAPI_KYCNext(this: *mut IMTServerAPI, pos: UINT, kyc: *mut IMTConKYC) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_next(pos, &mut *kyc)
}

unsafe extern "C" fn IMTServerAPI_KYCGet(
    this: *mut IMTServerAPI,
    name: LPCWSTR,
    kyc: *mut IMTConKYC,
) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_get(conversion::to_utf16_str(name), &mut *kyc)
}

unsafe extern "C" fn IMTServerAPI_KYCStart(this: *mut IMTServerAPI, client_id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_start(client_id)
}

unsafe extern "C" fn IMTServerAPI_KYCReserved1(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_reserved1()
}

unsafe extern "C" fn IMTServerAPI_KYCReserved2(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_reserved2()
}

unsafe extern "C" fn IMTServerAPI_KYCReserved3(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_reserved3()
}

unsafe extern "C" fn IMTServerAPI_KYCReserved4(this: *mut IMTServerAPI) -> MTAPIRES {
    (*(*this).impl_ptr).kyc_reserved4()
}

pub const fn new() -> mt5_apiserver::IMTServerAPI__bindgen_vtable {
    mt5_apiserver::IMTServerAPI__bindgen_vtable {
        IMTServerAPI_About,
        IMTServerAPI_LicenseCheck,
        IMTServerAPI_Allocate,
        IMTServerAPI_Free,
        IMTServerAPI_ServerRestart,
        IMTServerAPI_ServerSubscribe,
        IMTServerAPI_ServerUnsubscribe,
        IMTServerAPI_ServerRestartRemote,
        IMTServerAPI_ServerReserved4,
        IMTServerAPI_LoggerOut,
        IMTServerAPI_LoggerRequest,
        IMTServerAPI_LoggerFlush,
        IMTServerAPI_LoggerReserved1,
        IMTServerAPI_LoggerReserved2,
        IMTServerAPI_LoggerReserved3,
        IMTServerAPI_LoggerReserved4,
        IMTServerAPI_PluginCreate,
        IMTServerAPI_PluginModuleCreate,
        IMTServerAPI_PluginParamCreate,
        IMTServerAPI_PluginSubscribe,
        IMTServerAPI_PluginUnsubscribe,
        IMTServerAPI_PluginCurrent,
        IMTServerAPI_PluginAdd,
        IMTServerAPI_PluginDelete,
        IMTServerAPI_PluginDelete1,
        IMTServerAPI_PluginShift,
        IMTServerAPI_PluginTotal,
        IMTServerAPI_PluginNext,
        IMTServerAPI_PluginGet,
        IMTServerAPI_PluginModuleTotal,
        IMTServerAPI_PluginModuleNext,
        IMTServerAPI_PluginModuleGet,
        IMTServerAPI_PluginDelete2,
        IMTServerAPI_PluginGet1,
        IMTServerAPI_PluginModuleGet1,
        IMTServerAPI_PluginReserved4,
        IMTServerAPI_CommonCreate,
        IMTServerAPI_CommonSubscribe,
        IMTServerAPI_CommonUnsubscribe,
        IMTServerAPI_CommonGet,
        IMTServerAPI_CommonSet,
        IMTServerAPI_CommonReserved1,
        IMTServerAPI_CommonReserved2,
        IMTServerAPI_CommonReserved3,
        IMTServerAPI_CommonReserved4,
        IMTServerAPI_NetServerCreate,
        IMTServerAPI_NetServerRangeCreate,
        IMTServerAPI_NetServerSubscribe,
        IMTServerAPI_NetServerUnsubscribe,
        IMTServerAPI_NetServerAdd,
        IMTServerAPI_NetServerDelete,
        IMTServerAPI_NetServerShift,
        IMTServerAPI_NetServerTotal,
        IMTServerAPI_NetServerNext,
        IMTServerAPI_NetServerGet,
        IMTServerAPI_NetServerAddressRangeCreate,
        IMTServerAPI_NetServerClusterStateCreate,
        IMTServerAPI_NetServerReserved3,
        IMTServerAPI_NetServerReserved4,
        IMTServerAPI_TimeCreate,
        IMTServerAPI_TimeSubscribe,
        IMTServerAPI_TimeUnsubscribe,
        IMTServerAPI_TimeCurrent,
        IMTServerAPI_TimeGet,
        IMTServerAPI_TimeSet,
        IMTServerAPI_TimeCurrentMsc,
        IMTServerAPI_TimeReserved2,
        IMTServerAPI_TimeReserved3,
        IMTServerAPI_TimeReserved4,
        IMTServerAPI_HolidayCreate,
        IMTServerAPI_HolidaySubscribe,
        IMTServerAPI_HolidayUnsubscribe,
        IMTServerAPI_HolidayAdd,
        IMTServerAPI_HolidayDelete,
        IMTServerAPI_HolidayShift,
        IMTServerAPI_HolidayTotal,
        IMTServerAPI_HolidayNext,
        IMTServerAPI_HolidayReserved1,
        IMTServerAPI_HolidayReserved2,
        IMTServerAPI_HolidayReserved3,
        IMTServerAPI_HolidayReserved4,
        IMTServerAPI_FirewallCreate,
        IMTServerAPI_FirewallSubscribe,
        IMTServerAPI_FirewallUnsubscribe,
        IMTServerAPI_FirewallAdd,
        IMTServerAPI_FirewallDelete,
        IMTServerAPI_FirewallShift,
        IMTServerAPI_FirewallTotal,
        IMTServerAPI_FirewallNext,
        IMTServerAPI_FirewallReserved1,
        IMTServerAPI_FirewallReserved2,
        IMTServerAPI_FirewallReserved3,
        IMTServerAPI_FirewallReserved4,
        IMTServerAPI_SymbolCreate,
        IMTServerAPI_SymbolSessionCreate,
        IMTServerAPI_SymbolSubscribe,
        IMTServerAPI_SymbolUnsubscribe,
        IMTServerAPI_SymbolAdd,
        IMTServerAPI_SymbolDelete,
        IMTServerAPI_SymbolDelete1,
        IMTServerAPI_SymbolShift,
        IMTServerAPI_SymbolTotal,
        IMTServerAPI_SymbolNext,
        IMTServerAPI_SymbolGet,
        IMTServerAPI_SymbolGet1,
        IMTServerAPI_SymbolExist,
        IMTServerAPI_SymbolReserved1,
        IMTServerAPI_SymbolReserved2,
        IMTServerAPI_SymbolReserved3,
        IMTServerAPI_SymbolReserved4,
        IMTServerAPI_GroupCreate,
        IMTServerAPI_GroupSymbolCreate,
        IMTServerAPI_GroupCommissionCreate,
        IMTServerAPI_GroupTierCreate,
        IMTServerAPI_GroupSubscribe,
        IMTServerAPI_GroupUnsubscribe,
        IMTServerAPI_GroupAdd,
        IMTServerAPI_GroupDelete,
        IMTServerAPI_GroupDelete1,
        IMTServerAPI_GroupShift,
        IMTServerAPI_GroupTotal,
        IMTServerAPI_GroupNext,
        IMTServerAPI_GroupGet,
        IMTServerAPI_GroupReserved1,
        IMTServerAPI_GroupReserved2,
        IMTServerAPI_GroupReserved3,
        IMTServerAPI_GroupReserved4,
        IMTServerAPI_ManagerCreate,
        IMTServerAPI_ManagerAccessCreate,
        IMTServerAPI_ManagerSubscribe,
        IMTServerAPI_ManagerUnsubscribe,
        IMTServerAPI_ManagerAdd,
        IMTServerAPI_ManagerDelete,
        IMTServerAPI_ManagerShift,
        IMTServerAPI_ManagerTotal,
        IMTServerAPI_ManagerNext,
        IMTServerAPI_ManagerGet,
        IMTServerAPI_ManagerReserved1,
        IMTServerAPI_ManagerReserved2,
        IMTServerAPI_ManagerReserved3,
        IMTServerAPI_ManagerReserved4,
        IMTServerAPI_HistorySyncCreate,
        IMTServerAPI_HistorySyncSubscribe,
        IMTServerAPI_HistorySyncUnsubscribe,
        IMTServerAPI_HistorySyncAdd,
        IMTServerAPI_HistorySyncDelete,
        IMTServerAPI_HistorySyncShift,
        IMTServerAPI_HistorySyncTotal,
        IMTServerAPI_HistorySyncNext,
        IMTServerAPI_HistorySyncReserved1,
        IMTServerAPI_HistorySyncReserved2,
        IMTServerAPI_HistorySyncReserved3,
        IMTServerAPI_HistorySyncReserved4,
        IMTServerAPI_FeederCreate,
        IMTServerAPI_FeederModuleCreate,
        IMTServerAPI_FeederParamCreate,
        IMTServerAPI_FeederTranslateCreate,
        IMTServerAPI_FeederSubscribe,
        IMTServerAPI_FeederUnsubscribe,
        IMTServerAPI_FeederAdd,
        IMTServerAPI_FeederDelete,
        IMTServerAPI_FeederDelete1,
        IMTServerAPI_FeederShift,
        IMTServerAPI_FeederTotal,
        IMTServerAPI_FeederNext,
        IMTServerAPI_FeederGet,
        IMTServerAPI_FeederModuleTotal,
        IMTServerAPI_FeederModuleNext,
        IMTServerAPI_FeederModuleGet,
        IMTServerAPI_FeederRestart,
        IMTServerAPI_FeederReserved2,
        IMTServerAPI_FeederReserved3,
        IMTServerAPI_FeederReserved4,
        IMTServerAPI_GatewayCreate,
        IMTServerAPI_GatewayModuleCreate,
        IMTServerAPI_GatewayParamCreate,
        IMTServerAPI_GatewayTranslateCreate,
        IMTServerAPI_GatewaySubscribe,
        IMTServerAPI_GatewayUnsubscribe,
        IMTServerAPI_GatewayAdd,
        IMTServerAPI_GatewayDelete,
        IMTServerAPI_GatewayDelete1,
        IMTServerAPI_GatewayShift,
        IMTServerAPI_GatewayTotal,
        IMTServerAPI_GatewayNext,
        IMTServerAPI_GatewayGet,
        IMTServerAPI_GatewayModuleTotal,
        IMTServerAPI_GatewayModuleNext,
        IMTServerAPI_GatewayModuleGet,
        IMTServerAPI_GatewayRestart,
        IMTServerAPI_GatewayReserved2,
        IMTServerAPI_GatewayReserved3,
        IMTServerAPI_GatewayReserved4,
        IMTServerAPI_ReportCreate,
        IMTServerAPI_ReportModuleCreate,
        IMTServerAPI_ReportParamCreate,
        IMTServerAPI_ReportSubscribe,
        IMTServerAPI_ReportUnsubscribe,
        IMTServerAPI_ReportAdd,
        IMTServerAPI_ReportDelete,
        IMTServerAPI_ReportDelete1,
        IMTServerAPI_ReportShift,
        IMTServerAPI_ReportTotal,
        IMTServerAPI_ReportNext,
        IMTServerAPI_ReportGet,
        IMTServerAPI_ReportModuleTotal,
        IMTServerAPI_ReportModuleNext,
        IMTServerAPI_ReportModuleGet,
        IMTServerAPI_ReportDelete2,
        IMTServerAPI_ReportGet1,
        IMTServerAPI_ReportModuleGet1,
        IMTServerAPI_ReportReserved4,
        IMTServerAPI_RouteCreate,
        IMTServerAPI_RouteConditionCreate,
        IMTServerAPI_RouteDealerCreate,
        IMTServerAPI_RouteSubscribe,
        IMTServerAPI_RouteUnsubscribe,
        IMTServerAPI_RouteAdd,
        IMTServerAPI_RouteDelete,
        IMTServerAPI_RouteDelete1,
        IMTServerAPI_RouteShift,
        IMTServerAPI_RouteTotal,
        IMTServerAPI_RouteNext,
        IMTServerAPI_RouteGet,
        IMTServerAPI_RouteReserved1,
        IMTServerAPI_RouteReserved2,
        IMTServerAPI_RouteReserved3,
        IMTServerAPI_RouteReserved4,
        IMTServerAPI_UserCreate,
        IMTServerAPI_UserCreateAccount,
        IMTServerAPI_UserSubscribe,
        IMTServerAPI_UserUnsubscribe,
        IMTServerAPI_UserAdd,
        IMTServerAPI_UserDelete,
        IMTServerAPI_UserUpdate,
        IMTServerAPI_UserTotal,
        IMTServerAPI_UserGet,
        IMTServerAPI_UserGroup,
        IMTServerAPI_UserLogins,
        IMTServerAPI_UserPasswordCheck,
        IMTServerAPI_UserPasswordChange,
        IMTServerAPI_UserCertDelete,
        IMTServerAPI_UserCertConfirm,
        IMTServerAPI_UserDepositChangeRaw,
        IMTServerAPI_UserDepositChange,
        IMTServerAPI_UserAccountGet,
        IMTServerAPI_UserArchive,
        IMTServerAPI_UserArchiveGet,
        IMTServerAPI_UserRestore,
        IMTServerAPI_UserArchiveLogins,
        IMTServerAPI_DealCreate,
        IMTServerAPI_DealCreateArray,
        IMTServerAPI_DealSubscribe,
        IMTServerAPI_DealUnsubscribe,
        IMTServerAPI_DealDelete,
        IMTServerAPI_DealUpdate,
        IMTServerAPI_DealGet,
        IMTServerAPI_DealGet1,
        IMTServerAPI_DealAdd,
        IMTServerAPI_DealPerform,
        IMTServerAPI_DealPerformCloseBy,
        IMTServerAPI_DealDeleteBatch,
        IMTServerAPI_PositionCreate,
        IMTServerAPI_PositionCreateArray,
        IMTServerAPI_PositionSubscribe,
        IMTServerAPI_PositionUnsubscribe,
        IMTServerAPI_PositionDelete,
        IMTServerAPI_PositionUpdate,
        IMTServerAPI_PositionGet,
        IMTServerAPI_PositionGet1,
        IMTServerAPI_PositionDeleteByTicket,
        IMTServerAPI_PositionGetByTicket,
        IMTServerAPI_PositionCheck,
        IMTServerAPI_PositionFix,
        IMTServerAPI_OrderCreate,
        IMTServerAPI_OrderCreateArray,
        IMTServerAPI_OrderSubscribe,
        IMTServerAPI_OrderUnsubscribe,
        IMTServerAPI_OrderDelete,
        IMTServerAPI_OrderUpdate,
        IMTServerAPI_OrderGet,
        IMTServerAPI_OrderGet1,
        IMTServerAPI_OrderAdd,
        IMTServerAPI_OrderDeleteBatch,
        IMTServerAPI_OrderUpdateBatch,
        IMTServerAPI_OrderUpdateBatchArray,
        IMTServerAPI_HistorySubscribe,
        IMTServerAPI_HistoryUnsubscribe,
        IMTServerAPI_HistoryDelete,
        IMTServerAPI_HistoryUpdate,
        IMTServerAPI_HistoryGet,
        IMTServerAPI_HistoryGet1,
        IMTServerAPI_HistoryReopen,
        IMTServerAPI_HistoryAdd,
        IMTServerAPI_HistoryDeleteBatch,
        IMTServerAPI_HistoryUpdateBatch,
        IMTServerAPI_DailyCreate,
        IMTServerAPI_DailyCreateArray,
        IMTServerAPI_DailySubscribe,
        IMTServerAPI_DailyUnsubscribe,
        IMTServerAPI_DailyGet,
        IMTServerAPI_DailyGetLight,
        IMTServerAPI_DailySelectByGroup,
        IMTServerAPI_DailySelectByLogins,
        IMTServerAPI_DailyReserved4,
        IMTServerAPI_TickSubscribe,
        IMTServerAPI_TickUnsubscribe,
        IMTServerAPI_TickAdd,
        IMTServerAPI_TickAddStat,
        IMTServerAPI_TickLast,
        IMTServerAPI_TickLast1,
        IMTServerAPI_TickStat,
        IMTServerAPI_TickGet,
        IMTServerAPI_TickGet1,
        IMTServerAPI_TickHistoryGetRaw,
        IMTServerAPI_TickHistoryGet,
        IMTServerAPI_TickAddBatch,
        IMTServerAPI_TickReserved3,
        IMTServerAPI_TickReserved4,
        IMTServerAPI_MailCreate,
        IMTServerAPI_MailSubscribe,
        IMTServerAPI_MailUnsubscribe,
        IMTServerAPI_MailSend,
        IMTServerAPI_MailReserved1,
        IMTServerAPI_MailReserved2,
        IMTServerAPI_MailReserved3,
        IMTServerAPI_MailReserved4,
        IMTServerAPI_NewsCreate,
        IMTServerAPI_NewsSubscribe,
        IMTServerAPI_NewsUnsubscribe,
        IMTServerAPI_NewsSend,
        IMTServerAPI_NewsReserved1,
        IMTServerAPI_NewsReserved2,
        IMTServerAPI_NewsReserved3,
        IMTServerAPI_NewsReserved4,
        IMTServerAPI_CustomSubscribe,
        IMTServerAPI_CustomUnsubscribe,
        IMTServerAPI_CustomCreateStream,
        IMTServerAPI_CustomReserved2,
        IMTServerAPI_CustomReserved3,
        IMTServerAPI_CustomReserved4,
        IMTServerAPI_TradeRequestCreate,
        IMTServerAPI_TradeSubscribe,
        IMTServerAPI_TradeUnsubscribe,
        IMTServerAPI_TradeRequest,
        IMTServerAPI_TradeProfit,
        IMTServerAPI_TradeRateBuy,
        IMTServerAPI_TradeRateSell,
        IMTServerAPI_TradeMarginCheck,
        IMTServerAPI_TradeMarginCheck1,
        IMTServerAPI_TradeBalanceCheckObsolete,
        IMTServerAPI_TradeSubscribeEOD,
        IMTServerAPI_TradeUnsubscribeEOD,
        IMTServerAPI_TradeBalanceCheck,
        IMTServerAPI_TradeAccountSet,
        IMTServerAPI_TradeConfirmCreate,
        IMTServerAPI_TradeExecutionCreate,
        IMTServerAPI_TradeRequestCreateArray,
        IMTServerAPI_TradeProfitExt,
        IMTServerAPI_BookSubscribe,
        IMTServerAPI_BookUnsubscribe,
        IMTServerAPI_BookGet,
        IMTServerAPI_BookReserved1,
        IMTServerAPI_BookReserved2,
        IMTServerAPI_BookReserved3,
        IMTServerAPI_BookReserved4,
        IMTServerAPI_ChartGet,
        IMTServerAPI_ChartDelete,
        IMTServerAPI_ChartUpdate,
        IMTServerAPI_ChartSplit,
        IMTServerAPI_ChartReserved2,
        IMTServerAPI_ChartReserved3,
        IMTServerAPI_ChartReserved4,
        IMTServerAPI_UserCertCreate,
        IMTServerAPI_UserCertUpdate,
        IMTServerAPI_UserCertGet,
        IMTServerAPI_UserCertReserved1,
        IMTServerAPI_UserCertReserved2,
        IMTServerAPI_UserCertReserved3,
        IMTServerAPI_UserCertReserved4,
        IMTServerAPI_SpreadCreate,
        IMTServerAPI_SpreadLegCreate,
        IMTServerAPI_SpreadSubscribe,
        IMTServerAPI_SpreadUnsubscribe,
        IMTServerAPI_SpreadAdd,
        IMTServerAPI_SpreadDelete,
        IMTServerAPI_SpreadShift,
        IMTServerAPI_SpreadTotal,
        IMTServerAPI_SpreadNext,
        IMTServerAPI_SpreadGet,
        IMTServerAPI_SpreadReserved1,
        IMTServerAPI_SpreadReserved2,
        IMTServerAPI_SpreadReserved3,
        IMTServerAPI_SpreadReserved4,
        IMTServerAPI_OnlineCreate,
        IMTServerAPI_OnlineCreateArray,
        IMTServerAPI_OnlineTotal,
        IMTServerAPI_OnlineNext,
        IMTServerAPI_OnlineGet,
        IMTServerAPI_OnlineDisconnect,
        IMTServerAPI_OnlineDisconnectBatch,
        IMTServerAPI_OnlineDisconnectBatchArray,
        IMTServerAPI_OnlineReserved4,
        IMTServerAPI_NotificationsSend,
        IMTServerAPI_NotificationsSend1,
        IMTServerAPI_NotificationsReserved1,
        IMTServerAPI_NotificationsReserved2,
        IMTServerAPI_NotificationsReserved3,
        IMTServerAPI_NotificationsReserved4,
        IMTServerAPI_DealUpdateBatch,
        IMTServerAPI_DealUpdateBatchArray,
        IMTServerAPI_DealAddBatch,
        IMTServerAPI_DealAddBatchArray,
        IMTServerAPI_DealPerformBatch,
        IMTServerAPI_DealPerformBatchArray,
        IMTServerAPI_DealSelectByGroup,
        IMTServerAPI_DealSelectByLogins,
        IMTServerAPI_DealGetByGroup,
        IMTServerAPI_DealGetByLogins,
        IMTServerAPI_OrderAddBatch,
        IMTServerAPI_OrderAddBatchArray,
        IMTServerAPI_OrderSelectByGroup,
        IMTServerAPI_OrderSelectByLogins,
        IMTServerAPI_OrderGetByGroup,
        IMTServerAPI_OrderGetByLogins,
        IMTServerAPI_HistoryUpdateBatchArray,
        IMTServerAPI_HistoryAddBatch,
        IMTServerAPI_HistoryAddBatchArray,
        IMTServerAPI_HistorySelectByGroup,
        IMTServerAPI_HistorySelectByLogins,
        IMTServerAPI_HistoryGetByGroup,
        IMTServerAPI_HistoryGetByLogins,
        IMTServerAPI_DealerStart,
        IMTServerAPI_DealerStop,
        IMTServerAPI_DealerGet,
        IMTServerAPI_DealerLock,
        IMTServerAPI_DealerAnswer,
        IMTServerAPI_DealerRequestTotal,
        IMTServerAPI_DealerRequestNext,
        IMTServerAPI_DealerRequestGet,
        IMTServerAPI_DealerRequestGetAll,
        IMTServerAPI_DealerExecution,
        IMTServerAPI_DealerReserved2,
        IMTServerAPI_DealerReserved3,
        IMTServerAPI_DealerReserved4,
        IMTServerAPI_TradeMarginCheckExt,
        IMTServerAPI_TradeReserved1,
        IMTServerAPI_TradeReserved2,
        IMTServerAPI_TradeReserved3,
        IMTServerAPI_TradeReserved4,
        IMTServerAPI_TradeReserved5,
        IMTServerAPI_TradeReserved6,
        IMTServerAPI_EmailCreate,
        IMTServerAPI_EmailSubscribe,
        IMTServerAPI_EmailUnsubscribe,
        IMTServerAPI_EmailAdd,
        IMTServerAPI_EmailDelete,
        IMTServerAPI_EmailDelete1,
        IMTServerAPI_EmailShift,
        IMTServerAPI_EmailTotal,
        IMTServerAPI_EmailNext,
        IMTServerAPI_EmailGet,
        IMTServerAPI_EmailSend,
        IMTServerAPI_EmailReserved2,
        IMTServerAPI_EmailReserved3,
        IMTServerAPI_EmailReserved4,
        IMTServerAPI_MessengerCreate,
        IMTServerAPI_MessengerSubscribe,
        IMTServerAPI_MessengerUnsubscribe,
        IMTServerAPI_MessengerAdd,
        IMTServerAPI_MessengerDelete,
        IMTServerAPI_MessengerDelete1,
        IMTServerAPI_MessengerShift,
        IMTServerAPI_MessengerTotal,
        IMTServerAPI_MessengerNext,
        IMTServerAPI_MessengerGet,
        IMTServerAPI_MessengerSend,
        IMTServerAPI_MessengerVerifyPhone,
        IMTServerAPI_MessengerCountryCreate,
        IMTServerAPI_MessengerGroupCreate,
        IMTServerAPI_PositionSelectByGroup,
        IMTServerAPI_PositionSelectByLogins,
        IMTServerAPI_PositionGetByGroup,
        IMTServerAPI_PositionGetByLogins,
        IMTServerAPI_PositionGetByTickets,
        IMTServerAPI_PositionSplit,
        IMTServerAPI_PositionReserved2,
        IMTServerAPI_PositionReserved3,
        IMTServerAPI_PositionReserved4,
        IMTServerAPI_DatasetRequestCreate,
        IMTServerAPI_DatasetCreate,
        IMTServerAPI_DatasetReserved1,
        IMTServerAPI_DatasetReserved2,
        IMTServerAPI_DatasetReserved3,
        IMTServerAPI_DatasetReserved4,
        IMTServerAPI_OrderGetByTickets,
        IMTServerAPI_OrderReserved1,
        IMTServerAPI_OrderReserved2,
        IMTServerAPI_OrderReserved3,
        IMTServerAPI_OrderReserved4,
        IMTServerAPI_HistoryGetByTickets,
        IMTServerAPI_HistoryReserved1,
        IMTServerAPI_HistoryReserved2,
        IMTServerAPI_HistoryReserved3,
        IMTServerAPI_HistoryReserved4,
        IMTServerAPI_DealGetByTickets,
        IMTServerAPI_DealReserved1,
        IMTServerAPI_DealReserved2,
        IMTServerAPI_DealReserved3,
        IMTServerAPI_DealReserved4,
        IMTServerAPI_ClientCreate,
        IMTServerAPI_ClientCreateArray,
        IMTServerAPI_ClientSubscribe,
        IMTServerAPI_ClientUnsubscribe,
        IMTServerAPI_ClientAdd,
        IMTServerAPI_ClientUpdate,
        IMTServerAPI_ClientDelete,
        IMTServerAPI_ClientGet,
        IMTServerAPI_ClientGetHistory,
        IMTServerAPI_ClientIdsAll,
        IMTServerAPI_ClientIdsByGroup,
        IMTServerAPI_ClientIdsByManager,
        IMTServerAPI_ClientUserAdd,
        IMTServerAPI_ClientUserDelete,
        IMTServerAPI_ClientUserLogins,
        IMTServerAPI_ClientReserved1,
        IMTServerAPI_ClientReserved2,
        IMTServerAPI_ClientReserved3,
        IMTServerAPI_ClientReserved4,
        IMTServerAPI_DocumentCreate,
        IMTServerAPI_DocumentCreateArray,
        IMTServerAPI_DocumentSubscribe,
        IMTServerAPI_DocumentUnsubscribe,
        IMTServerAPI_DocumentAdd,
        IMTServerAPI_DocumentUpdate,
        IMTServerAPI_DocumentDelete,
        IMTServerAPI_DocumentGet,
        IMTServerAPI_DocumentGetByClient,
        IMTServerAPI_DocumentGetHistory,
        IMTServerAPI_DocumentReserved1,
        IMTServerAPI_DocumentReserved2,
        IMTServerAPI_DocumentReserved3,
        IMTServerAPI_DocumentReserved4,
        IMTServerAPI_CommentCreate,
        IMTServerAPI_CommentCreateArray,
        IMTServerAPI_CommentSubscribe,
        IMTServerAPI_CommentUnsubscribe,
        IMTServerAPI_CommentAdd,
        IMTServerAPI_CommentUpdate,
        IMTServerAPI_CommentDelete,
        IMTServerAPI_CommentGet,
        IMTServerAPI_CommentGetByClient,
        IMTServerAPI_CommentGetByDocument,
        IMTServerAPI_CommentReserved1,
        IMTServerAPI_CommentReserved2,
        IMTServerAPI_CommentReserved3,
        IMTServerAPI_CommentReserved4,
        IMTServerAPI_AttachmentCreate,
        IMTServerAPI_AttachmentAdd,
        IMTServerAPI_AttachmentGet,
        IMTServerAPI_AttachmentReserved1,
        IMTServerAPI_AttachmentReserved2,
        IMTServerAPI_AttachmentReserved3,
        IMTServerAPI_AttachmentReserved4,
        IMTServerAPI_TLSCertificateUpdate,
        IMTServerAPI_TLSCertificateDelete,
        IMTServerAPI_TLSCertificateShift,
        IMTServerAPI_TLSCertificateTotal,
        IMTServerAPI_TLSCertificateNext,
        IMTServerAPI_TLSCertificatePfx,
        IMTServerAPI_TLSCertificateReserved1,
        IMTServerAPI_TLSCertificateReserved2,
        IMTServerAPI_TLSCertificateReserved3,
        IMTServerAPI_TLSCertificateReserved4,
        IMTServerAPI_AutomationCreate,
        IMTServerAPI_AutomationConditionCreate,
        IMTServerAPI_AutomationActionCreate,
        IMTServerAPI_AutomationParamCreate,
        IMTServerAPI_AutomationSubscribe,
        IMTServerAPI_AutomationUnsubscribe,
        IMTServerAPI_AutomationAdd,
        IMTServerAPI_AutomationDelete,
        IMTServerAPI_AutomationDelete1,
        IMTServerAPI_AutomationShift,
        IMTServerAPI_AutomationTotal,
        IMTServerAPI_AutomationNext,
        IMTServerAPI_AutomationGet,
        IMTServerAPI_AutomationTrigger,
        IMTServerAPI_AutomationReserved2,
        IMTServerAPI_AutomationReserved3,
        IMTServerAPI_AutomationReserved4,
        IMTServerAPI_SubscriptionCfgCreate,
        IMTServerAPI_SubscriptionCfgSymbolCreate,
        IMTServerAPI_SubscriptionCfgNewsCreate,
        IMTServerAPI_SubscriptionCfgSubscribe,
        IMTServerAPI_SubscriptionCfgUnsubscribe,
        IMTServerAPI_SubscriptionCfgAdd,
        IMTServerAPI_SubscriptionCfgDelete,
        IMTServerAPI_SubscriptionCfgDelete1,
        IMTServerAPI_SubscriptionCfgDeleteByID,
        IMTServerAPI_SubscriptionCfgShift,
        IMTServerAPI_SubscriptionCfgTotal,
        IMTServerAPI_SubscriptionCfgNext,
        IMTServerAPI_SubscriptionCfgGet,
        IMTServerAPI_SubscriptionCfgGetByID,
        IMTServerAPI_SubscriptionCfgReserved1,
        IMTServerAPI_SubscriptionCfgReserved2,
        IMTServerAPI_SubscriptionCfgReserved3,
        IMTServerAPI_SubscriptionCfgReserved4,
        IMTServerAPI_SubscriptionCreate,
        IMTServerAPI_SubscriptionCreateArray,
        IMTServerAPI_SubscriptionSubscribe,
        IMTServerAPI_SubscriptionUnsubscribe,
        IMTServerAPI_SubscriptionJoin,
        IMTServerAPI_SubscriptionCancel,
        IMTServerAPI_SubscriptionExist,
        IMTServerAPI_SubscriptionAdd,
        IMTServerAPI_SubscriptionUpdate,
        IMTServerAPI_SubscriptionDelete,
        IMTServerAPI_SubscriptionGet,
        IMTServerAPI_SubscriptionGetBySubscription,
        IMTServerAPI_SubscriptionGetByID,
        IMTServerAPI_SubscriptionGetByLogins,
        IMTServerAPI_SubscriptionReserved1,
        IMTServerAPI_SubscriptionReserved2,
        IMTServerAPI_SubscriptionReserved3,
        IMTServerAPI_SubscriptionReserved4,
        IMTServerAPI_SubscriptionHistoryCreate,
        IMTServerAPI_SubscriptionHistoryCreateArray,
        IMTServerAPI_SubscriptionHistorySubscribe,
        IMTServerAPI_SubscriptionHistoryUnsubscribe,
        IMTServerAPI_SubscriptionHistoryAdd,
        IMTServerAPI_SubscriptionHistoryUpdate,
        IMTServerAPI_SubscriptionHistoryDelete,
        IMTServerAPI_SubscriptionHistoryGet,
        IMTServerAPI_SubscriptionHistoryGetByID,
        IMTServerAPI_SubscriptionHistoryGetByLogins,
        IMTServerAPI_SubscriptionHistoryReserved1,
        IMTServerAPI_SubscriptionHistoryReserved2,
        IMTServerAPI_SubscriptionHistoryReserved3,
        IMTServerAPI_SubscriptionHistoryReserved4,
        IMTServerAPI_GeoResolveAny,
        IMTServerAPI_GeoResolveIPv4,
        IMTServerAPI_GeoResolveIPv4Bulk,
        IMTServerAPI_GeoResolveIPv6,
        IMTServerAPI_GeoResolveIPv6Bulk,
        IMTServerAPI_GeoResolveReserved1,
        IMTServerAPI_GeoResolveReserved2,
        IMTServerAPI_GeoResolveReserved3,
        IMTServerAPI_GeoResolveReserved4,
        IMTServerAPI_VPSCreate,
        IMTServerAPI_VPSCreateGroup,
        IMTServerAPI_VPSSubscribe,
        IMTServerAPI_VPSUnsubscribe,
        IMTServerAPI_VPSGet,
        IMTServerAPI_VPSSet,
        IMTServerAPI_VPSReserved1,
        IMTServerAPI_VPSReserved2,
        IMTServerAPI_VPSReserved3,
        IMTServerAPI_VPSReserved4,
        IMTServerAPI_KYCCreate,
        IMTServerAPI_KYCCountryCreate,
        IMTServerAPI_KYCGroupCreate,
        IMTServerAPI_KYCSubscribe,
        IMTServerAPI_KYCUnsubscribe,
        IMTServerAPI_KYCAdd,
        IMTServerAPI_KYCDelete,
        IMTServerAPI_KYCDelete1,
        IMTServerAPI_KYCShift,
        IMTServerAPI_KYCTotal,
        IMTServerAPI_KYCNext,
        IMTServerAPI_KYCGet,
        IMTServerAPI_KYCStart,
        IMTServerAPI_KYCReserved1,
        IMTServerAPI_KYCReserved2,
        IMTServerAPI_KYCReserved3,
        IMTServerAPI_KYCReserved4,
    }
}
