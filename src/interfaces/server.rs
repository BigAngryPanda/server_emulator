use crate::mt5_apiserver;

use server_emulator_macro::log_trait_calls;

use std::os::raw::{
    c_longlong,
    c_uint,
    c_int,
    c_ulonglong,
    c_ulong,
    c_void
};

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MT5Server {
    fn about(&mut self, info: &mut mt5_apiserver::MTServerInfo) -> c_uint {
        0
    }

    fn license_check(&mut self, license_name: &u16) -> c_uint {
        0
    }

    fn allocate(&mut self, bytes: std::os::raw::c_uint) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn free(&mut self, ptr: *mut std::os::raw::c_void) {}

    fn server_restart(&mut self) -> c_uint {
        0
    }

    fn server_subscribe(&mut self, sink: &mut mt5_apiserver::IMTServerSink) -> c_uint {
        0
    }

    fn server_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTServerSink,
    ) -> c_uint {
        0
    }

    fn server_restart_remote(&mut self, id: c_ulonglong, reason: &[u16]) -> c_uint {
        0
    }

    fn server_reserved4(&mut self) -> c_uint {
        0
    }

    fn logger_out(&mut self, code: c_uint, msg: &[u16]) -> c_uint {
        0
    }

    fn logger_request(
        &mut self,
        mode: c_uint,
        type_: c_uint,
        from: c_longlong,
        to: c_longlong,
        filter: &[u16],
        records: &mut &mut mt5_apiserver::MTLogRecord,
        records_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn logger_flush(&mut self) {}

    fn logger_reserved1(&mut self) -> c_uint {
        0
    }

    fn logger_reserved2(&mut self) -> c_uint {
        0
    }

    fn logger_reserved3(&mut self) -> c_uint {
        0
    }

    fn logger_reserved4(&mut self) -> c_uint {
        0
    }

    fn plugin_create(&mut self) -> *mut mt5_apiserver::IMTConPlugin {
        std::ptr::null_mut()
    }

    fn plugin_module_create(&mut self) -> *mut mt5_apiserver::IMTConPluginModule {
        std::ptr::null_mut()
    }

    fn plugin_param_create(&mut self) -> *mut mt5_apiserver::IMTConParam {
        std::ptr::null_mut()
    }

    fn plugin_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConPluginSink,
    ) -> c_uint {
        0
    }

    fn plugin_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConPluginSink,
    ) -> c_uint {
        0
    }

    fn plugin_current(
        &mut self,
        plugin: &mut mt5_apiserver::IMTConPlugin,
    ) -> c_uint {
        0
    }

    fn plugin_add(&mut self, plugin: &mut mt5_apiserver::IMTConPlugin) -> c_uint {
        0
    }

    fn plugin_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn plugin_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn plugin_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn plugin_total(&mut self) -> c_uint {
        0
    }

    fn plugin_next(
        &mut self,
        pos: c_uint,
        plugin: &mut mt5_apiserver::IMTConPlugin,
    ) -> c_uint {
        0
    }

    fn plugin_get(
        &mut self,
        name: &[u16],
        plugin: &mut mt5_apiserver::IMTConPlugin,
    ) -> c_uint {
        0
    }

    fn plugin_module_total(&mut self) -> c_uint {
        0
    }

    fn plugin_module_next(
        &mut self,
        pos: c_uint,
        module: &mut mt5_apiserver::IMTConPluginModule,
    ) -> c_uint {
        0
    }

    fn plugin_module_get(
        &mut self,
        name: &[u16],
        module: &mut mt5_apiserver::IMTConPluginModule,
    ) -> c_uint {
        0
    }

    fn plugin_delete2(&mut self, server: c_ulonglong, name: &[u16]) -> c_uint {
        0
    }

    fn plugin_get1(
        &mut self,
        server: c_ulonglong,
        name: &[u16],
        plugin: &mut mt5_apiserver::IMTConPlugin,
    ) -> c_uint {
        0
    }

    fn plugin_module_get1(
        &mut self,
        server: c_ulonglong,
        name: &[u16],
        module: &mut mt5_apiserver::IMTConPluginModule,
    ) -> c_uint {
        0
    }

    fn plugin_reserved4(&mut self) -> c_uint {
        0
    }

    fn common_create(&mut self) -> *mut mt5_apiserver::IMTConCommon {
        std::ptr::null_mut()
    }

    fn common_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConCommonSink,
    ) -> c_uint {
        0
    }

    fn common_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConCommonSink,
    ) -> c_uint {
        0
    }

    fn common_get(&mut self, common: &mut mt5_apiserver::IMTConCommon) -> c_uint {
        0
    }

    fn common_set(
        &mut self,
        common: *const mt5_apiserver::IMTConCommon,
    ) -> c_uint {
        0
    }

    fn common_reserved1(&mut self) -> c_uint {
        0
    }

    fn common_reserved2(&mut self) -> c_uint {
        0
    }

    fn common_reserved3(&mut self) -> c_uint {
        0
    }

    fn common_reserved4(&mut self) -> c_uint {
        0
    }

    fn net_server_create(&mut self) -> *mut mt5_apiserver::IMTConServer {
        std::ptr::null_mut()
    }

    fn net_server_range_create(&mut self) -> *mut mt5_apiserver::IMTConServerRange {
        std::ptr::null_mut()
    }

    fn net_server_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConServerSink,
    ) -> c_uint {
        0
    }

    fn net_server_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConServerSink,
    ) -> c_uint {
        0
    }

    fn net_server_add(
        &mut self,
        config: &mut mt5_apiserver::IMTConServer,
    ) -> c_uint {
        0
    }

    fn net_server_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn net_server_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn net_server_total(&mut self) -> c_uint {
        0
    }

    fn net_server_next(
        &mut self,
        pos: c_uint,
        config: &mut mt5_apiserver::IMTConServer,
    ) -> c_uint {
        0
    }

    fn net_server_get(
        &mut self,
        id: c_ulonglong,
        config: &mut mt5_apiserver::IMTConServer,
    ) -> c_uint {
        0
    }

    fn net_server_address_range_create(
        &mut self,
    ) -> *mut mt5_apiserver::IMTConAddressRange {
        std::ptr::null_mut()
    }

    fn net_server_cluster_state_create(
        &mut self,
    ) -> *mut mt5_apiserver::IMTConClusterState {
        std::ptr::null_mut()
    }

    fn net_server_reserved3(&mut self) -> c_uint {
        0
    }

    fn net_server_reserved4(&mut self) -> c_uint {
        0
    }

    fn time_create(&mut self) -> *mut mt5_apiserver::IMTConTime {
        std::ptr::null_mut()
    }

    fn time_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConTimeSink,
    ) -> c_uint {
        0
    }

    fn time_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConTimeSink,
    ) -> c_uint {
        0
    }

    fn time_current(&mut self) -> c_longlong {
        0
    }

    fn time_get(&mut self, config: &mut mt5_apiserver::IMTConTime) -> c_uint {
        0
    }

    fn time_set(&mut self, config: *const mt5_apiserver::IMTConTime) -> c_uint {
        0
    }

    fn time_current_msc(&mut self) -> c_longlong {
        0
    }

    fn time_reserved2(&mut self) -> c_uint {
        0
    }

    fn time_reserved3(&mut self) -> c_uint {
        0
    }

    fn time_reserved4(&mut self) -> c_uint {
        0
    }

    fn holiday_create(&mut self) -> *mut mt5_apiserver::IMTConHoliday {
        std::ptr::null_mut()
    }

    fn holiday_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConHolidaySink,
    ) -> c_uint {
        0
    }

    fn holiday_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConHolidaySink,
    ) -> c_uint {
        0
    }

    fn holiday_add(
        &mut self,
        config: &mut mt5_apiserver::IMTConHoliday,
    ) -> c_uint {
        0
    }

    fn holiday_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn holiday_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn holiday_total(&mut self) -> c_uint {
        0
    }

    fn holiday_next(
        &mut self,
        pos: c_uint,
        config: &mut mt5_apiserver::IMTConHoliday,
    ) -> c_uint {
        0
    }

    fn holiday_reserved1(&mut self) -> c_uint {
        0
    }

    fn holiday_reserved2(&mut self) -> c_uint {
        0
    }

    fn holiday_reserved3(&mut self) -> c_uint {
        0
    }

    fn holiday_reserved4(&mut self) -> c_uint {
        0
    }

    fn firewall_create(&mut self) -> *mut mt5_apiserver::IMTConFirewall {
        std::ptr::null_mut()
    }

    fn firewall_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConFirewallSink,
    ) -> c_uint {
        0
    }

    fn firewall_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConFirewallSink,
    ) -> c_uint {
        0
    }

    fn firewall_add(
        &mut self,
        config: &mut mt5_apiserver::IMTConFirewall,
    ) -> c_uint {
        0
    }

    fn firewall_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn firewall_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn firewall_total(&mut self) -> c_uint {
        0
    }

    fn firewall_next(
        &mut self,
        pos: c_uint,
        config: &mut mt5_apiserver::IMTConFirewall,
    ) -> c_uint {
        0
    }

    fn firewall_reserved1(&mut self) -> c_uint {
        0
    }

    fn firewall_reserved2(&mut self) -> c_uint {
        0
    }

    fn firewall_reserved3(&mut self) -> c_uint {
        0
    }

    fn firewall_reserved4(&mut self) -> c_uint {
        0
    }

    fn symbol_create(&mut self) -> *mut mt5_apiserver::IMTConSymbol {
        std::ptr::null_mut()
    }

    fn symbol_session_create(&mut self) -> *mut mt5_apiserver::IMTConSymbolSession {
        std::ptr::null_mut()
    }

    fn symbol_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConSymbolSink,
    ) -> c_uint {
        0
    }

    fn symbol_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConSymbolSink,
    ) -> c_uint {
        0
    }

    fn symbol_add(&mut self, symbol: &mut mt5_apiserver::IMTConSymbol) -> c_uint {
        0
    }

    fn symbol_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn symbol_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn symbol_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn symbol_total(&mut self) -> c_uint {
        0
    }

    fn symbol_next(
        &mut self,
        pos: c_uint,
        symbol: &mut mt5_apiserver::IMTConSymbol,
    ) -> c_uint {
        0
    }

    fn symbol_get(
        &mut self,
        name: &[u16],
        symbol: &mut mt5_apiserver::IMTConSymbol,
    ) -> c_uint {
        0
    }

    fn symbol_get1(
        &mut self,
        name: &[u16],
        group: &mt5_apiserver::IMTConGroup,
        symbol: &mut mt5_apiserver::IMTConSymbol,
    ) -> c_uint {
        0
    }

    fn symbol_exist(
        &mut self,
        symbol: &mt5_apiserver::IMTConSymbol,
        group: &mt5_apiserver::IMTConGroup,
    ) -> c_uint {
        0
    }

    fn symbol_reserved1(&mut self) -> c_uint {
        0
    }

    fn symbol_reserved2(&mut self) -> c_uint {
        0
    }

    fn symbol_reserved3(&mut self) -> c_uint {
        0
    }

    fn symbol_reserved4(&mut self) -> c_uint {
        0
    }

    fn group_create(&mut self) -> *mut mt5_apiserver::IMTConGroup {
        std::ptr::null_mut()
    }

    fn group_symbol_create(&mut self) -> *mut mt5_apiserver::IMTConGroupSymbol {
        std::ptr::null_mut()
    }

    fn group_commission_create(&mut self) -> *mut mt5_apiserver::IMTConCommission {
        std::ptr::null_mut()
    }

    fn group_tier_create(&mut self) -> *mut mt5_apiserver::IMTConCommTier {
        std::ptr::null_mut()
    }

    fn group_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConGroupSink,
    ) -> c_uint {
        0
    }

    fn group_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConGroupSink,
    ) -> c_uint {
        0
    }

    fn group_add(&mut self, group: &mut mt5_apiserver::IMTConGroup) -> c_uint {
        0
    }

    fn group_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn group_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn group_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn group_total(&mut self) -> c_uint {
        0
    }

    fn group_next(
        &mut self,
        pos: c_uint,
        group: &mut mt5_apiserver::IMTConGroup,
    ) -> c_uint {
        0
    }

    fn group_get(
        &mut self,
        name: &[u16],
        group: &mut mt5_apiserver::IMTConGroup,
    ) -> c_uint {
        0
    }

    fn group_reserved1(&mut self) -> c_uint {
        0
    }

    fn group_reserved2(&mut self) -> c_uint {
        0
    }

    fn group_reserved3(&mut self) -> c_uint {
        0
    }

    fn group_reserved4(&mut self) -> c_uint {
        0
    }

    fn manager_create(&mut self) -> *mut mt5_apiserver::IMTConManager {
        std::ptr::null_mut()
    }

    fn manager_access_create(&mut self) -> *mut mt5_apiserver::IMTConManagerAccess {
        std::ptr::null_mut()
    }

    fn manager_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConManagerSink) -> c_uint {
        0
    }

    fn manager_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConManagerSink) -> c_uint {
        0
    }

    fn manager_add(&mut self, manager: &mut mt5_apiserver::IMTConManager) -> c_uint {
        0
    }

    fn manager_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn manager_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn manager_total(&mut self) -> c_uint {
        0
    }

    fn manager_next(
        &mut self,
        pos: c_uint,
        manager: &mut mt5_apiserver::IMTConManager,
    ) -> c_uint {
        0
    }

    fn manager_get(
        &mut self,
        login: c_ulonglong,
        manager: &mut mt5_apiserver::IMTConManager,
    ) -> c_uint {
        0
    }

    fn manager_reserved1(&mut self) -> c_uint {
        0
    }

    fn manager_reserved2(&mut self) -> c_uint {
        0
    }

    fn manager_reserved3(&mut self) -> c_uint {
        0
    }

    fn manager_reserved4(&mut self) -> c_uint {
        0
    }

    fn history_sync_create(&mut self) -> *mut mt5_apiserver::IMTConHistorySync {
        std::ptr::null_mut()
    }

    fn history_sync_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConHistorySyncSink,
    ) -> c_uint {
        0
    }

    fn history_sync_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConHistorySyncSink,
    ) -> c_uint {
        0
    }

    fn history_sync_add(&mut self, config: &mut mt5_apiserver::IMTConHistorySync) -> c_uint {
        0
    }

    fn history_sync_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn history_sync_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn history_sync_total(&mut self) -> c_uint {
        0
    }

    fn history_sync_next(
        &mut self,
        pos: c_uint,
        config: &mut mt5_apiserver::IMTConHistorySync,
    ) -> c_uint {
        0
    }

    fn history_sync_reserved1(&mut self) -> c_uint {
        0
    }

    fn history_sync_reserved2(&mut self) -> c_uint {
        0
    }

    fn history_sync_reserved3(&mut self) -> c_uint {
        0
    }

    fn history_sync_reserved4(&mut self) -> c_uint {
        0
    }

    fn feeder_create(&mut self) -> *mut mt5_apiserver::IMTConFeeder {
        std::ptr::null_mut()
    }

    fn feeder_module_create(&mut self) -> *mut mt5_apiserver::IMTConFeederModule {
        std::ptr::null_mut()
    }

    fn feeder_param_create(&mut self) -> *mut mt5_apiserver::IMTConParam {
        std::ptr::null_mut()
    }

    fn feeder_translate_create(&mut self) -> *mut mt5_apiserver::IMTConFeederTranslate {
        std::ptr::null_mut()
    }

    fn feeder_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConFeederSink) -> c_uint {
        0
    }

    fn feeder_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConFeederSink) -> c_uint {
        0
    }

    fn feeder_add(&mut self, feeder: &mut mt5_apiserver::IMTConFeeder) -> c_uint {
        0
    }

    fn feeder_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn feeder_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn feeder_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn feeder_total(&mut self) -> c_uint {
        0
    }

    fn feeder_next(&mut self, pos: c_uint, feeder: &mut mt5_apiserver::IMTConFeeder) -> c_uint {
        0
    }

    fn feeder_get(&mut self, name: &[u16], feeder: &mut mt5_apiserver::IMTConFeeder) -> c_uint {
        0
    }

    fn feeder_module_total(&mut self) -> c_uint {
        0
    }

    fn feeder_module_next(
        &mut self,
        pos: c_uint,
        module: &mut mt5_apiserver::IMTConFeederModule,
    ) -> c_uint {
        0
    }

    fn feeder_module_get(
        &mut self,
        name: &[u16],
        module: &mut mt5_apiserver::IMTConFeederModule,
    ) -> c_uint {
        0
    }

    fn feeder_restart(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn feeder_reserved2(&mut self) -> c_uint {
        0
    }

    fn feeder_reserved3(&mut self) -> c_uint {
        0
    }

    fn feeder_reserved4(&mut self) -> c_uint {
        0
    }

    fn gateway_create(&mut self) -> *mut mt5_apiserver::IMTConGateway {
        std::ptr::null_mut()
    }

    fn gateway_module_create(&mut self) -> *mut mt5_apiserver::IMTConGatewayModule {
        std::ptr::null_mut()
    }

    fn gateway_param_create(&mut self) -> *mut mt5_apiserver::IMTConParam {
        std::ptr::null_mut()
    }

    fn gateway_translate_create(&mut self) -> *mut mt5_apiserver::IMTConGatewayTranslate {
        std::ptr::null_mut()
    }

    fn gateway_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConGatewaySink) -> c_uint {
        0
    }

    fn gateway_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConGatewaySink) -> c_uint {
        0
    }

    fn gateway_add(&mut self, gateway: &mut mt5_apiserver::IMTConGateway) -> c_uint {
        0
    }

    fn gateway_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn gateway_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn gateway_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn gateway_total(&mut self) -> c_uint {
        0
    }

    fn gateway_next(
        &mut self,
        pos: c_uint,
        gateway: &mut mt5_apiserver::IMTConGateway,
    ) -> c_uint {
        0
    }

    fn gateway_get(
        &mut self,
        name: &[u16],
        gateway: &mut mt5_apiserver::IMTConGateway,
    ) -> c_uint {
        0
    }

    fn gateway_module_total(&mut self) -> c_uint {
        0
    }

    fn gateway_module_next(
        &mut self,
        pos: c_uint,
        module: &mut mt5_apiserver::IMTConGatewayModule,
    ) -> c_uint {
        0
    }

    fn gateway_module_get(
        &mut self,
        name: &[u16],
        module: &mut mt5_apiserver::IMTConGatewayModule,
    ) -> c_uint {
        0
    }

    fn gateway_restart(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn gateway_reserved2(&mut self) -> c_uint {
        0
    }

    fn gateway_reserved3(&mut self) -> c_uint {
        0
    }

    fn gateway_reserved4(&mut self) -> c_uint {
        0
    }

    fn report_create(&mut self) -> *mut mt5_apiserver::IMTConReport {
        std::ptr::null_mut()
    }

    fn report_module_create(&mut self) -> *mut mt5_apiserver::IMTConReportModule {
        std::ptr::null_mut()
    }

    fn report_param_create(&mut self) -> *mut mt5_apiserver::IMTConParam {
        std::ptr::null_mut()
    }

    fn report_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConReportSink) -> c_uint {
        0
    }

    fn report_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConReportSink) -> c_uint {
        0
    }

    fn report_add(&mut self, report: &mut mt5_apiserver::IMTConReport) -> c_uint {
        0
    }

    fn report_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn report_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn report_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn report_total(&mut self) -> c_uint {
        0
    }

    fn report_next(&mut self, pos: c_uint, report: &mut mt5_apiserver::IMTConReport) -> c_uint {
        0
    }

    fn report_get(&mut self, name: &[u16], report: &mut mt5_apiserver::IMTConReport) -> c_uint {
        0
    }

    fn report_module_total(&mut self) -> c_uint {
        0
    }

    fn report_module_next(
        &mut self,
        pos: c_uint,
        module: &mut mt5_apiserver::IMTConReportModule,
    ) -> c_uint {
        0
    }

    fn report_module_get(
        &mut self,
        name: &[u16],
        module: &mut mt5_apiserver::IMTConReportModule,
    ) -> c_uint {
        0
    }

    fn report_delete2(&mut self, server: c_ulonglong, name: &[u16]) -> c_uint {
        0
    }

    fn report_get1(
        &mut self,
        server: c_ulonglong,
        name: &[u16],
        report: &mut mt5_apiserver::IMTConReport,
    ) -> c_uint {
        0
    }

    fn report_module_get1(
        &mut self,
        server: c_ulonglong,
        name: &[u16],
        module: &mut mt5_apiserver::IMTConReportModule,
    ) -> c_uint {
        0
    }

    fn report_reserved4(&mut self) -> c_uint {
        0
    }

    fn route_create(&mut self) -> *mut mt5_apiserver::IMTConRoute {
        std::ptr::null_mut()
    }

    fn route_condition_create(&mut self) -> *mut mt5_apiserver::IMTConCondition {
        std::ptr::null_mut()
    }

    fn route_dealer_create(&mut self) -> *mut mt5_apiserver::IMTConRouteDealer {
        std::ptr::null_mut()
    }

    fn route_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConRouteSink) -> c_uint {
        0
    }

    fn route_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConRouteSink) -> c_uint {
        0
    }

    fn route_add(&mut self, route: &mut mt5_apiserver::IMTConRoute) -> c_uint {
        0
    }

    fn route_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn route_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn route_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn route_total(&mut self) -> c_uint {
        0
    }

    fn route_next(&mut self, pos: c_uint, route: &mut mt5_apiserver::IMTConRoute) -> c_uint {
        0
    }

    fn route_get(&mut self, name: &[u16], route: &mut mt5_apiserver::IMTConRoute) -> c_uint {
        0
    }

    fn route_reserved1(&mut self) -> c_uint {
        0
    }

    fn route_reserved2(&mut self) -> c_uint {
        0
    }

    fn route_reserved3(&mut self) -> c_uint {
        0
    }

    fn route_reserved4(&mut self) -> c_uint {
        0
    }

    fn user_create(&mut self) -> *mut mt5_apiserver::IMTUser {
        std::ptr::null_mut()
    }

    fn user_create_account(&mut self) -> *mut mt5_apiserver::IMTAccount {
        std::ptr::null_mut()
    }

    fn user_subscribe(&mut self, sink: &mut mt5_apiserver::IMTUserSink) -> c_uint {
        0
    }

    fn user_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTUserSink) -> c_uint {
        0
    }

    fn user_add(
        &mut self,
        user: &mut mt5_apiserver::IMTUser,
        master_pass: &[u16],
        investor_pass: &[u16],
    ) -> c_uint {
        0
    }

    fn user_delete(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn user_update(&mut self, user: &mut mt5_apiserver::IMTUser) -> c_uint {
        0
    }

    fn user_total(&mut self) -> c_uint {
        0
    }

    fn user_get(&mut self, login: c_ulonglong, user: &mut mt5_apiserver::IMTUser) -> c_uint {
        0
    }

    fn user_group(&mut self, login: c_ulonglong, group: &mut [u16; 260usize]) -> c_uint {
        0
    }

    fn user_logins(
        &mut self,
        group: &[u16],
        logins: &mut &mut c_ulonglong,
        logins_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn user_password_check(
        &mut self,
        type_: c_uint,
        login: c_ulonglong,
        password: &[u16],
    ) -> c_uint {
        0
    }

    fn user_password_change(
        &mut self,
        type_: c_uint,
        login: c_ulonglong,
        password: &[u16],
    ) -> c_uint {
        0
    }

    fn user_cert_delete(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn user_cert_confirm(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn user_deposit_change_raw(
        &mut self,
        login: c_ulonglong,
        value: f64,
        type_: c_uint,
        comment: &[u16],
        deal_id: &mut c_ulonglong,
    ) -> c_uint {
        0
    }

    fn user_deposit_change(
        &mut self,
        login: c_ulonglong,
        value: f64,
        action: c_uint,
        comment: &[u16],
        deal_id: &mut c_ulonglong,
    ) -> c_uint {
        0
    }

    fn user_account_get(
        &mut self,
        login: c_ulonglong,
        account: &mut mt5_apiserver::IMTAccount,
    ) -> c_uint {
        0
    }

    fn user_archive(&mut self, login: c_ulonglong) -> c_uint {
        0
    }

    fn user_archive_get(
        &mut self,
        login: c_ulonglong,
        user: &mut mt5_apiserver::IMTUser,
    ) -> c_uint {
        0
    }

    fn user_restore(&mut self, user: &mut mt5_apiserver::IMTUser) -> c_uint {
        0
    }

    fn user_archive_logins(
        &mut self,
        group: &[u16],
        logins: &mut &mut c_ulonglong,
        logins_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn deal_create(&mut self) -> *mut mt5_apiserver::IMTDeal {
        std::ptr::null_mut()
    }

    fn deal_create_array(&mut self) -> *mut mt5_apiserver::IMTDealArray {
        std::ptr::null_mut()
    }

    fn deal_subscribe(&mut self, sink: &mut mt5_apiserver::IMTDealSink) -> c_uint {
        0
    }

    fn deal_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTDealSink) -> c_uint {
        0
    }

    fn deal_delete(&mut self, ticket: c_ulonglong) -> c_uint {
        0
    }

    fn deal_update(&mut self, deal: &mut mt5_apiserver::IMTDeal) -> c_uint {
        0
    }

    fn deal_get(&mut self, ticket: c_ulonglong, deal: &mut mt5_apiserver::IMTDeal) -> c_uint {
        0
    }

    fn deal_get1(
        &mut self,
        from: c_longlong,
        to: c_longlong,
        login: c_ulonglong,
        deals: &mut mt5_apiserver::IMTDealArray,
    ) -> c_uint {
        0
    }

    fn deal_add(&mut self, deal: &mut mt5_apiserver::IMTDeal) -> c_uint {
        0
    }

    fn deal_perform(&mut self, deal: &mut mt5_apiserver::IMTDeal) -> c_uint {
        0
    }

    fn deal_perform_close_by(
        &mut self,
        deal: &mut mt5_apiserver::IMTDeal,
        dealby: &mut mt5_apiserver::IMTDeal,
    ) -> c_uint {
        0
    }

    fn deal_delete_batch(
        &mut self,
        tickets: *const c_ulonglong,
        tickets_total: c_uint,
        results: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn position_create(&mut self) -> *mut mt5_apiserver::IMTPosition {
        std::ptr::null_mut()
    }

    fn position_create_array(&mut self) -> *mut mt5_apiserver::IMTPositionArray {
        std::ptr::null_mut()
    }

    fn position_subscribe(&mut self, sink: &mut mt5_apiserver::IMTPositionSink) -> c_uint {
        0
    }

    fn position_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTPositionSink) -> c_uint {
        0
    }

    fn position_delete(&mut self, login: c_ulonglong, symbol: &[u16]) -> c_uint {
        0
    }

    fn position_update(&mut self, position: &mut mt5_apiserver::IMTPosition) -> c_uint {
        0
    }

    fn position_get(
        &mut self,
        login: c_ulonglong,
        symbol: &[u16],
        position: &mut mt5_apiserver::IMTPosition,
    ) -> c_uint {
        0
    }

    fn position_get1(
        &mut self,
        login: c_ulonglong,
        position: &mut mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn position_delete_by_ticket(&mut self, ticket: c_ulonglong) -> c_uint {
        0
    }

    fn position_get_by_ticket(
        &mut self,
        ticket: c_ulonglong,
        position: &mut mt5_apiserver::IMTPosition,
    ) -> c_uint {
        0
    }

    fn position_check(
        &mut self,
        login: c_ulonglong,
        current: &mut mt5_apiserver::IMTPositionArray,
        invalid: &mut mt5_apiserver::IMTPositionArray,
        missed: &mut mt5_apiserver::IMTPositionArray,
        nonexist: &mut mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn position_fix(
        &mut self,
        login: c_ulonglong,
        current: &mut mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn order_create(&mut self) -> *mut mt5_apiserver::IMTOrder {
        std::ptr::null_mut()
    }

    fn order_create_array(&mut self) -> *mut mt5_apiserver::IMTOrderArray {
        std::ptr::null_mut()
    }

    fn order_subscribe(&mut self, sink: &mut mt5_apiserver::IMTOrderSink) -> c_uint {
        0
    }

    fn order_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTOrderSink) -> c_uint {
        0
    }

    fn order_delete(&mut self, ticket: c_ulonglong) -> c_uint {
        0
    }

    fn order_update(&mut self, order: &mut mt5_apiserver::IMTOrder) -> c_uint {
        0
    }

    fn order_get(&mut self, ticket: c_ulonglong, order: &mut mt5_apiserver::IMTOrder) -> c_uint {
        0
    }

    fn order_get1(
        &mut self,
        login: c_ulonglong,
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn order_add(&mut self, order: &mut mt5_apiserver::IMTOrder) -> c_uint {
        0
    }

    fn order_delete_batch(
        &mut self,
        tickets: &[c_ulonglong],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn order_update_batch(
        &mut self,
        orders: &mut mt5_apiserver::IMTOrderArray,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn order_update_batch_array(
        &mut self,
        orders: &mut [&mut mt5_apiserver::IMTOrder],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn history_subscribe(&mut self, sink: &mut mt5_apiserver::IMTHistorySink) -> c_uint {
        0
    }

    fn history_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTHistorySink) -> c_uint {
        0
    }

    fn history_delete(&mut self, ticket: c_ulonglong) -> c_uint {
        0
    }

    fn history_update(&mut self, order: &mut mt5_apiserver::IMTOrder) -> c_uint {
        0
    }

    fn history_get(
        &mut self,
        ticket: c_ulonglong,
        order: &mut mt5_apiserver::IMTOrder,
    ) -> c_uint {
        0
    }

    fn history_get1(
        &mut self,
        from: c_longlong,
        to: c_longlong,
        login: c_ulonglong,
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn history_reopen(&mut self, ticket: c_ulonglong) -> c_uint {
        0
    }

    fn history_add(&mut self, order: &mut mt5_apiserver::IMTOrder) -> c_uint {
        0
    }

    fn history_delete_batch(
        &mut self,
        tickets: &[c_ulonglong],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn history_update_batch(
        &mut self,
        orders: &mut mt5_apiserver::IMTOrderArray,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn daily_create(&mut self) -> *mut mt5_apiserver::IMTDaily {
        std::ptr::null_mut()
    }

    fn daily_create_array(&mut self) -> *mut mt5_apiserver::IMTDailyArray {
        std::ptr::null_mut()
    }

    fn daily_subscribe(&mut self, sink: &mut mt5_apiserver::IMTDailySink) -> c_uint {
        0
    }

    fn daily_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTDailySink) -> c_uint {
        0
    }

    fn daily_get(
        &mut self,
        from: c_longlong,
        to: c_longlong,
        login: c_ulonglong,
        daily: &mut mt5_apiserver::IMTDailyArray,
    ) -> c_uint {
        0
    }

    fn daily_get_light(
        &mut self,
        from: c_longlong,
        to: c_longlong,
        login: c_ulonglong,
        daily: &mut mt5_apiserver::IMTDailyArray,
    ) -> c_uint {
        0
    }

    fn daily_select_by_group(
        &mut self,
        group: &[u16],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn daily_select_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn daily_reserved4(&mut self) -> c_uint {
        0
    }

    fn tick_subscribe(&mut self, sink: &mut mt5_apiserver::IMTTickSink) -> c_uint {
        0
    }

    fn tick_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTTickSink) -> c_uint {
        0
    }

    fn tick_add(&mut self, tick: &mut mt5_apiserver::MTTick) -> c_uint {
        0
    }

    fn tick_add_stat(
        &mut self,
        tick: &mut mt5_apiserver::MTTick,
        stat: &mut mt5_apiserver::MTTickStat,
    ) -> c_uint {
        0
    }

    fn tick_last(&mut self, symbol: &[u16], tick: &mut mt5_apiserver::MTTickShort) -> c_uint {
        0
    }

    fn tick_last1(
        &mut self,
        symbol: &mt5_apiserver::IMTConSymbol,
        tick: &mut mt5_apiserver::MTTickShort,
    ) -> c_uint {
        0
    }

    fn tick_stat(&mut self, symbol: &[u16], stat: &mut mt5_apiserver::MTTickStat) -> c_uint {
        0
    }

    fn tick_get(
        &mut self,
        symbol: &[u16],
        from: c_longlong,
        to: c_longlong,
        ticks: &mut &mut mt5_apiserver::MTTickShort,
        ticks_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn tick_get1(
        &mut self,
        symbol: &mt5_apiserver::IMTConSymbol,
        from: c_longlong,
        to: c_longlong,
        ticks: &mut &mut mt5_apiserver::MTTickShort,
        ticks_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn tick_history_get_raw(
        &mut self,
        symbol: &[u16],
        from: c_longlong,
        to: c_longlong,
        ticks: &mut &mut mt5_apiserver::MTTickShort,
        ticks_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn tick_history_get(
        &mut self,
        symbol: &[u16],
        from: c_longlong,
        to: c_longlong,
        ticks: &mut &mut mt5_apiserver::MTTickShort,
        ticks_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn tick_add_batch(
        &mut self,
        ticks: &mut [mt5_apiserver::MTTick],
    ) -> c_uint {
        0
    }

    fn tick_reserved3(&mut self) -> c_uint {
        0
    }

    fn tick_reserved4(&mut self) -> c_uint {
        0
    }

    fn mail_create(&mut self) -> *mut mt5_apiserver::IMTMail {
        std::ptr::null_mut()
    }

    fn mail_subscribe(&mut self, sink: &mut mt5_apiserver::IMTMailSink) -> c_uint {
        0
    }

    fn mail_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTMailSink) -> c_uint {
        0
    }

    fn mail_send(&mut self, mail: &mut mt5_apiserver::IMTMail) -> c_uint {
        0
    }

    fn mail_reserved1(&mut self) -> c_uint {
        0
    }

    fn mail_reserved2(&mut self) -> c_uint {
        0
    }

    fn mail_reserved3(&mut self) -> c_uint {
        0
    }

    fn mail_reserved4(&mut self) -> c_uint {
        0
    }

    fn news_create(&mut self) -> *mut mt5_apiserver::IMTNews {
        std::ptr::null_mut()
    }

    fn news_subscribe(&mut self, sink: &mut mt5_apiserver::IMTNewsSink) -> c_uint {
        0
    }

    fn news_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTNewsSink) -> c_uint {
        0
    }

    fn news_send(&mut self, news: &mut mt5_apiserver::IMTNews) -> c_uint {
        0
    }

    fn news_reserved1(&mut self) -> c_uint {
        0
    }

    fn news_reserved2(&mut self) -> c_uint {
        0
    }

    fn news_reserved3(&mut self) -> c_uint {
        0
    }

    fn news_reserved4(&mut self) -> c_uint {
        0
    }

    fn custom_subscribe(&mut self, sink: &mut mt5_apiserver::IMTCustomSink) -> c_uint {
        0
    }

    fn custom_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTCustomSink) -> c_uint {
        0
    }

    fn custom_create_stream(&mut self) -> *mut mt5_apiserver::IMTByteStream {
        std::ptr::null_mut()
    }

    fn custom_reserved2(&mut self) -> c_uint {
        0
    }

    fn custom_reserved3(&mut self) -> c_uint {
        0
    }

    fn custom_reserved4(&mut self) -> c_uint {
        0
    }

    fn trade_request_create(&mut self) -> *mut mt5_apiserver::IMTRequest {
        std::ptr::null_mut()
    }

    fn trade_subscribe(&mut self, sink: &mut mt5_apiserver::IMTTradeSink) -> c_uint {
        0
    }

    fn trade_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTTradeSink) -> c_uint {
        0
    }

    fn trade_request(&mut self, request: &mut mt5_apiserver::IMTRequest) -> c_uint {
        0
    }

    fn trade_profit(
        &mut self,
        group: &[u16],
        symbol: &[u16],
        type_: c_uint,
        volume: c_ulonglong,
        price_open: f64,
        price_close: f64,
        profit: &mut f64,
        profit_rate: &mut f64,
    ) -> c_uint {
        0
    }

    fn trade_rate_buy(
        &mut self,
        base: &[u16],
        currency: &[u16],
        rate: &mut f64,
        group: &[u16],
        symbol: &[u16],
        price: f64,
    ) -> c_uint {
        0
    }

    fn trade_rate_sell(
        &mut self,
        base: &[u16],
        currency: &[u16],
        rate: &mut f64,
        group: &[u16],
        symbol: &[u16],
        price: f64,
    ) -> c_uint {
        0
    }

    fn trade_margin_check(
        &mut self,
        login: c_ulonglong,
        symbol: &[u16],
        type_: c_uint,
        volume: c_ulonglong,
        price: f64,
        account_new: &mut mt5_apiserver::IMTAccount,
        account_current: &mut mt5_apiserver::IMTAccount,
    ) -> c_uint {
        0
    }

    fn trade_margin_check1(
        &mut self,
        order: &mt5_apiserver::IMTOrder,
        account_new: &mut mt5_apiserver::IMTAccount,
        account_current: &mut mt5_apiserver::IMTAccount,
    ) -> c_uint {
        0
    }

    fn trade_balance_check_obsolete(
        &mut self,
        login: c_ulonglong,
        fixflag: c_uint,
        balance_user: &mut f64,
        balance_history: &mut f64,
    ) -> c_uint {
        0
    }

    fn trade_subscribe_eod(&mut self, sink: &mut mt5_apiserver::IMTEndOfDaySink) -> c_uint {
        0
    }

    fn trade_unsubscribe_eod(&mut self, sink: &mut mt5_apiserver::IMTEndOfDaySink) -> c_uint {
        0
    }

    fn trade_balance_check(
        &mut self,
        login: c_ulonglong,
        fixflag: c_uint,
        balance_user: &mut f64,
        balance_history: &mut f64,
        credit_user: &mut f64,
        credit_history: &mut f64,
    ) -> c_uint {
        0
    }

    fn trade_account_set(
        &mut self,
        user: &mt5_apiserver::IMTUser,
        account: &mt5_apiserver::IMTAccount,
        orders: &mt5_apiserver::IMTOrderArray,
        positions: &mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn trade_confirm_create(&mut self) -> *mut mt5_apiserver::IMTConfirm {
        std::ptr::null_mut()
    }

    fn trade_execution_create(&mut self) -> *mut mt5_apiserver::IMTExecution {
        std::ptr::null_mut()
    }

    fn trade_request_create_array(&mut self) -> *mut mt5_apiserver::IMTRequestArray {
        std::ptr::null_mut()
    }

    fn trade_profit_ext(
        &mut self,
        group: &[u16],
        symbol: &[u16],
        type_: c_uint,
        volume: c_ulonglong,
        price_open: f64,
        price_close: f64,
        profit: &mut f64,
        profit_rate: &mut f64,
    ) -> c_uint {
        0
    }

    fn book_subscribe(&mut self, sink: &mut mt5_apiserver::IMTBookSink) -> c_uint {
        0
    }

    fn book_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTBookSink) -> c_uint {
        0
    }

    fn book_get(&mut self, symbol: &[u16], book: &mut mt5_apiserver::MTBook) -> c_uint {
        0
    }

    fn book_reserved1(&mut self) -> c_uint {
        0
    }

    fn book_reserved2(&mut self) -> c_uint {
        0
    }

    fn book_reserved3(&mut self) -> c_uint {
        0
    }

    fn book_reserved4(&mut self) -> c_uint {
        0
    }

    fn chart_get(
        &mut self,
        symbol: &[u16],
        from: c_longlong,
        to: c_longlong,
        bars: &mut &mut mt5_apiserver::MTChartBar,
        bars_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn chart_delete(
        &mut self,
        symbol: &[u16],
        bars_dates: &[c_longlong],
    ) -> c_uint {
        0
    }

    fn chart_update(
        &mut self,
        symbol: &[u16],
        bars: &[mt5_apiserver::MTChartBar],
    ) -> c_uint {
        0
    }

    fn chart_split(
        &mut self,
        symbol: &[u16],
        new_shares: c_uint,
        old_shares: c_uint,
        rounding_rule: c_uint,
        datetime_from: c_longlong,
        datetime_to: c_longlong,
    ) -> c_uint {
        0
    }

    fn chart_reserved2(&mut self) -> c_uint {
        0
    }
    fn chart_reserved3(&mut self) -> c_uint {
        0
    }
    fn chart_reserved4(&mut self) -> c_uint {
        0
    }

    fn user_cert_create(&mut self) -> *mut mt5_apiserver::IMTCertificate {
        std::ptr::null_mut()
    }

    fn user_cert_update(
        &mut self,
        login: c_ulonglong,
        certificate: &mt5_apiserver::IMTCertificate,
    ) -> c_uint {
        0
    }

    fn user_cert_get(
        &mut self,
        login: c_ulonglong,
        certificate: &mut mt5_apiserver::IMTCertificate,
    ) -> c_uint {
        0
    }

    fn user_cert_reserved1(&mut self) -> c_uint {
        0
    }
    fn user_cert_reserved2(&mut self) -> c_uint {
        0
    }
    fn user_cert_reserved3(&mut self) -> c_uint {
        0
    }
    fn user_cert_reserved4(&mut self) -> c_uint {
        0
    }

    fn spread_create(&mut self) -> *mut mt5_apiserver::IMTConSpread {
        std::ptr::null_mut()
    }

    fn spread_leg_create(&mut self) -> *mut mt5_apiserver::IMTConSpreadLeg {
        std::ptr::null_mut()
    }

    fn spread_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConSpreadSink) -> c_uint {
        0
    }

    fn spread_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConSpreadSink) -> c_uint {
        0
    }

    fn spread_add(&mut self, spread: &mut mt5_apiserver::IMTConSpread) -> c_uint {
        0
    }

    fn spread_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn spread_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn spread_total(&mut self) -> c_uint {
        0
    }

    fn spread_next(&mut self, pos: c_uint, spread: &mut mt5_apiserver::IMTConSpread) -> c_uint {
        0
    }

    fn spread_get(&mut self, id: c_uint, spread: &mut mt5_apiserver::IMTConSpread) -> c_uint {
        0
    }

    fn spread_reserved1(&mut self) -> c_uint {
        0
    }

    fn spread_reserved2(&mut self) -> c_uint {
        0
    }

    fn spread_reserved3(&mut self) -> c_uint {
        0
    }

    fn spread_reserved4(&mut self) -> c_uint {
        0
    }

    fn online_create(&mut self) -> *mut mt5_apiserver::IMTOnline {
        std::ptr::null_mut()
    }

    fn online_create_array(&mut self) -> *mut mt5_apiserver::IMTOnlineArray {
        std::ptr::null_mut()
    }

    fn online_total(&mut self) -> c_uint {
        0
    }

    fn online_next(&mut self, pos: c_uint, online: &mut mt5_apiserver::IMTOnline) -> c_uint {
        0
    }

    fn online_get(
        &mut self,
        login: c_ulonglong,
        online: &mut mt5_apiserver::IMTOnlineArray,
    ) -> c_uint {
        0
    }

    fn online_disconnect(&mut self, online: &mut mt5_apiserver::IMTOnline) -> c_uint {
        0
    }

    fn online_disconnect_batch(
        &mut self,
        online: &mut mt5_apiserver::IMTOnlineArray,
        results: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn online_disconnect_batch_array(
        &mut self,
        online: &mut [&mut mt5_apiserver::IMTOnline],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn online_reserved4(&mut self) -> c_uint {
        0
    }

    fn notifications_send(
        &mut self,
        metaquotes_ids: &[u16],
        message: &[u16],
    ) -> c_uint {
        0
    }

    fn notifications_send1(
        &mut self,
        logins: &[c_ulonglong],
        message: &[u16],
    ) -> c_uint {
        0
    }

    fn notifications_reserved1(&mut self) -> c_uint {
        0
    }

    fn notifications_reserved2(&mut self) -> c_uint {
        0
    }

    fn notifications_reserved3(&mut self) -> c_uint {
        0
    }

    fn notifications_reserved4(&mut self) -> c_uint {
        0
    }

    fn deal_update_batch(
        &mut self,
        deals: &mut mt5_apiserver::IMTDealArray,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn deal_update_batch_array(
        &mut self,
        deals: &mut [&mut mt5_apiserver::IMTDeal],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn deal_add_batch(
        &mut self,
        deals: &mut mt5_apiserver::IMTDealArray,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn deal_add_batch_array(
        &mut self,
        deals: &mut [&mut mt5_apiserver::IMTDeal],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn deal_perform_batch(
        &mut self,
        deals: &mut mt5_apiserver::IMTDealArray,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn deal_perform_batch_array(
        &mut self,
        deals: &mut [&mut mt5_apiserver::IMTDeal],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn deal_select_by_group(
        &mut self,
        group: &[u16],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn deal_select_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn deal_get_by_group(
        &mut self,
        group: &[u16],
        from: c_longlong,
        to: c_longlong,
        deals: &mut mt5_apiserver::IMTDealArray,
    ) -> c_uint {
        0
    }

    fn deal_get_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        from: c_longlong,
        to: c_longlong,
        deals: &mut mt5_apiserver::IMTDealArray,
    ) -> c_uint {
        0
    }

    fn order_add_batch(
        &mut self,
        orders: &mut mt5_apiserver::IMTOrderArray,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn order_add_batch_array(
        &mut self,
        orders: &mut [&mut mt5_apiserver::IMTOrder],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn order_select_by_group(
        &mut self,
        group: &[u16],
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn order_select_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn order_get_by_group(
        &mut self,
        group: &[u16],
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn order_get_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn history_update_batch_array(
        &mut self,
        orders: &mut [&mut mt5_apiserver::IMTOrder],
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn history_add_batch(
        &mut self,
        orders: &mut mt5_apiserver::IMTOrderArray,
        results: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn history_add_batch_array(
        &mut self,
        orders: &mut &mut mt5_apiserver::IMTOrder,
        orders_total: c_uint,
        results: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn history_select_by_group(
        &mut self,
        group: &[u16],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn history_select_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn history_get_by_group(
        &mut self,
        group: &[u16],
        from: c_longlong,
        to: c_longlong,
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn history_get_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        from: c_longlong,
        to: c_longlong,
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn dealer_start(
        &mut self,
        dealer: c_ulonglong,
        sink: &mut mt5_apiserver::IMTRequestSink,
    ) -> c_uint {
        0
    }

    fn dealer_stop(
        &mut self,
        dealer: c_ulonglong,
        sink: &mut mt5_apiserver::IMTRequestSink,
    ) -> c_uint {
        0
    }

    fn dealer_get(
        &mut self,
        dealer: c_ulonglong,
        request: &mut mt5_apiserver::IMTRequest,
    ) -> c_uint {
        0
    }

    fn dealer_lock(
        &mut self,
        dealer: c_ulonglong,
        id: c_uint,
        request: &mut mt5_apiserver::IMTRequest,
    ) -> c_uint {
        0
    }

    fn dealer_answer(
        &mut self,
        dealer: c_ulonglong,
        confirm: &mut mt5_apiserver::IMTConfirm,
    ) -> c_uint {
        0
    }

    fn dealer_request_total(&mut self, dealer: c_ulonglong) -> c_uint {
        0
    }

    fn dealer_request_next(
        &mut self,
        dealer: c_ulonglong,
        pos: c_uint,
        request: &mut mt5_apiserver::IMTRequest,
    ) -> c_uint {
        0
    }

    fn dealer_request_get(
        &mut self,
        dealer: c_ulonglong,
        id: c_uint,
        request: &mut mt5_apiserver::IMTRequest,
    ) -> c_uint {
        0
    }

    fn dealer_request_get_all(
        &mut self,
        dealer: c_ulonglong,
        requests: &mut mt5_apiserver::IMTRequestArray,
    ) -> c_uint {
        0
    }

    fn dealer_execution(
        &mut self,
        gateway_name: &[u16],
        gateway_type: &[u16],
        execution: &mut mt5_apiserver::IMTExecution,
    ) -> c_uint {
        0
    }

    fn dealer_reserved2(&mut self) -> c_uint {
        0
    }

    fn dealer_reserved3(&mut self) -> c_uint {
        0
    }

    fn dealer_reserved4(&mut self) -> c_uint {
        0
    }

    fn trade_margin_check_ext(
        &mut self,
        login: c_ulonglong,
        symbol: &[u16],
        type_: c_uint,
        volume: c_ulonglong,
        price: f64,
        account_new: &mut mt5_apiserver::IMTAccount,
        account_current: &mut mt5_apiserver::IMTAccount,
    ) -> c_uint {
        0
    }

    fn trade_reserved1(&mut self) -> c_uint {
        0
    }

    fn trade_reserved2(&mut self) -> c_uint {
        0
    }

    fn trade_reserved3(&mut self) -> c_uint {
        0
    }

    fn trade_reserved4(&mut self) -> c_uint {
        0
    }

    fn trade_reserved5(&mut self) -> c_uint {
        0
    }

    fn trade_reserved6(&mut self) -> c_uint {
        0
    }

    fn email_create(&mut self) -> *mut mt5_apiserver::IMTConEmail {
        std::ptr::null_mut()
    }

    fn email_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConEmailSink) -> c_uint {
        0
    }

    fn email_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConEmailSink) -> c_uint {
        0
    }

    fn email_add(&mut self, config: &mut mt5_apiserver::IMTConEmail) -> c_uint {
        0
    }

    fn email_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn email_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn email_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn email_total(&mut self) -> c_uint {
        0
    }

    fn email_next(&mut self, pos: c_uint, email: &mut mt5_apiserver::IMTConEmail) -> c_uint {
        0
    }

    fn email_get(&mut self, name: &[u16], email: &mut mt5_apiserver::IMTConEmail) -> c_uint {
        0
    }

    fn email_send(
        &mut self,
        account: &[u16],
        to: &[u16],
        to_name: &[u16],
        subject: &[u16],
        body: &[u16],
    ) -> c_uint {
        0
    }

    fn email_reserved2(&mut self) -> c_uint {
        0
    }

    fn email_reserved3(&mut self) -> c_uint {
        0
    }

    fn email_reserved4(&mut self) -> c_uint {
        0
    }

    fn messenger_create(&mut self) -> *mut mt5_apiserver::IMTConMessenger {
        std::ptr::null_mut()
    }

    fn messenger_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConMessengerSink) -> c_uint {
        0
    }

    fn messenger_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConMessengerSink) -> c_uint {
        0
    }

    fn messenger_add(&mut self, config: &mut mt5_apiserver::IMTConMessenger) -> c_uint {
        0
    }

    fn messenger_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn messenger_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn messenger_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn messenger_total(&mut self) -> c_uint {
        0
    }

    fn messenger_next(
        &mut self,
        pos: c_uint,
        messenger: &mut mt5_apiserver::IMTConMessenger,
    ) -> c_uint {
        0
    }

    fn messenger_get(
        &mut self,
        name: &[u16],
        messenger: &mut mt5_apiserver::IMTConMessenger,
    ) -> c_uint {
        0
    }

    fn messenger_send(
        &mut self,
        destination: &[u16],
        group: &[u16],
        sender: &[u16],
        text: &[u16],
    ) -> c_uint {
        0
    }

    fn messenger_verify_phone(&mut self, phone_number: &[u16]) -> c_uint {
        0
    }

    fn messenger_country_create(&mut self) -> *mut mt5_apiserver::IMTConMessengerCountry {
        std::ptr::null_mut()
    }

    fn messenger_group_create(&mut self) -> *mut mt5_apiserver::IMTConMessengerGroup {
        std::ptr::null_mut()
    }

    fn position_select_by_group(
        &mut self,
        group: &[u16],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn position_select_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        from: c_longlong,
        to: c_longlong,
        request: &mt5_apiserver::IMTDatasetRequest,
        dataset: &mut mt5_apiserver::IMTDataset,
    ) -> c_uint {
        0
    }

    fn position_get_by_group(
        &mut self,
        group: &[u16],
        positions: &mut mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn position_get_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        positions: &mut mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn position_get_by_tickets(
        &mut self,
        tickets: &[c_ulonglong],
        positions: &mut mt5_apiserver::IMTPositionArray,
    ) -> c_uint {
        0
    }

    fn position_split(
        &mut self,
        tickets: &[c_ulonglong],
        adjustments: &[f64],
        new_shares: c_uint,
        old_shares: c_uint,
        round_rule_price: c_uint,
        round_rule_volume: c_uint,
        flags: c_uint,
        results: &mut [c_uint],
    ) -> c_uint {
        0
    }

    fn position_reserved2(&mut self) -> c_uint {
        0
    }
    fn position_reserved3(&mut self) -> c_uint {
        0
    }
    fn position_reserved4(&mut self) -> c_uint {
        0
    }

    fn dataset_request_create(&mut self) -> *mut mt5_apiserver::IMTDatasetRequest {
        std::ptr::null_mut()
    }

    fn dataset_create(&mut self) -> *mut mt5_apiserver::IMTDataset {
        std::ptr::null_mut()
    }

    fn dataset_reserved1(&mut self) -> c_uint {
        0
    }

    fn dataset_reserved2(&mut self) -> c_uint {
        0
    }

    fn dataset_reserved3(&mut self) -> c_uint {
        0
    }

    fn dataset_reserved4(&mut self) -> c_uint {
        0
    }

    fn order_get_by_tickets(
        &mut self,
        tickets: &[c_ulonglong],
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn order_reserved1(&mut self) -> c_uint {
        0
    }

    fn order_reserved2(&mut self) -> c_uint {
        0
    }

    fn order_reserved3(&mut self) -> c_uint {
        0
    }

    fn order_reserved4(&mut self) -> c_uint {
        0
    }

    fn history_get_by_tickets(
        &mut self,
        tickets: &[c_ulonglong],
        orders: &mut mt5_apiserver::IMTOrderArray,
    ) -> c_uint {
        0
    }

    fn history_reserved1(&mut self) -> c_uint {
        0
    }

    fn history_reserved2(&mut self) -> c_uint {
        0
    }

    fn history_reserved3(&mut self) -> c_uint {
        0
    }

    fn history_reserved4(&mut self) -> c_uint {
        0
    }

    fn deal_get_by_tickets(
        &mut self,
        tickets: &[c_ulonglong],
        deals: &mut mt5_apiserver::IMTDealArray,
    ) -> c_uint {
        0
    }

    fn deal_reserved1(&mut self) -> c_uint {
        0
    }

    fn deal_reserved2(&mut self) -> c_uint {
        0
    }

    fn deal_reserved3(&mut self) -> c_uint {
        0
    }

    fn deal_reserved4(&mut self) -> c_uint {
        0
    }

    fn client_create(&mut self) -> *mut mt5_apiserver::IMTClient {
        std::ptr::null_mut()
    }

    fn client_create_array(&mut self) -> *mut mt5_apiserver::IMTClientArray {
        std::ptr::null_mut()
    }

    fn client_subscribe(&mut self, sink: &mut mt5_apiserver::IMTClientSink) -> c_uint {
        0
    }

    fn client_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTClientSink) -> c_uint {
        0
    }

    fn client_add(
        &mut self,
        client: &mut mt5_apiserver::IMTClient,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn client_update(
        &mut self,
        client: &mut mt5_apiserver::IMTClient,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn client_delete(
        &mut self,
        client_id: c_ulonglong,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn client_get(
        &mut self,
        client_id: c_ulonglong,
        client: &mut mt5_apiserver::IMTClient,
    ) -> c_uint {
        0
    }

    fn client_get_history(
        &mut self,
        client_id: c_ulonglong,
        author: c_ulonglong,
        from: c_longlong,
        to: c_longlong,
        history: &mut mt5_apiserver::IMTClientArray,
    ) -> c_uint {
        0
    }

    fn client_ids_all(
        &mut self,
        ids: &mut &mut c_ulonglong,
        ids_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn client_ids_by_group(
        &mut self,
        groups: &[u16],
        ids: &mut &mut c_ulonglong,
        ids_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn client_ids_by_manager(
        &mut self,
        manager: c_ulonglong,
        ids: &mut &mut c_ulonglong,
        ids_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn client_user_add(
        &mut self,
        client_id: c_ulonglong,
        login: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn client_user_delete(
        &mut self,
        client_id: c_ulonglong,
        login: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn client_user_logins(
        &mut self,
        client_id: c_ulonglong,
        logins: &mut &mut c_ulonglong,
        logins_total: &mut c_uint,
    ) -> c_uint {
        0
    }

    fn client_reserved1(&mut self) -> c_uint {
        0
    }

    fn client_reserved2(&mut self) -> c_uint {
        0
    }

    fn client_reserved3(&mut self) -> c_uint {
        0
    }

    fn client_reserved4(&mut self) -> c_uint {
        0
    }

    fn document_create(&mut self) -> *mut mt5_apiserver::IMTDocument {
        std::ptr::null_mut()
    }

    fn document_create_array(&mut self) -> *mut mt5_apiserver::IMTDocumentArray {
        std::ptr::null_mut()
    }

    fn document_subscribe(&mut self, sink: &mut mt5_apiserver::IMTDocumentSink) -> c_uint {
        0
    }

    fn document_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTDocumentSink) -> c_uint {
        0
    }

    fn document_add(
        &mut self,
        document: &mut mt5_apiserver::IMTDocument,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn document_update(
        &mut self,
        document: &mut mt5_apiserver::IMTDocument,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn document_delete(
        &mut self,
        document_id: c_ulonglong,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn document_get(
        &mut self,
        document_id: c_ulonglong,
        document: &mut mt5_apiserver::IMTDocument,
    ) -> c_uint {
        0
    }

    fn document_get_by_client(
        &mut self,
        client_id: c_ulonglong,
        position: c_uint,
        total: c_uint,
        documents: &mut mt5_apiserver::IMTDocumentArray,
    ) -> c_uint {
        0
    }

    fn document_get_history(
        &mut self,
        document_id: c_ulonglong,
        author: c_ulonglong,
        from: c_longlong,
        to: c_longlong,
        history: &mut mt5_apiserver::IMTDocumentArray,
    ) -> c_uint {
        0
    }

    fn document_reserved1(&mut self) -> c_uint {
        0
    }

    fn document_reserved2(&mut self) -> c_uint {
        0
    }

    fn document_reserved3(&mut self) -> c_uint {
        0
    }

    fn document_reserved4(&mut self) -> c_uint {
        0
    }

    fn comment_create(&mut self) -> *mut mt5_apiserver::IMTComment {
        std::ptr::null_mut()
    }

    fn comment_create_array(&mut self) -> *mut mt5_apiserver::IMTCommentArray {
        std::ptr::null_mut()
    }

    fn comment_subscribe(&mut self, sink: &mut mt5_apiserver::IMTCommentSink) -> c_uint {
        0
    }

    fn comment_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTCommentSink) -> c_uint {
        0
    }

    fn comment_add(
        &mut self,
        comment: &mut mt5_apiserver::IMTComment,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn comment_update(
        &mut self,
        comment: &mut mt5_apiserver::IMTComment,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn comment_delete(
        &mut self,
        comment_id: c_ulonglong,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn comment_get(
        &mut self,
        comment_id: c_ulonglong,
        comment: &mut mt5_apiserver::IMTComment,
    ) -> c_uint {
        0
    }

    fn comment_get_by_client(
        &mut self,
        client_id: c_ulonglong,
        position: c_uint,
        total: c_uint,
        comments: &mut mt5_apiserver::IMTCommentArray,
    ) -> c_uint {
        0
    }

    fn comment_get_by_document(
        &mut self,
        document_id: c_ulonglong,
        position: c_uint,
        total: c_uint,
        comments: &mut mt5_apiserver::IMTCommentArray,
    ) -> c_uint {
        0
    }

    fn comment_reserved1(&mut self) -> c_uint {
        0
    }

    fn comment_reserved2(&mut self) -> c_uint {
        0
    }

    fn comment_reserved3(&mut self) -> c_uint {
        0
    }

    fn comment_reserved4(&mut self) -> c_uint {
        0
    }

    fn attachment_create(&mut self) -> *mut mt5_apiserver::IMTAttachment {
        std::ptr::null_mut()
    }

    fn attachment_add(
        &mut self,
        attachment: &mut mt5_apiserver::IMTAttachment,
        author: c_ulonglong,
    ) -> c_uint {
        0
    }

    fn attachment_get(
        &mut self,
        attachment_id: c_ulonglong,
        attachment: &mut mt5_apiserver::IMTAttachment,
    ) -> c_uint {
        0
    }

    fn attachment_reserved1(&mut self) -> c_uint {
        0
    }

    fn attachment_reserved2(&mut self) -> c_uint {
        0
    }

    fn attachment_reserved3(&mut self) -> c_uint {
        0
    }

    fn attachment_reserved4(&mut self) -> c_uint {
        0
    }

    fn tls_certificate_update(
        &mut self,
        pfx_certificate: &[c_void],
        password: &[u16],
    ) -> c_uint {
        0
    }

    fn tls_certificate_delete(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn tls_certificate_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn tls_certificate_total(&mut self) -> c_uint {
        0
    }

    fn tls_certificate_next(
        &mut self,
        pos: c_uint,
        name: &mut [u16; 260usize],
        thumbprint: &mut [u16; 260usize],
    ) -> c_uint {
        0
    }

    fn tls_certificate_pfx(
        &mut self,
        pos: c_uint,
        pfx_certificate_size: &mut c_uint,
    ) -> *mut c_void {
        std::ptr::null_mut()
    }

    fn tls_certificate_reserved1(&mut self) -> c_uint {
        0
    }

    fn tls_certificate_reserved2(&mut self) -> c_uint {
        0
    }

    fn tls_certificate_reserved3(&mut self) -> c_uint {
        0
    }

    fn tls_certificate_reserved4(&mut self) -> c_uint {
        0
    }

    fn automation_create(&mut self) -> *mut mt5_apiserver::IMTConAutomation {
        std::ptr::null_mut()
    }

    fn automation_condition_create(&mut self) -> *mut mt5_apiserver::IMTConAutoCondition {
        std::ptr::null_mut()
    }

    fn automation_action_create(&mut self) -> *mut mt5_apiserver::IMTConAutoAction {
        std::ptr::null_mut()
    }

    fn automation_param_create(&mut self) -> *mut mt5_apiserver::IMTConAutoParam {
        std::ptr::null_mut()
    }

    fn automation_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConAutomationSink) -> c_uint {
        0
    }

    fn automation_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConAutomationSink,
    ) -> c_uint {
        0
    }

    fn automation_add(&mut self, config: &mut mt5_apiserver::IMTConAutomation) -> c_uint {
        0
    }

    fn automation_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn automation_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn automation_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn automation_total(&mut self) -> c_uint {
        0
    }

    fn automation_next(
        &mut self,
        pos: c_uint,
        config: &mut mt5_apiserver::IMTConAutomation,
    ) -> c_uint {
        0
    }

    fn automation_get(
        &mut self,
        name: &[u16],
        config: &mut mt5_apiserver::IMTConAutomation,
    ) -> c_uint {
        0
    }

    fn automation_trigger(
        &mut self,
        name: &[u16],
        user: &mt5_apiserver::IMTUser,
        account: &mt5_apiserver::IMTAccount,
        deal: &mt5_apiserver::IMTDeal,
        order: &mt5_apiserver::IMTOrder,
        position: &mt5_apiserver::IMTPosition,
    ) -> c_uint {
        0
    }

    fn automation_reserved2(&mut self) -> c_uint {
        0
    }

    fn automation_reserved3(&mut self) -> c_uint {
        0
    }

    fn automation_reserved4(&mut self) -> c_uint {
        0
    }

    fn subscription_cfg_create(&mut self) -> *mut mt5_apiserver::IMTConSubscription {
        std::ptr::null_mut()
    }

    fn subscription_cfg_symbol_create(&mut self) -> *mut mt5_apiserver::IMTConSubscriptionSymbol {
        std::ptr::null_mut()
    }

    fn subscription_cfg_news_create(&mut self) -> *mut mt5_apiserver::IMTConSubscriptionNews {
        std::ptr::null_mut()
    }

    fn subscription_cfg_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConSubscriptionSink,
    ) -> c_uint {
        0
    }

    fn subscription_cfg_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTConSubscriptionSink,
    ) -> c_uint {
        0
    }

    fn subscription_cfg_add(&mut self, config: &mut mt5_apiserver::IMTConSubscription) -> c_uint {
        0
    }

    fn subscription_cfg_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn subscription_cfg_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn subscription_cfg_delete_by_id(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn subscription_cfg_shift(
        &mut self,
        pos: c_uint,
        shift: c_int,
    ) -> c_uint {
        0
    }

    fn subscription_cfg_total(&mut self) -> c_uint {
        0
    }

    fn subscription_cfg_next(
        &mut self,
        pos: c_uint,
        config: &mut mt5_apiserver::IMTConSubscription,
    ) -> c_uint {
        0
    }

    fn subscription_cfg_get(
        &mut self,
        name: &[u16],
        config: &mut mt5_apiserver::IMTConSubscription,
    ) -> c_uint {
        0
    }

    fn subscription_cfg_get_by_id(
        &mut self,
        id: c_ulonglong,
        config: &mut mt5_apiserver::IMTConSubscription,
    ) -> c_uint {
        0
    }

    fn subscription_cfg_reserved1(&mut self) -> c_uint {
        0
    }

    fn subscription_cfg_reserved2(&mut self) -> c_uint {
        0
    }

    fn subscription_cfg_reserved3(&mut self) -> c_uint {
        0
    }

    fn subscription_cfg_reserved4(&mut self) -> c_uint {
        0
    }

    fn subscription_create(&mut self) -> *mut mt5_apiserver::IMTSubscription {
        std::ptr::null_mut()
    }

    fn subscription_create_array(&mut self) -> *mut mt5_apiserver::IMTSubscriptionArray {
        std::ptr::null_mut()
    }

    fn subscription_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTSubscriptionSink,
    ) -> c_uint {
        0
    }

    fn subscription_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTSubscriptionSink,
    ) -> c_uint {
        0
    }

    fn subscription_join(
        &mut self,
        manager: c_ulonglong,
        login: c_ulonglong,
        subscription: c_ulonglong,
        record: &mut mt5_apiserver::IMTSubscription,
        history: &mut mt5_apiserver::IMTSubscriptionHistory,
    ) -> c_uint {
        0
    }

    fn subscription_cancel(
        &mut self,
        manager: c_ulonglong,
        login: c_ulonglong,
        subscription: c_ulonglong,
        record: &mut mt5_apiserver::IMTSubscription,
        history: &mut mt5_apiserver::IMTSubscriptionHistory,
    ) -> c_uint {
        0
    }

    fn subscription_exist(
        &mut self,
        login: c_ulonglong,
        subscription: c_ulonglong,
    ) -> bool {
        false
    }

    fn subscription_add(&mut self, record: &mut mt5_apiserver::IMTSubscription) -> c_uint {
        0
    }

    fn subscription_update(&mut self, record: &mut mt5_apiserver::IMTSubscription) -> c_uint {
        0
    }

    fn subscription_delete(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn subscription_get(
        &mut self,
        login: c_ulonglong,
        records: &mut mt5_apiserver::IMTSubscriptionArray,
    ) -> c_uint {
        0
    }

    fn subscription_get_by_subscription(
        &mut self,
        login: c_ulonglong,
        subscription: c_ulonglong,
        record: &mut mt5_apiserver::IMTSubscription,
    ) -> c_uint {
        0
    }

    fn subscription_get_by_id(
        &mut self,
        id: c_ulonglong,
        record: &mut mt5_apiserver::IMTSubscription,
    ) -> c_uint {
        0
    }

    fn subscription_get_by_logins(
        &mut self,
        logins: &[c_ulonglong],
        records: &mut mt5_apiserver::IMTSubscriptionArray,
    ) -> c_uint {
        0
    }

    fn subscription_reserved1(&mut self) -> c_uint {
        0
    }

    fn subscription_reserved2(&mut self) -> c_uint {
        0
    }

    fn subscription_reserved3(&mut self) -> c_uint {
        0
    }

    fn subscription_reserved4(&mut self) -> c_uint {
        0
    }

    fn subscription_history_create(&mut self) -> *mut mt5_apiserver::IMTSubscriptionHistory {
        std::ptr::null_mut()
    }

    fn subscription_history_create_array(
        &mut self,
    ) -> *mut mt5_apiserver::IMTSubscriptionHistoryArray {
        std::ptr::null_mut()
    }

    fn subscription_history_subscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTSubscriptionHistorySink,
    ) -> c_uint {
        0
    }

    fn subscription_history_unsubscribe(
        &mut self,
        sink: &mut mt5_apiserver::IMTSubscriptionHistorySink,
    ) -> c_uint {
        0
    }

    fn subscription_history_add(
        &mut self,
        record: &mut mt5_apiserver::IMTSubscriptionHistory,
    ) -> c_uint {
        0
    }

    fn subscription_history_update(
        &mut self,
        record: &mut mt5_apiserver::IMTSubscriptionHistory,
    ) -> c_uint {
        0
    }

    fn subscription_history_delete(&mut self, id: c_ulonglong) -> c_uint {
        0
    }

    fn subscription_history_get(
        &mut self,
        from: c_longlong,
        to: c_longlong,
        login: c_ulonglong,
        records: &mut mt5_apiserver::IMTSubscriptionHistoryArray,
    ) -> c_uint {
        0
    }

    fn subscription_history_get_by_id(
        &mut self,
        id: c_ulonglong,
        record: &mut mt5_apiserver::IMTSubscriptionHistory,
    ) -> c_uint {
        0
    }

    fn subscription_history_get_by_logins(
        &mut self,
        from: c_longlong,
        to: c_longlong,
        logins: *const c_ulonglong,
        logins_total: c_uint,
        records: &mut mt5_apiserver::IMTSubscriptionHistoryArray,
    ) -> c_uint {
        0
    }

    fn subscription_history_reserved1(&mut self) -> c_uint {
        0
    }

    fn subscription_history_reserved2(&mut self) -> c_uint {
        0
    }

    fn subscription_history_reserved3(&mut self) -> c_uint {
        0
    }

    fn subscription_history_reserved4(&mut self) -> c_uint {
        0
    }

    fn geo_resolve_any(
        &mut self,
        ip_list: &[u16],
        result: &mut [u16],
        flags: c_uint,
    ) -> c_uint {
        0
    }

    fn geo_resolve_ipv4(
        &mut self,
        ip: c_ulong,
        result: &mut [u16],
        flags: c_uint,
    ) -> c_uint {
        0
    }

    fn geo_resolve_ipv4_bulk(
        &mut self,
        ip_list: &[c_ulong],
        result: &mut [u16],
        flags: c_uint,
    ) -> c_uint {
        0
    }

    fn geo_resolve_ipv6(
        &mut self,
        ip: &mt5_apiserver::in6_addr,
        result: &mut [u16],
        flags: c_uint,
    ) -> c_uint {
        0
    }

    fn geo_resolve_ipv6_bulk(
        &mut self,
        ip_list: &[mt5_apiserver::in6_addr],
        result: &mut [u16],
        flags: c_uint,
    ) -> c_uint {
        0
    }

    fn geo_resolve_reserved1(&mut self) -> c_uint {
        0
    }

    fn geo_resolve_reserved2(&mut self) -> c_uint {
        0
    }

    fn geo_resolve_reserved3(&mut self) -> c_uint {
        0
    }

    fn geo_resolve_reserved4(&mut self) -> c_uint {
        0
    }

    fn vps_create(&mut self) -> *mut mt5_apiserver::IMTConVPS {
        std::ptr::null_mut()
    }

    fn vps_create_group(&mut self) -> *mut mt5_apiserver::IMTConVPSGroup {
        std::ptr::null_mut()
    }

    fn vps_subscribe(&mut self, sink: &mut mt5_apiserver::IMTConVPSSink) -> c_uint {
        0
    }

    fn vps_unsubscribe(&mut self, sink: &mut mt5_apiserver::IMTConVPSSink) -> c_uint {
        0
    }

    fn vps_get(&mut self, config: &mut mt5_apiserver::IMTConVPS) -> c_uint {
        0
    }

    fn vps_set(&mut self, config: *const mt5_apiserver::IMTConVPS) -> c_uint {
        0
    }

    fn vps_reserved1(&mut self) -> c_uint {
        0
    }

    fn vps_reserved2(&mut self) -> c_uint {
        0
    }

    fn vps_reserved3(&mut self) -> c_uint {
        0
    }

    fn vps_reserved4(&mut self) -> c_uint {
        0
    }

    fn kyc_create(&mut self) -> *mut mt5_apiserver::IMTConKYC {
        std::ptr::null_mut()
    }

    fn kyc_country_create(&mut self) -> *mut mt5_apiserver::IMTConKYCCountry {
        std::ptr::null_mut()
    }

    fn kyc_group_create(&mut self) -> *mut mt5_apiserver::IMTConKYCGroup {
        std::ptr::null_mut()
    }

    fn kyc_subscribe(&mut self, sink: *mut mt5_apiserver::IMTConKYCSink) -> c_uint {
        0
    }

    fn kyc_unsubscribe(&mut self, sink: *mut mt5_apiserver::IMTConKYCSink) -> c_uint {
        0
    }

    fn kyc_add(&mut self, config: &mut mt5_apiserver::IMTConKYC) -> c_uint {
        0
    }

    fn kyc_delete(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn kyc_delete1(&mut self, pos: c_uint) -> c_uint {
        0
    }

    fn kyc_shift(&mut self, pos: c_uint, shift: c_int) -> c_uint {
        0
    }

    fn kyc_total(&mut self) -> c_uint {
        0
    }

    fn kyc_next(&mut self, pos: c_uint, kyc: &mut mt5_apiserver::IMTConKYC) -> c_uint {
        0
    }

    fn kyc_get(&mut self, name: &[u16], kyc: &mut mt5_apiserver::IMTConKYC) -> c_uint {
        0
    }

    fn kyc_start(&mut self, client_id: c_ulonglong) -> c_uint {
        0
    }

    fn kyc_reserved1(&mut self) -> c_uint {
        0
    }

    fn kyc_reserved2(&mut self) -> c_uint {
        0
    }

    fn kyc_reserved3(&mut self) -> c_uint {
        0
    }

    fn kyc_reserved4(&mut self) -> c_uint {
        0
    }
}
