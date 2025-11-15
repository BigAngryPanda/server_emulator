pub const NULL: u32 = 0;
pub type UINT = ::std::os::raw::c_uint;
pub type MTAPIRES = UINT;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type PUINT64 = *mut ::std::os::raw::c_ulonglong;
pub type WCHAR = u16;
pub type LPCWSTR = *const WCHAR;
pub type PCWSTR = *const WCHAR;
pub type INT = ::std::os::raw::c_int;
pub type PUINT = *mut ::std::os::raw::c_uint;
pub type INT8 = ::std::os::raw::c_schar;
pub type PINT8 = *mut ::std::os::raw::c_schar;
pub type INT16 = ::std::os::raw::c_short;
pub type PINT16 = *mut ::std::os::raw::c_short;
pub type INT32 = ::std::os::raw::c_int;
pub type PINT32 = *mut ::std::os::raw::c_int;
pub type INT64 = ::std::os::raw::c_longlong;
pub type PINT64 = *mut ::std::os::raw::c_longlong;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type PUINT8 = *mut ::std::os::raw::c_uchar;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type PUINT16 = *mut ::std::os::raw::c_ushort;
pub type UINT32 = ::std::os::raw::c_uint;
pub type PUINT32 = *mut ::std::os::raw::c_uint;
pub type MTAPISTR = [u16; 260usize];
pub type DWORD = ::std::os::raw::c_ulong;
pub type COLORREF = DWORD;
pub type USHORT = ::std::os::raw::c_ushort;
pub type UCHAR = ::std::os::raw::c_uchar;
pub type MTSortFunctionPtr = ::std::option::Option<
    unsafe extern "C" fn(
        left: *const ::std::os::raw::c_void,
        right: *const ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type LPCVOID = *const ::std::os::raw::c_void;
pub type LPVOID = *mut ::std::os::raw::c_void;
pub type SHORT = ::std::os::raw::c_short;
pub type NWPSTR = *mut WCHAR;
pub type LPWSTR = *mut WCHAR;
pub type PWSTR = *mut WCHAR;
pub type ULONG = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub u: in6_addr__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union in6_addr__bindgen_ty_1 {
    pub Byte: [UCHAR; 16usize],
    pub Word: [USHORT; 8usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of in6_addr__bindgen_ty_1"][::std::mem::size_of::<in6_addr__bindgen_ty_1>() - 16usize];
    ["Alignment of in6_addr__bindgen_ty_1"]
        [::std::mem::align_of::<in6_addr__bindgen_ty_1>() - 2usize];
    ["Offset of field: in6_addr__bindgen_ty_1::Byte"]
        [::std::mem::offset_of!(in6_addr__bindgen_ty_1, Byte) - 0usize];
    ["Offset of field: in6_addr__bindgen_ty_1::Word"]
        [::std::mem::offset_of!(in6_addr__bindgen_ty_1, Word) - 0usize];
};
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of in6_addr"][::std::mem::size_of::<in6_addr>() - 16usize];
    ["Alignment of in6_addr"][::std::mem::align_of::<in6_addr>() - 2usize];
    ["Offset of field: in6_addr::u"][::std::mem::offset_of!(in6_addr, u) - 0usize];
};
pub type IN6_ADDR = in6_addr;
pub const EnMTAPIRetcode_MT_RET_OK: EnMTAPIRetcode = 0;
pub const EnMTAPIRetcode_MT_RET_OK_NONE: EnMTAPIRetcode = 1;
pub const EnMTAPIRetcode_MT_RET_ERROR: EnMTAPIRetcode = 2;
pub const EnMTAPIRetcode_MT_RET_ERR_PARAMS: EnMTAPIRetcode = 3;
pub const EnMTAPIRetcode_MT_RET_ERR_DATA: EnMTAPIRetcode = 4;
pub const EnMTAPIRetcode_MT_RET_ERR_DISK: EnMTAPIRetcode = 5;
pub const EnMTAPIRetcode_MT_RET_ERR_MEM: EnMTAPIRetcode = 6;
pub const EnMTAPIRetcode_MT_RET_ERR_NETWORK: EnMTAPIRetcode = 7;
pub const EnMTAPIRetcode_MT_RET_ERR_PERMISSIONS: EnMTAPIRetcode = 8;
pub const EnMTAPIRetcode_MT_RET_ERR_TIMEOUT: EnMTAPIRetcode = 9;
pub const EnMTAPIRetcode_MT_RET_ERR_CONNECTION: EnMTAPIRetcode = 10;
pub const EnMTAPIRetcode_MT_RET_ERR_NOSERVICE: EnMTAPIRetcode = 11;
pub const EnMTAPIRetcode_MT_RET_ERR_FREQUENT: EnMTAPIRetcode = 12;
pub const EnMTAPIRetcode_MT_RET_ERR_NOTFOUND: EnMTAPIRetcode = 13;
pub const EnMTAPIRetcode_MT_RET_ERR_PARTIAL: EnMTAPIRetcode = 14;
pub const EnMTAPIRetcode_MT_RET_ERR_SHUTDOWN: EnMTAPIRetcode = 15;
pub const EnMTAPIRetcode_MT_RET_ERR_CANCEL: EnMTAPIRetcode = 16;
pub const EnMTAPIRetcode_MT_RET_ERR_DUPLICATE: EnMTAPIRetcode = 17;
pub const EnMTAPIRetcode_MT_RET_AUTH_CLIENT_INVALID: EnMTAPIRetcode = 1000;
pub const EnMTAPIRetcode_MT_RET_AUTH_ACCOUNT_INVALID: EnMTAPIRetcode = 1001;
pub const EnMTAPIRetcode_MT_RET_AUTH_ACCOUNT_DISABLED: EnMTAPIRetcode = 1002;
pub const EnMTAPIRetcode_MT_RET_AUTH_ADVANCED: EnMTAPIRetcode = 1003;
pub const EnMTAPIRetcode_MT_RET_AUTH_CERTIFICATE: EnMTAPIRetcode = 1004;
pub const EnMTAPIRetcode_MT_RET_AUTH_CERTIFICATE_BAD: EnMTAPIRetcode = 1005;
pub const EnMTAPIRetcode_MT_RET_AUTH_NOTCONFIRMED: EnMTAPIRetcode = 1006;
pub const EnMTAPIRetcode_MT_RET_AUTH_SERVER_INTERNAL: EnMTAPIRetcode = 1007;
pub const EnMTAPIRetcode_MT_RET_AUTH_SERVER_BAD: EnMTAPIRetcode = 1008;
pub const EnMTAPIRetcode_MT_RET_AUTH_UPDATE_ONLY: EnMTAPIRetcode = 1009;
pub const EnMTAPIRetcode_MT_RET_AUTH_CLIENT_OLD: EnMTAPIRetcode = 1010;
pub const EnMTAPIRetcode_MT_RET_AUTH_MANAGER_NOCONFIG: EnMTAPIRetcode = 1011;
pub const EnMTAPIRetcode_MT_RET_AUTH_MANAGER_IPBLOCK: EnMTAPIRetcode = 1012;
pub const EnMTAPIRetcode_MT_RET_AUTH_GROUP_INVALID: EnMTAPIRetcode = 1013;
pub const EnMTAPIRetcode_MT_RET_AUTH_CA_DISABLED: EnMTAPIRetcode = 1014;
pub const EnMTAPIRetcode_MT_RET_AUTH_INVALID_ID: EnMTAPIRetcode = 1015;
pub const EnMTAPIRetcode_MT_RET_AUTH_INVALID_IP: EnMTAPIRetcode = 1016;
pub const EnMTAPIRetcode_MT_RET_AUTH_INVALID_TYPE: EnMTAPIRetcode = 1017;
pub const EnMTAPIRetcode_MT_RET_AUTH_SERVER_BUSY: EnMTAPIRetcode = 1018;
pub const EnMTAPIRetcode_MT_RET_AUTH_SERVER_CERT: EnMTAPIRetcode = 1019;
pub const EnMTAPIRetcode_MT_RET_AUTH_ACCOUNT_UNKNOWN: EnMTAPIRetcode = 1020;
pub const EnMTAPIRetcode_MT_RET_AUTH_SERVER_OLD: EnMTAPIRetcode = 1021;
pub const EnMTAPIRetcode_MT_RET_AUTH_SERVER_LIMIT: EnMTAPIRetcode = 1022;
pub const EnMTAPIRetcode_MT_RET_AUTH_MOBILE_DISABLED: EnMTAPIRetcode = 1023;
pub const EnMTAPIRetcode_MT_RET_AUTH_MANAGER_TYPE: EnMTAPIRetcode = 1024;
pub const EnMTAPIRetcode_MT_RET_AUTH_DEMO_DISABLED: EnMTAPIRetcode = 1025;
pub const EnMTAPIRetcode_MT_RET_AUTH_RESET_PASSWORD: EnMTAPIRetcode = 1026;
pub const EnMTAPIRetcode_MT_RET_AUTH_OTP_INVALID: EnMTAPIRetcode = 1027;
pub const EnMTAPIRetcode_MT_RET_AUTH_OTP_NEED_SECRET: EnMTAPIRetcode = 1028;
pub const EnMTAPIRetcode_MT_RET_AUTH_MIGRATION_MT4: EnMTAPIRetcode = 1029;
pub const EnMTAPIRetcode_MT_RET_AUTH_MIGRATION_MT5: EnMTAPIRetcode = 1030;
pub const EnMTAPIRetcode_MT_RET_AUTH_INVALID_VERIFY: EnMTAPIRetcode = 1031;
pub const EnMTAPIRetcode_MT_RET_AUTH_VERIFY_BAD_EMAIL: EnMTAPIRetcode = 1032;
pub const EnMTAPIRetcode_MT_RET_AUTH_VERIFY_BAD_PHONE: EnMTAPIRetcode = 1033;
pub const EnMTAPIRetcode_MT_RET_AUTH_API_DISABLED: EnMTAPIRetcode = 1034;
pub const EnMTAPIRetcode_MT_RET_CFG_LAST_ADMIN: EnMTAPIRetcode = 2000;
pub const EnMTAPIRetcode_MT_RET_CFG_LAST_ADMIN_GROUP: EnMTAPIRetcode = 2001;
pub const EnMTAPIRetcode_MT_RET_CFG_NOT_EMPTY: EnMTAPIRetcode = 2003;
pub const EnMTAPIRetcode_MT_RET_CFG_INVALID_RANGE: EnMTAPIRetcode = 2004;
pub const EnMTAPIRetcode_MT_RET_CFG_NOT_MANAGER_LOGIN: EnMTAPIRetcode = 2005;
pub const EnMTAPIRetcode_MT_RET_CFG_BUILTIN: EnMTAPIRetcode = 2006;
pub const EnMTAPIRetcode_MT_RET_CFG_DUPLICATE: EnMTAPIRetcode = 2007;
pub const EnMTAPIRetcode_MT_RET_CFG_LIMIT_REACHED: EnMTAPIRetcode = 2008;
pub const EnMTAPIRetcode_MT_RET_CFG_NO_ACCESS_TO_MAIN: EnMTAPIRetcode = 2009;
pub const EnMTAPIRetcode_MT_RET_CFG_DEALER_ID_EXIST: EnMTAPIRetcode = 2010;
pub const EnMTAPIRetcode_MT_RET_CFG_BIND_ADDR_EXIST: EnMTAPIRetcode = 2011;
pub const EnMTAPIRetcode_MT_RET_CFG_WORKING_TRADE: EnMTAPIRetcode = 2012;
pub const EnMTAPIRetcode_MT_RET_CFG_GATEWAY_NAME_EXIST: EnMTAPIRetcode = 2013;
pub const EnMTAPIRetcode_MT_RET_CFG_SWITCH_TO_BACKUP: EnMTAPIRetcode = 2014;
pub const EnMTAPIRetcode_MT_RET_CFG_NO_BACKUP_MODULE: EnMTAPIRetcode = 2015;
pub const EnMTAPIRetcode_MT_RET_CFG_NO_TRADE_MODULE: EnMTAPIRetcode = 2016;
pub const EnMTAPIRetcode_MT_RET_CFG_NO_HISTORY_MODULE: EnMTAPIRetcode = 2017;
pub const EnMTAPIRetcode_MT_RET_CFG_ANOTHER_SWITCH: EnMTAPIRetcode = 2018;
pub const EnMTAPIRetcode_MT_RET_CFG_NO_LICENSE_FILE: EnMTAPIRetcode = 2019;
pub const EnMTAPIRetcode_MT_RET_CFG_GATEWAY_LOGIN_EXIST: EnMTAPIRetcode = 2020;
pub const EnMTAPIRetcode_MT_RET_USR_LAST_ADMIN: EnMTAPIRetcode = 3001;
pub const EnMTAPIRetcode_MT_RET_USR_LOGIN_EXHAUSTED: EnMTAPIRetcode = 3002;
pub const EnMTAPIRetcode_MT_RET_USR_LOGIN_PROHIBITED: EnMTAPIRetcode = 3003;
pub const EnMTAPIRetcode_MT_RET_USR_LOGIN_EXIST: EnMTAPIRetcode = 3004;
pub const EnMTAPIRetcode_MT_RET_USR_SUICIDE: EnMTAPIRetcode = 3005;
pub const EnMTAPIRetcode_MT_RET_USR_INVALID_PASSWORD: EnMTAPIRetcode = 3006;
pub const EnMTAPIRetcode_MT_RET_USR_LIMIT_REACHED: EnMTAPIRetcode = 3007;
pub const EnMTAPIRetcode_MT_RET_USR_HAS_TRADES: EnMTAPIRetcode = 3008;
pub const EnMTAPIRetcode_MT_RET_USR_DIFFERENT_SERVERS: EnMTAPIRetcode = 3009;
pub const EnMTAPIRetcode_MT_RET_USR_DIFFERENT_CURRENCY: EnMTAPIRetcode = 3010;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_BALANCE: EnMTAPIRetcode = 3011;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_GROUP: EnMTAPIRetcode = 3012;
pub const EnMTAPIRetcode_MT_RET_USR_ACCOUNT_EXIST: EnMTAPIRetcode = 3013;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_ACCOUNT: EnMTAPIRetcode = 3014;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_POSITIONS: EnMTAPIRetcode = 3015;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_ORDERS: EnMTAPIRetcode = 3016;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_DEALS: EnMTAPIRetcode = 3017;
pub const EnMTAPIRetcode_MT_RET_USR_IMPORT_HISTORY: EnMTAPIRetcode = 3018;
pub const EnMTAPIRetcode_MT_RET_USR_API_LIMIT_REACHED: EnMTAPIRetcode = 3019;
pub const EnMTAPIRetcode_MT_RET_TRADE_LIMIT_REACHED: EnMTAPIRetcode = 4001;
pub const EnMTAPIRetcode_MT_RET_TRADE_ORDER_EXIST: EnMTAPIRetcode = 4002;
pub const EnMTAPIRetcode_MT_RET_TRADE_ORDER_EXHAUSTED: EnMTAPIRetcode = 4003;
pub const EnMTAPIRetcode_MT_RET_TRADE_DEAL_EXHAUSTED: EnMTAPIRetcode = 4004;
pub const EnMTAPIRetcode_MT_RET_TRADE_MAX_MONEY: EnMTAPIRetcode = 4005;
pub const EnMTAPIRetcode_MT_RET_TRADE_DEAL_EXIST: EnMTAPIRetcode = 4006;
pub const EnMTAPIRetcode_MT_RET_TRADE_ORDER_PROHIBITED: EnMTAPIRetcode = 4007;
pub const EnMTAPIRetcode_MT_RET_TRADE_DEAL_PROHIBITED: EnMTAPIRetcode = 4008;
pub const EnMTAPIRetcode_MT_RET_TRADE_SPLIT_VOLUME: EnMTAPIRetcode = 4009;
pub const EnMTAPIRetcode_MT_RET_REPORT_SNAPSHOT: EnMTAPIRetcode = 5001;
pub const EnMTAPIRetcode_MT_RET_REPORT_NOTSUPPORTED: EnMTAPIRetcode = 5002;
pub const EnMTAPIRetcode_MT_RET_REPORT_NODATA: EnMTAPIRetcode = 5003;
pub const EnMTAPIRetcode_MT_RET_REPORT_TEMPLATE_BAD: EnMTAPIRetcode = 5004;
pub const EnMTAPIRetcode_MT_RET_REPORT_TEMPLATE_END: EnMTAPIRetcode = 5005;
pub const EnMTAPIRetcode_MT_RET_REPORT_INVALID_ROW: EnMTAPIRetcode = 5006;
pub const EnMTAPIRetcode_MT_RET_REPORT_LIMIT_REPEAT: EnMTAPIRetcode = 5007;
pub const EnMTAPIRetcode_MT_RET_REPORT_LIMIT_REPORT: EnMTAPIRetcode = 5008;
pub const EnMTAPIRetcode_MT_RET_HST_SYMBOL_NOTFOUND: EnMTAPIRetcode = 6001;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INWAY: EnMTAPIRetcode = 10001;
pub const EnMTAPIRetcode_MT_RET_REQUEST_ACCEPTED: EnMTAPIRetcode = 10002;
pub const EnMTAPIRetcode_MT_RET_REQUEST_PROCESS: EnMTAPIRetcode = 10003;
pub const EnMTAPIRetcode_MT_RET_REQUEST_REQUOTE: EnMTAPIRetcode = 10004;
pub const EnMTAPIRetcode_MT_RET_REQUEST_PRICES: EnMTAPIRetcode = 10005;
pub const EnMTAPIRetcode_MT_RET_REQUEST_REJECT: EnMTAPIRetcode = 10006;
pub const EnMTAPIRetcode_MT_RET_REQUEST_CANCEL: EnMTAPIRetcode = 10007;
pub const EnMTAPIRetcode_MT_RET_REQUEST_PLACED: EnMTAPIRetcode = 10008;
pub const EnMTAPIRetcode_MT_RET_REQUEST_DONE: EnMTAPIRetcode = 10009;
pub const EnMTAPIRetcode_MT_RET_REQUEST_DONE_PARTIAL: EnMTAPIRetcode = 10010;
pub const EnMTAPIRetcode_MT_RET_REQUEST_ERROR: EnMTAPIRetcode = 10011;
pub const EnMTAPIRetcode_MT_RET_REQUEST_TIMEOUT: EnMTAPIRetcode = 10012;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID: EnMTAPIRetcode = 10013;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_VOLUME: EnMTAPIRetcode = 10014;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_PRICE: EnMTAPIRetcode = 10015;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_STOPS: EnMTAPIRetcode = 10016;
pub const EnMTAPIRetcode_MT_RET_REQUEST_TRADE_DISABLED: EnMTAPIRetcode = 10017;
pub const EnMTAPIRetcode_MT_RET_REQUEST_MARKET_CLOSED: EnMTAPIRetcode = 10018;
pub const EnMTAPIRetcode_MT_RET_REQUEST_NO_MONEY: EnMTAPIRetcode = 10019;
pub const EnMTAPIRetcode_MT_RET_REQUEST_PRICE_CHANGED: EnMTAPIRetcode = 10020;
pub const EnMTAPIRetcode_MT_RET_REQUEST_PRICE_OFF: EnMTAPIRetcode = 10021;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_EXP: EnMTAPIRetcode = 10022;
pub const EnMTAPIRetcode_MT_RET_REQUEST_ORDER_CHANGED: EnMTAPIRetcode = 10023;
pub const EnMTAPIRetcode_MT_RET_REQUEST_TOO_MANY: EnMTAPIRetcode = 10024;
pub const EnMTAPIRetcode_MT_RET_REQUEST_NO_CHANGES: EnMTAPIRetcode = 10025;
pub const EnMTAPIRetcode_MT_RET_REQUEST_AT_DISABLED_SERVER: EnMTAPIRetcode = 10026;
pub const EnMTAPIRetcode_MT_RET_REQUEST_AT_DISABLED_CLIENT: EnMTAPIRetcode = 10027;
pub const EnMTAPIRetcode_MT_RET_REQUEST_LOCKED: EnMTAPIRetcode = 10028;
pub const EnMTAPIRetcode_MT_RET_REQUEST_FROZEN: EnMTAPIRetcode = 10029;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_FILL: EnMTAPIRetcode = 10030;
pub const EnMTAPIRetcode_MT_RET_REQUEST_CONNECTION: EnMTAPIRetcode = 10031;
pub const EnMTAPIRetcode_MT_RET_REQUEST_ONLY_REAL: EnMTAPIRetcode = 10032;
pub const EnMTAPIRetcode_MT_RET_REQUEST_LIMIT_ORDERS: EnMTAPIRetcode = 10033;
pub const EnMTAPIRetcode_MT_RET_REQUEST_LIMIT_VOLUME: EnMTAPIRetcode = 10034;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_ORDER: EnMTAPIRetcode = 10035;
pub const EnMTAPIRetcode_MT_RET_REQUEST_POSITION_CLOSED: EnMTAPIRetcode = 10036;
pub const EnMTAPIRetcode_MT_RET_REQUEST_EXECUTION_SKIPPED: EnMTAPIRetcode = 10037;
pub const EnMTAPIRetcode_MT_RET_REQUEST_INVALID_CLOSE_VOLUME: EnMTAPIRetcode = 10038;
pub const EnMTAPIRetcode_MT_RET_REQUEST_CLOSE_ORDER_EXIST: EnMTAPIRetcode = 10039;
pub const EnMTAPIRetcode_MT_RET_REQUEST_LIMIT_POSITIONS: EnMTAPIRetcode = 10040;
pub const EnMTAPIRetcode_MT_RET_REQUEST_REJECT_CANCEL: EnMTAPIRetcode = 10041;
pub const EnMTAPIRetcode_MT_RET_REQUEST_LONG_ONLY: EnMTAPIRetcode = 10042;
pub const EnMTAPIRetcode_MT_RET_REQUEST_SHORT_ONLY: EnMTAPIRetcode = 10043;
pub const EnMTAPIRetcode_MT_RET_REQUEST_CLOSE_ONLY: EnMTAPIRetcode = 10044;
pub const EnMTAPIRetcode_MT_RET_REQUEST_PROHIBITED_BY_FIFO: EnMTAPIRetcode = 10045;
pub const EnMTAPIRetcode_MT_RET_REQUEST_HEDGE_PROHIBITED: EnMTAPIRetcode = 10046;
pub const EnMTAPIRetcode_MT_RET_REQUEST_RETURN: EnMTAPIRetcode = 11000;
pub const EnMTAPIRetcode_MT_RET_REQUEST_DONE_CANCEL: EnMTAPIRetcode = 11001;
pub const EnMTAPIRetcode_MT_RET_REQUEST_REQUOTE_RETURN: EnMTAPIRetcode = 11002;
pub const EnMTAPIRetcode_MT_RET_ERR_NOTIMPLEMENT: EnMTAPIRetcode = 12000;
pub const EnMTAPIRetcode_MT_RET_ERR_NOTMAIN: EnMTAPIRetcode = 12001;
pub const EnMTAPIRetcode_MT_RET_ERR_NOTSUPPORTED: EnMTAPIRetcode = 12002;
pub const EnMTAPIRetcode_MT_RET_ERR_DEADLOCK: EnMTAPIRetcode = 12003;
pub const EnMTAPIRetcode_MT_RET_ERR_LOCKED: EnMTAPIRetcode = 12004;
pub const EnMTAPIRetcode_MT_RET_MESSENGER_INVALID_PHONE: EnMTAPIRetcode = 14000;
pub const EnMTAPIRetcode_MT_RET_MESSENGER_NOT_MOBILE: EnMTAPIRetcode = 14001;
pub const EnMTAPIRetcode_MT_RET_SUBS_NOT_FOUND: EnMTAPIRetcode = 15000;
pub const EnMTAPIRetcode_MT_RET_SUBS_NOT_FOUND_CFG: EnMTAPIRetcode = 15001;
pub const EnMTAPIRetcode_MT_RET_SUBS_NOT_FOUND_USER: EnMTAPIRetcode = 15002;
pub const EnMTAPIRetcode_MT_RET_SUBS_DISABLED: EnMTAPIRetcode = 15003;
pub const EnMTAPIRetcode_MT_RET_SUBS_PERMISSION_USER: EnMTAPIRetcode = 15004;
pub const EnMTAPIRetcode_MT_RET_SUBS_PERMISSION_SUBSCRIBE: EnMTAPIRetcode = 15005;
pub const EnMTAPIRetcode_MT_RET_SUBS_PERMISSION_UNSUBSCRIBE: EnMTAPIRetcode = 15006;
pub const EnMTAPIRetcode_MT_RET_SUBS_REAL_ONLY: EnMTAPIRetcode = 15007;
pub type EnMTAPIRetcode = ::std::os::raw::c_int;
#[repr(C)]
pub struct IMTDatasetColumn__bindgen_vtable {
    pub IMTDatasetColumn_Release: unsafe extern "C" fn(this: *mut IMTDatasetColumn),
    pub IMTDatasetColumn_Assign: unsafe extern "C" fn(
        this: *mut IMTDatasetColumn,
        column: *const IMTDatasetColumn,
    ) -> MTAPIRES,
    pub IMTDatasetColumn_Clear: unsafe extern "C" fn(this: *mut IMTDatasetColumn) -> MTAPIRES,
    pub IMTDatasetColumn_Name1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, name: LPCWSTR) -> MTAPIRES,
    pub IMTDatasetColumn_Name: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> LPCWSTR,
    pub IMTDatasetColumn_ColumnID1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, column_id: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_ColumnID: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_Type1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, type_: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_Type: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_Width1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, width: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_Width: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_WidthMax1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, width_max: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_WidthMax: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_Digits1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, digits: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_Digits: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_DigitsColumn1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, column_id: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_DigitsColumn: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_Flags1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, flags: UINT64) -> MTAPIRES,
    pub IMTDatasetColumn_Flags: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT64,
    pub IMTDatasetColumn_Offset1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, offset: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_Offset: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
    pub IMTDatasetColumn_Size1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, size: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_Size: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,

    pub IMTDatasetColumn_Color1:
        unsafe extern "C" fn(this: *mut IMTDatasetColumn, color: UINT) -> MTAPIRES,
    pub IMTDatasetColumn_Color: unsafe extern "C" fn(this: *const IMTDatasetColumn) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDatasetColumn {
    pub vtable_: *const IMTDatasetColumn__bindgen_vtable,
}
pub const IMTDatasetColumn_EnType_TYPE_INT8: IMTDatasetColumn_EnType = 0;
pub const IMTDatasetColumn_EnType_TYPE_UINT8: IMTDatasetColumn_EnType = 1;
pub const IMTDatasetColumn_EnType_TYPE_INT16: IMTDatasetColumn_EnType = 2;
pub const IMTDatasetColumn_EnType_TYPE_UINT16: IMTDatasetColumn_EnType = 3;
pub const IMTDatasetColumn_EnType_TYPE_INT32: IMTDatasetColumn_EnType = 4;
pub const IMTDatasetColumn_EnType_TYPE_UINT32: IMTDatasetColumn_EnType = 5;
pub const IMTDatasetColumn_EnType_TYPE_INT64: IMTDatasetColumn_EnType = 6;
pub const IMTDatasetColumn_EnType_TYPE_UINT64: IMTDatasetColumn_EnType = 7;
pub const IMTDatasetColumn_EnType_TYPE_DOUBLE: IMTDatasetColumn_EnType = 8;
pub const IMTDatasetColumn_EnType_TYPE_MONEY: IMTDatasetColumn_EnType = 9;
pub const IMTDatasetColumn_EnType_TYPE_STRING: IMTDatasetColumn_EnType = 10;
pub const IMTDatasetColumn_EnType_TYPE_DATE: IMTDatasetColumn_EnType = 11;
pub const IMTDatasetColumn_EnType_TYPE_TIME: IMTDatasetColumn_EnType = 12;
pub const IMTDatasetColumn_EnType_TYPE_DATETIME: IMTDatasetColumn_EnType = 13;
pub const IMTDatasetColumn_EnType_TYPE_TIME_MSC: IMTDatasetColumn_EnType = 14;
pub const IMTDatasetColumn_EnType_TYPE_DATETIME_MSC: IMTDatasetColumn_EnType = 15;
pub const IMTDatasetColumn_EnType_TYPE_PRICE: IMTDatasetColumn_EnType = 100;
pub const IMTDatasetColumn_EnType_TYPE_PRICES: IMTDatasetColumn_EnType = 101;
pub const IMTDatasetColumn_EnType_TYPE_PRICE_POSITION: IMTDatasetColumn_EnType = 102;
pub const IMTDatasetColumn_EnType_TYPE_VOLUME: IMTDatasetColumn_EnType = 200;
pub const IMTDatasetColumn_EnType_TYPE_VOLUME_ORDER: IMTDatasetColumn_EnType = 201;
pub const IMTDatasetColumn_EnType_TYPE_VOLUME_EXT: IMTDatasetColumn_EnType = 202;
pub const IMTDatasetColumn_EnType_TYPE_VOLUME_ORDER_EXT: IMTDatasetColumn_EnType = 203;
pub const IMTDatasetColumn_EnType_TYPE_POSITION_TYPE: IMTDatasetColumn_EnType = 300;
pub const IMTDatasetColumn_EnType_TYPE_ORDER_TYPE: IMTDatasetColumn_EnType = 400;
pub const IMTDatasetColumn_EnType_TYPE_ORDER_TYPE_TIME: IMTDatasetColumn_EnType = 401;
pub const IMTDatasetColumn_EnType_TYPE_ORDER_TYPE_REASON: IMTDatasetColumn_EnType = 402;
pub const IMTDatasetColumn_EnType_TYPE_ORDER_STATUS: IMTDatasetColumn_EnType = 403;
pub const IMTDatasetColumn_EnType_TYPE_ORDER_FILLING: IMTDatasetColumn_EnType = 404;
pub const IMTDatasetColumn_EnType_TYPE_DEAL_ACTION: IMTDatasetColumn_EnType = 500;
pub const IMTDatasetColumn_EnType_TYPE_DEAL_ENTRY: IMTDatasetColumn_EnType = 501;
pub const IMTDatasetColumn_EnType_TYPE_USER_LOGIN: IMTDatasetColumn_EnType = 600;
pub const IMTDatasetColumn_EnType_TYPE_USER_LEVERAGE: IMTDatasetColumn_EnType = 601;
pub const IMTDatasetColumn_EnType_TYPE_CLIENT_ID: IMTDatasetColumn_EnType = 700;
pub const IMTDatasetColumn_EnType_TYPE_FIRST: IMTDatasetColumn_EnType = 0;
pub const IMTDatasetColumn_EnType_TYPE_LAST: IMTDatasetColumn_EnType = 700;
pub type IMTDatasetColumn_EnType = ::std::os::raw::c_int;
pub const IMTDatasetColumn_EnFlags_FLAG_NONE: IMTDatasetColumn_EnFlags = 0;
pub const IMTDatasetColumn_EnFlags_FLAG_PRIMARY: IMTDatasetColumn_EnFlags = 1;
pub const IMTDatasetColumn_EnFlags_FLAG_HIDDEN_VIEW: IMTDatasetColumn_EnFlags = 2;
pub const IMTDatasetColumn_EnFlags_FLAG_HIDDEN_SAVE: IMTDatasetColumn_EnFlags = 4;
pub const IMTDatasetColumn_EnFlags_FLAG_HIDDEN: IMTDatasetColumn_EnFlags = 6;
pub const IMTDatasetColumn_EnFlags_FLAG_LEFT: IMTDatasetColumn_EnFlags = 8;
pub const IMTDatasetColumn_EnFlags_FLAG_RIGHT: IMTDatasetColumn_EnFlags = 16;
pub const IMTDatasetColumn_EnFlags_FLAG_CENTER: IMTDatasetColumn_EnFlags = 24;
pub const IMTDatasetColumn_EnFlags_FLAG_SORT_DEFAULT: IMTDatasetColumn_EnFlags = 256;
pub const IMTDatasetColumn_EnFlags_FLAG_ALL: IMTDatasetColumn_EnFlags = 287;
pub type IMTDatasetColumn_EnFlags = ::std::os::raw::c_int;
pub const IMTDatasetColumn_EnColumnColor_COLUMN_COLOR_AUTO: IMTDatasetColumn_EnColumnColor = -1;
pub type IMTDatasetColumn_EnColumnColor = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDatasetColumn"][::std::mem::size_of::<IMTDatasetColumn>() - 8usize];
    ["Alignment of IMTDatasetColumn"][::std::mem::align_of::<IMTDatasetColumn>() - 8usize];
};
#[repr(C)]
pub struct IMTDatasetSummary__bindgen_vtable {
    pub IMTDatasetSummary_Release: unsafe extern "C" fn(this: *mut IMTDatasetSummary),
    pub IMTDatasetSummary_Assign: unsafe extern "C" fn(
        this: *mut IMTDatasetSummary,
        summary: *const IMTDatasetSummary,
    ) -> MTAPIRES,
    pub IMTDatasetSummary_Clear: unsafe extern "C" fn(this: *mut IMTDatasetSummary) -> MTAPIRES,
    pub IMTDatasetSummary_ColumnID1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, column_id: UINT) -> MTAPIRES,
    pub IMTDatasetSummary_ColumnID: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT,
    pub IMTDatasetSummary_Line1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, line: UINT) -> MTAPIRES,
    pub IMTDatasetSummary_Line: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT,
    pub IMTDatasetSummary_MergeColumn1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, column_id: UINT) -> MTAPIRES,
    pub IMTDatasetSummary_MergeColumn: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT,
    pub IMTDatasetSummary_Color1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, color: UINT) -> MTAPIRES,
    pub IMTDatasetSummary_Color: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT,
    pub IMTDatasetSummary_Flags1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, flags: UINT64) -> MTAPIRES,
    pub IMTDatasetSummary_Flags: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_Type: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT,
    pub IMTDatasetSummary_Digits1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, digits: UINT) -> MTAPIRES,
    pub IMTDatasetSummary_Digits: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT,
    pub IMTDatasetSummary_ValueInt1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: INT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueInt: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> INT64,
    pub IMTDatasetSummary_ValueUInt1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: UINT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueUInt: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueDouble1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: f64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueDouble: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> f64,
    pub IMTDatasetSummary_ValueMoney1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: f64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueMoney: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> f64,
    pub IMTDatasetSummary_ValueString1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: LPCWSTR) -> MTAPIRES,
    pub IMTDatasetSummary_ValueString:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> LPCWSTR,
    pub IMTDatasetSummary_ValueDate1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: INT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueDate: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> INT64,
    pub IMTDatasetSummary_ValueTime1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: INT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueTime: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> INT64,
    pub IMTDatasetSummary_ValueDateTime1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: INT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueDateTime:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> INT64,
        pub IMTDatasetSummary_ValuePrice1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: f64) -> MTAPIRES,
    pub IMTDatasetSummary_ValuePrice: unsafe extern "C" fn(this: *const IMTDatasetSummary) -> f64,
    pub IMTDatasetSummary_ValuePricesBid:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> f64,
    pub IMTDatasetSummary_ValuePricesAsk:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> f64,
    pub IMTDatasetSummary_ValuePrices: unsafe extern "C" fn(
        this: *mut IMTDatasetSummary,
        value_bid: f64,
        value_ask: f64,
    ) -> MTAPIRES,
    pub IMTDatasetSummary_ValueVolume2: unsafe extern "C" fn(
        this: *mut IMTDatasetSummary,
        value_initial: UINT64,
        value_current: UINT64,
    ) -> MTAPIRES,
    pub IMTDatasetSummary_ValueVolume1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: UINT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueVolume:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueVolumeInitial:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueVolumeCurrent:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueVolumeExt1:
        unsafe extern "C" fn(this: *mut IMTDatasetSummary, value: UINT64) -> MTAPIRES,
    pub IMTDatasetSummary_ValueVolumeExt:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueVolumeExtInitial:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueVolumeExtCurrent:
        unsafe extern "C" fn(this: *const IMTDatasetSummary) -> UINT64,
    pub IMTDatasetSummary_ValueVolumeExt2: unsafe extern "C" fn(
        this: *mut IMTDatasetSummary,
        value_initial: UINT64,
        value_current: UINT64,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDatasetSummary {
    pub vtable_: *const IMTDatasetSummary__bindgen_vtable,
}
pub const IMTDatasetSummary_EnType_TYPE_INT: IMTDatasetSummary_EnType = 0;
pub const IMTDatasetSummary_EnType_TYPE_UINT: IMTDatasetSummary_EnType = 1;
pub const IMTDatasetSummary_EnType_TYPE_DOUBLE: IMTDatasetSummary_EnType = 2;
pub const IMTDatasetSummary_EnType_TYPE_MONEY: IMTDatasetSummary_EnType = 3;
pub const IMTDatasetSummary_EnType_TYPE_STRING: IMTDatasetSummary_EnType = 4;
pub const IMTDatasetSummary_EnType_TYPE_DATE: IMTDatasetSummary_EnType = 5;
pub const IMTDatasetSummary_EnType_TYPE_TIME: IMTDatasetSummary_EnType = 6;
pub const IMTDatasetSummary_EnType_TYPE_DATETIME: IMTDatasetSummary_EnType = 7;
pub const IMTDatasetSummary_EnType_TYPE_PRICE: IMTDatasetSummary_EnType = 100;
pub const IMTDatasetSummary_EnType_TYPE_PRICES: IMTDatasetSummary_EnType = 101;
pub const IMTDatasetSummary_EnType_TYPE_VOLUME: IMTDatasetSummary_EnType = 200;
pub const IMTDatasetSummary_EnType_TYPE_VOLUME_ORDER: IMTDatasetSummary_EnType = 201;
pub const IMTDatasetSummary_EnType_TYPE_VOLUME_EXT: IMTDatasetSummary_EnType = 202;
pub const IMTDatasetSummary_EnType_TYPE_VOLUME_ORDER_EXT: IMTDatasetSummary_EnType = 203;
pub const IMTDatasetSummary_EnType_TYPE_FIRST: IMTDatasetSummary_EnType = 0;
pub const IMTDatasetSummary_EnType_TYPE_LAST: IMTDatasetSummary_EnType = 203;
pub type IMTDatasetSummary_EnType = ::std::os::raw::c_int;
pub const IMTDatasetSummary_EnFlags_FLAG_NONE: IMTDatasetSummary_EnFlags = 0;
pub type IMTDatasetSummary_EnFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDatasetSummary"][::std::mem::size_of::<IMTDatasetSummary>() - 8usize];
    ["Alignment of IMTDatasetSummary"][::std::mem::align_of::<IMTDatasetSummary>() - 8usize];
};
#[repr(C)]
pub struct IMTDataset__bindgen_vtable {
    pub IMTDataset_Assign:
        unsafe extern "C" fn(this: *mut IMTDataset, data: *const IMTDataset) -> MTAPIRES,
    pub IMTDataset_Clear: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_Flags1: unsafe extern "C" fn(this: *mut IMTDataset, flags: UINT64) -> MTAPIRES,
    pub IMTDataset_Flags: unsafe extern "C" fn(this: *const IMTDataset) -> UINT64,
    pub IMTDataset_Reserved1: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_Reserved2: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_Reserved3: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_Reserved4: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_ColumnCreate:
        unsafe extern "C" fn(this: *mut IMTDataset) -> *mut IMTDatasetColumn,
    pub IMTDataset_ColumnClear: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_ColumnAdd:
        unsafe extern "C" fn(this: *mut IMTDataset, column: *const IMTDatasetColumn) -> MTAPIRES,
    pub IMTDataset_ColumnDelete: unsafe extern "C" fn(this: *mut IMTDataset, pos: UINT) -> MTAPIRES,
    pub IMTDataset_ColumnTotal: unsafe extern "C" fn(this: *const IMTDataset) -> UINT,
    pub IMTDataset_ColumnSize: unsafe extern "C" fn(this: *const IMTDataset) -> UINT,
    pub IMTDataset_ColumnNext: unsafe extern "C" fn(
        this: *mut IMTDataset,
        pos: UINT,
        column: *mut IMTDatasetColumn,
    ) -> MTAPIRES,
    pub IMTDataset_ColumnReserved1: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_ColumnReserved2: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_RowClear: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_RowWrite: unsafe extern "C" fn(
        this: *mut IMTDataset,
        data: *const ::std::os::raw::c_void,
        size: UINT,
    ) -> MTAPIRES,
    pub IMTDataset_RowTotal: unsafe extern "C" fn(this: *const IMTDataset) -> UINT,
    pub IMTDataset_RowRead: unsafe extern "C" fn(
        this: *const IMTDataset,
        pos: UINT,
        data: *mut ::std::os::raw::c_void,
        size: UINT,
    ) -> MTAPIRES,
    pub IMTDataset_RowReserved2: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_SummaryCreate:
        unsafe extern "C" fn(this: *mut IMTDataset) -> *mut IMTDatasetSummary,
    pub IMTDataset_SummaryClear: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_SummaryAdd:
        unsafe extern "C" fn(this: *mut IMTDataset, summary: *const IMTDatasetSummary) -> MTAPIRES,
    pub IMTDataset_SummaryDelete:
        unsafe extern "C" fn(this: *mut IMTDataset, pos: UINT) -> MTAPIRES,
    pub IMTDataset_SummaryNext: unsafe extern "C" fn(
        this: *mut IMTDataset,
        pos: UINT,
        summary: *mut IMTDatasetSummary,
    ) -> MTAPIRES,
    pub IMTDataset_SummaryTotal: unsafe extern "C" fn(this: *const IMTDataset) -> UINT,
    pub IMTDataset_SummaryReserved1: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_SummaryReserved2: unsafe extern "C" fn(this: *mut IMTDataset) -> MTAPIRES,
    pub IMTDataset_Release: unsafe extern "C" fn(this: *mut IMTDataset),
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDataset {
    pub vtable_: *const IMTDataset__bindgen_vtable,
}
pub const IMTDataset_EnDataSetFlags_DATASET_FLAG_NONE: IMTDataset_EnDataSetFlags = 0;
pub type IMTDataset_EnDataSetFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDataset"][::std::mem::size_of::<IMTDataset>() - 8usize];
    ["Alignment of IMTDataset"][::std::mem::align_of::<IMTDataset>() - 8usize];
};
#[repr(C)]
pub struct IMTDatasetField__bindgen_vtable {
    pub IMTDatasetField_Release: unsafe extern "C" fn(this: *mut IMTDatasetField),
    pub IMTDatasetField_Assign:
        unsafe extern "C" fn(this: *mut IMTDatasetField, field: *const IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_Clear: unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_Id1: unsafe extern "C" fn(this: *mut IMTDatasetField, id: UINT) -> MTAPIRES,
    pub IMTDatasetField_Id: unsafe extern "C" fn(this: *const IMTDatasetField) -> UINT,
    pub IMTDatasetField_Type: unsafe extern "C" fn(this: *const IMTDatasetField) -> UINT,
    pub IMTDatasetField_Offset1:
        unsafe extern "C" fn(this: *mut IMTDatasetField, offset: UINT) -> MTAPIRES,
    pub IMTDatasetField_Offset: unsafe extern "C" fn(this: *const IMTDatasetField) -> UINT,
    pub IMTDatasetField_Size1:
        unsafe extern "C" fn(this: *mut IMTDatasetField, size: UINT) -> MTAPIRES,
    pub IMTDatasetField_Size: unsafe extern "C" fn(this: *const IMTDatasetField) -> UINT,
    pub IMTDatasetField_Flags1:
        unsafe extern "C" fn(this: *mut IMTDatasetField, flags: UINT64) -> MTAPIRES,
    pub IMTDatasetField_Flags: unsafe extern "C" fn(this: *const IMTDatasetField) -> UINT64,
    pub IMTDatasetField_Reserved1: unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_Reserved2: unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_Reserved3: unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_Reserved4: unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_WhereAddInt:
        unsafe extern "C" fn(this: *mut IMTDatasetField, value: INT64) -> MTAPIRES,
    pub IMTDatasetField_WhereAddIntArray: unsafe extern "C" fn(
        this: *mut IMTDatasetField,
        values: *const INT64,
        values_total: UINT,
    ) -> MTAPIRES,
    pub IMTDatasetField_WhereAddUInt:
        unsafe extern "C" fn(this: *mut IMTDatasetField, value: UINT64) -> MTAPIRES,
    pub IMTDatasetField_WhereAddUIntArray: unsafe extern "C" fn(
        this: *mut IMTDatasetField,
        values: *const UINT64,
        values_total: UINT,
    ) -> MTAPIRES,
    pub IMTDatasetField_WhereAddDouble:
        unsafe extern "C" fn(this: *mut IMTDatasetField, value: f64) -> MTAPIRES,
    pub IMTDatasetField_WhereAddDoubleArray: unsafe extern "C" fn(
        this: *mut IMTDatasetField,
        values: *const f64,
        values_total: UINT,
    ) -> MTAPIRES,
    pub IMTDatasetField_WhereAddString:
        unsafe extern "C" fn(this: *mut IMTDatasetField, value: LPCWSTR) -> MTAPIRES,
    pub IMTDatasetField_WhereAddStringArray: unsafe extern "C" fn(
        this: *mut IMTDatasetField,
        values: *mut LPCWSTR,
        values_total: UINT,
    ) -> MTAPIRES,
    pub IMTDatasetField_WhereUIntSet: unsafe extern "C" fn(
        this: *mut IMTDatasetField,
        values: *const UINT64,
        values_total: UINT,
    ) -> MTAPIRES,
    pub IMTDatasetField_WhereReserved1:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_WhereReserved2:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_WhereReserved3:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_BetweenInt:
        unsafe extern "C" fn(this: *mut IMTDatasetField, from: INT64, to: INT64) -> MTAPIRES,
    pub IMTDatasetField_BetweenUInt:
        unsafe extern "C" fn(this: *mut IMTDatasetField, from: UINT64, to: UINT64) -> MTAPIRES,
    pub IMTDatasetField_BetweenDouble:
        unsafe extern "C" fn(this: *mut IMTDatasetField, from: f64, to: f64) -> MTAPIRES,
    pub IMTDatasetField_BetweenReserved1:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_BetweenReserved2:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_BetweenReserved3:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
    pub IMTDatasetField_BetweenReserved4:
        unsafe extern "C" fn(this: *mut IMTDatasetField) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDatasetField {
    pub vtable_: *const IMTDatasetField__bindgen_vtable,
}
pub const IMTDatasetField_EnFieldType_TYPE_NONE: IMTDatasetField_EnFieldType = 0;
pub const IMTDatasetField_EnFieldType_TYPE_INT: IMTDatasetField_EnFieldType = 1;
pub const IMTDatasetField_EnFieldType_TYPE_UINT: IMTDatasetField_EnFieldType = 2;
pub const IMTDatasetField_EnFieldType_TYPE_DOUBLE: IMTDatasetField_EnFieldType = 3;
pub const IMTDatasetField_EnFieldType_TYPE_STRING: IMTDatasetField_EnFieldType = 4;
pub const IMTDatasetField_EnFieldType_TYPE_FIRST: IMTDatasetField_EnFieldType = 0;
pub const IMTDatasetField_EnFieldType_TYPE_LAST: IMTDatasetField_EnFieldType = 4;
pub type IMTDatasetField_EnFieldType = ::std::os::raw::c_int;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LOGIN: IMTDatasetField_EnFieldId = 1;
pub const IMTDatasetField_EnFieldId_FIELD_USER_GROUP: IMTDatasetField_EnFieldId = 2;
pub const IMTDatasetField_EnFieldId_FIELD_USER_CERT_SERIAL_NUMBER: IMTDatasetField_EnFieldId = 3;
pub const IMTDatasetField_EnFieldId_FIELD_USER_RIGHTS: IMTDatasetField_EnFieldId = 4;
pub const IMTDatasetField_EnFieldId_FIELD_USER_REGISTRATION: IMTDatasetField_EnFieldId = 5;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LAST_ACCESS: IMTDatasetField_EnFieldId = 6;
pub const IMTDatasetField_EnFieldId_FIELD_USER_NAME: IMTDatasetField_EnFieldId = 7;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COMPANY: IMTDatasetField_EnFieldId = 8;
pub const IMTDatasetField_EnFieldId_FIELD_USER_ACCOUNT: IMTDatasetField_EnFieldId = 9;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COUNTRY: IMTDatasetField_EnFieldId = 10;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LANGUAGE: IMTDatasetField_EnFieldId = 11;
pub const IMTDatasetField_EnFieldId_FIELD_USER_CITY: IMTDatasetField_EnFieldId = 12;
pub const IMTDatasetField_EnFieldId_FIELD_USER_STATE: IMTDatasetField_EnFieldId = 13;
pub const IMTDatasetField_EnFieldId_FIELD_USER_ZIP_CODE: IMTDatasetField_EnFieldId = 14;
pub const IMTDatasetField_EnFieldId_FIELD_USER_ADDRESS: IMTDatasetField_EnFieldId = 15;
pub const IMTDatasetField_EnFieldId_FIELD_USER_PHONE: IMTDatasetField_EnFieldId = 16;
pub const IMTDatasetField_EnFieldId_FIELD_USER_EMAIL: IMTDatasetField_EnFieldId = 17;
pub const IMTDatasetField_EnFieldId_FIELD_USER_ID: IMTDatasetField_EnFieldId = 18;
pub const IMTDatasetField_EnFieldId_FIELD_USER_STATUS: IMTDatasetField_EnFieldId = 19;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COMMENT: IMTDatasetField_EnFieldId = 20;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COLOR: IMTDatasetField_EnFieldId = 21;
pub const IMTDatasetField_EnFieldId_FIELD_USER_PHONE_PASSWORD: IMTDatasetField_EnFieldId = 22;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LEVERAGE: IMTDatasetField_EnFieldId = 23;
pub const IMTDatasetField_EnFieldId_FIELD_USER_AGENT: IMTDatasetField_EnFieldId = 24;
pub const IMTDatasetField_EnFieldId_FIELD_USER_BALANCE: IMTDatasetField_EnFieldId = 25;
pub const IMTDatasetField_EnFieldId_FIELD_USER_CREDIT: IMTDatasetField_EnFieldId = 26;
pub const IMTDatasetField_EnFieldId_FIELD_USER_INTEREST_RATE: IMTDatasetField_EnFieldId = 27;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COMMISSION_DAILY: IMTDatasetField_EnFieldId = 28;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COMMISSION_MONTHLY: IMTDatasetField_EnFieldId = 29;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COMMISSION_AGENT_DAILY: IMTDatasetField_EnFieldId =
    30;
pub const IMTDatasetField_EnFieldId_FIELD_USER_COMMISSION_AGENT_MONTHLY: IMTDatasetField_EnFieldId =
    31;
pub const IMTDatasetField_EnFieldId_FIELD_USER_BALANCE_PREV_DAY: IMTDatasetField_EnFieldId = 32;
pub const IMTDatasetField_EnFieldId_FIELD_USER_BALANCE_PREV_MONTH: IMTDatasetField_EnFieldId = 33;
pub const IMTDatasetField_EnFieldId_FIELD_USER_EQUITY_PREV_DAY: IMTDatasetField_EnFieldId = 34;
pub const IMTDatasetField_EnFieldId_FIELD_USER_EQUITY_PREV_MONTH: IMTDatasetField_EnFieldId = 35;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LAST_PASS_CHANGE: IMTDatasetField_EnFieldId = 36;
pub const IMTDatasetField_EnFieldId_FIELD_USER_MQID: IMTDatasetField_EnFieldId = 37;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LEAD_CAMPAIGN: IMTDatasetField_EnFieldId = 38;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LEAD_SOURCE: IMTDatasetField_EnFieldId = 39;
pub const IMTDatasetField_EnFieldId_FIELD_USER_CLIENT_ID: IMTDatasetField_EnFieldId = 40;
pub const IMTDatasetField_EnFieldId_FIELD_USER_FIRST_NAME: IMTDatasetField_EnFieldId = 41;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LAST_NAME: IMTDatasetField_EnFieldId = 42;
pub const IMTDatasetField_EnFieldId_FIELD_USER_MIDDLE_NAME: IMTDatasetField_EnFieldId = 43;
pub const IMTDatasetField_EnFieldId_FIELD_USER_FIRST: IMTDatasetField_EnFieldId = 1;
pub const IMTDatasetField_EnFieldId_FIELD_USER_LAST: IMTDatasetField_EnFieldId = 43;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ID: IMTDatasetField_EnFieldId = 1001;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CREATED_TIME: IMTDatasetField_EnFieldId = 1002;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CREATED_BY: IMTDatasetField_EnFieldId = 1003;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_MODIFIED_TIME: IMTDatasetField_EnFieldId = 1004;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_MODIFIED_BY: IMTDatasetField_EnFieldId = 1005;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_TYPE: IMTDatasetField_EnFieldId = 1006;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_STATUS: IMTDatasetField_EnFieldId = 1007;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ASSIGNED_MANAGER: IMTDatasetField_EnFieldId = 1008;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMMENT: IMTDatasetField_EnFieldId = 1009;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPLIANCE_APPROVED_BY: IMTDatasetField_EnFieldId =
    1010;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPLIANCE_CLIENT_CATEGORY:
    IMTDatasetField_EnFieldId = 1011;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPLIANCE_TIME_APPROVAL:
    IMTDatasetField_EnFieldId = 1012;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPLIANCE_TIME_TERMINATION:
    IMTDatasetField_EnFieldId = 1013;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_LEAD_CAMPAIGN: IMTDatasetField_EnFieldId = 1014;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_LEAD_SOURCE: IMTDatasetField_EnFieldId = 1015;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_INTRODUCER: IMTDatasetField_EnFieldId = 1016;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_TITLE: IMTDatasetField_EnFieldId = 1017;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_NAME: IMTDatasetField_EnFieldId = 1018;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_MIDDLE_NAME: IMTDatasetField_EnFieldId =
    1019;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_LAST_NAME: IMTDatasetField_EnFieldId = 1020;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_BIRTH_DATE: IMTDatasetField_EnFieldId =
    1021;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_CITIZENSHIP: IMTDatasetField_EnFieldId =
    1022;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_GENDER: IMTDatasetField_EnFieldId = 1023;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_TAX_ID: IMTDatasetField_EnFieldId = 1024;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_DOCUMENT_TYPE: IMTDatasetField_EnFieldId =
    1025;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_DOCUMENT_NUMBER: IMTDatasetField_EnFieldId =
    1026;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_DOCUMENT_DATE: IMTDatasetField_EnFieldId =
    1027;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_DOCUMENT_EXTRA: IMTDatasetField_EnFieldId =
    1028;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_EMPLOYMENT: IMTDatasetField_EnFieldId =
    1029;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_INDUSTRY: IMTDatasetField_EnFieldId = 1030;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_EDUCATION: IMTDatasetField_EnFieldId = 1031;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_WEALTH_SOURCE: IMTDatasetField_EnFieldId =
    1032;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_ANNUAL_INCOME: IMTDatasetField_EnFieldId =
    1033;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_NET_WORTH: IMTDatasetField_EnFieldId = 1034;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_PERSON_ANNUAL_DEPOSIT: IMTDatasetField_EnFieldId =
    1035;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_NAME: IMTDatasetField_EnFieldId = 1036;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_REG_NUMBER: IMTDatasetField_EnFieldId =
    1037;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_REG_DATE: IMTDatasetField_EnFieldId = 1038;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_REG_AUTHORITY: IMTDatasetField_EnFieldId =
    1039;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_VAT: IMTDatasetField_EnFieldId = 1040;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_LEI: IMTDatasetField_EnFieldId = 1041;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_LICENSE_NUMBER: IMTDatasetField_EnFieldId =
    1042;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_LICENSE_AUTHORITY:
    IMTDatasetField_EnFieldId = 1043;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_COUNTRY: IMTDatasetField_EnFieldId = 1044;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_ADDRESS: IMTDatasetField_EnFieldId = 1045;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_COMPANY_WEBSITE: IMTDatasetField_EnFieldId = 1046;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_PREFERRED: IMTDatasetField_EnFieldId =
    1047;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_LANGUAGE: IMTDatasetField_EnFieldId = 1048;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_EMAIL: IMTDatasetField_EnFieldId = 1049;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_PHONE: IMTDatasetField_EnFieldId = 1050;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_MESSENGERS: IMTDatasetField_EnFieldId =
    1051;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_SOCIALNETWORKS: IMTDatasetField_EnFieldId =
    1052;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_CONTACT_LAST_DATE: IMTDatasetField_EnFieldId =
    1053;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ADDRESS_COUNTRY: IMTDatasetField_EnFieldId = 1054;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ADDRESS_POSTCODE: IMTDatasetField_EnFieldId = 1055;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ADDRESS_STREET: IMTDatasetField_EnFieldId = 1056;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ADDRESS_STATE: IMTDatasetField_EnFieldId = 1057;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_ADDRESS_CITY: IMTDatasetField_EnFieldId = 1058;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_EXPERIENCE_FX: IMTDatasetField_EnFieldId = 1059;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_EXPERIENCE_CFD: IMTDatasetField_EnFieldId = 1060;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_EXPERIENCE_FUTURES: IMTDatasetField_EnFieldId =
    1061;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_EXPERIENCE_STOCKS: IMTDatasetField_EnFieldId =
    1062;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_FIRST: IMTDatasetField_EnFieldId = 1001;
pub const IMTDatasetField_EnFieldId_FIELD_CLIENT_LAST: IMTDatasetField_EnFieldId = 1062;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_DEAL: IMTDatasetField_EnFieldId = 2001;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_EXTERNAL_ID: IMTDatasetField_EnFieldId = 2002;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_LOGIN: IMTDatasetField_EnFieldId = 2003;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_DEALER: IMTDatasetField_EnFieldId = 2004;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_ORDER: IMTDatasetField_EnFieldId = 2005;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_ACTION: IMTDatasetField_EnFieldId = 2006;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_ENTRY: IMTDatasetField_EnFieldId = 2007;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_DIGITS: IMTDatasetField_EnFieldId = 2008;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_DIGITS_CURRENCY: IMTDatasetField_EnFieldId = 2009;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_CONTRACT_SIZE: IMTDatasetField_EnFieldId = 2010;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_TIME: IMTDatasetField_EnFieldId = 2011;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_SYMBOL: IMTDatasetField_EnFieldId = 2012;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PRICE: IMTDatasetField_EnFieldId = 2013;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_VOLUME_EXT: IMTDatasetField_EnFieldId = 2014;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PROFIT: IMTDatasetField_EnFieldId = 2015;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_STORAGE: IMTDatasetField_EnFieldId = 2016;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_COMMISSION: IMTDatasetField_EnFieldId = 2017;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_RATE_PROFIT: IMTDatasetField_EnFieldId = 2018;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_RATE_MARGIN: IMTDatasetField_EnFieldId = 2019;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_EXPERT_ID: IMTDatasetField_EnFieldId = 2020;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_POSITION_ID: IMTDatasetField_EnFieldId = 2021;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_COMMENT: IMTDatasetField_EnFieldId = 2022;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PROFIT_RAW: IMTDatasetField_EnFieldId = 2023;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PRICE_POSITION: IMTDatasetField_EnFieldId = 2024;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_VOLUME_CLOSED_EXT: IMTDatasetField_EnFieldId = 2025;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_TICK_VALUE: IMTDatasetField_EnFieldId = 2026;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_TICK_SIZE: IMTDatasetField_EnFieldId = 2027;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_FLAGS: IMTDatasetField_EnFieldId = 2028;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_TIME_MSC: IMTDatasetField_EnFieldId = 2029;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_REASON: IMTDatasetField_EnFieldId = 2030;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_GATEWAY: IMTDatasetField_EnFieldId = 2031;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PRICE_GATEWAY: IMTDatasetField_EnFieldId = 2032;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_MODIFICATION_FLAGS: IMTDatasetField_EnFieldId = 2033;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PRICE_SL: IMTDatasetField_EnFieldId = 2034;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_PRICE_TP: IMTDatasetField_EnFieldId = 2035;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_FEE: IMTDatasetField_EnFieldId = 2036;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_VALUE: IMTDatasetField_EnFieldId = 2037;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_FIRST: IMTDatasetField_EnFieldId = 2001;
pub const IMTDatasetField_EnFieldId_FIELD_DEAL_LAST: IMTDatasetField_EnFieldId = 2037;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_ORDER: IMTDatasetField_EnFieldId = 3001;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_EXTERNAL_ID: IMTDatasetField_EnFieldId = 3002;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_LOGIN: IMTDatasetField_EnFieldId = 3003;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_DEALER: IMTDatasetField_EnFieldId = 3004;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_SYMBOL: IMTDatasetField_EnFieldId = 3005;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TIME_SETUP: IMTDatasetField_EnFieldId = 3006;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TIME_EXPIRATION: IMTDatasetField_EnFieldId = 3007;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TIME_DONE: IMTDatasetField_EnFieldId = 3008;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TYPE: IMTDatasetField_EnFieldId = 3009;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TYPE_FILL: IMTDatasetField_EnFieldId = 3010;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TYPE_TIME: IMTDatasetField_EnFieldId = 3011;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TYPE_REASON: IMTDatasetField_EnFieldId = 3012;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_PRICE_ORDER: IMTDatasetField_EnFieldId = 3013;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_PRICE_TRIGGER: IMTDatasetField_EnFieldId = 3014;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_PRICE_CURRENT: IMTDatasetField_EnFieldId = 3015;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_PRICE_SL: IMTDatasetField_EnFieldId = 3016;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_PRICE_TP: IMTDatasetField_EnFieldId = 3017;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_VOLUME_INITIAL: IMTDatasetField_EnFieldId = 3018;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_VOLUME_CURRENT: IMTDatasetField_EnFieldId = 3019;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_STATE: IMTDatasetField_EnFieldId = 3020;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_EXPERT_ID: IMTDatasetField_EnFieldId = 3021;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_POSITION_ID: IMTDatasetField_EnFieldId = 3022;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_COMMENT: IMTDatasetField_EnFieldId = 3023;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_CONTRACT_SIZE: IMTDatasetField_EnFieldId = 3024;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_DIGITS: IMTDatasetField_EnFieldId = 3025;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_DIGITS_CURRENCY: IMTDatasetField_EnFieldId = 3026;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_POSITION_BY_ID: IMTDatasetField_EnFieldId = 3027;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_MARGIN_RATE: IMTDatasetField_EnFieldId = 3028;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TIME_SETUP_MSC: IMTDatasetField_EnFieldId = 3029;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_TIME_DONE_MSC: IMTDatasetField_EnFieldId = 3030;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_MODIFICATION_FLAGS: IMTDatasetField_EnFieldId =
    3031;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_ACTIVATION_MODE: IMTDatasetField_EnFieldId = 3032;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_ACTIVATION_TIME: IMTDatasetField_EnFieldId = 3033;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_ACTIVATION_PRICE: IMTDatasetField_EnFieldId = 3034;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_ACTIVATION_FLAGS: IMTDatasetField_EnFieldId = 3035;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_GROUP: IMTDatasetField_EnFieldId = 3036;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_FIRST: IMTDatasetField_EnFieldId = 3001;
pub const IMTDatasetField_EnFieldId_FIELD_ORDER_LAST: IMTDatasetField_EnFieldId = 3036;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DATE_TIME: IMTDatasetField_EnFieldId = 4001;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DATE_TIME_PREV: IMTDatasetField_EnFieldId = 4002;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_LOGIN: IMTDatasetField_EnFieldId = 4003;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_NAME: IMTDatasetField_EnFieldId = 4004;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_GROUP: IMTDatasetField_EnFieldId = 4005;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_CURRENCY: IMTDatasetField_EnFieldId = 4006;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DIGITS_CURRENCY: IMTDatasetField_EnFieldId = 4007;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_COMPANY: IMTDatasetField_EnFieldId = 4008;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_EMAIL: IMTDatasetField_EnFieldId = 4009;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_BALANCE: IMTDatasetField_EnFieldId = 4010;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_CREDIT: IMTDatasetField_EnFieldId = 4011;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_INTEREST_RATE: IMTDatasetField_EnFieldId = 4012;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_COMMISSION_DAILY: IMTDatasetField_EnFieldId = 4013;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_COMMISSION_MONTHLY: IMTDatasetField_EnFieldId =
    4014;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_AGENT_DAILY: IMTDatasetField_EnFieldId = 4015;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_AGENT_MONTHLY: IMTDatasetField_EnFieldId = 4016;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_BALANCE_PREV_DAY: IMTDatasetField_EnFieldId = 4017;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_BALANCE_PREV_MONTH: IMTDatasetField_EnFieldId =
    4018;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_EQUITY_PREV_DAY: IMTDatasetField_EnFieldId = 4019;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_EQUITY_PREV_MONTH: IMTDatasetField_EnFieldId = 4020;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_MARGIN: IMTDatasetField_EnFieldId = 4021;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_MARGIN_FREE: IMTDatasetField_EnFieldId = 4022;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_MARGIN_LEVEL: IMTDatasetField_EnFieldId = 4023;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_MARGIN_LEVERAGE: IMTDatasetField_EnFieldId = 4024;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_PROFIT: IMTDatasetField_EnFieldId = 4025;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_PROFIT_STORAGE: IMTDatasetField_EnFieldId = 4026;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_PROFIT_COMMISSION: IMTDatasetField_EnFieldId = 4027;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_PROFIT_EQUITY: IMTDatasetField_EnFieldId = 4028;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_PROFIT: IMTDatasetField_EnFieldId = 4029;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_BALANCE: IMTDatasetField_EnFieldId = 4030;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_CREDIT: IMTDatasetField_EnFieldId = 4031;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_CHARGE: IMTDatasetField_EnFieldId = 4032;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_CORRECTION: IMTDatasetField_EnFieldId = 4033;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_BONUS: IMTDatasetField_EnFieldId = 4034;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_STORAGE: IMTDatasetField_EnFieldId = 4035;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_COMM_INSTANT: IMTDatasetField_EnFieldId =
    4036;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_COMM_ROUND: IMTDatasetField_EnFieldId = 4037;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_AGENT: IMTDatasetField_EnFieldId = 4038;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_DAILY_INTEREST: IMTDatasetField_EnFieldId = 4039;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_PROFIT_ASSETS: IMTDatasetField_EnFieldId = 4040;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_PROFIT_LIABILITIES: IMTDatasetField_EnFieldId =
    4041;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_FIRST: IMTDatasetField_EnFieldId = 4001;
pub const IMTDatasetField_EnFieldId_FIELD_DAILY_LAST: IMTDatasetField_EnFieldId = 4041;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_LOGIN: IMTDatasetField_EnFieldId = 5001;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_SYMBOL: IMTDatasetField_EnFieldId = 5002;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_ACTION: IMTDatasetField_EnFieldId = 5003;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_DIGITS: IMTDatasetField_EnFieldId = 5004;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_DIGITS_CURRENCY: IMTDatasetField_EnFieldId =
    5005;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_CONTRACT_SIZE: IMTDatasetField_EnFieldId = 5006;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_TIME_CREATE: IMTDatasetField_EnFieldId = 5007;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_TIME_UPDATE: IMTDatasetField_EnFieldId = 5008;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_PRICE_OPEN: IMTDatasetField_EnFieldId = 5009;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_PRICE_CURRENT: IMTDatasetField_EnFieldId = 5010;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_PRICE_SL: IMTDatasetField_EnFieldId = 5011;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_PRICE_TP: IMTDatasetField_EnFieldId = 5012;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_VOLUME: IMTDatasetField_EnFieldId = 5013;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_PROFIT: IMTDatasetField_EnFieldId = 5014;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_STORAGE: IMTDatasetField_EnFieldId = 5015;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_RATE_PROFIT: IMTDatasetField_EnFieldId = 5016;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_RATE_MARGIN: IMTDatasetField_EnFieldId = 5017;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_EXPERT_ID: IMTDatasetField_EnFieldId = 5018;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_EXPERT_POSITION_ID: IMTDatasetField_EnFieldId =
    5019;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_COMMENT: IMTDatasetField_EnFieldId = 5020;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_ACTIVATION_MODE: IMTDatasetField_EnFieldId =
    5021;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_ACTIVATION_TIME: IMTDatasetField_EnFieldId =
    5022;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_ACTIVATION_PRICE: IMTDatasetField_EnFieldId =
    5023;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_ACTIVATION_FLAGS: IMTDatasetField_EnFieldId =
    5024;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_TIME_CREATE_MSC: IMTDatasetField_EnFieldId =
    5025;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_TIME_UPDATE_MSC: IMTDatasetField_EnFieldId =
    5026;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_DEALER: IMTDatasetField_EnFieldId = 5027;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_POSITION: IMTDatasetField_EnFieldId = 5028;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_EXTERNAL_ID: IMTDatasetField_EnFieldId = 5029;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_MODIFICATION_FLAGS: IMTDatasetField_EnFieldId =
    5030;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_REASON: IMTDatasetField_EnFieldId = 5031;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_VOLUME_EXT: IMTDatasetField_EnFieldId = 5032;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_GROUP: IMTDatasetField_EnFieldId = 5033;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_FIRST: IMTDatasetField_EnFieldId = 5001;
pub const IMTDatasetField_EnFieldId_FIELD_POSITION_LAST: IMTDatasetField_EnFieldId = 5033;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_LOGIN: IMTDatasetField_EnFieldId = 6001;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_GROUP: IMTDatasetField_EnFieldId = 6002;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_CURRENCY_DIGITS: IMTDatasetField_EnFieldId = 6003;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_BALANCE: IMTDatasetField_EnFieldId = 6004;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_CREDIT: IMTDatasetField_EnFieldId = 6005;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_MARGIN: IMTDatasetField_EnFieldId = 6006;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_MARGIN_FREE: IMTDatasetField_EnFieldId = 6007;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_MARGIN_LEVEL: IMTDatasetField_EnFieldId = 6008;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_MARGIN_LEVERAGE: IMTDatasetField_EnFieldId = 6009;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_MARGIN_INITIAL: IMTDatasetField_EnFieldId = 6010;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_MARGIN_MAINTENANCE: IMTDatasetField_EnFieldId =
    6011;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_PROFIT: IMTDatasetField_EnFieldId = 6012;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_STORAGE: IMTDatasetField_EnFieldId = 6013;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_COMMISSION: IMTDatasetField_EnFieldId = 6014;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_FLOATING: IMTDatasetField_EnFieldId = 6015;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_EQUITY: IMTDatasetField_EnFieldId = 6016;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_BLOCKED_COMMISSION: IMTDatasetField_EnFieldId =
    6017;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_BLOCKED_PROFIT: IMTDatasetField_EnFieldId = 6018;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_ASSETS: IMTDatasetField_EnFieldId = 6019;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_LIABILITIES: IMTDatasetField_EnFieldId = 6020;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_STOP_OUT_ACTIVATION: IMTDatasetField_EnFieldId =
    6021;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_STOP_OUT_TIME: IMTDatasetField_EnFieldId = 6022;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_STOP_OUT_LEVEL: IMTDatasetField_EnFieldId = 6023;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_STOP_OUT_EQUITY: IMTDatasetField_EnFieldId = 6024;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_STOP_OUT_MARGIN: IMTDatasetField_EnFieldId = 6025;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_FIRST: IMTDatasetField_EnFieldId = 6001;
pub const IMTDatasetField_EnFieldId_FIELD_ACCOUNT_LAST: IMTDatasetField_EnFieldId = 6020;
pub const IMTDatasetField_EnFieldId_FIELD_FIRST: IMTDatasetField_EnFieldId = 1;
pub const IMTDatasetField_EnFieldId_FIELD_LAST: IMTDatasetField_EnFieldId = 6020;
pub type IMTDatasetField_EnFieldId = ::std::os::raw::c_int;
pub const IMTDatasetField_EnFieldFlags_FLAG_NONE: IMTDatasetField_EnFieldFlags = 0;
pub const IMTDatasetField_EnFieldFlags_FLAG_SELECT: IMTDatasetField_EnFieldFlags = 1;
pub const IMTDatasetField_EnFieldFlags_FLAG_DEFAULT: IMTDatasetField_EnFieldFlags = 1;
pub const IMTDatasetField_EnFieldFlags_FLAG_ALL: IMTDatasetField_EnFieldFlags = 1;
pub type IMTDatasetField_EnFieldFlags = ::std::os::raw::c_int;
pub const IMTDatasetField_EnGender_GENDER_UNSPECIFIED: IMTDatasetField_EnGender = 0;
pub const IMTDatasetField_EnGender_GENDER_MALE: IMTDatasetField_EnGender = 1;
pub const IMTDatasetField_EnGender_GENDER_FEMALE: IMTDatasetField_EnGender = 2;
pub const IMTDatasetField_EnGender_GENDER_FIRST: IMTDatasetField_EnGender = 0;
pub const IMTDatasetField_EnGender_GENDER_LAST: IMTDatasetField_EnGender = 2;
pub type IMTDatasetField_EnGender = ::std::os::raw::c_int;
pub const IMTDatasetField_EnClientType_CLIENT_TYPE_UNDEFINED: IMTDatasetField_EnClientType = 0;
pub const IMTDatasetField_EnClientType_CLIENT_TYPE_INDIVIDUAL: IMTDatasetField_EnClientType = 1;
pub const IMTDatasetField_EnClientType_CLIENT_TYPE_CORPORATE: IMTDatasetField_EnClientType = 2;
pub const IMTDatasetField_EnClientType_CLIENT_TYPE_FUND: IMTDatasetField_EnClientType = 3;
pub const IMTDatasetField_EnClientType_CLIENT_TYPE_FIRST: IMTDatasetField_EnClientType = 0;
pub const IMTDatasetField_EnClientType_CLIENT_TYPE_LAST: IMTDatasetField_EnClientType = 3;
pub type IMTDatasetField_EnClientType = ::std::os::raw::c_int;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_UNREGISTERED:
    IMTDatasetField_EnClientStatus = 0;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_REGISTERED: IMTDatasetField_EnClientStatus =
    100;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_NOTINTERESTED:
    IMTDatasetField_EnClientStatus = 200;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_APPLICATION_INCOMPLETED:
    IMTDatasetField_EnClientStatus = 300;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_APPLICATION_COMPLETED:
    IMTDatasetField_EnClientStatus = 400;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_APPLICATION_INFORMATION:
    IMTDatasetField_EnClientStatus = 500;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_APPLICATION_REJECTED:
    IMTDatasetField_EnClientStatus = 600;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_APPROVED: IMTDatasetField_EnClientStatus =
    700;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_FUNDED: IMTDatasetField_EnClientStatus = 800;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_ACTIVE: IMTDatasetField_EnClientStatus = 900;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_INACTIVE: IMTDatasetField_EnClientStatus =
    1000;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_SUSPENDED: IMTDatasetField_EnClientStatus =
    1100;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_CLOSED: IMTDatasetField_EnClientStatus =
    1200;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_TERMINATED: IMTDatasetField_EnClientStatus =
    1300;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_FIRST: IMTDatasetField_EnClientStatus = 0;
pub const IMTDatasetField_EnClientStatus_CLIENT_STATUS_LAST: IMTDatasetField_EnClientStatus = 1300;
pub type IMTDatasetField_EnClientStatus = ::std::os::raw::c_int;
pub const IMTDatasetField_EnEmployment_EMPLOY_UNEMPLOYED: IMTDatasetField_EnEmployment = 0;
pub const IMTDatasetField_EnEmployment_EMPLOY_EMPLOYED: IMTDatasetField_EnEmployment = 1;
pub const IMTDatasetField_EnEmployment_EMPLOY_SELF_EMPLOYED: IMTDatasetField_EnEmployment = 2;
pub const IMTDatasetField_EnEmployment_EMPLOY_RETIRED: IMTDatasetField_EnEmployment = 3;
pub const IMTDatasetField_EnEmployment_EMPLOY_STUDENT: IMTDatasetField_EnEmployment = 4;
pub const IMTDatasetField_EnEmployment_EMPLOY_OTHER: IMTDatasetField_EnEmployment = 5;
pub const IMTDatasetField_EnEmployment_EMPLOY_FIRST: IMTDatasetField_EnEmployment = 0;
pub const IMTDatasetField_EnEmployment_EMPLOY_LAST: IMTDatasetField_EnEmployment = 5;
pub type IMTDatasetField_EnEmployment = ::std::os::raw::c_int;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_NONE: IMTDatasetField_EnEmploymentIndustry =
    0;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_AGRICULTURE:
    IMTDatasetField_EnEmploymentIndustry = 1;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_CONSTRUCTION:
    IMTDatasetField_EnEmploymentIndustry = 2;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_MANAGEMENT:
    IMTDatasetField_EnEmploymentIndustry = 3;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_COMMUNICATION:
    IMTDatasetField_EnEmploymentIndustry = 4;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_EDUCATION:
    IMTDatasetField_EnEmploymentIndustry = 5;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_GOVERNMENT:
    IMTDatasetField_EnEmploymentIndustry = 6;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_HEALTHCARE:
    IMTDatasetField_EnEmploymentIndustry = 7;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_TOURISM:
    IMTDatasetField_EnEmploymentIndustry = 8;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_IT: IMTDatasetField_EnEmploymentIndustry =
    9;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_SECURITY:
    IMTDatasetField_EnEmploymentIndustry = 10;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_MANUFACTURING:
    IMTDatasetField_EnEmploymentIndustry = 11;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_MARKETING:
    IMTDatasetField_EnEmploymentIndustry = 12;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_SCIENCE:
    IMTDatasetField_EnEmploymentIndustry = 13;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_ENGINEERING:
    IMTDatasetField_EnEmploymentIndustry = 14;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_TRANSPORT:
    IMTDatasetField_EnEmploymentIndustry = 15;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_OTHER:
    IMTDatasetField_EnEmploymentIndustry = 16;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_FIRST:
    IMTDatasetField_EnEmploymentIndustry = 1;
pub const IMTDatasetField_EnEmploymentIndustry_INDUSTRY_LAST: IMTDatasetField_EnEmploymentIndustry =
    16;
pub type IMTDatasetField_EnEmploymentIndustry = ::std::os::raw::c_int;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_NONE: IMTDatasetField_EnEducationLevel =
    0;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_HIGH_SCHOOL:
    IMTDatasetField_EnEducationLevel = 1;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_BACHELOR:
    IMTDatasetField_EnEducationLevel = 2;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_MASTER:
    IMTDatasetField_EnEducationLevel = 3;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_PHD: IMTDatasetField_EnEducationLevel =
    4;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_OTHER: IMTDatasetField_EnEducationLevel =
    5;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_FIRST: IMTDatasetField_EnEducationLevel =
    0;
pub const IMTDatasetField_EnEducationLevel_EDUCATION_LEVEL_LAST: IMTDatasetField_EnEducationLevel =
    5;
pub type IMTDatasetField_EnEducationLevel = ::std::os::raw::c_int;
pub const IMTDatasetField_EnWealthSource_WEALTH_SOURCE_EMPLOYMENT: IMTDatasetField_EnWealthSource =
    0;
pub const IMTDatasetField_EnWealthSource_WEALTH_SOURCE_SAVINGS: IMTDatasetField_EnWealthSource = 1;
pub const IMTDatasetField_EnWealthSource_WEALTH_SOURCE_INHERITANCE: IMTDatasetField_EnWealthSource =
    2;
pub const IMTDatasetField_EnWealthSource_WEALTH_SOURCE_OTHER: IMTDatasetField_EnWealthSource = 3;
pub const IMTDatasetField_EnWealthSource_WEALTH_SOURCE_FIRST: IMTDatasetField_EnWealthSource = 0;
pub const IMTDatasetField_EnWealthSource_WEALTH_SOURCE_LAST: IMTDatasetField_EnWealthSource = 3;
pub type IMTDatasetField_EnWealthSource = ::std::os::raw::c_int;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_UNDEFINED:
    IMTDatasetField_EnPreferredCommunication = 0;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_EMAIL:
    IMTDatasetField_EnPreferredCommunication = 1;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_PHONE:
    IMTDatasetField_EnPreferredCommunication = 2;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_PHONE_SMS:
    IMTDatasetField_EnPreferredCommunication = 3;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_MESSENGER:
    IMTDatasetField_EnPreferredCommunication = 4;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_FIRST:
    IMTDatasetField_EnPreferredCommunication = 0;
pub const IMTDatasetField_EnPreferredCommunication_PREFERRED_COMMUNICATION_LAST:
    IMTDatasetField_EnPreferredCommunication = 4;
pub type IMTDatasetField_EnPreferredCommunication = ::std::os::raw::c_int;
pub const IMTDatasetField_EnTradingExperience_EXPERIENCE_LESS_1_YEAR:
    IMTDatasetField_EnTradingExperience = 0;
pub const IMTDatasetField_EnTradingExperience_EXPERIENCE_1_3_YEAR:
    IMTDatasetField_EnTradingExperience = 1;
pub const IMTDatasetField_EnTradingExperience_EXPERIENCE_ABOVE_3_YEAR:
    IMTDatasetField_EnTradingExperience = 2;
pub const IMTDatasetField_EnTradingExperience_EXPERIENCE_FIRST:
    IMTDatasetField_EnTradingExperience = 0;
pub const IMTDatasetField_EnTradingExperience_EXPERIENCE_LAST: IMTDatasetField_EnTradingExperience =
    2;
pub type IMTDatasetField_EnTradingExperience = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDatasetField"][::std::mem::size_of::<IMTDatasetField>() - 8usize];
    ["Alignment of IMTDatasetField"][::std::mem::align_of::<IMTDatasetField>() - 8usize];
};
#[repr(C)]
pub struct IMTDatasetRequest__bindgen_vtable {
    pub IMTDatasetRequest_Release: unsafe extern "C" fn(this: *mut IMTDatasetRequest),
    pub IMTDatasetRequest_Assign: unsafe extern "C" fn(
        this: *mut IMTDatasetRequest,
        request: *const IMTDatasetRequest,
    ) -> MTAPIRES,
    pub IMTDatasetRequest_Clear: unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_Reserved1: unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_Reserved2: unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_Reserved3: unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_Reserved4: unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_FieldCreate:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> *mut IMTDatasetField,
    pub IMTDatasetRequest_FieldAdd: unsafe extern "C" fn(
        this: *mut IMTDatasetRequest,
        field: *const IMTDatasetField,
    ) -> MTAPIRES,
    pub IMTDatasetRequest_FieldUpdate: unsafe extern "C" fn(
        this: *mut IMTDatasetRequest,
        pos: UINT,
        field: *const IMTDatasetField,
    ) -> MTAPIRES,
    pub IMTDatasetRequest_FieldDelete:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest, pos: UINT) -> MTAPIRES,
    pub IMTDatasetRequest_FieldClear:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_FieldShift: unsafe extern "C" fn(
        this: *mut IMTDatasetRequest,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTDatasetRequest_FieldTotal: unsafe extern "C" fn(this: *const IMTDatasetRequest) -> UINT,
    pub IMTDatasetRequest_FieldNext: unsafe extern "C" fn(
        this: *const IMTDatasetRequest,
        pos: UINT,
        field: *mut IMTDatasetField,
    ) -> MTAPIRES,
    pub IMTDatasetRequest_FieldCreateReference:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest, pos: UINT) -> *mut IMTDatasetField,
    pub IMTDatasetRequest_FieldReserved1:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_FieldReserved2:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_FieldReserved3:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest) -> MTAPIRES,
    pub IMTDatasetRequest_RowLimit1:
        unsafe extern "C" fn(this: *mut IMTDatasetRequest, rows: UINT) -> MTAPIRES,
    pub IMTDatasetRequest_RowLimit: unsafe extern "C" fn(this: *const IMTDatasetRequest) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDatasetRequest {
    pub vtable_: *const IMTDatasetRequest__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDatasetRequest"][::std::mem::size_of::<IMTDatasetRequest>() - 8usize];
    ["Alignment of IMTDatasetRequest"][::std::mem::align_of::<IMTDatasetRequest>() - 8usize];
};
#[repr(C)]
pub struct IMTConAddressRange__bindgen_vtable {
    pub IMTConAddressRange_Release: unsafe extern "C" fn(this: *mut IMTConAddressRange),
    pub IMTConAddressRange_Assign: unsafe extern "C" fn(
        this: *mut IMTConAddressRange,
        range: *const IMTConAddressRange,
    ) -> MTAPIRES,
    pub IMTConAddressRange_Clear: unsafe extern "C" fn(this: *mut IMTConAddressRange) -> MTAPIRES,
    pub IMTConAddressRange_From1:
        unsafe extern "C" fn(this: *mut IMTConAddressRange, name: LPCWSTR) -> MTAPIRES,
    pub IMTConAddressRange_From: unsafe extern "C" fn(this: *const IMTConAddressRange) -> LPCWSTR,
    pub IMTConAddressRange_To1:
        unsafe extern "C" fn(this: *mut IMTConAddressRange, value: LPCWSTR) -> MTAPIRES,
    pub IMTConAddressRange_To: unsafe extern "C" fn(this: *const IMTConAddressRange) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConAddressRange {
    pub vtable_: *const IMTConAddressRange__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConAddressRange"][::std::mem::size_of::<IMTConAddressRange>() - 8usize];
    ["Alignment of IMTConAddressRange"][::std::mem::align_of::<IMTConAddressRange>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerAntiDDoS__bindgen_vtable {
    pub IMTConServerAntiDDoS_Release: unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS),
    pub IMTConServerAntiDDoS_Assign: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        param: *const IMTConServerAntiDDoS,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_Clear:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS) -> MTAPIRES,
    pub IMTConServerAntiDDoS_Priority1:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, priority: UINT) -> MTAPIRES,
    pub IMTConServerAntiDDoS_Priority:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS) -> UINT,
    pub IMTConServerAntiDDoS_AccessMask1:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, mask: UINT) -> MTAPIRES,
    pub IMTConServerAntiDDoS_AccessMask:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS) -> UINT,
    pub IMTConServerAntiDDoS_PointsAdd:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, path: LPCWSTR) -> MTAPIRES,
    pub IMTConServerAntiDDoS_PointsUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        pos: UINT,
        address: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_PointsDelete:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, pos: UINT) -> MTAPIRES,
    pub IMTConServerAntiDDoS_PointsClear:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS) -> MTAPIRES,
    pub IMTConServerAntiDDoS_PointsShift: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_PointsTotal:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS) -> UINT,
    pub IMTConServerAntiDDoS_PointsNext:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS, pos: UINT) -> LPCWSTR,
    pub IMTConServerAntiDDoS_ServersAdd:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, server_id: UINT64) -> MTAPIRES,
    pub IMTConServerAntiDDoS_ServersUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        pos: UINT,
        server_id: UINT64,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_ServersDelete:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, pos: UINT) -> MTAPIRES,
    pub IMTConServerAntiDDoS_ServersClear:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS) -> MTAPIRES,
    pub IMTConServerAntiDDoS_ServersShift: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_ServersTotal:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS) -> UINT,
    pub IMTConServerAntiDDoS_ServersNext:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS, pos: UINT) -> UINT64,
    pub IMTConServerAntiDDoS_SourcesAdd: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        range: *mut IMTConAddressRange,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_SourcesUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        pos: UINT,
        range: *const IMTConAddressRange,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_SourcesDelete:
        unsafe extern "C" fn(this: *mut IMTConServerAntiDDoS, pos: UINT) -> MTAPIRES,
    pub IMTConServerAntiDDoS_SourcesShift: unsafe extern "C" fn(
        this: *mut IMTConServerAntiDDoS,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerAntiDDoS_SourcesTotal:
        unsafe extern "C" fn(this: *const IMTConServerAntiDDoS) -> UINT,
    pub IMTConServerAntiDDoS_SourcesNext: unsafe extern "C" fn(
        this: *const IMTConServerAntiDDoS,
        pos: UINT,
        access: *mut IMTConAddressRange,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServerAntiDDoS {
    pub vtable_: *const IMTConServerAntiDDoS__bindgen_vtable,
}
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_CLIENT: IMTConServerAntiDDoS_EnAccessMask =
    1;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_MANAGER:
    IMTConServerAntiDDoS_EnAccessMask = 2;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_ADMIN: IMTConServerAntiDDoS_EnAccessMask =
    4;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_CLIENT_API:
    IMTConServerAntiDDoS_EnAccessMask = 8;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_MANAGER_API:
    IMTConServerAntiDDoS_EnAccessMask = 16;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_WEB_API:
    IMTConServerAntiDDoS_EnAccessMask = 32;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_NONE: IMTConServerAntiDDoS_EnAccessMask =
    0;
pub const IMTConServerAntiDDoS_EnAccessMask_ACCESS_ALLOW_ALL: IMTConServerAntiDDoS_EnAccessMask =
    63;
pub type IMTConServerAntiDDoS_EnAccessMask = ::std::os::raw::c_int;
pub const IMTConServerAntiDDoS_EnServerPriority_PRIORITY_HIGHEST:
    IMTConServerAntiDDoS_EnServerPriority = 0;
pub const IMTConServerAntiDDoS_EnServerPriority_PRIORITY_LOWEST:
    IMTConServerAntiDDoS_EnServerPriority = 15;
pub const IMTConServerAntiDDoS_EnServerPriority_PRIORITY_IDLE:
    IMTConServerAntiDDoS_EnServerPriority = 255;
pub const IMTConServerAntiDDoS_EnServerPriority_PRIORITY_FIRST:
    IMTConServerAntiDDoS_EnServerPriority = 0;
pub const IMTConServerAntiDDoS_EnServerPriority_PRIORITY_LAST:
    IMTConServerAntiDDoS_EnServerPriority = 255;
pub type IMTConServerAntiDDoS_EnServerPriority = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerAntiDDoS"][::std::mem::size_of::<IMTConServerAntiDDoS>() - 8usize];
    ["Alignment of IMTConServerAntiDDoS"][::std::mem::align_of::<IMTConServerAntiDDoS>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerBackup__bindgen_vtable {
    pub IMTConServerBackup_Release: unsafe extern "C" fn(this: *mut IMTConServerBackup),
    pub IMTConServerBackup_Assign: unsafe extern "C" fn(
        this: *mut IMTConServerBackup,
        param: *const IMTConServerBackup,
    ) -> MTAPIRES,
    pub IMTConServerBackup_Clear: unsafe extern "C" fn(this: *mut IMTConServerBackup) -> MTAPIRES,
    pub IMTConServerBackup_MasterServer1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, id: UINT64) -> MTAPIRES,
    pub IMTConServerBackup_MasterServer:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT64,
    pub IMTConServerBackup_BackupPath1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, path: LPCWSTR) -> MTAPIRES,
    pub IMTConServerBackup_BackupPath:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> LPCWSTR,
    pub IMTConServerBackup_BackupFullTime1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, time: UINT) -> MTAPIRES,
    pub IMTConServerBackup_BackupFullTime:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT,
    pub IMTConServerBackup_BackupPeriod1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, period: UINT) -> MTAPIRES,
    pub IMTConServerBackup_BackupPeriod:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT,
    pub IMTConServerBackup_BackupTTL1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, period: UINT) -> MTAPIRES,
    pub IMTConServerBackup_BackupTTL: unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT,
    pub IMTConServerBackup_BackupFlags1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, flags: UINT64) -> MTAPIRES,
    pub IMTConServerBackup_BackupFlags:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT64,
    pub IMTConServerBackup_SQLExportMode1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, mode: UINT) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportMode:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT,
    pub IMTConServerBackup_SQLExportFlags1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, flags: UINT64) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportFlags:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT64,
    pub IMTConServerBackup_SQLExportPeriod1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, period: UINT) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportPeriod:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> UINT,
    pub IMTConServerBackup_SQLExportServer1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, server: LPCWSTR) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportServer:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> LPCWSTR,
    pub IMTConServerBackup_SQLExportLogin1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, login: LPCWSTR) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportLogin:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> LPCWSTR,
    pub IMTConServerBackup_SQLExportPassword1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, password: LPCWSTR) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportPassword:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> LPCWSTR,
    pub IMTConServerBackup_SQLExportFolder1:
        unsafe extern "C" fn(this: *mut IMTConServerBackup, folder: LPCWSTR) -> MTAPIRES,
    pub IMTConServerBackup_SQLExportFolder:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> LPCWSTR,
    pub IMTConServerBackup_BackupLastSync:
        unsafe extern "C" fn(this: *const IMTConServerBackup) -> INT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServerBackup {
    pub vtable_: *const IMTConServerBackup__bindgen_vtable,
}
pub const IMTConServerBackup_EnBackupFlags_FLAG_ENABLE_BACKUPS: IMTConServerBackup_EnBackupFlags =
    1;
pub const IMTConServerBackup_EnBackupFlags_FLAG_ENABLE_TICKS: IMTConServerBackup_EnBackupFlags = 2;
pub const IMTConServerBackup_EnBackupFlags_FLAG_ENABLE_FAILOVER: IMTConServerBackup_EnBackupFlags =
    4;
pub const IMTConServerBackup_EnBackupFlags_FLAG_ENABLE_LOGS: IMTConServerBackup_EnBackupFlags = 8;
pub type IMTConServerBackup_EnBackupFlags = ::std::os::raw::c_int;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_DISABLED: IMTConServerBackup_EnBackupPeriod = 0;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_15MINUTES: IMTConServerBackup_EnBackupPeriod = 1;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_30MINUTES: IMTConServerBackup_EnBackupPeriod = 2;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_1HOUR: IMTConServerBackup_EnBackupPeriod = 3;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_4HOURS: IMTConServerBackup_EnBackupPeriod = 4;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_1DAY: IMTConServerBackup_EnBackupPeriod = 5;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_FIRST: IMTConServerBackup_EnBackupPeriod = 0;
pub const IMTConServerBackup_EnBackupPeriod_BACKUP_LAST: IMTConServerBackup_EnBackupPeriod = 5;
pub type IMTConServerBackup_EnBackupPeriod = ::std::os::raw::c_int;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_1DAY: IMTConServerBackup_EnBackupTTL = 1;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_3DAYS: IMTConServerBackup_EnBackupTTL = 2;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_1WEEK: IMTConServerBackup_EnBackupTTL = 3;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_1MONTH: IMTConServerBackup_EnBackupTTL = 4;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_3MONTHS: IMTConServerBackup_EnBackupTTL = 5;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_6MONTHS: IMTConServerBackup_EnBackupTTL = 6;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_FIRST: IMTConServerBackup_EnBackupTTL = 1;
pub const IMTConServerBackup_EnBackupTTL_BACKUP_TTL_LAST: IMTConServerBackup_EnBackupTTL = 6;
pub type IMTConServerBackup_EnBackupTTL = ::std::os::raw::c_int;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_NONE: IMTConServerBackup_EnSQLExportMode = 0;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_MSSQL: IMTConServerBackup_EnSQLExportMode = 1;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_FIREBIRD: IMTConServerBackup_EnSQLExportMode =
    2;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_MYSQL: IMTConServerBackup_EnSQLExportMode = 3;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_ORACLE: IMTConServerBackup_EnSQLExportMode =
    4;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_POSTGRESQL:
    IMTConServerBackup_EnSQLExportMode = 5;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_FIRST: IMTConServerBackup_EnSQLExportMode = 0;
pub const IMTConServerBackup_EnSQLExportMode_SQL_MODE_LAST: IMTConServerBackup_EnSQLExportMode = 5;
pub type IMTConServerBackup_EnSQLExportMode = ::std::os::raw::c_int;
pub const IMTConServerBackup_EnSQLExportFlags_SQL_FLAG_NONE: IMTConServerBackup_EnSQLExportFlags =
    0;
pub const IMTConServerBackup_EnSQLExportFlags_SQL_FLAG_PARTITIONS:
    IMTConServerBackup_EnSQLExportFlags = 1;
pub const IMTConServerBackup_EnSQLExportFlags_SQL_FLAG_SKIP_DEMO:
    IMTConServerBackup_EnSQLExportFlags = 2;
pub const IMTConServerBackup_EnSQLExportFlags_SQL_FLAG_ALL: IMTConServerBackup_EnSQLExportFlags = 3;
pub type IMTConServerBackup_EnSQLExportFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerBackup"][::std::mem::size_of::<IMTConServerBackup>() - 8usize];
    ["Alignment of IMTConServerBackup"][::std::mem::align_of::<IMTConServerBackup>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerAccess__bindgen_vtable {
    pub IMTConServerAccess_Release: unsafe extern "C" fn(this: *mut IMTConServerAccess),
    pub IMTConServerAccess_Assign: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        param: *const IMTConServerAccess,
    ) -> MTAPIRES,
    pub IMTConServerAccess_Clear: unsafe extern "C" fn(this: *mut IMTConServerAccess) -> MTAPIRES,
    pub IMTConServerAccess_Priority1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, priority: UINT) -> MTAPIRES,
    pub IMTConServerAccess_Priority: unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_AccessMask1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, mask: UINT) -> MTAPIRES,
    pub IMTConServerAccess_PriorityCurrent:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_AccessMask:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_NewsMax1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, news_max: UINT) -> MTAPIRES,
    pub IMTConServerAccess_NewsMax: unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_AntifloodEnabled1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, enabled: UINT) -> MTAPIRES,
    pub IMTConServerAccess_AntifloodEnabled:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_AntifloodConnects1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, connects: UINT) -> MTAPIRES,
    pub IMTConServerAccess_AntifloodConnects:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_AntifloodErrors1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, errors: UINT) -> MTAPIRES,
    pub IMTConServerAccess_AntifloodErrors:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_PointsAdd:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, path: LPCWSTR) -> MTAPIRES,
    pub IMTConServerAccess_PointsUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        pos: UINT,
        address: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTConServerAccess_PointsDelete:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, pos: UINT) -> MTAPIRES,
    pub IMTConServerAccess_PointsClear:
        unsafe extern "C" fn(this: *mut IMTConServerAccess) -> MTAPIRES,
    pub IMTConServerAccess_PointsShift: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerAccess_PointsTotal:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_PointsNext:
        unsafe extern "C" fn(this: *const IMTConServerAccess, pos: UINT) -> LPCWSTR,
    pub IMTConServerAccess_BindingsAdd:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, path: LPCWSTR) -> MTAPIRES,
    pub IMTConServerAccess_BindingsUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        pos: UINT,
        address: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTConServerAccess_BindingsDelete:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, pos: UINT) -> MTAPIRES,
    pub IMTConServerAccess_BindingsClear:
        unsafe extern "C" fn(this: *mut IMTConServerAccess) -> MTAPIRES,
    pub IMTConServerAccess_BindingsShift: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerAccess_BindingsTotal:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_BindingsNext:
        unsafe extern "C" fn(this: *const IMTConServerAccess, pos: UINT) -> LPCWSTR,
    pub IMTConServerAccess_ServersAdd:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, server_id: UINT64) -> MTAPIRES,
    pub IMTConServerAccess_ServersUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        pos: UINT,
        server_id: UINT64,
    ) -> MTAPIRES,
    pub IMTConServerAccess_ServersDelete:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, pos: UINT) -> MTAPIRES,
    pub IMTConServerAccess_ServersClear:
        unsafe extern "C" fn(this: *mut IMTConServerAccess) -> MTAPIRES,
    pub IMTConServerAccess_ServersShift: unsafe extern "C" fn(
        this: *mut IMTConServerAccess,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerAccess_ServersTotal:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
    pub IMTConServerAccess_ServersNext:
        unsafe extern "C" fn(this: *const IMTConServerAccess, pos: UINT) -> UINT64,
    pub IMTConServerAccess_AccessFlags1:
        unsafe extern "C" fn(this: *mut IMTConServerAccess, flags: UINT) -> MTAPIRES,
    pub IMTConServerAccess_AccessFlags:
        unsafe extern "C" fn(this: *const IMTConServerAccess) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServerAccess {
    pub vtable_: *const IMTConServerAccess__bindgen_vtable,
}
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_CLIENT: IMTConServerAccess_EnAccessMask = 1;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_MANAGER: IMTConServerAccess_EnAccessMask = 2;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_ADMIN: IMTConServerAccess_EnAccessMask = 4;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_RESERVED: IMTConServerAccess_EnAccessMask =
    8;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_MANAGER_API:
    IMTConServerAccess_EnAccessMask = 16;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_WEB_API: IMTConServerAccess_EnAccessMask =
    32;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_NONE: IMTConServerAccess_EnAccessMask = 0;
pub const IMTConServerAccess_EnAccessMask_ACCESS_ALLOW_ALL: IMTConServerAccess_EnAccessMask = 63;
pub type IMTConServerAccess_EnAccessMask = ::std::os::raw::c_int;
pub const IMTConServerAccess_EnAccessFlags_ACCESS_FLAGS_INVISIBLE:
    IMTConServerAccess_EnAccessFlags = 1;
pub const IMTConServerAccess_EnAccessFlags_ACCESS_FLAGS_NONE: IMTConServerAccess_EnAccessFlags = 0;
pub const IMTConServerAccess_EnAccessFlags_ACCESS_FLAGS_ALL: IMTConServerAccess_EnAccessFlags = 1;
pub type IMTConServerAccess_EnAccessFlags = ::std::os::raw::c_int;
pub const IMTConServerAccess_EnServerPriority_PRIORITY_HIGHEST:
    IMTConServerAccess_EnServerPriority = 0;
pub const IMTConServerAccess_EnServerPriority_PRIORITY_LOWEST: IMTConServerAccess_EnServerPriority =
    15;
pub const IMTConServerAccess_EnServerPriority_PRIORITY_IDLE: IMTConServerAccess_EnServerPriority =
    255;
pub const IMTConServerAccess_EnServerPriority_PRIORITY_FIRST: IMTConServerAccess_EnServerPriority =
    0;
pub const IMTConServerAccess_EnServerPriority_PRIORITY_LAST: IMTConServerAccess_EnServerPriority =
    255;
pub type IMTConServerAccess_EnServerPriority = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerAccess"][::std::mem::size_of::<IMTConServerAccess>() - 8usize];
    ["Alignment of IMTConServerAccess"][::std::mem::align_of::<IMTConServerAccess>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerHistory__bindgen_vtable {
    pub IMTConServerHistory_Release: unsafe extern "C" fn(this: *mut IMTConServerHistory),
    pub IMTConServerHistory_Assign: unsafe extern "C" fn(
        this: *mut IMTConServerHistory,
        param: *const IMTConServerHistory,
    ) -> MTAPIRES,
    pub IMTConServerHistory_Clear: unsafe extern "C" fn(this: *mut IMTConServerHistory) -> MTAPIRES,
    pub IMTConServerHistory_DatafeedsTimeout1:
        unsafe extern "C" fn(this: *mut IMTConServerHistory, timeout: UINT) -> MTAPIRES,
    pub IMTConServerHistory_DatafeedsTimeout:
        unsafe extern "C" fn(this: *const IMTConServerHistory) -> UINT,
    pub IMTConServerHistory_NewsMax1:
        unsafe extern "C" fn(this: *mut IMTConServerHistory, news_max: UINT) -> MTAPIRES,
    pub IMTConServerHistory_NewsMax: unsafe extern "C" fn(this: *const IMTConServerHistory) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServerHistory {
    pub vtable_: *const IMTConServerHistory__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerHistory"][::std::mem::size_of::<IMTConServerHistory>() - 8usize];
    ["Alignment of IMTConServerHistory"][::std::mem::align_of::<IMTConServerHistory>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerRange__bindgen_vtable {
    pub IMTConServerRange_Release: unsafe extern "C" fn(this: *mut IMTConServerRange),
    pub IMTConServerRange_Assign: unsafe extern "C" fn(
        this: *mut IMTConServerRange,
        param: *const IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerRange_Clear: unsafe extern "C" fn(this: *mut IMTConServerRange) -> MTAPIRES,
    pub IMTConServerRange_From1:
        unsafe extern "C" fn(this: *mut IMTConServerRange, from: UINT64) -> MTAPIRES,
    pub IMTConServerRange_From: unsafe extern "C" fn(this: *const IMTConServerRange) -> UINT64,
    pub IMTConServerRange_To1:
        unsafe extern "C" fn(this: *mut IMTConServerRange, to: UINT64) -> MTAPIRES,
    pub IMTConServerRange_To: unsafe extern "C" fn(this: *const IMTConServerRange) -> UINT64,
    pub IMTConServerRange_UsedFrom: unsafe extern "C" fn(this: *const IMTConServerRange) -> UINT64,
    pub IMTConServerRange_UsedTo: unsafe extern "C" fn(this: *const IMTConServerRange) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServerRange {
    pub vtable_: *const IMTConServerRange__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerRange"][::std::mem::size_of::<IMTConServerRange>() - 8usize];
    ["Alignment of IMTConServerRange"][::std::mem::align_of::<IMTConServerRange>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerTrade__bindgen_vtable {
    pub IMTConServerTrade_Release: unsafe extern "C" fn(this: *mut IMTConServerTrade),
    pub IMTConServerTrade_Assign: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        param: *const IMTConServerTrade,
    ) -> MTAPIRES,
    pub IMTConServerTrade_Clear: unsafe extern "C" fn(this: *mut IMTConServerTrade) -> MTAPIRES,
    pub IMTConServerTrade_DemoMode1:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, mode: UINT) -> MTAPIRES,
    pub IMTConServerTrade_DemoMode: unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_DemoPeriod1:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, period: UINT) -> MTAPIRES,
    pub IMTConServerTrade_DemoPeriod: unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_OvernightMode1:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, mode: UINT) -> MTAPIRES,
    pub IMTConServerTrade_OvernightMode:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_OvernightTime1:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, time: UINT) -> MTAPIRES,
    pub IMTConServerTrade_OvernightTime:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_OvernightTimeLast:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> INT64,
    pub IMTConServerTrade_OvernightTimePrev:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> INT64,
    pub IMTConServerTrade_OvermonthMode1:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, mode: UINT) -> MTAPIRES,
    pub IMTConServerTrade_OvermonthMode:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_OvermonthTimeLast:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> INT64,
    pub IMTConServerTrade_OvermonthTimePrev:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> INT64,
    pub IMTConServerTrade_LoginsRangeAdd: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        range: *mut IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_LoginsRangeUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        pos: UINT,
        range: *const IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_LoginsRangeDelete:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, pos: UINT) -> MTAPIRES,
    pub IMTConServerTrade_LoginsRangeClear:
        unsafe extern "C" fn(this: *mut IMTConServerTrade) -> MTAPIRES,
    pub IMTConServerTrade_LoginsRangeShift: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerTrade_LoginsRangeTotal:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_LoginsRangeNext: unsafe extern "C" fn(
        this: *const IMTConServerTrade,
        pos: UINT,
        range: *mut IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_OrdersRangeAdd: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        range: *mut IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_OrdersRangeUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        pos: UINT,
        range: *const IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_OrdersRangeDelete:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, pos: UINT) -> MTAPIRES,
    pub IMTConServerTrade_OrdersRangeClear:
        unsafe extern "C" fn(this: *mut IMTConServerTrade) -> MTAPIRES,
    pub IMTConServerTrade_OrdersRangeShift: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerTrade_OrdersRangeTotal:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_OrdersRangeNext: unsafe extern "C" fn(
        this: *const IMTConServerTrade,
        pos: UINT,
        range: *mut IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_DealsRangeAdd: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        range: *mut IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_DealsRangeUpdate: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        pos: UINT,
        range: *const IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_DealsRangeDelete:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, pos: UINT) -> MTAPIRES,
    pub IMTConServerTrade_DealsRangeClear:
        unsafe extern "C" fn(this: *mut IMTConServerTrade) -> MTAPIRES,
    pub IMTConServerTrade_DealsRangeShift: unsafe extern "C" fn(
        this: *mut IMTConServerTrade,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServerTrade_DealsRangeTotal:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_DealsRangeNext: unsafe extern "C" fn(
        this: *const IMTConServerTrade,
        pos: UINT,
        range: *mut IMTConServerRange,
    ) -> MTAPIRES,
    pub IMTConServerTrade_TotalUsers: unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_TotalUsersReal:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_TotalDeals: unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_TotalOrders: unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_TotalOrdersHistory:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_TotalPositions:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
    pub IMTConServerTrade_OvernightDays1:
        unsafe extern "C" fn(this: *mut IMTConServerTrade, days: UINT) -> MTAPIRES,
    pub IMTConServerTrade_OvernightDays:
        unsafe extern "C" fn(this: *const IMTConServerTrade) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServerTrade {
    pub vtable_: *const IMTConServerTrade__bindgen_vtable,
}
pub const IMTConServerTrade_EnDemoMode_DEMO_DISABLED: IMTConServerTrade_EnDemoMode = 0;
pub const IMTConServerTrade_EnDemoMode_DEMO_PROLONG: IMTConServerTrade_EnDemoMode = 1;
pub const IMTConServerTrade_EnDemoMode_DEMO_FIXED: IMTConServerTrade_EnDemoMode = 2;
pub const IMTConServerTrade_EnDemoMode_DEMO_FIRST: IMTConServerTrade_EnDemoMode = 0;
pub const IMTConServerTrade_EnDemoMode_DEMO_LAST: IMTConServerTrade_EnDemoMode = 2;
pub type IMTConServerTrade_EnDemoMode = ::std::os::raw::c_int;
pub const IMTConServerTrade_EnOvernightMode_OVERNIGHT_END_DAY: IMTConServerTrade_EnOvernightMode =
    0;
pub const IMTConServerTrade_EnOvernightMode_OVERNIGHT_START_DAY: IMTConServerTrade_EnOvernightMode =
    1;
pub const IMTConServerTrade_EnOvernightMode_OVERNIGHT_FIRST: IMTConServerTrade_EnOvernightMode = 0;
pub const IMTConServerTrade_EnOvernightMode_OVERNIGHT_LAST: IMTConServerTrade_EnOvernightMode = 1;
pub type IMTConServerTrade_EnOvernightMode = ::std::os::raw::c_int;
pub const IMTConServerTrade_EnOvermonthMode_OVERMONTH_LAST_DAY: IMTConServerTrade_EnOvermonthMode =
    0;
pub const IMTConServerTrade_EnOvermonthMode_OVERMONTH_FIRST_DAY: IMTConServerTrade_EnOvermonthMode =
    1;
pub const IMTConServerTrade_EnOvermonthMode_OVERMONTH_FIRST: IMTConServerTrade_EnOvermonthMode = 0;
pub const IMTConServerTrade_EnOvermonthMode_OVERMONTH_LAST: IMTConServerTrade_EnOvermonthMode = 1;
pub type IMTConServerTrade_EnOvermonthMode = ::std::os::raw::c_int;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_SUN: IMTConServerTrade_EnOvernightDays =
    1;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_MON: IMTConServerTrade_EnOvernightDays =
    2;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_TUE: IMTConServerTrade_EnOvernightDays =
    4;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_WED: IMTConServerTrade_EnOvernightDays =
    8;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_THU: IMTConServerTrade_EnOvernightDays =
    16;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_FRI: IMTConServerTrade_EnOvernightDays =
    32;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_SAT: IMTConServerTrade_EnOvernightDays =
    64;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_SUN:
    IMTConServerTrade_EnOvernightDays = 128;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_MON:
    IMTConServerTrade_EnOvernightDays = 256;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_TUE:
    IMTConServerTrade_EnOvernightDays = 512;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_WED:
    IMTConServerTrade_EnOvernightDays = 1024;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_THU:
    IMTConServerTrade_EnOvernightDays = 2048;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_FRI:
    IMTConServerTrade_EnOvernightDays = 4096;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ROLLOVER_SAT:
    IMTConServerTrade_EnOvernightDays = 8192;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_NONE: IMTConServerTrade_EnOvernightDays =
    0;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_DEFAULT:
    IMTConServerTrade_EnOvernightDays = 7998;
pub const IMTConServerTrade_EnOvernightDays_OVERNIGHT_DAYS_ALL: IMTConServerTrade_EnOvernightDays =
    16383;
pub type IMTConServerTrade_EnOvernightDays = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerTrade"][::std::mem::size_of::<IMTConServerTrade>() - 8usize];
    ["Alignment of IMTConServerTrade"][::std::mem::align_of::<IMTConServerTrade>() - 8usize];
};
#[repr(C)]
pub struct IMTConClusterState__bindgen_vtable {
    pub IMTConClusterState_Release: unsafe extern "C" fn(this: *mut IMTConClusterState),
    pub IMTConClusterState_Assign: unsafe extern "C" fn(
        this: *mut IMTConClusterState,
        state: *const IMTConClusterState,
    ) -> MTAPIRES,
    pub IMTConClusterState_Clear: unsafe extern "C" fn(this: *mut IMTConClusterState) -> MTAPIRES,
    pub IMTConClusterState_Id: unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT64,
    pub IMTConClusterState_Connected: unsafe extern "C" fn(this: *const IMTConClusterState) -> bool,
    pub IMTConClusterState_ConnectedAddress:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> LPCWSTR,
    pub IMTConClusterState_ConnectedTime:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> INT64,
    pub IMTConClusterState_StatsDay: unsafe extern "C" fn(this: *const IMTConClusterState) -> INT64,
    pub IMTConClusterState_StatsPing: unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT,
    pub IMTConClusterState_StatsPingMin:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT,
    pub IMTConClusterState_StatsPingMax:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT,
    pub IMTConClusterState_StatsSpeed:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT,
    pub IMTConClusterState_StatsSpeedMin:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT,
    pub IMTConClusterState_StatsSpeedMax:
        unsafe extern "C" fn(this: *const IMTConClusterState) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConClusterState {
    pub vtable_: *const IMTConClusterState__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConClusterState"][::std::mem::size_of::<IMTConClusterState>() - 8usize];
    ["Alignment of IMTConClusterState"][::std::mem::align_of::<IMTConClusterState>() - 8usize];
};
#[repr(C)]
pub struct IMTConServer__bindgen_vtable {
    pub IMTConServer_Release: unsafe extern "C" fn(this: *mut IMTConServer),
    pub IMTConServer_Assign:
        unsafe extern "C" fn(this: *mut IMTConServer, param: *const IMTConServer) -> MTAPIRES,
    pub IMTConServer_Clear: unsafe extern "C" fn(this: *mut IMTConServer) -> MTAPIRES,
    pub IMTConServer_Type1: unsafe extern "C" fn(this: *mut IMTConServer, type_: UINT) -> MTAPIRES,
    pub IMTConServer_Type: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_Name1:
        unsafe extern "C" fn(this: *mut IMTConServer, name: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_Name: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_Address1:
        unsafe extern "C" fn(this: *mut IMTConServer, name: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_Address: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_Id1: unsafe extern "C" fn(this: *mut IMTConServer, id: UINT64) -> MTAPIRES,
    pub IMTConServer_Id: unsafe extern "C" fn(this: *const IMTConServer) -> UINT64,
    pub IMTConServer_Password:
        unsafe extern "C" fn(this: *mut IMTConServer, password: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_PasswordCheck:
        unsafe extern "C" fn(this: *const IMTConServer, password: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_ServiceTime1:
        unsafe extern "C" fn(this: *mut IMTConServer, stime: UINT) -> MTAPIRES,
    pub IMTConServer_ServiceTime: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_AdaptersCurrent1:
        unsafe extern "C" fn(this: *mut IMTConServer, current: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_AdaptersCurrent: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_AdaptersTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_AdaptersNext:
        unsafe extern "C" fn(this: *const IMTConServer, pos: UINT) -> LPCWSTR,
    pub IMTConServer_AddressTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_AddressNext:
        unsafe extern "C" fn(this: *const IMTConServer, pos: UINT) -> UINT,
    pub IMTConServer_Version: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_Build: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_BuildDate: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_LastBootTime: unsafe extern "C" fn(this: *const IMTConServer) -> INT64,
    pub IMTConServer_Connected: unsafe extern "C" fn(this: *const IMTConServer) -> bool,
    pub IMTConServer_OS: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_CPU: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_CPUTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_CPUUsageMax: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_CPUUsageCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_MemoryTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_MemoryFree: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_MemoryFreeMin: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_MemoryFreeCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDFree: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDFreeCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDFragments: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDFragmentsCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDSpeedRead: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDSpeedReadCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDSpeedWrite: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_HDDSpeedWriteCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_ConnectsMax: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_ConnectsCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_NetworkMax: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_NetworkCritical: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_TradeServer:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut IMTConServerTrade,
    pub IMTConServer_HistoryServer:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut IMTConServerHistory,
    pub IMTConServer_AccessServer:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut IMTConServerAccess,
    pub IMTConServer_BackupServer:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut IMTConServerBackup,
    pub IMTConServer_AntiDDoSServer:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut IMTConServerAntiDDoS,
    pub IMTConServer_ReservedServer1:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void,
    pub IMTConServer_ReservedServer2:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void,
    pub IMTConServer_ReservedServer3:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void,
    pub IMTConServer_ReservedServer4:
        unsafe extern "C" fn(this: *mut IMTConServer) -> *mut ::std::os::raw::c_void,
    pub IMTConServer_PointsAdd:
        unsafe extern "C" fn(this: *mut IMTConServer, path: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_PointsUpdate:
        unsafe extern "C" fn(this: *mut IMTConServer, pos: UINT, address: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_PointsDelete:
        unsafe extern "C" fn(this: *mut IMTConServer, pos: UINT) -> MTAPIRES,
    pub IMTConServer_PointsClear: unsafe extern "C" fn(this: *mut IMTConServer) -> MTAPIRES,
    pub IMTConServer_PointsShift: unsafe extern "C" fn(
        this: *mut IMTConServer,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServer_PointsTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_PointsNext:
        unsafe extern "C" fn(this: *const IMTConServer, pos: UINT) -> LPCWSTR,
    pub IMTConServer_BindingsAdd:
        unsafe extern "C" fn(this: *mut IMTConServer, path: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_BindingsUpdate:
        unsafe extern "C" fn(this: *mut IMTConServer, pos: UINT, address: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_BindingsDelete:
        unsafe extern "C" fn(this: *mut IMTConServer, pos: UINT) -> MTAPIRES,
    pub IMTConServer_BindingsClear: unsafe extern "C" fn(this: *mut IMTConServer) -> MTAPIRES,
    pub IMTConServer_BindingsShift: unsafe extern "C" fn(
        this: *mut IMTConServer,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConServer_BindingsTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_BindingsNext:
        unsafe extern "C" fn(this: *const IMTConServer, pos: UINT) -> LPCWSTR,
    pub IMTConServer_FailoverMode1:
        unsafe extern "C" fn(this: *mut IMTConServer, mode: UINT) -> MTAPIRES,
    pub IMTConServer_FailoverMode: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_FailoverTimeout1:
        unsafe extern "C" fn(this: *mut IMTConServer, timeout: UINT) -> MTAPIRES,
    pub IMTConServer_FailoverTimeout: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_ClusterStateTotal: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_ClusterStateNext: unsafe extern "C" fn(
        this: *const IMTConServer,
        pos: UINT,
        state: *mut IMTConClusterState,
    ) -> MTAPIRES,
    pub IMTConServer_ClusterStateGet: unsafe extern "C" fn(
        this: *const IMTConServer,
        id: UINT64,
        state: *mut IMTConClusterState,
    ) -> MTAPIRES,
    pub IMTConServer_AddressIPv61:
        unsafe extern "C" fn(this: *mut IMTConServer, name: LPCWSTR) -> MTAPIRES,
    pub IMTConServer_AddressIPv6: unsafe extern "C" fn(this: *const IMTConServer) -> LPCWSTR,
    pub IMTConServer_AddressIPv6Total: unsafe extern "C" fn(this: *const IMTConServer) -> UINT,
    pub IMTConServer_AddressIPv6Next: unsafe extern "C" fn(
        this: *const IMTConServer,
        pos: UINT,
        address: *mut MTAPISTR,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConServer {
    pub vtable_: *const IMTConServer__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::con_server::MT5ConServer,
}
pub const IMTConServer_EnServerTypes_NET_MAIN_TRADE_SERVER: IMTConServer_EnServerTypes = 0;
pub const IMTConServer_EnServerTypes_NET_TRADE_SERVER: IMTConServer_EnServerTypes = 1;
pub const IMTConServer_EnServerTypes_NET_HISTORY_SERVER: IMTConServer_EnServerTypes = 2;
pub const IMTConServer_EnServerTypes_NET_ACCESS_SERVER: IMTConServer_EnServerTypes = 3;
pub const IMTConServer_EnServerTypes_NET_BACKUP_SERVER: IMTConServer_EnServerTypes = 4;
pub const IMTConServer_EnServerTypes_NET_OBSOLETE_SERVER_1: IMTConServer_EnServerTypes = 5;
pub const IMTConServer_EnServerTypes_NET_OBSOLETE_SERVER_2: IMTConServer_EnServerTypes = 6;
pub const IMTConServer_EnServerTypes_NET_ANTIDDOS_SERVER: IMTConServer_EnServerTypes = 7;
pub const IMTConServer_EnServerTypes_NET_SERVER_FIRST: IMTConServer_EnServerTypes = 0;
pub const IMTConServer_EnServerTypes_NET_SERVER_LAST: IMTConServer_EnServerTypes = 7;
pub type IMTConServer_EnServerTypes = ::std::os::raw::c_int;
pub const IMTConServer_EnFailoverModes_FAILOVER_MODE_DISABLED: IMTConServer_EnFailoverModes = 0;
pub const IMTConServer_EnFailoverModes_FAILOVER_MODE_BY_MOST: IMTConServer_EnFailoverModes = 1;
pub const IMTConServer_EnFailoverModes_FAILOVER_MODE_BY_ALL: IMTConServer_EnFailoverModes = 2;
pub const IMTConServer_EnFailoverModes_FAILOVER_MODE_FIRST: IMTConServer_EnFailoverModes = 0;
pub const IMTConServer_EnFailoverModes_FAILOVER_MODE_LAST: IMTConServer_EnFailoverModes = 2;
pub type IMTConServer_EnFailoverModes = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServer"][::std::mem::size_of::<IMTConServer>() - 24usize];
    ["Alignment of IMTConServer"][::std::mem::align_of::<IMTConServer>() - 8usize];
};
#[repr(C)]
pub struct IMTConTime__bindgen_vtable {
    pub IMTConTime_Release: unsafe extern "C" fn(this: *mut IMTConTime),
    pub IMTConTime_Assign:
        unsafe extern "C" fn(this: *mut IMTConTime, param: *mut IMTConTime) -> MTAPIRES,
    pub IMTConTime_Clear: unsafe extern "C" fn(this: *mut IMTConTime) -> MTAPIRES,
    pub IMTConTime_Daylight1: unsafe extern "C" fn(this: *mut IMTConTime, enable: bool) -> MTAPIRES,
    pub IMTConTime_Daylight: unsafe extern "C" fn(this: *const IMTConTime) -> bool,
    pub IMTConTime_TimeZone1:
        unsafe extern "C" fn(this: *mut IMTConTime, zone: ::std::os::raw::c_int) -> MTAPIRES,
    pub IMTConTime_TimeZone: unsafe extern "C" fn(this: *const IMTConTime) -> ::std::os::raw::c_int,
    pub IMTConTime_TimeServer1:
        unsafe extern "C" fn(this: *mut IMTConTime, server: LPCWSTR) -> MTAPIRES,
    pub IMTConTime_TimeServer: unsafe extern "C" fn(this: *const IMTConTime) -> LPCWSTR,
    pub IMTConTime_TimeTableGet: unsafe extern "C" fn(
        this: *const IMTConTime,
        wday: UINT,
        hour: UINT,
        mode: *mut UINT,
    ) -> MTAPIRES,
    pub IMTConTime_TimeTableSet:
        unsafe extern "C" fn(this: *mut IMTConTime, wday: UINT, hour: UINT, mode: UINT) -> MTAPIRES,
    pub IMTConTime_DaylightState:
        unsafe extern "C" fn(this: *const IMTConTime) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConTime {
    pub vtable_: *const IMTConTime__bindgen_vtable,
}
pub const IMTConTime_EnTimeTableMode_TIME_MODE_DISABLED: IMTConTime_EnTimeTableMode = 0;
pub const IMTConTime_EnTimeTableMode_TIME_MODE_ENABLED: IMTConTime_EnTimeTableMode = 1;
pub const IMTConTime_EnTimeTableMode_TIME_MODE_FIRST: IMTConTime_EnTimeTableMode = 0;
pub const IMTConTime_EnTimeTableMode_TIME_MODE_LAST: IMTConTime_EnTimeTableMode = 1;
pub type IMTConTime_EnTimeTableMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConTime"][::std::mem::size_of::<IMTConTime>() - 8usize];
    ["Alignment of IMTConTime"][::std::mem::align_of::<IMTConTime>() - 8usize];
};
#[repr(C)]
pub struct IMTConTimeSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConTimeSink {
    pub vtable_: *const IMTConTimeSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConTimeSink"][::std::mem::size_of::<IMTConTimeSink>() - 8usize];
    ["Alignment of IMTConTimeSink"][::std::mem::align_of::<IMTConTimeSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConServerSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConServerSink {
    pub vtable_: *const IMTConServerSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConServerSink"][::std::mem::size_of::<IMTConServerSink>() - 8usize];
    ["Alignment of IMTConServerSink"][::std::mem::align_of::<IMTConServerSink>() - 8usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct IMTConCommon__bindgen_vtable {
    pub IMTConCommon_Release: unsafe extern "C" fn(this: *mut IMTConCommon),
    pub IMTConCommon_Assign:
        unsafe extern "C" fn(this: *mut IMTConCommon, param: *const IMTConCommon) -> MTAPIRES,
    pub IMTConCommon_Clear: unsafe extern "C" fn(this: *mut IMTConCommon) -> MTAPIRES,
    pub IMTConCommon_Name1:
        unsafe extern "C" fn(this: *mut IMTConCommon, name: LPCWSTR) -> MTAPIRES,
    pub IMTConCommon_Name: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_NameFull: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_Owner: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_OwnerID: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_OwnerHost: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_OwnerEmail: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_Product: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_ExpirationLicense: unsafe extern "C" fn(this: *const IMTConCommon) -> INT64,
    pub IMTConCommon_ExpirationSupport: unsafe extern "C" fn(this: *const IMTConCommon) -> INT64,
    pub IMTConCommon_LimitTradeServers: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LimitWebServers: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LimitAccounts: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LimitDeals: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LimitGroups: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LiveUpdateMode: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LiveUpdateMode1:
        unsafe extern "C" fn(this: *mut IMTConCommon, mode: UINT) -> MTAPIRES,
    pub IMTConCommon_TotalUsers: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_TotalUsersReal: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_TotalDeals: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_TotalOrders: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_TotalOrdersHistory: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_TotalPositions: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_LimitSymbols: unsafe extern "C" fn(this: *const IMTConCommon) -> UINT,
    pub IMTConCommon_AccountURL1:
        unsafe extern "C" fn(this: *mut IMTConCommon, url: LPCWSTR) -> MTAPIRES,
    pub IMTConCommon_AccountURL: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_AccountDepositURL1:
        unsafe extern "C" fn(this: *mut IMTConCommon, url: LPCWSTR) -> MTAPIRES,
    pub IMTConCommon_AccountDepositURL: unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
    pub IMTConCommon_AccountWithdrawalURL1:
        unsafe extern "C" fn(this: *mut IMTConCommon, url: LPCWSTR) -> MTAPIRES,
    pub IMTConCommon_AccountWithdrawalURL:
        unsafe extern "C" fn(this: *const IMTConCommon) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConCommon {
    pub vtable_: *const IMTConCommon__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::con_common::MT5ConCommon,
}

pub const IMTConCommon_EnUpdateMode_UPDATE_DISABLE: IMTConCommon_EnUpdateMode = 0;
pub const IMTConCommon_EnUpdateMode_UPDATE_ENABLE: IMTConCommon_EnUpdateMode = 1;
pub const IMTConCommon_EnUpdateMode_UPDATE_ENABLE_BETA: IMTConCommon_EnUpdateMode = 2;
pub const IMTConCommon_EnUpdateMode_UPDATE_FIRST: IMTConCommon_EnUpdateMode = 0;
pub const IMTConCommon_EnUpdateMode_UPDATE_LAST: IMTConCommon_EnUpdateMode = 2;
pub type IMTConCommon_EnUpdateMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConCommon"][::std::mem::size_of::<IMTConCommon>() - 24usize];
    ["Alignment of IMTConCommon"][::std::mem::align_of::<IMTConCommon>() - 8usize];
};
#[repr(C)]
pub struct IMTConCommonSink__bindgen_vtable {
    // manually added
    pub IMTConCommonSink_OnCommonUpdate:
        unsafe extern "C" fn(this: *mut IMTConCommonSink, config: *const IMTConCommon),
    pub IMTConCommonSink_OnCommonSync:
        unsafe extern "C" fn(this: *mut IMTConCommonSink),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConCommonSink {
    pub vtable_: *const IMTConCommonSink__bindgen_vtable
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConCommonSink"][::std::mem::size_of::<IMTConCommonSink>() - 8usize];
    ["Alignment of IMTConCommonSink"][::std::mem::align_of::<IMTConCommonSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConParam__bindgen_vtable {
    pub IMTConParam_Release: unsafe extern "C" fn(this: *mut IMTConParam),
    pub IMTConParam_Assign:
        unsafe extern "C" fn(this: *mut IMTConParam, param: *const IMTConParam) -> MTAPIRES,
    pub IMTConParam_Clear: unsafe extern "C" fn(this: *mut IMTConParam) -> MTAPIRES,
    pub IMTConParam_Name1: unsafe extern "C" fn(this: *mut IMTConParam, name: LPCWSTR) -> MTAPIRES,
    pub IMTConParam_Name: unsafe extern "C" fn(this: *const IMTConParam) -> LPCWSTR,
    pub IMTConParam_Type1: unsafe extern "C" fn(this: *mut IMTConParam, type_: UINT) -> MTAPIRES,
    pub IMTConParam_Type: unsafe extern "C" fn(this: *const IMTConParam) -> UINT,
    pub IMTConParam_Value1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: LPCWSTR) -> MTAPIRES,
    pub IMTConParam_Value: unsafe extern "C" fn(this: *const IMTConParam) -> LPCWSTR,
    pub IMTConParam_ValueString1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: LPCWSTR) -> MTAPIRES,
    pub IMTConParam_ValueString: unsafe extern "C" fn(this: *const IMTConParam) -> LPCWSTR,
    pub IMTConParam_ValueInt1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: INT64) -> MTAPIRES,
    pub IMTConParam_ValueInt: unsafe extern "C" fn(this: *const IMTConParam) -> INT64,
    pub IMTConParam_ValueFloat1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: f64) -> MTAPIRES,
    pub IMTConParam_ValueFloat: unsafe extern "C" fn(this: *const IMTConParam) -> f64,
    pub IMTConParam_ValueTime1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: INT64) -> MTAPIRES,
    pub IMTConParam_ValueTime: unsafe extern "C" fn(this: *const IMTConParam) -> INT64,
    pub IMTConParam_ValueDatetime1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: INT64) -> MTAPIRES,
    pub IMTConParam_ValueDatetime: unsafe extern "C" fn(this: *const IMTConParam) -> INT64,
    pub IMTConParam_ValueGroups1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: LPCWSTR) -> MTAPIRES,
    pub IMTConParam_ValueGroups: unsafe extern "C" fn(this: *const IMTConParam) -> LPCWSTR,
    pub IMTConParam_ValueSymbols1:
        unsafe extern "C" fn(this: *mut IMTConParam, value: LPCWSTR) -> MTAPIRES,
    pub IMTConParam_ValueSymbols: unsafe extern "C" fn(this: *const IMTConParam) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConParam {
    pub vtable_: *const IMTConParam__bindgen_vtable,
}
pub const IMTConParam_ParamType_TYPE_STRING: IMTConParam_ParamType = 0;
pub const IMTConParam_ParamType_TYPE_INT: IMTConParam_ParamType = 1;
pub const IMTConParam_ParamType_TYPE_FLOAT: IMTConParam_ParamType = 2;
pub const IMTConParam_ParamType_TYPE_TIME: IMTConParam_ParamType = 3;
pub const IMTConParam_ParamType_TYPE_DATE: IMTConParam_ParamType = 4;
pub const IMTConParam_ParamType_TYPE_DATETIME: IMTConParam_ParamType = 5;
pub const IMTConParam_ParamType_TYPE_GROUPS: IMTConParam_ParamType = 6;
pub const IMTConParam_ParamType_TYPE_SYMBOLS: IMTConParam_ParamType = 7;
pub const IMTConParam_ParamType_TYPE_FIRST: IMTConParam_ParamType = 0;
pub const IMTConParam_ParamType_TYPE_LAST: IMTConParam_ParamType = 7;
pub type IMTConParam_ParamType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConParam"][::std::mem::size_of::<IMTConParam>() - 8usize];
    ["Alignment of IMTConParam"][::std::mem::align_of::<IMTConParam>() - 8usize];
};
#[repr(C)]
pub struct IMTConPluginModule__bindgen_vtable {
    pub IMTConPluginModule_Release: unsafe extern "C" fn(this: *mut IMTConPluginModule),
    pub IMTConPluginModule_Assign: unsafe extern "C" fn(
        this: *mut IMTConPluginModule,
        param: *const IMTConPluginModule,
    ) -> MTAPIRES,
    pub IMTConPluginModule_Clear: unsafe extern "C" fn(this: *mut IMTConPluginModule) -> MTAPIRES,
    pub IMTConPluginModule_Name: unsafe extern "C" fn(this: *const IMTConPluginModule) -> LPCWSTR,
    pub IMTConPluginModule_Vendor: unsafe extern "C" fn(this: *const IMTConPluginModule) -> LPCWSTR,
    pub IMTConPluginModule_Description:
        unsafe extern "C" fn(this: *const IMTConPluginModule) -> LPCWSTR,
    pub IMTConPluginModule_Module: unsafe extern "C" fn(this: *const IMTConPluginModule) -> LPCWSTR,
    pub IMTConPluginModule_Server: unsafe extern "C" fn(this: *const IMTConPluginModule) -> UINT64,
    pub IMTConPluginModule_Version: unsafe extern "C" fn(this: *const IMTConPluginModule) -> UINT,
    pub IMTConPluginModule_VersionAPI:
        unsafe extern "C" fn(this: *const IMTConPluginModule) -> UINT,
    pub IMTConPluginModule_ParameterTotal:
        unsafe extern "C" fn(this: *const IMTConPluginModule) -> UINT,
    pub IMTConPluginModule_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConPluginModule,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConPluginModule_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConPluginModule,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConPluginModule {
    pub vtable_: *const IMTConPluginModule__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConPluginModule"][::std::mem::size_of::<IMTConPluginModule>() - 8usize];
    ["Alignment of IMTConPluginModule"][::std::mem::align_of::<IMTConPluginModule>() - 8usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTLogRecord {
    pub flags: UINT,
    pub code: INT,
    pub type_: UINT,
    pub datetime: INT64,
    pub source: [u16; 64usize],
    pub message: [u16; 512usize],
    pub datetime_msc: INT64,
    pub reserved: [::std::os::raw::c_int; 2usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTLogRecord"][::std::mem::size_of::<MTLogRecord>() - 1188usize];
    ["Alignment of MTLogRecord"][::std::mem::align_of::<MTLogRecord>() - 1usize];
    ["Offset of field: MTLogRecord::flags"][::std::mem::offset_of!(MTLogRecord, flags) - 0usize];
    ["Offset of field: MTLogRecord::code"][::std::mem::offset_of!(MTLogRecord, code) - 4usize];
    ["Offset of field: MTLogRecord::type_"][::std::mem::offset_of!(MTLogRecord, type_) - 8usize];
    ["Offset of field: MTLogRecord::datetime"]
        [::std::mem::offset_of!(MTLogRecord, datetime) - 12usize];
    ["Offset of field: MTLogRecord::source"][::std::mem::offset_of!(MTLogRecord, source) - 20usize];
    ["Offset of field: MTLogRecord::message"]
        [::std::mem::offset_of!(MTLogRecord, message) - 148usize];
    ["Offset of field: MTLogRecord::datetime_msc"]
        [::std::mem::offset_of!(MTLogRecord, datetime_msc) - 1172usize];
    ["Offset of field: MTLogRecord::reserved"]
        [::std::mem::offset_of!(MTLogRecord, reserved) - 1180usize];
};
#[repr(C)]
pub struct IMTServerSink__bindgen_vtable {
    pub IMTServerSink_OnServerLog: unsafe extern "C" fn(
        this: *mut IMTServerSink,
        code: INT,
        t: UINT,
        datetime_msc: INT64,
        source: LPCWSTR,
        message: LPCWSTR,
    ),
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTServerSink {
    pub vtable_: *const IMTServerSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTServerSink"][::std::mem::size_of::<IMTServerSink>() - 8usize];
    ["Alignment of IMTServerSink"][::std::mem::align_of::<IMTServerSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConPlugin__bindgen_vtable {
    pub IMTConPlugin_Release: unsafe extern "C" fn(this: *mut IMTConPlugin),
    pub IMTConPlugin_Assign:
        unsafe extern "C" fn(this: *mut IMTConPlugin, param: *const IMTConPlugin) -> MTAPIRES,
    pub IMTConPlugin_Clear: unsafe extern "C" fn(this: *mut IMTConPlugin) -> MTAPIRES,
    pub IMTConPlugin_Name1:
        unsafe extern "C" fn(this: *mut IMTConPlugin, name: LPCWSTR) -> MTAPIRES,
    pub IMTConPlugin_Name: unsafe extern "C" fn(this: *const IMTConPlugin) -> LPCWSTR,
    pub IMTConPlugin_Server1:
        unsafe extern "C" fn(this: *mut IMTConPlugin, server: UINT64) -> MTAPIRES,
    pub IMTConPlugin_Server: unsafe extern "C" fn(this: *const IMTConPlugin) -> UINT64,
    pub IMTConPlugin_Module1:
        unsafe extern "C" fn(this: *mut IMTConPlugin, name: LPCWSTR) -> MTAPIRES,
    pub IMTConPlugin_Module: unsafe extern "C" fn(this: *const IMTConPlugin) -> LPCWSTR,
    pub IMTConPlugin_Mode1: unsafe extern "C" fn(this: *mut IMTConPlugin, mode: UINT) -> MTAPIRES,
    pub IMTConPlugin_Mode: unsafe extern "C" fn(this: *const IMTConPlugin) -> UINT,
    pub IMTConPlugin_ParameterAdd:
        unsafe extern "C" fn(this: *mut IMTConPlugin, param: *mut IMTConParam) -> MTAPIRES,
    pub IMTConPlugin_ParameterUpdate: unsafe extern "C" fn(
        this: *mut IMTConPlugin,
        pos: UINT,
        param: *const IMTConParam,
    ) -> MTAPIRES,
    pub IMTConPlugin_ParameterDelete:
        unsafe extern "C" fn(this: *mut IMTConPlugin, pos: UINT) -> MTAPIRES,
    pub IMTConPlugin_ParameterClear: unsafe extern "C" fn(this: *mut IMTConPlugin) -> MTAPIRES,
    pub IMTConPlugin_ParameterShift: unsafe extern "C" fn(
        this: *mut IMTConPlugin,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConPlugin_ParameterTotal: unsafe extern "C" fn(this: *const IMTConPlugin) -> UINT,
    pub IMTConPlugin_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConPlugin,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConPlugin_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConPlugin,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConPlugin_Flags1: unsafe extern "C" fn(this: *mut IMTConPlugin, flags: UINT) -> MTAPIRES,
    pub IMTConPlugin_Flags: unsafe extern "C" fn(this: *const IMTConPlugin) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConPlugin {
    pub vtable_: *const IMTConPlugin__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::con_plugin::MT5ConPlugin,
}
pub const IMTConPlugin_EnPluginFlags_PLUGIN_FLAG_MAN_CONFIG: IMTConPlugin_EnPluginFlags = 1;
pub const IMTConPlugin_EnPluginFlags_PLUGIN_FLAG_PROFILING: IMTConPlugin_EnPluginFlags = 2;
pub const IMTConPlugin_EnPluginFlags_PLUGIN_FLAG_NONE: IMTConPlugin_EnPluginFlags = 0;
pub const IMTConPlugin_EnPluginFlags_PLUGIN_FLAG_ALL: IMTConPlugin_EnPluginFlags = 3;
pub type IMTConPlugin_EnPluginFlags = ::std::os::raw::c_int;
pub const IMTConPlugin_EnPluginMode_PLUGIN_DISABLED: IMTConPlugin_EnPluginMode = 0;
pub const IMTConPlugin_EnPluginMode_PLUGIN_ENABLED: IMTConPlugin_EnPluginMode = 1;
pub const IMTConPlugin_EnPluginMode_PLUGIN_FIRST: IMTConPlugin_EnPluginMode = 0;
pub const IMTConPlugin_EnPluginMode_PLUGIN_LAST: IMTConPlugin_EnPluginMode = 1;
pub type IMTConPlugin_EnPluginMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConPlugin"][::std::mem::size_of::<IMTConPlugin>() - 24usize];
    ["Alignment of IMTConPlugin"][::std::mem::align_of::<IMTConPlugin>() - 8usize];
};
#[repr(C)]
pub struct IMTConPluginSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConPluginSink {
    pub vtable_: *const IMTConPluginSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConPluginSink"][::std::mem::size_of::<IMTConPluginSink>() - 8usize];
    ["Alignment of IMTConPluginSink"][::std::mem::align_of::<IMTConPluginSink>() - 8usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTServerInfo {
    pub platform_name: [u16; 64usize],
    pub platform_owner: [u16; 128usize],
    pub server_version: UINT,
    pub server_build: UINT,
    pub server_type: UINT,
    pub server_id: UINT64,
    pub reserved: [UINT; 32usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTServerInfo"][::std::mem::size_of::<MTServerInfo>() - 532usize];
    ["Alignment of MTServerInfo"][::std::mem::align_of::<MTServerInfo>() - 1usize];
    ["Offset of field: MTServerInfo::platform_name"]
        [::std::mem::offset_of!(MTServerInfo, platform_name) - 0usize];
    ["Offset of field: MTServerInfo::platform_owner"]
        [::std::mem::offset_of!(MTServerInfo, platform_owner) - 128usize];
    ["Offset of field: MTServerInfo::server_version"]
        [::std::mem::offset_of!(MTServerInfo, server_version) - 384usize];
    ["Offset of field: MTServerInfo::server_build"]
        [::std::mem::offset_of!(MTServerInfo, server_build) - 388usize];
    ["Offset of field: MTServerInfo::server_type"]
        [::std::mem::offset_of!(MTServerInfo, server_type) - 392usize];
    ["Offset of field: MTServerInfo::server_id"]
        [::std::mem::offset_of!(MTServerInfo, server_id) - 396usize];
    ["Offset of field: MTServerInfo::reserved"]
        [::std::mem::offset_of!(MTServerInfo, reserved) - 404usize];
};
#[repr(C)]
pub struct IMTConHoliday__bindgen_vtable {
    pub IMTConHoliday_Release: unsafe extern "C" fn(this: *mut IMTConHoliday),
    pub IMTConHoliday_Assign:
        unsafe extern "C" fn(this: *mut IMTConHoliday, holiday: *const IMTConHoliday) -> MTAPIRES,
    pub IMTConHoliday_Clear: unsafe extern "C" fn(this: *mut IMTConHoliday) -> MTAPIRES,
    pub IMTConHoliday_Description1:
        unsafe extern "C" fn(this: *mut IMTConHoliday, descr: LPCWSTR) -> MTAPIRES,
    pub IMTConHoliday_Description: unsafe extern "C" fn(this: *const IMTConHoliday) -> LPCWSTR,
    pub IMTConHoliday_Mode1: unsafe extern "C" fn(this: *mut IMTConHoliday, mode: UINT) -> MTAPIRES,
    pub IMTConHoliday_Mode: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_Year1: unsafe extern "C" fn(this: *mut IMTConHoliday, year: UINT) -> MTAPIRES,
    pub IMTConHoliday_Year: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_Month1:
        unsafe extern "C" fn(this: *mut IMTConHoliday, month: UINT) -> MTAPIRES,
    pub IMTConHoliday_Month: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_Day1: unsafe extern "C" fn(this: *mut IMTConHoliday, day: UINT) -> MTAPIRES,
    pub IMTConHoliday_Day: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_WorkFrom1:
        unsafe extern "C" fn(this: *mut IMTConHoliday, from: UINT) -> MTAPIRES,
    pub IMTConHoliday_WorkFrom: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_WorkFromHours: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_WorkFromMinutes: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_WorkTo1:
        unsafe extern "C" fn(this: *mut IMTConHoliday, from: UINT) -> MTAPIRES,
    pub IMTConHoliday_WorkTo: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_WorkToHours: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_WorkToMinutes: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_SymbolAdd:
        unsafe extern "C" fn(this: *mut IMTConHoliday, path: LPCWSTR) -> MTAPIRES,
    pub IMTConHoliday_SymbolUpdate:
        unsafe extern "C" fn(this: *mut IMTConHoliday, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConHoliday_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTConHoliday, pos: UINT) -> MTAPIRES,
    pub IMTConHoliday_SymbolClear: unsafe extern "C" fn(this: *mut IMTConHoliday) -> MTAPIRES,
    pub IMTConHoliday_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTConHoliday,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConHoliday_SymbolTotal: unsafe extern "C" fn(this: *const IMTConHoliday) -> UINT,
    pub IMTConHoliday_SymbolNext:
        unsafe extern "C" fn(this: *const IMTConHoliday, pos: UINT) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConHoliday {
    pub vtable_: *const IMTConHoliday__bindgen_vtable,
}
pub const IMTConHoliday_EnHolidayMode_HOLIDAY_DISABLED: IMTConHoliday_EnHolidayMode = 0;
pub const IMTConHoliday_EnHolidayMode_HOLIDAY_ENABLED: IMTConHoliday_EnHolidayMode = 1;
pub const IMTConHoliday_EnHolidayMode_HOLIDAY_FIRST: IMTConHoliday_EnHolidayMode = 0;
pub const IMTConHoliday_EnHolidayMode_HOLIDAY_LAST: IMTConHoliday_EnHolidayMode = 1;
pub type IMTConHoliday_EnHolidayMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConHoliday"][::std::mem::size_of::<IMTConHoliday>() - 8usize];
    ["Alignment of IMTConHoliday"][::std::mem::align_of::<IMTConHoliday>() - 8usize];
};
#[repr(C)]
pub struct IMTConHolidaySink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConHolidaySink {
    pub vtable_: *const IMTConHolidaySink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConHolidaySink"][::std::mem::size_of::<IMTConHolidaySink>() - 8usize];
    ["Alignment of IMTConHolidaySink"][::std::mem::align_of::<IMTConHolidaySink>() - 8usize];
};
#[repr(C)]
pub struct IMTConFirewall__bindgen_vtable {
    pub IMTConFirewall_Release: unsafe extern "C" fn(this: *mut IMTConFirewall),
    pub IMTConFirewall_Assign:
        unsafe extern "C" fn(this: *mut IMTConFirewall, param: *const IMTConFirewall) -> MTAPIRES,
    pub IMTConFirewall_Clear: unsafe extern "C" fn(this: *mut IMTConFirewall) -> MTAPIRES,
    pub IMTConFirewall_Action1:
        unsafe extern "C" fn(this: *mut IMTConFirewall, action: UINT) -> MTAPIRES,
    pub IMTConFirewall_Action: unsafe extern "C" fn(this: *const IMTConFirewall) -> UINT,
    pub IMTConFirewall_From1:
        unsafe extern "C" fn(this: *mut IMTConFirewall, name: LPCWSTR) -> MTAPIRES,
    pub IMTConFirewall_From: unsafe extern "C" fn(this: *const IMTConFirewall) -> LPCWSTR,
    pub IMTConFirewall_To1:
        unsafe extern "C" fn(this: *mut IMTConFirewall, value: LPCWSTR) -> MTAPIRES,
    pub IMTConFirewall_To: unsafe extern "C" fn(this: *const IMTConFirewall) -> LPCWSTR,
    pub IMTConFirewall_Comment1:
        unsafe extern "C" fn(this: *mut IMTConFirewall, comment: LPCWSTR) -> MTAPIRES,
    pub IMTConFirewall_Comment: unsafe extern "C" fn(this: *const IMTConFirewall) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConFirewall {
    pub vtable_: *const IMTConFirewall__bindgen_vtable,
}
pub const IMTConFirewall_EnAction_ACCESS_BLOCK: IMTConFirewall_EnAction = 0;
pub const IMTConFirewall_EnAction_ACCESS_PERMIT: IMTConFirewall_EnAction = 1;
pub const IMTConFirewall_EnAction_ACCESS_WHITELIST: IMTConFirewall_EnAction = 2;
pub const IMTConFirewall_EnAction_ACCESS_FIRST: IMTConFirewall_EnAction = 0;
pub const IMTConFirewall_EnAction_ACCESS_LAST: IMTConFirewall_EnAction = 2;
pub type IMTConFirewall_EnAction = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConFirewall"][::std::mem::size_of::<IMTConFirewall>() - 8usize];
    ["Alignment of IMTConFirewall"][::std::mem::align_of::<IMTConFirewall>() - 8usize];
};
#[repr(C)]
pub struct IMTConFirewallSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConFirewallSink {
    pub vtable_: *const IMTConFirewallSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConFirewallSink"][::std::mem::size_of::<IMTConFirewallSink>() - 8usize];
    ["Alignment of IMTConFirewallSink"][::std::mem::align_of::<IMTConFirewallSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConSymbolSession__bindgen_vtable {
    pub IMTConSymbolSession_Release: unsafe extern "C" fn(this: *mut IMTConSymbolSession),
    pub IMTConSymbolSession_Assign: unsafe extern "C" fn(
        this: *mut IMTConSymbolSession,
        symbol: *const IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbolSession_Clear: unsafe extern "C" fn(this: *mut IMTConSymbolSession) -> MTAPIRES,
    pub IMTConSymbolSession_Open1:
        unsafe extern "C" fn(this: *mut IMTConSymbolSession, open: UINT) -> MTAPIRES,
    pub IMTConSymbolSession_Open: unsafe extern "C" fn(this: *const IMTConSymbolSession) -> UINT,
    pub IMTConSymbolSession_OpenHours:
        unsafe extern "C" fn(this: *const IMTConSymbolSession) -> UINT,
    pub IMTConSymbolSession_OpenMinutes:
        unsafe extern "C" fn(this: *const IMTConSymbolSession) -> UINT,
    pub IMTConSymbolSession_Close1:
        unsafe extern "C" fn(this: *mut IMTConSymbolSession, close: UINT) -> MTAPIRES,
    pub IMTConSymbolSession_Close: unsafe extern "C" fn(this: *const IMTConSymbolSession) -> UINT,
    pub IMTConSymbolSession_CloseHours:
        unsafe extern "C" fn(this: *const IMTConSymbolSession) -> UINT,
    pub IMTConSymbolSession_CloseMinutes:
        unsafe extern "C" fn(this: *const IMTConSymbolSession) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSymbolSession {
    pub vtable_: *const IMTConSymbolSession__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSymbolSession"][::std::mem::size_of::<IMTConSymbolSession>() - 8usize];
    ["Alignment of IMTConSymbolSession"][::std::mem::align_of::<IMTConSymbolSession>() - 8usize];
};
#[repr(C)]
pub struct IMTConSymbol__bindgen_vtable {
    pub IMTConSymbol_Release: unsafe extern "C" fn(this: *mut IMTConSymbol),
    pub IMTConSymbol_Assign:
        unsafe extern "C" fn(this: *mut IMTConSymbol, symbol: *const IMTConSymbol) -> MTAPIRES,
    pub IMTConSymbol_Clear: unsafe extern "C" fn(this: *mut IMTConSymbol) -> MTAPIRES,
    pub IMTConSymbol_Symbol1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Symbol: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Path1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, path: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Path: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_ISIN1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, isin: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_ISIN: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Description1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, descr: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Description: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_International1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, intern: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_International: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Basis1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, basis: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Basis: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Source1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, source: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Source: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Page1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, page: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Page: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_CurrencyBase1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_CurrencyBase: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_CurrencyBaseDigits: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_CurrencyProfit1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_CurrencyProfit: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_CurrencyProfitDigits: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_CurrencyMargin1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_CurrencyMargin: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_CurrencyMarginDigits: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_Color1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, color: COLORREF) -> MTAPIRES,
    pub IMTConSymbol_Color: unsafe extern "C" fn(this: *const IMTConSymbol) -> COLORREF,
    pub IMTConSymbol_ColorBackground1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, color: COLORREF) -> MTAPIRES,
    pub IMTConSymbol_ColorBackground: unsafe extern "C" fn(this: *const IMTConSymbol) -> COLORREF,
    pub IMTConSymbol_Digits1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES,
    pub IMTConSymbol_Digits: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_Point: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_Multiply: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_TickFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT64) -> MTAPIRES,
    pub IMTConSymbol_TickFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_TickBookDepth1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, depth: UINT) -> MTAPIRES,
    pub IMTConSymbol_TickBookDepth: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterSoft1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, filter: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterSoft: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterSoftTicks1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterSoftTicks: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterHard1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, filter: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterHard: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterHardTicks1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterHardTicks: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterDiscard1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterDiscard: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterSpreadMax1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, spread: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterSpreadMax: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterSpreadMin1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, spread: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterSpreadMin: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_TradeMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_TradeMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_CalcMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_CalcMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_ExecMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_ExecMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_GTCMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_GTCMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FillFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConSymbol_FillFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_ExpirFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConSymbol_ExpirFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_Spread1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, spread: UINT) -> MTAPIRES,
    pub IMTConSymbol_Spread: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SpreadBalance1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, spread: INT) -> MTAPIRES,
    pub IMTConSymbol_SpreadBalance: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT,
    pub IMTConSymbol_SpreadDiff1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, diff: INT) -> MTAPIRES,
    pub IMTConSymbol_SpreadDiff: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT,
    pub IMTConSymbol_SpreadDiffBalance1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, spread: INT) -> MTAPIRES,
    pub IMTConSymbol_SpreadDiffBalance: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT,
    pub IMTConSymbol_TickValue1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, value: f64) -> MTAPIRES,
    pub IMTConSymbol_TickValue: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_TickSize1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, size: f64) -> MTAPIRES,
    pub IMTConSymbol_TickSize: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_ContractSize1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, size: f64) -> MTAPIRES,
    pub IMTConSymbol_ContractSize: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_StopsLevel1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, level: INT) -> MTAPIRES,
    pub IMTConSymbol_StopsLevel: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT,
    pub IMTConSymbol_FreezeLevel1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, level: INT) -> MTAPIRES,
    pub IMTConSymbol_FreezeLevel: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT,
    pub IMTConSymbol_QuotesTimeout1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, timeout: UINT) -> MTAPIRES,
    pub IMTConSymbol_QuotesTimeout: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_VolumeMin1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeMin: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_VolumeMax1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeMax: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_VolumeStep1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeStep: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_VolumeLimit1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeLimit: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_MarginFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_MarginFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_MarginInitial1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginInitial: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginMaintenance1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginMaintenance: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginLong1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginLong: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginShort1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginShort: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginLimit1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginLimit: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginStop1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginStop: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginStopLimit1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginStopLimit: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_SwapMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SwapLong1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, swap: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapLong: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapShort1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, swap: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapShort: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_Swap3Day1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, day: UINT) -> MTAPIRES,
    pub IMTConSymbol_Swap3Day: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_TimeStart1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, start: INT64) -> MTAPIRES,
    pub IMTConSymbol_TimeStart: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT64,
    pub IMTConSymbol_TimeExpiration1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, expiration: INT64) -> MTAPIRES,
    pub IMTConSymbol_TimeExpiration: unsafe extern "C" fn(this: *const IMTConSymbol) -> INT64,
    pub IMTConSymbol_SessionQuoteAdd: unsafe extern "C" fn(
        this: *mut IMTConSymbol,
        wday: UINT,
        symbol: *mut IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionQuoteUpdate: unsafe extern "C" fn(
        this: *mut IMTConSymbol,
        wday: UINT,
        pos: UINT,
        session: *const IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionQuoteDelete:
        unsafe extern "C" fn(this: *mut IMTConSymbol, wday: UINT, pos: UINT) -> MTAPIRES,
    pub IMTConSymbol_SessionQuoteClear:
        unsafe extern "C" fn(this: *mut IMTConSymbol, wday: UINT) -> MTAPIRES,
    pub IMTConSymbol_SessionQuoteShift: unsafe extern "C" fn(
        this: *mut IMTConSymbol,
        wday: UINT,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionQuoteTotal:
        unsafe extern "C" fn(this: *const IMTConSymbol, wday: UINT) -> UINT,
    pub IMTConSymbol_SessionQuoteNext: unsafe extern "C" fn(
        this: *const IMTConSymbol,
        wday: UINT,
        pos: UINT,
        session: *mut IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionTradeAdd: unsafe extern "C" fn(
        this: *mut IMTConSymbol,
        wday: UINT,
        symbol: *mut IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionTradeUpdate: unsafe extern "C" fn(
        this: *mut IMTConSymbol,
        wday: UINT,
        pos: UINT,
        session: *const IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionTradeDelete:
        unsafe extern "C" fn(this: *mut IMTConSymbol, wday: UINT, pos: UINT) -> MTAPIRES,
    pub IMTConSymbol_SessionTradeClear:
        unsafe extern "C" fn(this: *mut IMTConSymbol, wday: UINT) -> MTAPIRES,
    pub IMTConSymbol_SessionTradeShift: unsafe extern "C" fn(
        this: *mut IMTConSymbol,
        wday: UINT,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSymbol_SessionTradeTotal:
        unsafe extern "C" fn(this: *const IMTConSymbol, wday: UINT) -> UINT,
    pub IMTConSymbol_SessionTradeNext: unsafe extern "C" fn(
        this: *const IMTConSymbol,
        wday: UINT,
        pos: UINT,
        session: *mut IMTConSymbolSession,
    ) -> MTAPIRES,
    pub IMTConSymbol_REFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConSymbol_REFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_RETimeout1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, timeout: UINT) -> MTAPIRES,
    pub IMTConSymbol_RETimeout: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_IECheckMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_IECheckMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_IETimeout1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, timeout: UINT) -> MTAPIRES,
    pub IMTConSymbol_IETimeout: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_IESlipProfit1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, slippage: UINT) -> MTAPIRES,
    pub IMTConSymbol_IESlipProfit: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_IESlipLosing1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, slippage: UINT) -> MTAPIRES,
    pub IMTConSymbol_IESlipLosing: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_IEVolumeMax1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_IEVolumeMax: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_PriceSettle1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, price: f64) -> MTAPIRES,
    pub IMTConSymbol_PriceSettle: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_PriceLimitMax1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, price: f64) -> MTAPIRES,
    pub IMTConSymbol_PriceLimitMax: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_PriceLimitMin1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, price: f64) -> MTAPIRES,
    pub IMTConSymbol_PriceLimitMin: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_TradeFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT64) -> MTAPIRES,
    pub IMTConSymbol_TradeFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_OrderFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConSymbol_OrderFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_MarginRateInitial1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, type_: UINT, margin_rate: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginRateInitial:
        unsafe extern "C" fn(this: *const IMTConSymbol, type_: UINT) -> f64,
    pub IMTConSymbol_MarginRateMaintenance1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, type_: UINT, margin_rate: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginRateMaintenance:
        unsafe extern "C" fn(this: *const IMTConSymbol, type_: UINT) -> f64,
    pub IMTConSymbol_OptionsMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_OptionsMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_PriceStrike1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, price: f64) -> MTAPIRES,
    pub IMTConSymbol_PriceStrike: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginRateLiquidity1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin_rate: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginRateLiquidity: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_FaceValue1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, value: f64) -> MTAPIRES,
    pub IMTConSymbol_FaceValue: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_AccruedInterest1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, interest: f64) -> MTAPIRES,
    pub IMTConSymbol_AccruedInterest: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SpliceType1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, type_: UINT) -> MTAPIRES,
    pub IMTConSymbol_SpliceType: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SpliceTimeType1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, time_type: UINT) -> MTAPIRES,
    pub IMTConSymbol_SpliceTimeType: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SpliceTimeDays1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, days: UINT) -> MTAPIRES,
    pub IMTConSymbol_SpliceTimeDays: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_MarginHedged1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginHedged: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_MarginRateCurrency1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, margin_rate: f64) -> MTAPIRES,
    pub IMTConSymbol_MarginRateCurrency: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_FilterGap1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, gap: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterGap: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_FilterGapTicks1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES,
    pub IMTConSymbol_FilterGapTicks: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_ChartMode1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSymbol_ChartMode: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_CurrencyBaseDigitsSet:
        unsafe extern "C" fn(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES,
    pub IMTConSymbol_CurrencyProfitDigitsSet:
        unsafe extern "C" fn(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES,
    pub IMTConSymbol_CurrencyMarginDigitsSet:
        unsafe extern "C" fn(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES,
    pub IMTConSymbol_IEFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConSymbol_IEFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_VolumeMinExt1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeMinExt: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_VolumeMaxExt1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeMaxExt: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_VolumeStepExt1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeStepExt: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_VolumeLimitExt1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_VolumeLimitExt: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_IEVolumeMaxExt1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConSymbol_IEVolumeMaxExt: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT64,
    pub IMTConSymbol_Category1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, category: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Category: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Exchange1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, exchange: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Exchange: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_CFI1: unsafe extern "C" fn(this: *mut IMTConSymbol, cfi: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_CFI: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_Sector1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, sector: UINT) -> MTAPIRES,
    pub IMTConSymbol_Sector: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_Industry1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, industry: UINT) -> MTAPIRES,
    pub IMTConSymbol_Industry: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_Country1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, country: LPCWSTR) -> MTAPIRES,
    pub IMTConSymbol_Country: unsafe extern "C" fn(this: *const IMTConSymbol) -> LPCWSTR,
    pub IMTConSymbol_SubscriptionsDelay1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, delay: UINT) -> MTAPIRES,
    pub IMTConSymbol_SubscriptionsDelay: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SwapYearDays1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, days: UINT) -> MTAPIRES,
    pub IMTConSymbol_SwapYearDays: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SwapFlags1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConSymbol_SwapFlags: unsafe extern "C" fn(this: *const IMTConSymbol) -> UINT,
    pub IMTConSymbol_SwapRateSunday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateSunday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapRateMonday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateMonday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapRateTuesday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateTuesday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapRateWednesday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateWednesday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapRateThursday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateThursday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapRateFriday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateFriday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
    pub IMTConSymbol_SwapRateSaturday1:
        unsafe extern "C" fn(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES,
    pub IMTConSymbol_SwapRateSaturday: unsafe extern "C" fn(this: *const IMTConSymbol) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSymbol {
    pub vtable_: *const IMTConSymbol__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::con_symbol::MTConSymbol,
}
pub const IMTConSymbol_EnSectors_SECTOR_UNDEFINED: IMTConSymbol_EnSectors = 0;
pub const IMTConSymbol_EnSectors_SECTOR_BASIC_MATERIALS: IMTConSymbol_EnSectors = 1;
pub const IMTConSymbol_EnSectors_SECTOR_COMMUNICATION_SERVICES: IMTConSymbol_EnSectors = 2;
pub const IMTConSymbol_EnSectors_SECTOR_CONSUMER_CYCLICAL: IMTConSymbol_EnSectors = 3;
pub const IMTConSymbol_EnSectors_SECTOR_CONSUMER_DEFENSIVE: IMTConSymbol_EnSectors = 4;
pub const IMTConSymbol_EnSectors_SECTOR_ENERGY: IMTConSymbol_EnSectors = 5;
pub const IMTConSymbol_EnSectors_SECTOR_FINANCIAL: IMTConSymbol_EnSectors = 6;
pub const IMTConSymbol_EnSectors_SECTOR_HEALTHCARE: IMTConSymbol_EnSectors = 7;
pub const IMTConSymbol_EnSectors_SECTOR_INDUSTRIALS: IMTConSymbol_EnSectors = 8;
pub const IMTConSymbol_EnSectors_SECTOR_REAL_ESTATE: IMTConSymbol_EnSectors = 9;
pub const IMTConSymbol_EnSectors_SECTOR_TECHNOLOGY: IMTConSymbol_EnSectors = 10;
pub const IMTConSymbol_EnSectors_SECTOR_UTILITIES: IMTConSymbol_EnSectors = 11;
pub const IMTConSymbol_EnSectors_SECTOR_CURRENCY: IMTConSymbol_EnSectors = 12;
pub const IMTConSymbol_EnSectors_SECTOR_CURRENCY_CRYPTO: IMTConSymbol_EnSectors = 13;
pub const IMTConSymbol_EnSectors_SECTOR_INDEXES: IMTConSymbol_EnSectors = 14;
pub const IMTConSymbol_EnSectors_SECTOR_COMMODITIES: IMTConSymbol_EnSectors = 15;
pub const IMTConSymbol_EnSectors_SECTOR_FIRST: IMTConSymbol_EnSectors = 0;
pub const IMTConSymbol_EnSectors_SECTOR_LAST: IMTConSymbol_EnSectors = 15;
pub type IMTConSymbol_EnSectors = ::std::os::raw::c_int;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UNDEFINED: IMTConSymbol_EnIndustries = 0;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AGRICULTURAL_INPUTS: IMTConSymbol_EnIndustries = 1;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ALUMINIUM: IMTConSymbol_EnIndustries = 2;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BUILDING_MATERIALS: IMTConSymbol_EnIndustries = 3;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CHEMICALS: IMTConSymbol_EnIndustries = 4;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COKING_COAL: IMTConSymbol_EnIndustries = 5;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COPPER: IMTConSymbol_EnIndustries = 6;
pub const IMTConSymbol_EnIndustries_INDUSTRY_GOLD: IMTConSymbol_EnIndustries = 7;
pub const IMTConSymbol_EnIndustries_INDUSTRY_LUMBER_WOOD: IMTConSymbol_EnIndustries = 8;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INDUSTRIAL_METALS: IMTConSymbol_EnIndustries = 9;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PRECIOUS_METALS: IMTConSymbol_EnIndustries = 10;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PAPER: IMTConSymbol_EnIndustries = 11;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SILVER: IMTConSymbol_EnIndustries = 12;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SPECIALTY_CHEMICALS: IMTConSymbol_EnIndustries = 13;
pub const IMTConSymbol_EnIndustries_INDUSTRY_STEEL: IMTConSymbol_EnIndustries = 14;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BASIC_MATERIALS_FIRST: IMTConSymbol_EnIndustries = 1;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BASIC_MATERIALS_LAST: IMTConSymbol_EnIndustries = 14;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BASIC_MATERIALS_END: IMTConSymbol_EnIndustries = 50;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ADVERTISING: IMTConSymbol_EnIndustries = 51;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BROADCASTING: IMTConSymbol_EnIndustries = 52;
pub const IMTConSymbol_EnIndustries_INDUSTRY_GAMING_MULTIMEDIA: IMTConSymbol_EnIndustries = 53;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ENTERTAINMENT: IMTConSymbol_EnIndustries = 54;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INTERNET_CONTENT: IMTConSymbol_EnIndustries = 55;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PUBLISHING: IMTConSymbol_EnIndustries = 56;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TELECOM: IMTConSymbol_EnIndustries = 57;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMUNICATION_FIRST: IMTConSymbol_EnIndustries = 51;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMUNICATION_LAST: IMTConSymbol_EnIndustries = 57;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMUNICATION_END: IMTConSymbol_EnIndustries = 100;
pub const IMTConSymbol_EnIndustries_INDUSTRY_APPAREL_MANUFACTURING: IMTConSymbol_EnIndustries = 101;
pub const IMTConSymbol_EnIndustries_INDUSTRY_APPAREL_RETAIL: IMTConSymbol_EnIndustries = 102;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AUTO_MANUFACTURERS: IMTConSymbol_EnIndustries = 103;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AUTO_PARTS: IMTConSymbol_EnIndustries = 104;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AUTO_DEALERSHIP: IMTConSymbol_EnIndustries = 105;
pub const IMTConSymbol_EnIndustries_INDUSTRY_DEPARTMENT_STORES: IMTConSymbol_EnIndustries = 106;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FOOTWEAR_ACCESSORIES: IMTConSymbol_EnIndustries = 107;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FURNISHINGS: IMTConSymbol_EnIndustries = 108;
pub const IMTConSymbol_EnIndustries_INDUSTRY_GAMBLING: IMTConSymbol_EnIndustries = 109;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HOME_IMPROV_RETAIL: IMTConSymbol_EnIndustries = 110;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INTERNET_RETAIL: IMTConSymbol_EnIndustries = 111;
pub const IMTConSymbol_EnIndustries_INDUSTRY_LEISURE: IMTConSymbol_EnIndustries = 112;
pub const IMTConSymbol_EnIndustries_INDUSTRY_LODGING: IMTConSymbol_EnIndustries = 113;
pub const IMTConSymbol_EnIndustries_INDUSTRY_LUXURY_GOODS: IMTConSymbol_EnIndustries = 114;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PACKAGING_CONTAINERS: IMTConSymbol_EnIndustries = 115;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PERSONAL_SERVICES: IMTConSymbol_EnIndustries = 116;
pub const IMTConSymbol_EnIndustries_INDUSTRY_RECREATIONAL_VEHICLES: IMTConSymbol_EnIndustries = 117;
pub const IMTConSymbol_EnIndustries_INDUSTRY_RESIDENT_CONSTRUCTION: IMTConSymbol_EnIndustries = 118;
pub const IMTConSymbol_EnIndustries_INDUSTRY_RESORTS_CASINOS: IMTConSymbol_EnIndustries = 119;
pub const IMTConSymbol_EnIndustries_INDUSTRY_RESTAURANTS: IMTConSymbol_EnIndustries = 120;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SPECIALTY_RETAIL: IMTConSymbol_EnIndustries = 121;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TEXTILE_MANUFACTURING: IMTConSymbol_EnIndustries = 122;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TRAVEL_SERVICES: IMTConSymbol_EnIndustries = 123;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_CYCL_FIRST: IMTConSymbol_EnIndustries = 101;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_CYCL_LAST: IMTConSymbol_EnIndustries = 123;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_CYCL_END: IMTConSymbol_EnIndustries = 150;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BEVERAGES_BREWERS: IMTConSymbol_EnIndustries = 151;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BEVERAGES_NON_ALCO: IMTConSymbol_EnIndustries = 152;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BEVERAGES_WINERIES: IMTConSymbol_EnIndustries = 153;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONFECTIONERS: IMTConSymbol_EnIndustries = 154;
pub const IMTConSymbol_EnIndustries_INDUSTRY_DISCOUNT_STORES: IMTConSymbol_EnIndustries = 155;
pub const IMTConSymbol_EnIndustries_INDUSTRY_EDUCATION_TRAINIG: IMTConSymbol_EnIndustries = 156;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FARM_PRODUCTS: IMTConSymbol_EnIndustries = 157;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FOOD_DISTRIBUTION: IMTConSymbol_EnIndustries = 158;
pub const IMTConSymbol_EnIndustries_INDUSTRY_GROCERY_STORES: IMTConSymbol_EnIndustries = 159;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HOUSEHOLD_PRODUCTS: IMTConSymbol_EnIndustries = 160;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PACKAGED_FOODS: IMTConSymbol_EnIndustries = 161;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TOBACCO: IMTConSymbol_EnIndustries = 162;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_DEF_FIRST: IMTConSymbol_EnIndustries = 151;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_DEF_LAST: IMTConSymbol_EnIndustries = 162;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_DEF_END: IMTConSymbol_EnIndustries = 200;
pub const IMTConSymbol_EnIndustries_INDUSTRY_OIL_GAS_DRILLING: IMTConSymbol_EnIndustries = 201;
pub const IMTConSymbol_EnIndustries_INDUSTRY_OIL_GAS_EP: IMTConSymbol_EnIndustries = 202;
pub const IMTConSymbol_EnIndustries_INDUSTRY_OIL_GAS_EQUIPMENT: IMTConSymbol_EnIndustries = 203;
pub const IMTConSymbol_EnIndustries_INDUSTRY_OIL_GAS_INTEGRATED: IMTConSymbol_EnIndustries = 204;
pub const IMTConSymbol_EnIndustries_INDUSTRY_OIL_GAS_MIDSTREAM: IMTConSymbol_EnIndustries = 205;
pub const IMTConSymbol_EnIndustries_INDUSTRY_OIL_GAS_REFINING: IMTConSymbol_EnIndustries = 206;
pub const IMTConSymbol_EnIndustries_INDUSTRY_THERMAL_COAL: IMTConSymbol_EnIndustries = 207;
pub const IMTConSymbol_EnIndustries_INDUSTRY_URANIUM: IMTConSymbol_EnIndustries = 208;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ENERGY_FIRST: IMTConSymbol_EnIndustries = 201;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ENERGY_LAST: IMTConSymbol_EnIndustries = 208;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ENERGY_END: IMTConSymbol_EnIndustries = 250;
pub const IMTConSymbol_EnIndustries_INDUSTRY_EXCHANGE_TRADED_FUND: IMTConSymbol_EnIndustries = 251;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ASSETS_MANAGEMENT: IMTConSymbol_EnIndustries = 252;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BANKS_DIVERSIFIED: IMTConSymbol_EnIndustries = 253;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BANKS_REGIONAL: IMTConSymbol_EnIndustries = 254;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CAPITAL_MARKETS: IMTConSymbol_EnIndustries = 255;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CLOSE_END_FUND_DEBT: IMTConSymbol_EnIndustries = 256;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CLOSE_END_FUND_EQUITY: IMTConSymbol_EnIndustries = 257;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CLOSE_END_FUND_FOREIGN: IMTConSymbol_EnIndustries =
    258;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CREDIT_SERVICES: IMTConSymbol_EnIndustries = 259;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FINANCIAL_CONGLOMERATE: IMTConSymbol_EnIndustries =
    260;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FINANCIAL_DATA_EXCHANGE: IMTConSymbol_EnIndustries =
    261;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INSURANCE_BROKERS: IMTConSymbol_EnIndustries = 262;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INSURANCE_DIVERSIFIED: IMTConSymbol_EnIndustries = 263;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INSURANCE_LIFE: IMTConSymbol_EnIndustries = 264;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INSURANCE_PROPERTY: IMTConSymbol_EnIndustries = 265;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INSURANCE_REINSURANCE: IMTConSymbol_EnIndustries = 266;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INSURANCE_SPECIALTY: IMTConSymbol_EnIndustries = 267;
pub const IMTConSymbol_EnIndustries_INDUSTRY_MORTGAGE_FINANCE: IMTConSymbol_EnIndustries = 268;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SHELL_COMPANIES: IMTConSymbol_EnIndustries = 269;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FINANCIAL_FIRST: IMTConSymbol_EnIndustries = 251;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FINANCIAL_LAST: IMTConSymbol_EnIndustries = 269;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FINANCIAL_END: IMTConSymbol_EnIndustries = 300;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BIOTECHNOLOGY: IMTConSymbol_EnIndustries = 301;
pub const IMTConSymbol_EnIndustries_INDUSTRY_DIAGNOSTICS_RESEARCH: IMTConSymbol_EnIndustries = 302;
pub const IMTConSymbol_EnIndustries_INDUSTRY_DRUGS_MANUFACTURERS: IMTConSymbol_EnIndustries = 303;
pub const IMTConSymbol_EnIndustries_INDUSTRY_DRUGS_MANUFACTURERS_SPEC: IMTConSymbol_EnIndustries =
    304;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HEALTHCARE_PLANS: IMTConSymbol_EnIndustries = 305;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HEALTH_INFORMATION: IMTConSymbol_EnIndustries = 306;
pub const IMTConSymbol_EnIndustries_INDUSTRY_MEDICAL_FACILITIES: IMTConSymbol_EnIndustries = 307;
pub const IMTConSymbol_EnIndustries_INDUSTRY_MEDICAL_DEVICES: IMTConSymbol_EnIndustries = 308;
pub const IMTConSymbol_EnIndustries_INDUSTRY_MEDICAL_DISTRIBUTION: IMTConSymbol_EnIndustries = 309;
pub const IMTConSymbol_EnIndustries_INDUSTRY_MEDICAL_INSTRUMENTS: IMTConSymbol_EnIndustries = 310;
pub const IMTConSymbol_EnIndustries_INDUSTRY_PHARM_RETAILERS: IMTConSymbol_EnIndustries = 311;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HEALTHCARE_FIRST: IMTConSymbol_EnIndustries = 301;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HEALTHCARE_LAST: IMTConSymbol_EnIndustries = 311;
pub const IMTConSymbol_EnIndustries_INDUSTRY_HEALTHCARE_END: IMTConSymbol_EnIndustries = 350;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AEROSPACE_DEFENSE: IMTConSymbol_EnIndustries = 351;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AIRLINES: IMTConSymbol_EnIndustries = 352;
pub const IMTConSymbol_EnIndustries_INDUSTRY_AIRPORTS_SERVICES: IMTConSymbol_EnIndustries = 353;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BUILDING_PRODUCTS: IMTConSymbol_EnIndustries = 354;
pub const IMTConSymbol_EnIndustries_INDUSTRY_BUSINESS_EQUIPMENT: IMTConSymbol_EnIndustries = 355;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONGLOMERATES: IMTConSymbol_EnIndustries = 356;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSULTING_SERVICES: IMTConSymbol_EnIndustries = 357;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ELECTRICAL_EQUIPMENT: IMTConSymbol_EnIndustries = 358;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ENGINEERING_CONSTRUCTION: IMTConSymbol_EnIndustries =
    359;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FARM_HEAVY_MACHINERY: IMTConSymbol_EnIndustries = 360;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INDUSTRIAL_DISTRIBUTION: IMTConSymbol_EnIndustries =
    361;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INFRASTRUCTURE_OPERATIONS: IMTConSymbol_EnIndustries =
    362;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FREIGHT_LOGISTICS: IMTConSymbol_EnIndustries = 363;
pub const IMTConSymbol_EnIndustries_INDUSTRY_MARINE_SHIPPING: IMTConSymbol_EnIndustries = 364;
pub const IMTConSymbol_EnIndustries_INDUSTRY_METAL_FABRICATION: IMTConSymbol_EnIndustries = 365;
pub const IMTConSymbol_EnIndustries_INDUSTRY_POLLUTION_CONTROL: IMTConSymbol_EnIndustries = 366;
pub const IMTConSymbol_EnIndustries_INDUSTRY_RAILROADS: IMTConSymbol_EnIndustries = 367;
pub const IMTConSymbol_EnIndustries_INDUSTRY_RENTAL_LEASING: IMTConSymbol_EnIndustries = 368;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SECURITY_PROTECTION: IMTConSymbol_EnIndustries = 369;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SPEALITY_BUSINESS_SERVICES: IMTConSymbol_EnIndustries =
    370;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SPEALITY_MACHINERY: IMTConSymbol_EnIndustries = 371;
pub const IMTConSymbol_EnIndustries_INDUSTRY_STUFFING_EMPLOYMENT: IMTConSymbol_EnIndustries = 372;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TOOLS_ACCESSORIES: IMTConSymbol_EnIndustries = 373;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TRUCKING: IMTConSymbol_EnIndustries = 374;
pub const IMTConSymbol_EnIndustries_INDUSTRY_WASTE_MANAGEMENT: IMTConSymbol_EnIndustries = 375;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INDUSTRIALS_FIRST: IMTConSymbol_EnIndustries = 351;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INDUSTRIALS_LAST: IMTConSymbol_EnIndustries = 375;
pub const IMTConSymbol_EnIndustries_INDUSTRY_INDUSTRIALS_END: IMTConSymbol_EnIndustries = 400;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REAL_ESTATE_DEVELOPMENT: IMTConSymbol_EnIndustries =
    401;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REAL_ESTATE_DIVERSIFIED: IMTConSymbol_EnIndustries =
    402;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REAL_ESTATE_SERVICES: IMTConSymbol_EnIndustries = 403;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_DIVERSIFIED: IMTConSymbol_EnIndustries = 404;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_HEALTCARE: IMTConSymbol_EnIndustries = 405;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_HOTEL_MOTEL: IMTConSymbol_EnIndustries = 406;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_INDUSTRIAL: IMTConSymbol_EnIndustries = 407;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_MORTAGE: IMTConSymbol_EnIndustries = 408;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_OFFICE: IMTConSymbol_EnIndustries = 409;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_RESIDENTAL: IMTConSymbol_EnIndustries = 410;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_RETAIL: IMTConSymbol_EnIndustries = 411;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REIT_SPECIALITY: IMTConSymbol_EnIndustries = 412;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REAL_ESTATE_FIRST: IMTConSymbol_EnIndustries = 401;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REAL_ESTATE_LAST: IMTConSymbol_EnIndustries = 412;
pub const IMTConSymbol_EnIndustries_INDUSTRY_REAL_ESTATE_END: IMTConSymbol_EnIndustries = 450;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMUNICATION_EQUIPMENT: IMTConSymbol_EnIndustries =
    451;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMPUTER_HARDWARE: IMTConSymbol_EnIndustries = 452;
pub const IMTConSymbol_EnIndustries_INDUSTRY_CONSUMER_ELECTRONICS: IMTConSymbol_EnIndustries = 453;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ELECTRONIC_COMPONENTS: IMTConSymbol_EnIndustries = 454;
pub const IMTConSymbol_EnIndustries_INDUSTRY_ELECTRONIC_DISTRIBUTION: IMTConSymbol_EnIndustries =
    455;
pub const IMTConSymbol_EnIndustries_INDUSTRY_IT_SERVICES: IMTConSymbol_EnIndustries = 456;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SCIENTIFIC_INSTRUMENTS: IMTConSymbol_EnIndustries =
    457;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SEMICONDUCTOR_EQUIPMENT: IMTConSymbol_EnIndustries =
    458;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SEMICONDUCTORS: IMTConSymbol_EnIndustries = 459;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SOFTWARE_APPLICATION: IMTConSymbol_EnIndustries = 460;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SOFTWARE_INFRASTRUCTURE: IMTConSymbol_EnIndustries =
    461;
pub const IMTConSymbol_EnIndustries_INDUSTRY_SOLAR: IMTConSymbol_EnIndustries = 462;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TECHNOLOGY_FIRST: IMTConSymbol_EnIndustries = 451;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TECHNOLOGY_LAST: IMTConSymbol_EnIndustries = 462;
pub const IMTConSymbol_EnIndustries_INDUSTRY_TECHNOLOGY_END: IMTConSymbol_EnIndustries = 500;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_DIVERSIFIED: IMTConSymbol_EnIndustries = 501;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_POWERPRODUCERS: IMTConSymbol_EnIndustries =
    502;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_RENEWABLE: IMTConSymbol_EnIndustries = 503;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_REGULATED_ELECTRIC:
    IMTConSymbol_EnIndustries = 504;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_REGULATED_GAS: IMTConSymbol_EnIndustries =
    505;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_REGULATED_WATER: IMTConSymbol_EnIndustries =
    506;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_FIRST: IMTConSymbol_EnIndustries = 501;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_LAST: IMTConSymbol_EnIndustries = 506;
pub const IMTConSymbol_EnIndustries_INDUSTRY_UTILITIES_END: IMTConSymbol_EnIndustries = 550;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_AGRICULTURAL: IMTConSymbol_EnIndustries =
    551;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_ENERGY: IMTConSymbol_EnIndustries = 552;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_METALS: IMTConSymbol_EnIndustries = 553;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_PRECIOUS: IMTConSymbol_EnIndustries = 554;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_FIRST: IMTConSymbol_EnIndustries = 551;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_LAST: IMTConSymbol_EnIndustries = 554;
pub const IMTConSymbol_EnIndustries_INDUSTRY_COMMODITIES_END: IMTConSymbol_EnIndustries = 600;
pub const IMTConSymbol_EnIndustries_INDUSTRY_FIRST: IMTConSymbol_EnIndustries = 0;
pub const IMTConSymbol_EnIndustries_INDUSTRY_LAST: IMTConSymbol_EnIndustries = 554;
pub type IMTConSymbol_EnIndustries = ::std::os::raw::c_int;
pub const IMTConSymbol_EnFillingFlags_FILL_FLAGS_NONE: IMTConSymbol_EnFillingFlags = 0;
pub const IMTConSymbol_EnFillingFlags_FILL_FLAGS_FOK: IMTConSymbol_EnFillingFlags = 1;
pub const IMTConSymbol_EnFillingFlags_FILL_FLAGS_IOC: IMTConSymbol_EnFillingFlags = 2;
pub const IMTConSymbol_EnFillingFlags_FILL_FLAGS_FIRST: IMTConSymbol_EnFillingFlags = 1;
pub const IMTConSymbol_EnFillingFlags_FILL_FLAGS_ALL: IMTConSymbol_EnFillingFlags = 3;
pub type IMTConSymbol_EnFillingFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_NONE: IMTConSymbol_EnExpirationFlags = 0;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_GTC: IMTConSymbol_EnExpirationFlags = 1;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_DAY: IMTConSymbol_EnExpirationFlags = 2;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_SPECIFIED: IMTConSymbol_EnExpirationFlags = 4;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_SPECIFIED_DAY: IMTConSymbol_EnExpirationFlags =
    8;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_FIRST: IMTConSymbol_EnExpirationFlags = 1;
pub const IMTConSymbol_EnExpirationFlags_TIME_FLAGS_ALL: IMTConSymbol_EnExpirationFlags = 15;
pub type IMTConSymbol_EnExpirationFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_NONE: IMTConSymbol_EnOrderFlags = 0;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_MARKET: IMTConSymbol_EnOrderFlags = 1;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_LIMIT: IMTConSymbol_EnOrderFlags = 2;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_STOP: IMTConSymbol_EnOrderFlags = 4;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_STOP_LIMIT: IMTConSymbol_EnOrderFlags = 8;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_SL: IMTConSymbol_EnOrderFlags = 16;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_TP: IMTConSymbol_EnOrderFlags = 32;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_CLOSEBY: IMTConSymbol_EnOrderFlags = 64;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_FIRST: IMTConSymbol_EnOrderFlags = 1;
pub const IMTConSymbol_EnOrderFlags_ORDER_FLAGS_ALL: IMTConSymbol_EnOrderFlags = 127;
pub type IMTConSymbol_EnOrderFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnTradeMode_TRADE_DISABLED: IMTConSymbol_EnTradeMode = 0;
pub const IMTConSymbol_EnTradeMode_TRADE_LONGONLY: IMTConSymbol_EnTradeMode = 1;
pub const IMTConSymbol_EnTradeMode_TRADE_SHORTONLY: IMTConSymbol_EnTradeMode = 2;
pub const IMTConSymbol_EnTradeMode_TRADE_CLOSEONLY: IMTConSymbol_EnTradeMode = 3;
pub const IMTConSymbol_EnTradeMode_TRADE_FULL: IMTConSymbol_EnTradeMode = 4;
pub const IMTConSymbol_EnTradeMode_TRADE_FIRST: IMTConSymbol_EnTradeMode = 0;
pub const IMTConSymbol_EnTradeMode_TRADE_LAST: IMTConSymbol_EnTradeMode = 4;
pub type IMTConSymbol_EnTradeMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnExecutionMode_EXECUTION_REQUEST: IMTConSymbol_EnExecutionMode = 0;
pub const IMTConSymbol_EnExecutionMode_EXECUTION_INSTANT: IMTConSymbol_EnExecutionMode = 1;
pub const IMTConSymbol_EnExecutionMode_EXECUTION_MARKET: IMTConSymbol_EnExecutionMode = 2;
pub const IMTConSymbol_EnExecutionMode_EXECUTION_EXCHANGE: IMTConSymbol_EnExecutionMode = 3;
pub const IMTConSymbol_EnExecutionMode_EXECUTION_FIRST: IMTConSymbol_EnExecutionMode = 0;
pub const IMTConSymbol_EnExecutionMode_EXECUTION_LAST: IMTConSymbol_EnExecutionMode = 3;
pub type IMTConSymbol_EnExecutionMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_FOREX: IMTConSymbol_EnCalcMode = 0;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_FUTURES: IMTConSymbol_EnCalcMode = 1;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_CFD: IMTConSymbol_EnCalcMode = 2;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_CFDINDEX: IMTConSymbol_EnCalcMode = 3;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_CFDLEVERAGE: IMTConSymbol_EnCalcMode = 4;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_FOREX_NO_LEVERAGE: IMTConSymbol_EnCalcMode = 5;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_MM_FIRST: IMTConSymbol_EnCalcMode = 0;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_MM_LAST: IMTConSymbol_EnCalcMode = 5;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_STOCKS: IMTConSymbol_EnCalcMode = 32;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_FUTURES: IMTConSymbol_EnCalcMode = 33;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_FUTURES_FORTS: IMTConSymbol_EnCalcMode = 34;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_OPTIONS: IMTConSymbol_EnCalcMode = 35;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_OPTIONS_MARGIN: IMTConSymbol_EnCalcMode = 36;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_BONDS: IMTConSymbol_EnCalcMode = 37;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_STOCKS_MOEX: IMTConSymbol_EnCalcMode = 38;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_BONDS_MOEX: IMTConSymbol_EnCalcMode = 39;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_FIRST: IMTConSymbol_EnCalcMode = 32;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_EXCH_LAST: IMTConSymbol_EnCalcMode = 39;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_SERV_COLLATERAL: IMTConSymbol_EnCalcMode = 64;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_SERV_FIRST: IMTConSymbol_EnCalcMode = 64;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_SERV_LAST: IMTConSymbol_EnCalcMode = 64;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_FIRST: IMTConSymbol_EnCalcMode = 0;
pub const IMTConSymbol_EnCalcMode_TRADE_MODE_LAST: IMTConSymbol_EnCalcMode = 64;
pub type IMTConSymbol_EnCalcMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnGTCMode_ORDERS_GTC: IMTConSymbol_EnGTCMode = 0;
pub const IMTConSymbol_EnGTCMode_ORDERS_DAILY: IMTConSymbol_EnGTCMode = 1;
pub const IMTConSymbol_EnGTCMode_ORDERS_DAILY_NO_STOPS: IMTConSymbol_EnGTCMode = 2;
pub const IMTConSymbol_EnGTCMode_ORDERS_FIRST: IMTConSymbol_EnGTCMode = 0;
pub const IMTConSymbol_EnGTCMode_ORDERS_LAST: IMTConSymbol_EnGTCMode = 2;
pub type IMTConSymbol_EnGTCMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnTickFlags_TICK_REALTIME: IMTConSymbol_EnTickFlags = 1;
pub const IMTConSymbol_EnTickFlags_TICK_COLLECTRAW: IMTConSymbol_EnTickFlags = 2;
pub const IMTConSymbol_EnTickFlags_TICK_FEED_STATS: IMTConSymbol_EnTickFlags = 4;
pub const IMTConSymbol_EnTickFlags_TICK_NEGATIVE_PRICES: IMTConSymbol_EnTickFlags = 8;
pub const IMTConSymbol_EnTickFlags_TICK_NONE: IMTConSymbol_EnTickFlags = 0;
pub const IMTConSymbol_EnTickFlags_TICK_ALL: IMTConSymbol_EnTickFlags = 15;
pub type IMTConSymbol_EnTickFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnChartMode_CHART_MODE_BID_PRICE: IMTConSymbol_EnChartMode = 0;
pub const IMTConSymbol_EnChartMode_CHART_MODE_LAST_PRICE: IMTConSymbol_EnChartMode = 1;
pub const IMTConSymbol_EnChartMode_CHART_MODE_OLD: IMTConSymbol_EnChartMode = 255;
pub const IMTConSymbol_EnChartMode_CHART_MODE_FIRST: IMTConSymbol_EnChartMode = 0;
pub const IMTConSymbol_EnChartMode_CHART_MODE_LAST: IMTConSymbol_EnChartMode = 255;
pub type IMTConSymbol_EnChartMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnMarginFlags_MARGIN_FLAGS_NONE: IMTConSymbol_EnMarginFlags = 0;
pub const IMTConSymbol_EnMarginFlags_MARGIN_FLAGS_CHECK_PROCESS: IMTConSymbol_EnMarginFlags = 1;
pub const IMTConSymbol_EnMarginFlags_MARGIN_FLAGS_CHECK_SLTP: IMTConSymbol_EnMarginFlags = 2;
pub const IMTConSymbol_EnMarginFlags_MARGIN_FLAGS_HEDGE_LARGE_LEG: IMTConSymbol_EnMarginFlags = 4;
pub const IMTConSymbol_EnMarginFlags_MARGIN_FLAGS_EXCLUDE_PL: IMTConSymbol_EnMarginFlags = 8;
pub const IMTConSymbol_EnMarginFlags_MARGIN_FLAGS_ALL: IMTConSymbol_EnMarginFlags = 15;
pub type IMTConSymbol_EnMarginFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnSwapMode_SWAP_DISABLED: IMTConSymbol_EnSwapMode = 0;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_POINTS: IMTConSymbol_EnSwapMode = 1;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_SYMBOL_CURRENCY: IMTConSymbol_EnSwapMode = 2;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_MARGIN_CURRENCY: IMTConSymbol_EnSwapMode = 3;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_GROUP_CURRENCY: IMTConSymbol_EnSwapMode = 4;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_INTEREST_CURRENT: IMTConSymbol_EnSwapMode = 5;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_INTEREST_OPEN: IMTConSymbol_EnSwapMode = 6;
pub const IMTConSymbol_EnSwapMode_SWAP_REOPEN_BY_CLOSE_PRICE: IMTConSymbol_EnSwapMode = 7;
pub const IMTConSymbol_EnSwapMode_SWAP_REOPEN_BY_BID: IMTConSymbol_EnSwapMode = 8;
pub const IMTConSymbol_EnSwapMode_SWAP_BY_PROFIT_CURRENCY: IMTConSymbol_EnSwapMode = 9;
pub const IMTConSymbol_EnSwapMode_SWAP_FIRST: IMTConSymbol_EnSwapMode = 0;
pub const IMTConSymbol_EnSwapMode_SWAP_LAST: IMTConSymbol_EnSwapMode = 9;
pub type IMTConSymbol_EnSwapMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_SUNDAY: IMTConSymbol_EnSwapDays = 0;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_MONDAY: IMTConSymbol_EnSwapDays = 1;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_TUESDAY: IMTConSymbol_EnSwapDays = 2;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_WEDNESDAY: IMTConSymbol_EnSwapDays = 3;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_THURSDAY: IMTConSymbol_EnSwapDays = 4;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_FRIDAY: IMTConSymbol_EnSwapDays = 5;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_SATURDAY: IMTConSymbol_EnSwapDays = 6;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_DISABLED: IMTConSymbol_EnSwapDays = 7;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_FIRST: IMTConSymbol_EnSwapDays = 0;
pub const IMTConSymbol_EnSwapDays_SWAP_DAY_LAST: IMTConSymbol_EnSwapDays = 7;
pub type IMTConSymbol_EnSwapDays = ::std::os::raw::c_int;
pub const IMTConSymbol_EnSwapFlags_SWAP_FLAGS_NONE: IMTConSymbol_EnSwapFlags = 0;
pub const IMTConSymbol_EnSwapFlags_SWAP_FLAGS_CONSIDER_HOLIDAYS: IMTConSymbol_EnSwapFlags = 1;
pub const IMTConSymbol_EnSwapFlags_SWAP_FLAGS_DEFAULT: IMTConSymbol_EnSwapFlags = 0;
pub const IMTConSymbol_EnSwapFlags_SWAP_FLAGS_ALL: IMTConSymbol_EnSwapFlags = 1;
pub type IMTConSymbol_EnSwapFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnInstantFlags_INSTANT_FLAGS_NONE: IMTConSymbol_EnInstantFlags = 0;
pub const IMTConSymbol_EnInstantFlags_INSTANT_FLAGS_FAST_CONFIRMATION: IMTConSymbol_EnInstantFlags =
    1;
pub const IMTConSymbol_EnInstantFlags_INSTANT_FLAGS_ALL: IMTConSymbol_EnInstantFlags = 1;
pub type IMTConSymbol_EnInstantFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnInstantMode_INSTANT_CHECK_NORMAL: IMTConSymbol_EnInstantMode = 0;
pub const IMTConSymbol_EnInstantMode_INSTANT_CHECK_FIRST: IMTConSymbol_EnInstantMode = 0;
pub const IMTConSymbol_EnInstantMode_INSTANT_CHECK_LAST: IMTConSymbol_EnInstantMode = 0;
pub type IMTConSymbol_EnInstantMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnRequestFlags_REQUEST_FLAGS_NONE: IMTConSymbol_EnRequestFlags = 0;
pub const IMTConSymbol_EnRequestFlags_REQUEST_FLAGS_ORDER: IMTConSymbol_EnRequestFlags = 1;
pub const IMTConSymbol_EnRequestFlags_REQUEST_FLAGS_ALL: IMTConSymbol_EnRequestFlags = 1;
pub type IMTConSymbol_EnRequestFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnTradeFlags_TRADE_FLAGS_NONE: IMTConSymbol_EnTradeFlags = 0;
pub const IMTConSymbol_EnTradeFlags_TRADE_FLAGS_PROFIT_BY_MARKET: IMTConSymbol_EnTradeFlags = 1;
pub const IMTConSymbol_EnTradeFlags_TRADE_FLAGS_ALLOW_SIGNALS: IMTConSymbol_EnTradeFlags = 2;
pub const IMTConSymbol_EnTradeFlags_TRADE_FLAGS_ALL: IMTConSymbol_EnTradeFlags = 3;
pub const IMTConSymbol_EnTradeFlags_TRADE_FLAGS_DEFAULT: IMTConSymbol_EnTradeFlags = 2;
pub type IMTConSymbol_EnTradeFlags = ::std::os::raw::c_int;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_BUY: IMTConSymbol_EnMarginRateTypes = 0;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_SELL: IMTConSymbol_EnMarginRateTypes = 1;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_BUY_LIMIT: IMTConSymbol_EnMarginRateTypes = 2;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_SELL_LIMIT: IMTConSymbol_EnMarginRateTypes = 3;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_BUY_STOP: IMTConSymbol_EnMarginRateTypes = 4;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_SELL_STOP: IMTConSymbol_EnMarginRateTypes = 5;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_BUY_STOP_LIMIT:
    IMTConSymbol_EnMarginRateTypes = 6;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_SELL_STOP_LIMIT:
    IMTConSymbol_EnMarginRateTypes = 7;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_FIRST: IMTConSymbol_EnMarginRateTypes = 0;
pub const IMTConSymbol_EnMarginRateTypes_MARGIN_RATE_LAST: IMTConSymbol_EnMarginRateTypes = 7;
pub type IMTConSymbol_EnMarginRateTypes = ::std::os::raw::c_int;
pub const IMTConSymbol_EnOptionMode_OPTION_MODE_EUROPEAN_CALL: IMTConSymbol_EnOptionMode = 0;
pub const IMTConSymbol_EnOptionMode_OPTION_MODE_EUROPEAN_PUT: IMTConSymbol_EnOptionMode = 1;
pub const IMTConSymbol_EnOptionMode_OPTION_MODE_AMERICAN_CALL: IMTConSymbol_EnOptionMode = 2;
pub const IMTConSymbol_EnOptionMode_OPTION_MODE_AMERICAN_PUT: IMTConSymbol_EnOptionMode = 3;
pub const IMTConSymbol_EnOptionMode_OPTION_MODE_FIRST: IMTConSymbol_EnOptionMode = 0;
pub const IMTConSymbol_EnOptionMode_OPTION_MODE_LAST: IMTConSymbol_EnOptionMode = 3;
pub type IMTConSymbol_EnOptionMode = ::std::os::raw::c_int;
pub const IMTConSymbol_EnSpliceType_SPLICE_NONE: IMTConSymbol_EnSpliceType = 0;
pub const IMTConSymbol_EnSpliceType_SPLICE_UNADJUSTED: IMTConSymbol_EnSpliceType = 1;
pub const IMTConSymbol_EnSpliceType_SPLICE_ADJUSTED: IMTConSymbol_EnSpliceType = 2;
pub const IMTConSymbol_EnSpliceType_SPLICE_FIRST: IMTConSymbol_EnSpliceType = 0;
pub const IMTConSymbol_EnSpliceType_SPLICE_LAST: IMTConSymbol_EnSpliceType = 2;
pub type IMTConSymbol_EnSpliceType = ::std::os::raw::c_int;
pub const IMTConSymbol_EnSpliceTimeType_SPLICE_TIME_EXPIRATION: IMTConSymbol_EnSpliceTimeType = 0;
pub const IMTConSymbol_EnSpliceTimeType_SPLICE_TIME_FIRST: IMTConSymbol_EnSpliceTimeType = 0;
pub const IMTConSymbol_EnSpliceTimeType_SPLICE_TIME_LAST: IMTConSymbol_EnSpliceTimeType = 0;
pub type IMTConSymbol_EnSpliceTimeType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSymbol"][::std::mem::size_of::<IMTConSymbol>() - 24usize];
    ["Alignment of IMTConSymbol"][::std::mem::align_of::<IMTConSymbol>() - 8usize];
};
#[repr(C)]
pub struct IMTConSymbolSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConSymbolSink {
    pub vtable_: *const IMTConSymbolSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSymbolSink"][::std::mem::size_of::<IMTConSymbolSink>() - 8usize];
    ["Alignment of IMTConSymbolSink"][::std::mem::align_of::<IMTConSymbolSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConGroupSymbol__bindgen_vtable {
    pub IMTConGroupSymbol_Release: unsafe extern "C" fn(this: *mut IMTConGroupSymbol),
    pub IMTConGroupSymbol_Assign: unsafe extern "C" fn(
        this: *mut IMTConGroupSymbol,
        group: *const IMTConGroupSymbol,
    ) -> MTAPIRES,
    pub IMTConGroupSymbol_Clear: unsafe extern "C" fn(this: *mut IMTConGroupSymbol) -> MTAPIRES,
    pub IMTConGroupSymbol_Default: unsafe extern "C" fn(this: *mut IMTConGroupSymbol) -> MTAPIRES,
    pub IMTConGroupSymbol_Path1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, path: LPCWSTR) -> MTAPIRES,
    pub IMTConGroupSymbol_Path: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> LPCWSTR,
    pub IMTConGroupSymbol_TradeMode1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_TradeMode: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_TradeModeDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_ExecMode1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_ExecMode: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_ExecModeDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_FillFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_FillFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_FillFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_ExpirFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_ExpirFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_ExpirFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SpreadDiff1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, spread: INT) -> MTAPIRES,
    pub IMTConGroupSymbol_SpreadDiff: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_SpreadDiffDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_SpreadDiffBalance1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, spread: INT) -> MTAPIRES,
    pub IMTConGroupSymbol_SpreadDiffBalance:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_SpreadDiffBalanceDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_StopsLevel1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, level: INT) -> MTAPIRES,
    pub IMTConGroupSymbol_StopsLevel: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_StopsLevelDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_FreezeLevel1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, level: INT) -> MTAPIRES,
    pub IMTConGroupSymbol_FreezeLevel: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_FreezeLevelDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> INT,
    pub IMTConGroupSymbol_VolumeMin1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeMin: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeMinDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeMax1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeMax: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeMaxDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeStep1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeStep:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeStepDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeLimit1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeLimit:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeLimitDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_MarginFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_MarginFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_MarginInitial1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginInitial:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginInitialDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginMaintenance1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginMaintenance:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginMaintenanceDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginLong1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginLong: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginLongDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginShort1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginShort: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginShortDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginLimit1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginLimit: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginLimitDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginStop1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginStop: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginStopDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginStopLimit1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginStopLimit:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginStopLimitDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapMode1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapMode: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SwapModeDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SwapLong1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, swap: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapLong: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapLongDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapShort1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, swap: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapShort: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapShortDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_Swap3Day1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, day: ::std::os::raw::c_int) -> MTAPIRES,
    pub IMTConGroupSymbol_Swap3Day:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> ::std::os::raw::c_int,
    pub IMTConGroupSymbol_Swap3DayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> ::std::os::raw::c_int,
    pub IMTConGroupSymbol_RETimeout1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, timeout: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_RETimeout: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_RETimeoutDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IECheckMode1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_IECheckMode: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IECheckModeDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IETimeout1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, timeout: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_IETimeout: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IETimeoutDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IESlipProfit1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, slippage: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_IESlipProfit:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IESlipProfitDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IESlipLosing1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, slippage: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_IESlipLosing:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IESlipLosingDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IEVolumeMax1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_IEVolumeMax:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_IEVolumeMaxDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_OrderFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_OrderFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_OrderFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_MarginRateInitial1: unsafe extern "C" fn(
        this: *mut IMTConGroupSymbol,
        type_: UINT,
        margin_rate: f64,
    ) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginRateInitial:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol, type_: UINT) -> f64,
    pub IMTConGroupSymbol_MarginRateInitialDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginRateMaintenance1: unsafe extern "C" fn(
        this: *mut IMTConGroupSymbol,
        type_: UINT,
        margin_rate: f64,
    ) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginRateMaintenance:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol, type_: UINT) -> f64,
    pub IMTConGroupSymbol_MarginRateMaintenanceDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginRateLiquidity1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginRateLiquidity:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginRateLiquidityDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_REFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_REFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_REFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_MarginHedged1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginHedged: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginHedgedDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_PermissionsFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_PermissionsFlags:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_MarginRateCurrency1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, margin_rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_MarginRateCurrency:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_MarginRateCurrencyDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_BookDepthLimit1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, depth: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_BookDepthLimit:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IEFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_IEFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_IEFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_VolumeMinExt1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeMinExt:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeMinExtDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeMaxExt1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeMaxExt:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeMaxExtDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeStepExt1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeStepExt:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeStepExtDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeLimitExt1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_VolumeLimitExt:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_VolumeLimitExtDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_IEVolumeMaxExt1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, volume: UINT64) -> MTAPIRES,
    pub IMTConGroupSymbol_IEVolumeMaxExt:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_IEVolumeMaxExtDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT64,
    pub IMTConGroupSymbol_SwapYearDays1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, days: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapYearDays:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SwapYearDaysDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SwapFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, flags: UINT) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapFlags: unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SwapFlagsDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> UINT,
    pub IMTConGroupSymbol_SwapRateSunday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateSunday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateSundayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateMonday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateMonday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateMondayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateTuesday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateTuesday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateTuesdayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateWednesday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateWednesday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateWednesdayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateThursday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateThursday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateThursdayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateFriday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateFriday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateFridayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateSaturday1:
        unsafe extern "C" fn(this: *mut IMTConGroupSymbol, rate: f64) -> MTAPIRES,
    pub IMTConGroupSymbol_SwapRateSaturday:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
    pub IMTConGroupSymbol_SwapRateSaturdayDefault:
        unsafe extern "C" fn(this: *const IMTConGroupSymbol) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConGroupSymbol {
    pub vtable_: *const IMTConGroupSymbol__bindgen_vtable,
}
pub const IMTConGroupSymbol_EnREFlags_RE_FLAGS_NONE: IMTConGroupSymbol_EnREFlags = 0;
pub const IMTConGroupSymbol_EnREFlags_RE_FLAGS_ORDER: IMTConGroupSymbol_EnREFlags = 1;
pub const IMTConGroupSymbol_EnREFlags_RE_FLAGS_ALL: IMTConGroupSymbol_EnREFlags = 1;
pub type IMTConGroupSymbol_EnREFlags = ::std::os::raw::c_int;
pub const IMTConGroupSymbol_EnPermissionsFlags_PERMISSION_NONE:
    IMTConGroupSymbol_EnPermissionsFlags = 0;
pub const IMTConGroupSymbol_EnPermissionsFlags_PERMISSION_BOOK:
    IMTConGroupSymbol_EnPermissionsFlags = 1;
pub const IMTConGroupSymbol_EnPermissionsFlags_PERMISSION_DEFAULT:
    IMTConGroupSymbol_EnPermissionsFlags = 1;
pub const IMTConGroupSymbol_EnPermissionsFlags_PERMISSION_ALL:
    IMTConGroupSymbol_EnPermissionsFlags = 1;
pub type IMTConGroupSymbol_EnPermissionsFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGroupSymbol"][::std::mem::size_of::<IMTConGroupSymbol>() - 8usize];
    ["Alignment of IMTConGroupSymbol"][::std::mem::align_of::<IMTConGroupSymbol>() - 8usize];
};
#[repr(C)]
pub struct IMTConCommTier__bindgen_vtable {
    pub IMTConCommTier_Release: unsafe extern "C" fn(this: *mut IMTConCommTier),
    pub IMTConCommTier_Assign:
        unsafe extern "C" fn(this: *mut IMTConCommTier, group: *const IMTConCommTier) -> MTAPIRES,
    pub IMTConCommTier_Clear: unsafe extern "C" fn(this: *mut IMTConCommTier) -> MTAPIRES,
    pub IMTConCommTier_Mode1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, mode: UINT) -> MTAPIRES,
    pub IMTConCommTier_Mode: unsafe extern "C" fn(this: *const IMTConCommTier) -> UINT,
    pub IMTConCommTier_Type1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, type_: UINT) -> MTAPIRES,
    pub IMTConCommTier_Type: unsafe extern "C" fn(this: *const IMTConCommTier) -> UINT,
    pub IMTConCommTier_Value1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, value: f64) -> MTAPIRES,
    pub IMTConCommTier_Value: unsafe extern "C" fn(this: *const IMTConCommTier) -> f64,
    pub IMTConCommTier_Minimal1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, value: f64) -> MTAPIRES,
    pub IMTConCommTier_Minimal: unsafe extern "C" fn(this: *const IMTConCommTier) -> f64,
    pub IMTConCommTier_RangeFrom1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, value: f64) -> MTAPIRES,
    pub IMTConCommTier_RangeFrom: unsafe extern "C" fn(this: *const IMTConCommTier) -> f64,
    pub IMTConCommTier_RangeTo1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, value: f64) -> MTAPIRES,
    pub IMTConCommTier_RangeTo: unsafe extern "C" fn(this: *const IMTConCommTier) -> f64,
    pub IMTConCommTier_Currency1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConCommTier_Currency: unsafe extern "C" fn(this: *const IMTConCommTier) -> LPCWSTR,
    pub IMTConCommTier_Maximal1:
        unsafe extern "C" fn(this: *mut IMTConCommTier, value: f64) -> MTAPIRES,
    pub IMTConCommTier_Maximal: unsafe extern "C" fn(this: *const IMTConCommTier) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConCommTier {
    pub vtable_: *const IMTConCommTier__bindgen_vtable,
}
pub const IMTConCommTier_EnCommissionMode_COMM_MONEY_DEPOSIT: IMTConCommTier_EnCommissionMode = 0;
pub const IMTConCommTier_EnCommissionMode_COMM_MONEY_SYMBOL_BASE: IMTConCommTier_EnCommissionMode =
    1;
pub const IMTConCommTier_EnCommissionMode_COMM_MONEY_SYMBOL_PROFIT:
    IMTConCommTier_EnCommissionMode = 2;
pub const IMTConCommTier_EnCommissionMode_COMM_MONEY_SYMBOL_MARGIN:
    IMTConCommTier_EnCommissionMode = 3;
pub const IMTConCommTier_EnCommissionMode_COMM_PIPS: IMTConCommTier_EnCommissionMode = 4;
pub const IMTConCommTier_EnCommissionMode_COMM_PERCENT: IMTConCommTier_EnCommissionMode = 5;
pub const IMTConCommTier_EnCommissionMode_COMM_MONEY_SPECIFIED: IMTConCommTier_EnCommissionMode = 6;
pub const IMTConCommTier_EnCommissionMode_COMM_PERCENT_PROFIT: IMTConCommTier_EnCommissionMode = 7;
pub const IMTConCommTier_EnCommissionMode_COMM_FIRST: IMTConCommTier_EnCommissionMode = 0;
pub const IMTConCommTier_EnCommissionMode_COMM_LAST: IMTConCommTier_EnCommissionMode = 7;
pub type IMTConCommTier_EnCommissionMode = ::std::os::raw::c_int;
pub const IMTConCommTier_EnCommissionVolumeType_COMM_TYPE_DEAL:
    IMTConCommTier_EnCommissionVolumeType = 0;
pub const IMTConCommTier_EnCommissionVolumeType_COMM_TYPE_VOLUME:
    IMTConCommTier_EnCommissionVolumeType = 1;
pub const IMTConCommTier_EnCommissionVolumeType_COMM_TYPE_FIRST:
    IMTConCommTier_EnCommissionVolumeType = 0;
pub const IMTConCommTier_EnCommissionVolumeType_COMM_TYPE_LAST:
    IMTConCommTier_EnCommissionVolumeType = 1;
pub type IMTConCommTier_EnCommissionVolumeType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConCommTier"][::std::mem::size_of::<IMTConCommTier>() - 8usize];
    ["Alignment of IMTConCommTier"][::std::mem::align_of::<IMTConCommTier>() - 8usize];
};
#[repr(C)]
pub struct IMTConCommission__bindgen_vtable {
    pub IMTConCommission_Release: unsafe extern "C" fn(this: *mut IMTConCommission),
    pub IMTConCommission_Assign: unsafe extern "C" fn(
        this: *mut IMTConCommission,
        group: *const IMTConCommission,
    ) -> MTAPIRES,
    pub IMTConCommission_Clear: unsafe extern "C" fn(this: *mut IMTConCommission) -> MTAPIRES,
    pub IMTConCommission_Name1:
        unsafe extern "C" fn(this: *mut IMTConCommission, name: LPCWSTR) -> MTAPIRES,
    pub IMTConCommission_Name: unsafe extern "C" fn(this: *const IMTConCommission) -> LPCWSTR,
    pub IMTConCommission_Description1:
        unsafe extern "C" fn(this: *mut IMTConCommission, descr: LPCWSTR) -> MTAPIRES,
    pub IMTConCommission_Description:
        unsafe extern "C" fn(this: *const IMTConCommission) -> LPCWSTR,
    pub IMTConCommission_Path1:
        unsafe extern "C" fn(this: *mut IMTConCommission, path: LPCWSTR) -> MTAPIRES,
    pub IMTConCommission_Path: unsafe extern "C" fn(this: *const IMTConCommission) -> LPCWSTR,
    pub IMTConCommission_Mode1:
        unsafe extern "C" fn(this: *mut IMTConCommission, mode: UINT) -> MTAPIRES,
    pub IMTConCommission_Mode: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_RangeMode1:
        unsafe extern "C" fn(this: *mut IMTConCommission, mode: UINT) -> MTAPIRES,
    pub IMTConCommission_RangeMode: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_ChargeMode1:
        unsafe extern "C" fn(this: *mut IMTConCommission, mode: UINT) -> MTAPIRES,
    pub IMTConCommission_ChargeMode: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_TierAdd:
        unsafe extern "C" fn(this: *mut IMTConCommission, tier: *mut IMTConCommTier) -> MTAPIRES,
    pub IMTConCommission_TierUpdate: unsafe extern "C" fn(
        this: *mut IMTConCommission,
        pos: UINT,
        tier: *const IMTConCommTier,
    ) -> MTAPIRES,
    pub IMTConCommission_TierDelete:
        unsafe extern "C" fn(this: *mut IMTConCommission, pos: UINT) -> MTAPIRES,
    pub IMTConCommission_TierClear: unsafe extern "C" fn(this: *mut IMTConCommission) -> MTAPIRES,
    pub IMTConCommission_TierShift: unsafe extern "C" fn(
        this: *mut IMTConCommission,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConCommission_TierTotal: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_TierNext: unsafe extern "C" fn(
        this: *const IMTConCommission,
        pos: UINT,
        tier: *mut IMTConCommTier,
    ) -> MTAPIRES,
    pub IMTConCommission_TurnoverCurrency1:
        unsafe extern "C" fn(this: *mut IMTConCommission, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConCommission_TurnoverCurrency:
        unsafe extern "C" fn(this: *const IMTConCommission) -> LPCWSTR,
    pub IMTConCommission_EntryMode1:
        unsafe extern "C" fn(this: *mut IMTConCommission, mode: UINT) -> MTAPIRES,
    pub IMTConCommission_EntryMode: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_ActionMode1:
        unsafe extern "C" fn(this: *mut IMTConCommission, mode: UINT) -> MTAPIRES,
    pub IMTConCommission_ActionMode: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_ProfitMode1:
        unsafe extern "C" fn(this: *mut IMTConCommission, mode: UINT) -> MTAPIRES,
    pub IMTConCommission_ProfitMode: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
    pub IMTConCommission_ReasonFlags1:
        unsafe extern "C" fn(this: *mut IMTConCommission, flags: UINT) -> MTAPIRES,
    pub IMTConCommission_ReasonFlags: unsafe extern "C" fn(this: *const IMTConCommission) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConCommission {
    pub vtable_: *const IMTConCommission__bindgen_vtable,
}
pub const IMTConCommission_EnCommMode_COMM_STANDARD: IMTConCommission_EnCommMode = 0;
pub const IMTConCommission_EnCommMode_COMM_AGENT: IMTConCommission_EnCommMode = 1;
pub const IMTConCommission_EnCommMode_COMM_FEE: IMTConCommission_EnCommMode = 2;
pub const IMTConCommission_EnCommMode_COMM_FIRST: IMTConCommission_EnCommMode = 0;
pub const IMTConCommission_EnCommMode_COMM_LAST: IMTConCommission_EnCommMode = 2;
pub type IMTConCommission_EnCommMode = ::std::os::raw::c_int;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_VOLUME: IMTConCommission_EnCommRangeMode = 0;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_TURNOVER_MONEY:
    IMTConCommission_EnCommRangeMode = 1;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_TURNOVER_VOLUME:
    IMTConCommission_EnCommRangeMode = 2;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_VALUE: IMTConCommission_EnCommRangeMode = 3;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_PROFIT: IMTConCommission_EnCommRangeMode = 4;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_FIRST: IMTConCommission_EnCommRangeMode = 0;
pub const IMTConCommission_EnCommRangeMode_COMM_RANGE_LAST: IMTConCommission_EnCommRangeMode = 4;
pub type IMTConCommission_EnCommRangeMode = ::std::os::raw::c_int;
pub const IMTConCommission_EnCommChargeMode_COMM_CHARGE_DAILY: IMTConCommission_EnCommChargeMode =
    0;
pub const IMTConCommission_EnCommChargeMode_COMM_CHARGE_MONTHLY: IMTConCommission_EnCommChargeMode =
    1;
pub const IMTConCommission_EnCommChargeMode_COMM_CHARGE_INSTANT: IMTConCommission_EnCommChargeMode =
    2;
pub const IMTConCommission_EnCommChargeMode_COMM_CHARGE_FIRST: IMTConCommission_EnCommChargeMode =
    0;
pub const IMTConCommission_EnCommChargeMode_COMM_CHARGE_LAST: IMTConCommission_EnCommChargeMode = 2;
pub type IMTConCommission_EnCommChargeMode = ::std::os::raw::c_int;
pub const IMTConCommission_EnCommEntryMode_COMM_ENTRY_ALL: IMTConCommission_EnCommEntryMode = 0;
pub const IMTConCommission_EnCommEntryMode_COMM_ENTRY_IN: IMTConCommission_EnCommEntryMode = 1;
pub const IMTConCommission_EnCommEntryMode_COMM_ENTRY_OUT: IMTConCommission_EnCommEntryMode = 2;
pub const IMTConCommission_EnCommEntryMode_COMM_ENTRY_FIRST: IMTConCommission_EnCommEntryMode = 0;
pub const IMTConCommission_EnCommEntryMode_COMM_ENTRY_LAST: IMTConCommission_EnCommEntryMode = 2;
pub type IMTConCommission_EnCommEntryMode = ::std::os::raw::c_int;
pub const IMTConCommission_EnCommActionMode_COMM_ACTION_ALL: IMTConCommission_EnCommActionMode = 0;
pub const IMTConCommission_EnCommActionMode_COMM_ACTION_BUY: IMTConCommission_EnCommActionMode = 1;
pub const IMTConCommission_EnCommActionMode_COMM_ACTION_SELL: IMTConCommission_EnCommActionMode = 2;
pub const IMTConCommission_EnCommActionMode_COMM_ACTION_FIRST: IMTConCommission_EnCommActionMode =
    0;
pub const IMTConCommission_EnCommActionMode_COMM_ACTION_LAST: IMTConCommission_EnCommActionMode = 2;
pub type IMTConCommission_EnCommActionMode = ::std::os::raw::c_int;
pub const IMTConCommission_EnCommProfitMode_COMM_PROFIT_ALL: IMTConCommission_EnCommProfitMode = 0;
pub const IMTConCommission_EnCommProfitMode_COMM_PROFIT_PROFIT: IMTConCommission_EnCommProfitMode =
    1;
pub const IMTConCommission_EnCommProfitMode_COMM_PROFIT_LOSS: IMTConCommission_EnCommProfitMode = 2;
pub const IMTConCommission_EnCommProfitMode_COMM_PROFIT_FIRST: IMTConCommission_EnCommProfitMode =
    0;
pub const IMTConCommission_EnCommProfitMode_COMM_PROFIT_LAST: IMTConCommission_EnCommProfitMode = 2;
pub type IMTConCommission_EnCommProfitMode = ::std::os::raw::c_int;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_NONE:
    IMTConCommission_EnCommReasonFlags = 0;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_CLIENT:
    IMTConCommission_EnCommReasonFlags = 1;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_EXPERT:
    IMTConCommission_EnCommReasonFlags = 2;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_DEALER:
    IMTConCommission_EnCommReasonFlags = 4;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_EXTERNAL_CLIENT:
    IMTConCommission_EnCommReasonFlags = 8;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_MOBILE:
    IMTConCommission_EnCommReasonFlags = 16;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_WEB:
    IMTConCommission_EnCommReasonFlags = 32;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_SIGNAL:
    IMTConCommission_EnCommReasonFlags = 64;
pub const IMTConCommission_EnCommReasonFlags_COMM_REASON_FLAG_ALL:
    IMTConCommission_EnCommReasonFlags = 127;
pub type IMTConCommission_EnCommReasonFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConCommission"][::std::mem::size_of::<IMTConCommission>() - 8usize];
    ["Alignment of IMTConCommission"][::std::mem::align_of::<IMTConCommission>() - 8usize];
};
#[repr(C)]
pub struct IMTConGroup__bindgen_vtable {
    pub IMTConGroup_Release: unsafe extern "C" fn(this: *mut IMTConGroup),
    pub IMTConGroup_Assign:
        unsafe extern "C" fn(this: *mut IMTConGroup, group: *const IMTConGroup) -> MTAPIRES,
    pub IMTConGroup_Clear: unsafe extern "C" fn(this: *mut IMTConGroup) -> MTAPIRES,
    pub IMTConGroup_Group1:
        unsafe extern "C" fn(this: *mut IMTConGroup, group: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_Group: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_Server1:
        unsafe extern "C" fn(this: *mut IMTConGroup, server: UINT64) -> MTAPIRES,
    pub IMTConGroup_Server: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT64,
    pub IMTConGroup_PermissionsFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES,
    pub IMTConGroup_PermissionsFlags: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT64,
    pub IMTConGroup_AuthMode1: unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_AuthMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_AuthPasswordMin1:
        unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_AuthPasswordMin: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_Company1:
        unsafe extern "C" fn(this: *mut IMTConGroup, company: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_Company: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanyPage1:
        unsafe extern "C" fn(this: *mut IMTConGroup, page: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanyPage: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanyEmail1:
        unsafe extern "C" fn(this: *mut IMTConGroup, email: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanyEmail: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanySupportPage1:
        unsafe extern "C" fn(this: *mut IMTConGroup, page: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanySupportPage: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanySupportEmail1:
        unsafe extern "C" fn(this: *mut IMTConGroup, email: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanySupportEmail: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanyCatalog1:
        unsafe extern "C" fn(this: *mut IMTConGroup, catalog: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanyCatalog: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_Currency1:
        unsafe extern "C" fn(this: *mut IMTConGroup, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_Currency: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CurrencyDigits: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_ReportsMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_ReportsMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_ReportsFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES,
    pub IMTConGroup_ReportsFlags: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT64,
    pub IMTConGroup_ReportsSMTP1:
        unsafe extern "C" fn(this: *mut IMTConGroup, server: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_ReportsSMTP: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_ReportsSMTPLogin1:
        unsafe extern "C" fn(this: *mut IMTConGroup, login: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_ReportsSMTPLogin: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_ReportsSMTPPass1:
        unsafe extern "C" fn(this: *mut IMTConGroup, password: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_ReportsSMTPPass: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_NewsMode1: unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_NewsMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_NewsCategory1:
        unsafe extern "C" fn(this: *mut IMTConGroup, category: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_NewsCategory: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_NewsLangAdd:
        unsafe extern "C" fn(this: *mut IMTConGroup, language: UINT) -> MTAPIRES,
    pub IMTConGroup_NewsLangUpdate:
        unsafe extern "C" fn(this: *mut IMTConGroup, pos: UINT, language: UINT) -> MTAPIRES,
    pub IMTConGroup_NewsLangDelete:
        unsafe extern "C" fn(this: *mut IMTConGroup, pos: UINT) -> MTAPIRES,
    pub IMTConGroup_NewsLangClear: unsafe extern "C" fn(this: *mut IMTConGroup) -> MTAPIRES,
    pub IMTConGroup_NewsLangTotal: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_NewsLangNext: unsafe extern "C" fn(this: *const IMTConGroup, pos: UINT) -> UINT,
    pub IMTConGroup_MailMode1: unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_MailMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_TradeFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES,
    pub IMTConGroup_TradeFlags: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT64,
    pub IMTConGroup_TradeInterestrate1:
        unsafe extern "C" fn(this: *mut IMTConGroup, rate: f64) -> MTAPIRES,
    pub IMTConGroup_TradeInterestrate: unsafe extern "C" fn(this: *const IMTConGroup) -> f64,
    pub IMTConGroup_TradeVirtualCredit1:
        unsafe extern "C" fn(this: *mut IMTConGroup, credit: f64) -> MTAPIRES,
    pub IMTConGroup_TradeVirtualCredit: unsafe extern "C" fn(this: *const IMTConGroup) -> f64,
    pub IMTConGroup_MarginFreeMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, freemode: UINT) -> MTAPIRES,
    pub IMTConGroup_MarginFreeMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_MarginSOMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, level: UINT) -> MTAPIRES,
    pub IMTConGroup_MarginSOMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_MarginCall1:
        unsafe extern "C" fn(this: *mut IMTConGroup, level: f64) -> MTAPIRES,
    pub IMTConGroup_MarginCall: unsafe extern "C" fn(this: *const IMTConGroup) -> f64,
    pub IMTConGroup_MarginStopOut1:
        unsafe extern "C" fn(this: *mut IMTConGroup, level: f64) -> MTAPIRES,
    pub IMTConGroup_MarginStopOut: unsafe extern "C" fn(this: *const IMTConGroup) -> f64,
    pub IMTConGroup_DemoLeverage1:
        unsafe extern "C" fn(this: *mut IMTConGroup, leverage: UINT) -> MTAPIRES,
    pub IMTConGroup_DemoLeverage: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_DemoDeposit1:
        unsafe extern "C" fn(this: *mut IMTConGroup, deposit: f64) -> MTAPIRES,
    pub IMTConGroup_DemoDeposit: unsafe extern "C" fn(this: *const IMTConGroup) -> f64,
    pub IMTConGroup_LimitHistory1:
        unsafe extern "C" fn(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES,
    pub IMTConGroup_LimitHistory: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_LimitOrders1:
        unsafe extern "C" fn(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES,
    pub IMTConGroup_LimitOrders: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_LimitSymbols1:
        unsafe extern "C" fn(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES,
    pub IMTConGroup_LimitSymbols: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_CommissionAdd:
        unsafe extern "C" fn(this: *mut IMTConGroup, commission: *mut IMTConCommission) -> MTAPIRES,
    pub IMTConGroup_CommissionUpdate: unsafe extern "C" fn(
        this: *mut IMTConGroup,
        pos: UINT,
        commission: *const IMTConCommission,
    ) -> MTAPIRES,
    pub IMTConGroup_CommissionDelete:
        unsafe extern "C" fn(this: *mut IMTConGroup, pos: UINT) -> MTAPIRES,
    pub IMTConGroup_CommissionClear: unsafe extern "C" fn(this: *mut IMTConGroup) -> MTAPIRES,
    pub IMTConGroup_CommissionShift: unsafe extern "C" fn(
        this: *mut IMTConGroup,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConGroup_CommissionTotal: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_CommissionNext: unsafe extern "C" fn(
        this: *const IMTConGroup,
        pos: UINT,
        commission: *mut IMTConCommission,
    ) -> MTAPIRES,
    pub IMTConGroup_CommissionGet: unsafe extern "C" fn(
        this: *const IMTConGroup,
        name: LPCWSTR,
        commission: *mut IMTConCommission,
    ) -> MTAPIRES,
    pub IMTConGroup_SymbolAdd:
        unsafe extern "C" fn(this: *mut IMTConGroup, symbol: *mut IMTConGroupSymbol) -> MTAPIRES,
    pub IMTConGroup_SymbolUpdate: unsafe extern "C" fn(
        this: *mut IMTConGroup,
        pos: UINT,
        symbol: *const IMTConGroupSymbol,
    ) -> MTAPIRES,
    pub IMTConGroup_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTConGroup, pos: UINT) -> MTAPIRES,
    pub IMTConGroup_SymbolClear: unsafe extern "C" fn(this: *mut IMTConGroup) -> MTAPIRES,
    pub IMTConGroup_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTConGroup,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConGroup_SymbolTotal: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_SymbolNext: unsafe extern "C" fn(
        this: *const IMTConGroup,
        pos: UINT,
        symbol: *mut IMTConGroupSymbol,
    ) -> MTAPIRES,
    pub IMTConGroup_SymbolGet: unsafe extern "C" fn(
        this: *const IMTConGroup,
        name: LPCWSTR,
        symbol: *mut IMTConGroupSymbol,
    ) -> MTAPIRES,
    pub IMTConGroup_MarginFreeProfitMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_MarginFreeProfitMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_MarginMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_MarginMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_AuthOTPMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_AuthOTPMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_TradeTransferMode1:
        unsafe extern "C" fn(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES,
    pub IMTConGroup_TradeTransferMode: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_MarginFlags1:
        unsafe extern "C" fn(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES,
    pub IMTConGroup_MarginFlags: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT64,
    pub IMTConGroup_LimitPositions1:
        unsafe extern "C" fn(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES,
    pub IMTConGroup_LimitPositions: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
    pub IMTConGroup_CurrencyDigitsSet:
        unsafe extern "C" fn(this: *mut IMTConGroup, currency_digits: UINT) -> MTAPIRES,
    pub IMTConGroup_ReportsEmail1:
        unsafe extern "C" fn(this: *mut IMTConGroup, email: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_ReportsEmail: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanyDepositPage1:
        unsafe extern "C" fn(this: *mut IMTConGroup, url: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanyDepositPage: unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_CompanyWithdrawalPage1:
        unsafe extern "C" fn(this: *mut IMTConGroup, url: LPCWSTR) -> MTAPIRES,
    pub IMTConGroup_CompanyWithdrawalPage:
        unsafe extern "C" fn(this: *const IMTConGroup) -> LPCWSTR,
    pub IMTConGroup_DemoInactivityPeriod1:
        unsafe extern "C" fn(this: *mut IMTConGroup, period: UINT) -> MTAPIRES,
    pub IMTConGroup_DemoInactivityPeriod: unsafe extern "C" fn(this: *const IMTConGroup) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConGroup {
    pub vtable_: *const IMTConGroup__bindgen_vtable,
}
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_NONE: IMTConGroup_EnPermissionsFlags = 0;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_CERT_CONFIRM: IMTConGroup_EnPermissionsFlags =
    1;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_ENABLE_CONNECTION:
    IMTConGroup_EnPermissionsFlags = 2;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_RESET_PASSWORD: IMTConGroup_EnPermissionsFlags =
    4;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_FORCED_OTP_USAGE:
    IMTConGroup_EnPermissionsFlags = 8;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_RISK_WARNING: IMTConGroup_EnPermissionsFlags =
    16;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_REGULATION_PROTECT:
    IMTConGroup_EnPermissionsFlags = 32;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_NOTIFY_DEALS: IMTConGroup_EnPermissionsFlags =
    64;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_NOTIFY_ORDERS: IMTConGroup_EnPermissionsFlags =
    128;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_NOTIFY_BALANCES:
    IMTConGroup_EnPermissionsFlags = 256;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_NOTIFY_ALL: IMTConGroup_EnPermissionsFlags =
    448;
pub const IMTConGroup_EnPermissionsFlags_PERMISSION_ALL: IMTConGroup_EnPermissionsFlags = 511;
pub type IMTConGroup_EnPermissionsFlags = ::std::os::raw::c_int;
pub const IMTConGroup_EnAuthMode_AUTH_STANDARD: IMTConGroup_EnAuthMode = 0;
pub const IMTConGroup_EnAuthMode_AUTH_RSA1024: IMTConGroup_EnAuthMode = 1;
pub const IMTConGroup_EnAuthMode_AUTH_RSA2048: IMTConGroup_EnAuthMode = 2;
pub const IMTConGroup_EnAuthMode_AUTH_RSA_CUSTOM: IMTConGroup_EnAuthMode = 3;
pub const IMTConGroup_EnAuthMode_AUTH_FIRST: IMTConGroup_EnAuthMode = 0;
pub const IMTConGroup_EnAuthMode_AUTH_LAST: IMTConGroup_EnAuthMode = 2;
pub type IMTConGroup_EnAuthMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnAuthOTPMode_AUTH_OTP_DISABLED: IMTConGroup_EnAuthOTPMode = 0;
pub const IMTConGroup_EnAuthOTPMode_AUTH_OTP_TOTP_SHA256: IMTConGroup_EnAuthOTPMode = 1;
pub const IMTConGroup_EnAuthOTPMode_AUTH_OTP_TOTP_SHA256_WEB: IMTConGroup_EnAuthOTPMode = 2;
pub const IMTConGroup_EnAuthOTPMode_AUTH_OTP_FIRST: IMTConGroup_EnAuthOTPMode = 0;
pub const IMTConGroup_EnAuthOTPMode_AUTH_OTP_LAST: IMTConGroup_EnAuthOTPMode = 2;
pub type IMTConGroup_EnAuthOTPMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnReportsMode_REPORTS_DISABLED: IMTConGroup_EnReportsMode = 0;
pub const IMTConGroup_EnReportsMode_REPORTS_STANDARD: IMTConGroup_EnReportsMode = 1;
pub const IMTConGroup_EnReportsMode_REPORTS_FIRST: IMTConGroup_EnReportsMode = 0;
pub const IMTConGroup_EnReportsMode_REPORTS_LAST: IMTConGroup_EnReportsMode = 1;
pub type IMTConGroup_EnReportsMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnReportsFlags_REPORTSFLAGS_NONE: IMTConGroup_EnReportsFlags = 0;
pub const IMTConGroup_EnReportsFlags_REPORTSFLAGS_EMAIL: IMTConGroup_EnReportsFlags = 1;
pub const IMTConGroup_EnReportsFlags_REPORTSFLAGS_SUPPORT: IMTConGroup_EnReportsFlags = 2;
pub const IMTConGroup_EnReportsFlags_REPORTSFLAGS_STATEMENTS: IMTConGroup_EnReportsFlags = 4;
pub const IMTConGroup_EnReportsFlags_REPORTSFLAGS_ALL: IMTConGroup_EnReportsFlags = 5;
pub type IMTConGroup_EnReportsFlags = ::std::os::raw::c_int;
pub const IMTConGroup_EnNewsMode_NEWS_MODE_DISABLED: IMTConGroup_EnNewsMode = 0;
pub const IMTConGroup_EnNewsMode_NEWS_MODE_HEADERS: IMTConGroup_EnNewsMode = 1;
pub const IMTConGroup_EnNewsMode_NEWS_MODE_FULL: IMTConGroup_EnNewsMode = 2;
pub const IMTConGroup_EnNewsMode_NEWS_MODE_FIRST: IMTConGroup_EnNewsMode = 0;
pub const IMTConGroup_EnNewsMode_NEWS_MODE_LAST: IMTConGroup_EnNewsMode = 2;
pub type IMTConGroup_EnNewsMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnMailMode_MAIL_MODE_DISABLED: IMTConGroup_EnMailMode = 0;
pub const IMTConGroup_EnMailMode_MAIL_MODE_FULL: IMTConGroup_EnMailMode = 1;
pub const IMTConGroup_EnMailMode_MAIL_MODE_FIRST: IMTConGroup_EnMailMode = 0;
pub const IMTConGroup_EnMailMode_MAIL_MODE_LAST: IMTConGroup_EnMailMode = 1;
pub type IMTConGroup_EnMailMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_ALL: IMTConGroup_EnHistoryLimit = 0;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_MONTHS_1: IMTConGroup_EnHistoryLimit = 1;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_MONTHS_3: IMTConGroup_EnHistoryLimit = 2;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_MONTHS_6: IMTConGroup_EnHistoryLimit = 3;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_YEAR_1: IMTConGroup_EnHistoryLimit = 4;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_YEAR_2: IMTConGroup_EnHistoryLimit = 5;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_YEAR_3: IMTConGroup_EnHistoryLimit = 6;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_FIRST: IMTConGroup_EnHistoryLimit = 0;
pub const IMTConGroup_EnHistoryLimit_TRADE_HISTORY_LAST: IMTConGroup_EnHistoryLimit = 6;
pub type IMTConGroup_EnHistoryLimit = ::std::os::raw::c_int;
pub const IMTConGroup_EnFreeMarginMode_FREE_MARGIN_NOT_USE_PL: IMTConGroup_EnFreeMarginMode = 0;
pub const IMTConGroup_EnFreeMarginMode_FREE_MARGIN_USE_PL: IMTConGroup_EnFreeMarginMode = 1;
pub const IMTConGroup_EnFreeMarginMode_FREE_MARGIN_PROFIT: IMTConGroup_EnFreeMarginMode = 2;
pub const IMTConGroup_EnFreeMarginMode_FREE_MARGIN_LOSS: IMTConGroup_EnFreeMarginMode = 3;
pub const IMTConGroup_EnFreeMarginMode_FREE_MARGIN_FIRST: IMTConGroup_EnFreeMarginMode = 0;
pub const IMTConGroup_EnFreeMarginMode_FREE_MARGIN_LAST: IMTConGroup_EnFreeMarginMode = 3;
pub type IMTConGroup_EnFreeMarginMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnTransferMode_TRANSFER_MODE_DISABLED: IMTConGroup_EnTransferMode = 0;
pub const IMTConGroup_EnTransferMode_TRANSFER_MODE_NAME: IMTConGroup_EnTransferMode = 1;
pub const IMTConGroup_EnTransferMode_TRANSFER_MODE_GROUP: IMTConGroup_EnTransferMode = 2;
pub const IMTConGroup_EnTransferMode_TRANSFER_MODE_NAME_GROUP: IMTConGroup_EnTransferMode = 3;
pub const IMTConGroup_EnTransferMode_TRANSFER_MODE_FIRST: IMTConGroup_EnTransferMode = 0;
pub const IMTConGroup_EnTransferMode_TRANSFER_MODE_LAST: IMTConGroup_EnTransferMode = 3;
pub type IMTConGroup_EnTransferMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnStopOutMode_STOPOUT_PERCENT: IMTConGroup_EnStopOutMode = 0;
pub const IMTConGroup_EnStopOutMode_STOPOUT_MONEY: IMTConGroup_EnStopOutMode = 1;
pub const IMTConGroup_EnStopOutMode_STOPOUT_FIRST: IMTConGroup_EnStopOutMode = 0;
pub const IMTConGroup_EnStopOutMode_STOPOUT_LAST: IMTConGroup_EnStopOutMode = 1;
pub type IMTConGroup_EnStopOutMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnMarginFreeProfitMode_FREE_MARGIN_PROFIT_PL:
    IMTConGroup_EnMarginFreeProfitMode = 0;
pub const IMTConGroup_EnMarginFreeProfitMode_FREE_MARGIN_PROFIT_LOSS:
    IMTConGroup_EnMarginFreeProfitMode = 1;
pub const IMTConGroup_EnMarginFreeProfitMode_FREE_MARGIN_PROFIT_FIRST:
    IMTConGroup_EnMarginFreeProfitMode = 0;
pub const IMTConGroup_EnMarginFreeProfitMode_FREE_MARGIN_PROFIT_LAST:
    IMTConGroup_EnMarginFreeProfitMode = 1;
pub type IMTConGroup_EnMarginFreeProfitMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnMarginMode_MARGIN_MODE_RETAIL: IMTConGroup_EnMarginMode = 0;
pub const IMTConGroup_EnMarginMode_MARGIN_MODE_EXCHANGE_DISCOUNT: IMTConGroup_EnMarginMode = 1;
pub const IMTConGroup_EnMarginMode_MARGIN_MODE_RETAIL_HEDGED: IMTConGroup_EnMarginMode = 2;
pub const IMTConGroup_EnMarginMode_MARGIN_MODE_FIRST: IMTConGroup_EnMarginMode = 0;
pub const IMTConGroup_EnMarginMode_MARGIN_MODE_LAST: IMTConGroup_EnMarginMode = 2;
pub type IMTConGroup_EnMarginMode = ::std::os::raw::c_int;
pub const IMTConGroup_EnMarginFlags_MARGIN_FLAGS_NONE: IMTConGroup_EnMarginFlags = 0;
pub const IMTConGroup_EnMarginFlags_MARGIN_FLAGS_CLEAR_ACC: IMTConGroup_EnMarginFlags = 1;
pub const IMTConGroup_EnMarginFlags_MARGIN_FLAGS_ALL: IMTConGroup_EnMarginFlags = 1;
pub type IMTConGroup_EnMarginFlags = ::std::os::raw::c_int;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_NONE: IMTConGroup_EnTradeFlags = 0;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_SWAPS: IMTConGroup_EnTradeFlags = 1;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_TRAILING: IMTConGroup_EnTradeFlags = 2;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_EXPERTS: IMTConGroup_EnTradeFlags = 4;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_EXPIRATION: IMTConGroup_EnTradeFlags = 8;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_SIGNALS_ALL: IMTConGroup_EnTradeFlags = 16;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_SIGNALS_OWN: IMTConGroup_EnTradeFlags = 32;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_SO_COMPENSATION: IMTConGroup_EnTradeFlags = 64;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_SO_FULLY_HEDGED: IMTConGroup_EnTradeFlags = 128;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_FIFO_CLOSE: IMTConGroup_EnTradeFlags = 256;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_HEDGE_PROHIBIT: IMTConGroup_EnTradeFlags = 512;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_DEAL_COST: IMTConGroup_EnTradeFlags = 1024;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_SO_COMPENSATION_CREDIT: IMTConGroup_EnTradeFlags =
    2048;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_DEFAULT: IMTConGroup_EnTradeFlags = 31;
pub const IMTConGroup_EnTradeFlags_TRADEFLAGS_ALL: IMTConGroup_EnTradeFlags = 4095;
pub type IMTConGroup_EnTradeFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGroup"][::std::mem::size_of::<IMTConGroup>() - 8usize];
    ["Alignment of IMTConGroup"][::std::mem::align_of::<IMTConGroup>() - 8usize];
};
#[repr(C)]
pub struct IMTConGroupSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConGroupSink {
    pub vtable_: *const IMTConGroupSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGroupSink"][::std::mem::size_of::<IMTConGroupSink>() - 8usize];
    ["Alignment of IMTConGroupSink"][::std::mem::align_of::<IMTConGroupSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConManagerAccess__bindgen_vtable {
    pub IMTConManagerAccess_Release: unsafe extern "C" fn(this: *mut IMTConManagerAccess),
    pub IMTConManagerAccess_Assign: unsafe extern "C" fn(
        this: *mut IMTConManagerAccess,
        access: *const IMTConManagerAccess,
    ) -> MTAPIRES,
    pub IMTConManagerAccess_Clear: unsafe extern "C" fn(this: *mut IMTConManagerAccess) -> MTAPIRES,
    pub IMTConManagerAccess_From1:
        unsafe extern "C" fn(this: *mut IMTConManagerAccess, name: LPCWSTR) -> MTAPIRES,
    pub IMTConManagerAccess_From: unsafe extern "C" fn(this: *const IMTConManagerAccess) -> LPCWSTR,
    pub IMTConManagerAccess_To1:
        unsafe extern "C" fn(this: *mut IMTConManagerAccess, value: LPCWSTR) -> MTAPIRES,
    pub IMTConManagerAccess_To: unsafe extern "C" fn(this: *const IMTConManagerAccess) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConManagerAccess {
    pub vtable_: *const IMTConManagerAccess__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConManagerAccess"][::std::mem::size_of::<IMTConManagerAccess>() - 8usize];
    ["Alignment of IMTConManagerAccess"][::std::mem::align_of::<IMTConManagerAccess>() - 8usize];
};
#[repr(C)]
pub struct IMTConManager__bindgen_vtable {
    pub IMTConManager_Release: unsafe extern "C" fn(this: *mut IMTConManager),
    pub IMTConManager_Assign:
        unsafe extern "C" fn(this: *mut IMTConManager, manager: *const IMTConManager) -> MTAPIRES,
    pub IMTConManager_Clear: unsafe extern "C" fn(this: *mut IMTConManager) -> MTAPIRES,
    pub IMTConManager_Login1:
        unsafe extern "C" fn(this: *mut IMTConManager, login: UINT64) -> MTAPIRES,
    pub IMTConManager_Login: unsafe extern "C" fn(this: *const IMTConManager) -> UINT64,
    pub IMTConManager_Mailbox1:
        unsafe extern "C" fn(this: *mut IMTConManager, mailbox: LPCWSTR) -> MTAPIRES,
    pub IMTConManager_Mailbox: unsafe extern "C" fn(this: *const IMTConManager) -> LPCWSTR,
    pub IMTConManager_Server: unsafe extern "C" fn(this: *const IMTConManager) -> UINT64,
    pub IMTConManager_LimitLogs1:
        unsafe extern "C" fn(this: *mut IMTConManager, limit: UINT) -> MTAPIRES,
    pub IMTConManager_LimitLogs: unsafe extern "C" fn(this: *const IMTConManager) -> UINT,
    pub IMTConManager_LimitReports1:
        unsafe extern "C" fn(this: *mut IMTConManager, limit: UINT) -> MTAPIRES,
    pub IMTConManager_LimitReports: unsafe extern "C" fn(this: *const IMTConManager) -> UINT,
    pub IMTConManager_Right1:
        unsafe extern "C" fn(this: *mut IMTConManager, right: UINT, flags: UINT) -> MTAPIRES,
    pub IMTConManager_Right: unsafe extern "C" fn(this: *const IMTConManager, right: UINT) -> UINT,
    pub IMTConManager_GroupAdd:
        unsafe extern "C" fn(this: *mut IMTConManager, path: LPCWSTR) -> MTAPIRES,
    pub IMTConManager_GroupUpdate:
        unsafe extern "C" fn(this: *mut IMTConManager, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConManager_GroupShift: unsafe extern "C" fn(
        this: *mut IMTConManager,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConManager_GroupDelete:
        unsafe extern "C" fn(this: *mut IMTConManager, pos: UINT) -> MTAPIRES,
    pub IMTConManager_GroupTotal: unsafe extern "C" fn(this: *const IMTConManager) -> UINT,
    pub IMTConManager_GroupNext:
        unsafe extern "C" fn(this: *const IMTConManager, pos: UINT) -> LPCWSTR,
    pub IMTConManager_AccessAdd: unsafe extern "C" fn(
        this: *mut IMTConManager,
        access: *mut IMTConManagerAccess,
    ) -> MTAPIRES,
    pub IMTConManager_AccessUpdate: unsafe extern "C" fn(
        this: *mut IMTConManager,
        pos: UINT,
        access: *const IMTConManagerAccess,
    ) -> MTAPIRES,
    pub IMTConManager_AccessDelete:
        unsafe extern "C" fn(this: *mut IMTConManager, pos: UINT) -> MTAPIRES,
    pub IMTConManager_AccessShift: unsafe extern "C" fn(
        this: *mut IMTConManager,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConManager_AccessTotal: unsafe extern "C" fn(this: *const IMTConManager) -> UINT,
    pub IMTConManager_AccessNext: unsafe extern "C" fn(
        this: *const IMTConManager,
        pos: UINT,
        access: *mut IMTConManagerAccess,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConManager {
    pub vtable_: *const IMTConManager__bindgen_vtable,
}
pub const IMTConManager_EnManagerRights_RIGHT_ADMIN: IMTConManager_EnManagerRights = 0;
pub const IMTConManager_EnManagerRights_RIGHT_MANAGER: IMTConManager_EnManagerRights = 1;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_SERVERS: IMTConManager_EnManagerRights = 10;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_ACCESS: IMTConManager_EnManagerRights = 11;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_TIME: IMTConManager_EnManagerRights = 12;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_HOLIDAYS: IMTConManager_EnManagerRights = 13;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_HST_SYNC: IMTConManager_EnManagerRights = 14;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_SYMBOLS: IMTConManager_EnManagerRights = 15;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_GROUPS: IMTConManager_EnManagerRights = 16;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_MANAGERS: IMTConManager_EnManagerRights = 17;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_DATAFEEDS: IMTConManager_EnManagerRights = 18;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_REQUESTS: IMTConManager_EnManagerRights = 19;
pub const IMTConManager_EnManagerRights_RIGHT_SRV_JOURNALS: IMTConManager_EnManagerRights = 20;
pub const IMTConManager_EnManagerRights_RIGHT_SRV_REPORTS: IMTConManager_EnManagerRights = 21;
pub const IMTConManager_EnManagerRights_RIGHT_CHARTS: IMTConManager_EnManagerRights = 22;
pub const IMTConManager_EnManagerRights_RIGHT_EMAIL: IMTConManager_EnManagerRights = 23;
pub const IMTConManager_EnManagerRights_RIGHT_ACCOUNTANT: IMTConManager_EnManagerRights = 24;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_READ: IMTConManager_EnManagerRights = 25;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_DETAILS: IMTConManager_EnManagerRights = 26;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_MANAGER: IMTConManager_EnManagerRights = 27;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_ONLINE: IMTConManager_EnManagerRights = 28;
pub const IMTConManager_EnManagerRights_RIGHT_TRADES_READ: IMTConManager_EnManagerRights = 29;
pub const IMTConManager_EnManagerRights_RIGHT_TRADES_MANAGER: IMTConManager_EnManagerRights = 30;
pub const IMTConManager_EnManagerRights_RIGHT_QUOTES: IMTConManager_EnManagerRights = 31;
pub const IMTConManager_EnManagerRights_RIGHT_RISK_MANAGER: IMTConManager_EnManagerRights = 32;
pub const IMTConManager_EnManagerRights_RIGHT_REPORTS: IMTConManager_EnManagerRights = 33;
pub const IMTConManager_EnManagerRights_RIGHT_NEWS: IMTConManager_EnManagerRights = 34;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_GATEWAYS: IMTConManager_EnManagerRights = 35;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_PLUGINS: IMTConManager_EnManagerRights = 36;
pub const IMTConManager_EnManagerRights_RIGHT_TRADES_DEALER: IMTConManager_EnManagerRights = 37;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_REPORTS: IMTConManager_EnManagerRights = 38;
pub const IMTConManager_EnManagerRights_RIGHT_EXPORT: IMTConManager_EnManagerRights = 39;
pub const IMTConManager_EnManagerRights_RIGHT_SYMBOL_DETAILS: IMTConManager_EnManagerRights = 40;
pub const IMTConManager_EnManagerRights_RIGHT_TECHSUPPORT: IMTConManager_EnManagerRights = 41;
pub const IMTConManager_EnManagerRights_RIGHT_TRADES_SUPERVISOR: IMTConManager_EnManagerRights = 42;
pub const IMTConManager_EnManagerRights_RIGHT_QUOTES_RAW: IMTConManager_EnManagerRights = 43;
pub const IMTConManager_EnManagerRights_RIGHT_MARKET: IMTConManager_EnManagerRights = 44;
pub const IMTConManager_EnManagerRights_RIGHT_GRP_DETAILS_MARGIN: IMTConManager_EnManagerRights =
    45;
pub const IMTConManager_EnManagerRights_RIGHT_NOTIFICATIONS: IMTConManager_EnManagerRights = 46;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_DELETE: IMTConManager_EnManagerRights = 47;
pub const IMTConManager_EnManagerRights_RIGHT_TRADES_DELETE: IMTConManager_EnManagerRights = 48;
pub const IMTConManager_EnManagerRights_RIGHT_CONFIRM_ACTIONS: IMTConManager_EnManagerRights = 49;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_ECN: IMTConManager_EnManagerRights = 50;
pub const IMTConManager_EnManagerRights_RIGHT_GRP_DETAILS_COMMISSION:
    IMTConManager_EnManagerRights = 51;
pub const IMTConManager_EnManagerRights_RIGHT_SUBSCRIPTIONS_VIEW: IMTConManager_EnManagerRights =
    52;
pub const IMTConManager_EnManagerRights_RIGHT_SUBSCRIPTIONS_EDIT: IMTConManager_EnManagerRights =
    53;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_FUNDS: IMTConManager_EnManagerRights = 54;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_MAILS: IMTConManager_EnManagerRights = 55;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_MESSENGERS: IMTConManager_EnManagerRights = 56;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_KYC: IMTConManager_EnManagerRights = 57;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_AUTOMATIONS: IMTConManager_EnManagerRights = 58;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_ALLOCATIONS: IMTConManager_EnManagerRights = 59;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_VPS: IMTConManager_EnManagerRights = 60;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_PAYMENTS: IMTConManager_EnManagerRights = 61;
pub const IMTConManager_EnManagerRights_RIGHT_ADMIN_COMPUTER: IMTConManager_EnManagerRights = 62;
pub const IMTConManager_EnManagerRights_RIGHT_CFG_WEB_SERVICES: IMTConManager_EnManagerRights = 63;
pub const IMTConManager_EnManagerRights_RIGHT_FINTEZA_ACCESS: IMTConManager_EnManagerRights = 64;
pub const IMTConManager_EnManagerRights_RIGHT_FINTEZA_WEBSITES: IMTConManager_EnManagerRights = 65;
pub const IMTConManager_EnManagerRights_RIGHT_FINTEZA_CAMPAIGNS: IMTConManager_EnManagerRights = 66;
pub const IMTConManager_EnManagerRights_RIGHT_FINTEZA_REPORTS: IMTConManager_EnManagerRights = 67;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_TECHNICAL: IMTConManager_EnManagerRights = 70;
pub const IMTConManager_EnManagerRights_RIGHT_ACC_TECHNICAL_MODIFY: IMTConManager_EnManagerRights =
    71;
pub const IMTConManager_EnManagerRights_RIGHT_CLIENTS_ACCESS: IMTConManager_EnManagerRights = 96;
pub const IMTConManager_EnManagerRights_RIGHT_CLIENTS_CREATE: IMTConManager_EnManagerRights = 97;
pub const IMTConManager_EnManagerRights_RIGHT_CLIENTS_EDIT: IMTConManager_EnManagerRights = 98;
pub const IMTConManager_EnManagerRights_RIGHT_CLIENTS_DELETE: IMTConManager_EnManagerRights = 99;
pub const IMTConManager_EnManagerRights_RIGHT_DOCUMENTS_ACCESS: IMTConManager_EnManagerRights = 100;
pub const IMTConManager_EnManagerRights_RIGHT_DOCUMENTS_CREATE: IMTConManager_EnManagerRights = 101;
pub const IMTConManager_EnManagerRights_RIGHT_DOCUMENTS_EDIT: IMTConManager_EnManagerRights = 102;
pub const IMTConManager_EnManagerRights_RIGHT_DOCUMENTS_DELETE: IMTConManager_EnManagerRights = 103;
pub const IMTConManager_EnManagerRights_RIGHT_DOCUMENTS_FILES_ADD: IMTConManager_EnManagerRights =
    104;
pub const IMTConManager_EnManagerRights_RIGHT_DOCUMENTS_FILES_DELETE:
    IMTConManager_EnManagerRights = 105;
pub const IMTConManager_EnManagerRights_RIGHT_COMMENTS_ACCESS: IMTConManager_EnManagerRights = 106;
pub const IMTConManager_EnManagerRights_RIGHT_COMMENTS_CREATE: IMTConManager_EnManagerRights = 107;
pub const IMTConManager_EnManagerRights_RIGHT_COMMENTS_DELETE: IMTConManager_EnManagerRights = 108;
pub const IMTConManager_EnManagerRights_RIGHT_CLIENTS_KYC: IMTConManager_EnManagerRights = 109;
pub const IMTConManager_EnManagerRights_RIGHT_FIRST: IMTConManager_EnManagerRights = 0;
pub const IMTConManager_EnManagerRights_RIGHT_LAST: IMTConManager_EnManagerRights = 128;
pub type IMTConManager_EnManagerRights = ::std::os::raw::c_int;
pub const IMTConManager_EnManagerRightFlags_RIGHT_FLAGS_DENIED: IMTConManager_EnManagerRightFlags =
    0;
pub const IMTConManager_EnManagerRightFlags_RIGHT_FLAGS_GRANTED: IMTConManager_EnManagerRightFlags =
    1;
pub const IMTConManager_EnManagerRightFlags_RIGHT_FLAGS_NONE: IMTConManager_EnManagerRightFlags = 0;
pub const IMTConManager_EnManagerRightFlags_RIGHT_FLAGS_ALL: IMTConManager_EnManagerRightFlags = 1;
pub type IMTConManager_EnManagerRightFlags = ::std::os::raw::c_int;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_ALL: IMTConManager_EnManagerLimit = 0;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_MONTHS_1: IMTConManager_EnManagerLimit = 1;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_MONTHS_3: IMTConManager_EnManagerLimit = 2;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_MONTHS_6: IMTConManager_EnManagerLimit = 3;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_YEAR_1: IMTConManager_EnManagerLimit = 4;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_YEAR_2: IMTConManager_EnManagerLimit = 5;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_YEAR_3: IMTConManager_EnManagerLimit = 6;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_FIRST: IMTConManager_EnManagerLimit = 0;
pub const IMTConManager_EnManagerLimit_MANAGER_LIMIT_LAST: IMTConManager_EnManagerLimit = 6;
pub type IMTConManager_EnManagerLimit = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConManager"][::std::mem::size_of::<IMTConManager>() - 8usize];
    ["Alignment of IMTConManager"][::std::mem::align_of::<IMTConManager>() - 8usize];
};
#[repr(C)]
pub struct IMTConManagerSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConManagerSink {
    pub vtable_: *const IMTConManagerSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConManagerSink"][::std::mem::size_of::<IMTConManagerSink>() - 8usize];
    ["Alignment of IMTConManagerSink"][::std::mem::align_of::<IMTConManagerSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConHistorySync__bindgen_vtable {
    pub IMTConHistorySync_Release: unsafe extern "C" fn(this: *mut IMTConHistorySync),
    pub IMTConHistorySync_Assign: unsafe extern "C" fn(
        this: *mut IMTConHistorySync,
        param: *const IMTConHistorySync,
    ) -> MTAPIRES,
    pub IMTConHistorySync_Clear: unsafe extern "C" fn(this: *mut IMTConHistorySync) -> MTAPIRES,
    pub IMTConHistorySync_Server1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, server: LPCWSTR) -> MTAPIRES,
    pub IMTConHistorySync_Server: unsafe extern "C" fn(this: *const IMTConHistorySync) -> LPCWSTR,
    pub IMTConHistorySync_ServerType1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, type_: UINT) -> MTAPIRES,
    pub IMTConHistorySync_ServerType: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT,
    pub IMTConHistorySync_Mode1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, mode: UINT) -> MTAPIRES,
    pub IMTConHistorySync_Mode: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT,
    pub IMTConHistorySync_ModeSync1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, type_: UINT) -> MTAPIRES,
    pub IMTConHistorySync_ModeSync: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT,
    pub IMTConHistorySync_TimeCorrection1: unsafe extern "C" fn(
        this: *mut IMTConHistorySync,
        correction: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConHistorySync_TimeCorrection:
        unsafe extern "C" fn(this: *const IMTConHistorySync) -> ::std::os::raw::c_int,
    pub IMTConHistorySync_From1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, from: INT64) -> MTAPIRES,
    pub IMTConHistorySync_From: unsafe extern "C" fn(this: *const IMTConHistorySync) -> INT64,
    pub IMTConHistorySync_To1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, to: INT64) -> MTAPIRES,
    pub IMTConHistorySync_To: unsafe extern "C" fn(this: *const IMTConHistorySync) -> INT64,
    pub IMTConHistorySync_SymbolAdd:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, path: LPCWSTR) -> MTAPIRES,
    pub IMTConHistorySync_SymbolUpdate:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConHistorySync_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTConHistorySync,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConHistorySync_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, pos: UINT) -> MTAPIRES,
    pub IMTConHistorySync_SymbolTotal: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT,
    pub IMTConHistorySync_SymbolNext:
        unsafe extern "C" fn(this: *const IMTConHistorySync, pos: UINT) -> LPCWSTR,
    pub IMTConHistorySync_Flags1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, flags: UINT64) -> MTAPIRES,
    pub IMTConHistorySync_Flags: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT64,
    pub IMTConHistorySync_HistoryData1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, data: UINT) -> MTAPIRES,
    pub IMTConHistorySync_HistoryData: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT,
    pub IMTConHistorySync_Login1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, login: UINT64) -> MTAPIRES,
    pub IMTConHistorySync_Login: unsafe extern "C" fn(this: *const IMTConHistorySync) -> UINT64,
    pub IMTConHistorySync_Password1:
        unsafe extern "C" fn(this: *mut IMTConHistorySync, password: LPCWSTR) -> MTAPIRES,
    pub IMTConHistorySync_Password: unsafe extern "C" fn(this: *const IMTConHistorySync) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConHistorySync {
    pub vtable_: *const IMTConHistorySync__bindgen_vtable,
}
pub const IMTConHistorySync_EnHistoryMode_HISTORY_DISABLED: IMTConHistorySync_EnHistoryMode = 0;
pub const IMTConHistorySync_EnHistoryMode_HISTORY_ENABLED: IMTConHistorySync_EnHistoryMode = 1;
pub const IMTConHistorySync_EnHistoryMode_HISTORY_FIRST: IMTConHistorySync_EnHistoryMode = 0;
pub const IMTConHistorySync_EnHistoryMode_HISTORY_LAST: IMTConHistorySync_EnHistoryMode = 1;
pub type IMTConHistorySync_EnHistoryMode = ::std::os::raw::c_int;
pub const IMTConHistorySync_EnHistorySyncMode_MODE_REPLACE: IMTConHistorySync_EnHistorySyncMode = 0;
pub const IMTConHistorySync_EnHistorySyncMode_MODE_MERGE: IMTConHistorySync_EnHistorySyncMode = 1;
pub const IMTConHistorySync_EnHistorySyncMode_MODE_FIRST: IMTConHistorySync_EnHistorySyncMode = 0;
pub const IMTConHistorySync_EnHistorySyncMode_MODE_LAST: IMTConHistorySync_EnHistorySyncMode = 1;
pub type IMTConHistorySync_EnHistorySyncMode = ::std::os::raw::c_int;
pub const IMTConHistorySync_EnHistorySyncServer_SERVER_MT4: IMTConHistorySync_EnHistorySyncServer =
    0;
pub const IMTConHistorySync_EnHistorySyncServer_SERVER_MT5: IMTConHistorySync_EnHistorySyncServer =
    1;
pub const IMTConHistorySync_EnHistorySyncServer_SERVER_FIRST:
    IMTConHistorySync_EnHistorySyncServer = 0;
pub const IMTConHistorySync_EnHistorySyncServer_SERVER_LAST: IMTConHistorySync_EnHistorySyncServer =
    1;
pub type IMTConHistorySync_EnHistorySyncServer = ::std::os::raw::c_int;
pub const IMTConHistorySync_EnHistorySyncFlags_FLAG_SESSIONS: IMTConHistorySync_EnHistorySyncFlags =
    1;
pub const IMTConHistorySync_EnHistorySyncFlags_FLAG_SYNONYMS: IMTConHistorySync_EnHistorySyncFlags =
    2;
pub const IMTConHistorySync_EnHistorySyncFlags_FLAG_NONE: IMTConHistorySync_EnHistorySyncFlags = 0;
pub const IMTConHistorySync_EnHistorySyncFlags_FLAG_ALL: IMTConHistorySync_EnHistorySyncFlags = 3;
pub type IMTConHistorySync_EnHistorySyncFlags = ::std::os::raw::c_int;
pub const IMTConHistorySync_EnHistoryData_DATA_HISTORY_CHARTS: IMTConHistorySync_EnHistoryData = 0;
pub const IMTConHistorySync_EnHistoryData_DATA_HISTORY_TICKS: IMTConHistorySync_EnHistoryData = 1;
pub const IMTConHistorySync_EnHistoryData_DATA_HISTORY_ALL: IMTConHistorySync_EnHistoryData = 2;
pub const IMTConHistorySync_EnHistoryData_DATA_HISTORY_FIRST: IMTConHistorySync_EnHistoryData = 0;
pub const IMTConHistorySync_EnHistoryData_DATA_HISTORY_LAST: IMTConHistorySync_EnHistoryData = 2;
pub type IMTConHistorySync_EnHistoryData = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConHistorySync"][::std::mem::size_of::<IMTConHistorySync>() - 8usize];
    ["Alignment of IMTConHistorySync"][::std::mem::align_of::<IMTConHistorySync>() - 8usize];
};
#[repr(C)]
pub struct IMTConHistorySyncSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConHistorySyncSink {
    pub vtable_: *const IMTConHistorySyncSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConHistorySyncSink"][::std::mem::size_of::<IMTConHistorySyncSink>() - 8usize];
    ["Alignment of IMTConHistorySyncSink"]
        [::std::mem::align_of::<IMTConHistorySyncSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConFeederTranslate__bindgen_vtable {
    pub IMTConFeederTranslate_Release: unsafe extern "C" fn(this: *mut IMTConFeederTranslate),
    pub IMTConFeederTranslate_Assign: unsafe extern "C" fn(
        this: *mut IMTConFeederTranslate,
        param: *const IMTConFeederTranslate,
    ) -> MTAPIRES,
    pub IMTConFeederTranslate_Clear:
        unsafe extern "C" fn(this: *mut IMTConFeederTranslate) -> MTAPIRES,
    pub IMTConFeederTranslate_Source1:
        unsafe extern "C" fn(this: *mut IMTConFeederTranslate, source: LPCWSTR) -> MTAPIRES,
    pub IMTConFeederTranslate_Source:
        unsafe extern "C" fn(this: *const IMTConFeederTranslate) -> LPCWSTR,
    pub IMTConFeederTranslate_Symbol1:
        unsafe extern "C" fn(this: *mut IMTConFeederTranslate, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTConFeederTranslate_Symbol:
        unsafe extern "C" fn(this: *const IMTConFeederTranslate) -> LPCWSTR,
    pub IMTConFeederTranslate_BidMarkup1:
        unsafe extern "C" fn(this: *mut IMTConFeederTranslate, markup: INT) -> MTAPIRES,
    pub IMTConFeederTranslate_BidMarkup:
        unsafe extern "C" fn(this: *const IMTConFeederTranslate) -> INT,
    pub IMTConFeederTranslate_AskMarkup1:
        unsafe extern "C" fn(this: *mut IMTConFeederTranslate, markup: INT) -> MTAPIRES,
    pub IMTConFeederTranslate_AskMarkup:
        unsafe extern "C" fn(this: *const IMTConFeederTranslate) -> INT,
    pub IMTConFeederTranslate_Digits:
        unsafe extern "C" fn(this: *const IMTConFeederTranslate) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConFeederTranslate {
    pub vtable_: *const IMTConFeederTranslate__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConFeederTranslate"][::std::mem::size_of::<IMTConFeederTranslate>() - 8usize];
    ["Alignment of IMTConFeederTranslate"]
        [::std::mem::align_of::<IMTConFeederTranslate>() - 8usize];
};
#[repr(C)]
pub struct IMTConFeederModule__bindgen_vtable {
    pub IMTConFeederModule_Release: unsafe extern "C" fn(this: *mut IMTConFeederModule),
    pub IMTConFeederModule_Assign: unsafe extern "C" fn(
        this: *mut IMTConFeederModule,
        param: *const IMTConFeederModule,
    ) -> MTAPIRES,
    pub IMTConFeederModule_Clear: unsafe extern "C" fn(this: *mut IMTConFeederModule) -> MTAPIRES,
    pub IMTConFeederModule_Name: unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_Vendor: unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_Description:
        unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_Module: unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_FeedServer:
        unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_FeedLogin:
        unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_FeedPassword:
        unsafe extern "C" fn(this: *const IMTConFeederModule) -> LPCWSTR,
    pub IMTConFeederModule_Version: unsafe extern "C" fn(this: *const IMTConFeederModule) -> UINT,
    pub IMTConFeederModule_Modes: unsafe extern "C" fn(this: *const IMTConFeederModule) -> UINT,
    pub IMTConFeederModule_Fields: unsafe extern "C" fn(this: *const IMTConFeederModule) -> UINT,
    pub IMTConFeederModule_ParameterTotal:
        unsafe extern "C" fn(this: *const IMTConFeederModule) -> UINT,
    pub IMTConFeederModule_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConFeederModule,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConFeederModule_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConFeederModule,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConFeederModule {
    pub vtable_: *const IMTConFeederModule__bindgen_vtable,
}
pub const IMTConFeederModule_EnFeedersFieldFlags_FEED_FIELD_SERVER:
    IMTConFeederModule_EnFeedersFieldFlags = 1;
pub const IMTConFeederModule_EnFeedersFieldFlags_FEED_FIELD_LOGIN:
    IMTConFeederModule_EnFeedersFieldFlags = 2;
pub const IMTConFeederModule_EnFeedersFieldFlags_FEED_FIELD_PASS:
    IMTConFeederModule_EnFeedersFieldFlags = 4;
pub const IMTConFeederModule_EnFeedersFieldFlags_FEED_FIELD_PARAM:
    IMTConFeederModule_EnFeedersFieldFlags = 8;
pub const IMTConFeederModule_EnFeedersFieldFlags_FEED_FIELD_NONE:
    IMTConFeederModule_EnFeedersFieldFlags = 0;
pub const IMTConFeederModule_EnFeedersFieldFlags_FEED_FIELD_ALL:
    IMTConFeederModule_EnFeedersFieldFlags = 15;
pub type IMTConFeederModule_EnFeedersFieldFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConFeederModule"][::std::mem::size_of::<IMTConFeederModule>() - 8usize];
    ["Alignment of IMTConFeederModule"][::std::mem::align_of::<IMTConFeederModule>() - 8usize];
};
#[repr(C)]
pub struct IMTConGatewayTranslate__bindgen_vtable {
    pub IMTConGatewayTranslate_Release: unsafe extern "C" fn(this: *mut IMTConGatewayTranslate),
    pub IMTConGatewayTranslate_Assign: unsafe extern "C" fn(
        this: *mut IMTConGatewayTranslate,
        param: *const IMTConGatewayTranslate,
    ) -> MTAPIRES,
    pub IMTConGatewayTranslate_Clear:
        unsafe extern "C" fn(this: *mut IMTConGatewayTranslate) -> MTAPIRES,
    pub IMTConGatewayTranslate_Source1:
        unsafe extern "C" fn(this: *mut IMTConGatewayTranslate, source: LPCWSTR) -> MTAPIRES,
    pub IMTConGatewayTranslate_Source:
        unsafe extern "C" fn(this: *const IMTConGatewayTranslate) -> LPCWSTR,
    pub IMTConGatewayTranslate_Symbol1:
        unsafe extern "C" fn(this: *mut IMTConGatewayTranslate, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTConGatewayTranslate_Symbol:
        unsafe extern "C" fn(this: *const IMTConGatewayTranslate) -> LPCWSTR,
    pub IMTConGatewayTranslate_BidMarkup1:
        unsafe extern "C" fn(this: *mut IMTConGatewayTranslate, markup: INT) -> MTAPIRES,
    pub IMTConGatewayTranslate_BidMarkup:
        unsafe extern "C" fn(this: *const IMTConGatewayTranslate) -> INT,
    pub IMTConGatewayTranslate_AskMarkup1:
        unsafe extern "C" fn(this: *mut IMTConGatewayTranslate, markup: INT) -> MTAPIRES,
    pub IMTConGatewayTranslate_AskMarkup:
        unsafe extern "C" fn(this: *const IMTConGatewayTranslate) -> INT,
    pub IMTConGatewayTranslate_Digits:
        unsafe extern "C" fn(this: *const IMTConGatewayTranslate) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConGatewayTranslate {
    pub vtable_: *const IMTConGatewayTranslate__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGatewayTranslate"][::std::mem::size_of::<IMTConGatewayTranslate>() - 8usize];
    ["Alignment of IMTConGatewayTranslate"]
        [::std::mem::align_of::<IMTConGatewayTranslate>() - 8usize];
};
#[repr(C)]
pub struct IMTConGateway__bindgen_vtable {
    pub IMTConGateway_Release: unsafe extern "C" fn(this: *mut IMTConGateway),
    pub IMTConGateway_Assign:
        unsafe extern "C" fn(this: *mut IMTConGateway, param: *const IMTConGateway) -> MTAPIRES,
    pub IMTConGateway_Clear: unsafe extern "C" fn(this: *mut IMTConGateway) -> MTAPIRES,
    pub IMTConGateway_Name1:
        unsafe extern "C" fn(this: *mut IMTConGateway, name: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_Name: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_ID1: unsafe extern "C" fn(this: *mut IMTConGateway, id: UINT64) -> MTAPIRES,
    pub IMTConGateway_ID: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT64,
    pub IMTConGateway_Module1:
        unsafe extern "C" fn(this: *mut IMTConGateway, name: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_Module: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_TradingServer1:
        unsafe extern "C" fn(this: *mut IMTConGateway, server: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_TradingServer: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_TradingLogin1:
        unsafe extern "C" fn(this: *mut IMTConGateway, login: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_TradingLogin: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_TradingPassword1:
        unsafe extern "C" fn(this: *mut IMTConGateway, password: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_TradingPassword: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_GatewayServer1:
        unsafe extern "C" fn(this: *mut IMTConGateway, server: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_GatewayServer: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_Mode1: unsafe extern "C" fn(this: *mut IMTConGateway, mode: UINT) -> MTAPIRES,
    pub IMTConGateway_Mode: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_Flags1:
        unsafe extern "C" fn(this: *mut IMTConGateway, flags: UINT) -> MTAPIRES,
    pub IMTConGateway_Flags: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_ObsoleteValue1:
        unsafe extern "C" fn(this: *mut IMTConGateway, value: UINT) -> MTAPIRES,
    pub IMTConGateway_ObsoleteValue: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_TimeoutReconnect1:
        unsafe extern "C" fn(this: *mut IMTConGateway, timeout: UINT) -> MTAPIRES,
    pub IMTConGateway_TimeoutReconnect: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_TimeoutSleep1:
        unsafe extern "C" fn(this: *mut IMTConGateway, timeout: UINT) -> MTAPIRES,
    pub IMTConGateway_TimeoutSleep: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_TimeoutAttempts1:
        unsafe extern "C" fn(this: *mut IMTConGateway, attempts: UINT) -> MTAPIRES,
    pub IMTConGateway_TimeoutAttempts: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_ParameterAdd:
        unsafe extern "C" fn(this: *mut IMTConGateway, param: *mut IMTConParam) -> MTAPIRES,
    pub IMTConGateway_ParameterUpdate: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        pos: UINT,
        param: *const IMTConParam,
    ) -> MTAPIRES,
    pub IMTConGateway_ParameterDelete:
        unsafe extern "C" fn(this: *mut IMTConGateway, pos: UINT) -> MTAPIRES,
    pub IMTConGateway_ParameterClear: unsafe extern "C" fn(this: *mut IMTConGateway) -> MTAPIRES,
    pub IMTConGateway_ParameterShift: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConGateway_ParameterTotal: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConGateway,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConGateway_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConGateway,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConGateway_SymbolAdd:
        unsafe extern "C" fn(this: *mut IMTConGateway, path: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_SymbolUpdate:
        unsafe extern "C" fn(this: *mut IMTConGateway, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTConGateway, pos: UINT) -> MTAPIRES,
    pub IMTConGateway_SymbolClear: unsafe extern "C" fn(this: *mut IMTConGateway) -> MTAPIRES,
    pub IMTConGateway_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConGateway_SymbolTotal: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_SymbolNext:
        unsafe extern "C" fn(this: *const IMTConGateway, pos: UINT) -> LPCWSTR,
    pub IMTConGateway_GroupAdd:
        unsafe extern "C" fn(this: *mut IMTConGateway, path: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_GroupUpdate:
        unsafe extern "C" fn(this: *mut IMTConGateway, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_GroupDelete:
        unsafe extern "C" fn(this: *mut IMTConGateway, pos: UINT) -> MTAPIRES,
    pub IMTConGateway_GroupClear: unsafe extern "C" fn(this: *mut IMTConGateway) -> MTAPIRES,
    pub IMTConGateway_GroupShift: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConGateway_GroupTotal: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_GroupNext:
        unsafe extern "C" fn(this: *const IMTConGateway, pos: UINT) -> LPCWSTR,
    pub IMTConGateway_TranslateAdd: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        param: *mut IMTConGatewayTranslate,
    ) -> MTAPIRES,
    pub IMTConGateway_TranslateUpdate: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        pos: UINT,
        param: *const IMTConGatewayTranslate,
    ) -> MTAPIRES,
    pub IMTConGateway_TranslateDelete:
        unsafe extern "C" fn(this: *mut IMTConGateway, pos: UINT) -> MTAPIRES,
    pub IMTConGateway_TranslateClear: unsafe extern "C" fn(this: *mut IMTConGateway) -> MTAPIRES,
    pub IMTConGateway_TranslateShift: unsafe extern "C" fn(
        this: *mut IMTConGateway,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConGateway_TranslateTotal: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_TranslateNext: unsafe extern "C" fn(
        this: *const IMTConGateway,
        pos: UINT,
        param: *mut IMTConGatewayTranslate,
    ) -> MTAPIRES,
    pub IMTConGateway_TranslateGet: unsafe extern "C" fn(
        this: *const IMTConGateway,
        symbol: LPCWSTR,
        param: *mut IMTConGatewayTranslate,
    ) -> MTAPIRES,
    pub IMTConGateway_GatewayLogin1:
        unsafe extern "C" fn(this: *mut IMTConGateway, login: UINT64) -> MTAPIRES,
    pub IMTConGateway_GatewayLogin: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT64,
    pub IMTConGateway_GatewayPassword1:
        unsafe extern "C" fn(this: *mut IMTConGateway, password: LPCWSTR) -> MTAPIRES,
    pub IMTConGateway_GatewayPassword: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_TranslateGetSource: unsafe extern "C" fn(
        this: *const IMTConGateway,
        source: LPCWSTR,
        param: *mut IMTConGatewayTranslate,
    ) -> MTAPIRES,
    pub IMTConGateway_Gateway: unsafe extern "C" fn(this: *const IMTConGateway) -> LPCWSTR,
    pub IMTConGateway_StateConnected: unsafe extern "C" fn(this: *const IMTConGateway) -> bool,
    pub IMTConGateway_StateReceivedTicks: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_StateReceivedBooks: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_StateTrafficIn: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_StateTrafficOut: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_StateTradesTotal: unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
    pub IMTConGateway_StateTradesAverageTime:
        unsafe extern "C" fn(this: *const IMTConGateway) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConGateway {
    pub vtable_: *const IMTConGateway__bindgen_vtable,
}
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_REMOTE: IMTConGateway_EnGatewayFlags = 1;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_IMPORT_SYMBOLS: IMTConGateway_EnGatewayFlags =
    2;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_IGNORE_QUOTES: IMTConGateway_EnGatewayFlags = 4;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_IMPORT_BALANCES: IMTConGateway_EnGatewayFlags =
    8;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_EXTENDED_LOG: IMTConGateway_EnGatewayFlags = 16;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_SUPP_POSITIONS: IMTConGateway_EnGatewayFlags =
    32;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_PROFILLING: IMTConGateway_EnGatewayFlags = 64;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_TRIAL: IMTConGateway_EnGatewayFlags = 128;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_NONE: IMTConGateway_EnGatewayFlags = 0;
pub const IMTConGateway_EnGatewayFlags_GATEWAY_FLAG_ALL: IMTConGateway_EnGatewayFlags = 255;
pub type IMTConGateway_EnGatewayFlags = ::std::os::raw::c_int;
pub const IMTConGateway_EnGatewayMode_GATEWAY_DISABLED: IMTConGateway_EnGatewayMode = 0;
pub const IMTConGateway_EnGatewayMode_GATEWAY_ENABLED: IMTConGateway_EnGatewayMode = 1;
pub const IMTConGateway_EnGatewayMode_GATEWAY_FIRST: IMTConGateway_EnGatewayMode = 0;
pub const IMTConGateway_EnGatewayMode_GATEWAY_LAST: IMTConGateway_EnGatewayMode = 1;
pub type IMTConGateway_EnGatewayMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGateway"][::std::mem::size_of::<IMTConGateway>() - 8usize];
    ["Alignment of IMTConGateway"][::std::mem::align_of::<IMTConGateway>() - 8usize];
};
#[repr(C)]
pub struct IMTConGatewayModule__bindgen_vtable {
    pub IMTConGatewayModule_Release: unsafe extern "C" fn(this: *mut IMTConGatewayModule),
    pub IMTConGatewayModule_Assign: unsafe extern "C" fn(
        this: *mut IMTConGatewayModule,
        param: *const IMTConGatewayModule,
    ) -> MTAPIRES,
    pub IMTConGatewayModule_Clear: unsafe extern "C" fn(this: *mut IMTConGatewayModule) -> MTAPIRES,
    pub IMTConGatewayModule_Name: unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_Vendor:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_Description:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_Module:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_TradingServer:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_TradingLogin:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_TradingPassword:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
    pub IMTConGatewayModule_Version: unsafe extern "C" fn(this: *const IMTConGatewayModule) -> UINT,
    pub IMTConGatewayModule_Flags: unsafe extern "C" fn(this: *const IMTConGatewayModule) -> UINT,
    pub IMTConGatewayModule_Fields: unsafe extern "C" fn(this: *const IMTConGatewayModule) -> UINT,
    pub IMTConGatewayModule_ParameterTotal:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> UINT,
    pub IMTConGatewayModule_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConGatewayModule,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConGatewayModule_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConGatewayModule,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConGatewayModule_Gateway:
        unsafe extern "C" fn(this: *const IMTConGatewayModule) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConGatewayModule {
    pub vtable_: *const IMTConGatewayModule__bindgen_vtable,
}
pub const IMTConGatewayModule_EnGatewayFieldMask_GATEWAY_FIELD_SERVER:
    IMTConGatewayModule_EnGatewayFieldMask = 1;
pub const IMTConGatewayModule_EnGatewayFieldMask_GATEWAY_FIELD_LOGIN:
    IMTConGatewayModule_EnGatewayFieldMask = 2;
pub const IMTConGatewayModule_EnGatewayFieldMask_GATEWAY_FIELD_PASS:
    IMTConGatewayModule_EnGatewayFieldMask = 4;
pub const IMTConGatewayModule_EnGatewayFieldMask_GATEWAY_FIELD_PARAM:
    IMTConGatewayModule_EnGatewayFieldMask = 8;
pub const IMTConGatewayModule_EnGatewayFieldMask_GATEWAY_FIELD_NONE:
    IMTConGatewayModule_EnGatewayFieldMask = 0;
pub const IMTConGatewayModule_EnGatewayFieldMask_GATEWAY_FIELD_ALL:
    IMTConGatewayModule_EnGatewayFieldMask = 15;
pub type IMTConGatewayModule_EnGatewayFieldMask = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGatewayModule"][::std::mem::size_of::<IMTConGatewayModule>() - 8usize];
    ["Alignment of IMTConGatewayModule"][::std::mem::align_of::<IMTConGatewayModule>() - 8usize];
};
#[repr(C)]
pub struct IMTConFeeder__bindgen_vtable {
    pub IMTConFeeder_Release: unsafe extern "C" fn(this: *mut IMTConFeeder),
    pub IMTConFeeder_Assign:
        unsafe extern "C" fn(this: *mut IMTConFeeder, param: *const IMTConFeeder) -> MTAPIRES,
    pub IMTConFeeder_Clear: unsafe extern "C" fn(this: *mut IMTConFeeder) -> MTAPIRES,
    pub IMTConFeeder_Name1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, name: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_Name: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_Module1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, name: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_Module: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_FeedServer1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, server: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_FeedServer: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_FeedLogin1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, login: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_FeedLogin: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_FeedPassword1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, password: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_FeedPassword: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_Mode1: unsafe extern "C" fn(this: *mut IMTConFeeder, mode: UINT) -> MTAPIRES,
    pub IMTConFeeder_Mode: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_Flags1: unsafe extern "C" fn(this: *mut IMTConFeeder, flags: UINT) -> MTAPIRES,
    pub IMTConFeeder_Flags: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_Keywords1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, keywords: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_Keywords: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_Categories1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, categories: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_Categories: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_ObsoleteValue1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, value: UINT) -> MTAPIRES,
    pub IMTConFeeder_ObsoleteValue: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_TimeoutReconnect1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, timeout: UINT) -> MTAPIRES,
    pub IMTConFeeder_TimeoutReconnect: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_TimeoutSleep1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, timeout: UINT) -> MTAPIRES,
    pub IMTConFeeder_TimeoutSleep: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_TimeoutAttempts1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, attempts: UINT) -> MTAPIRES,
    pub IMTConFeeder_TimeoutAttempts: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_ParameterAdd:
        unsafe extern "C" fn(this: *mut IMTConFeeder, param: *mut IMTConParam) -> MTAPIRES,
    pub IMTConFeeder_ParameterUpdate: unsafe extern "C" fn(
        this: *mut IMTConFeeder,
        pos: UINT,
        param: *const IMTConParam,
    ) -> MTAPIRES,
    pub IMTConFeeder_ParameterDelete:
        unsafe extern "C" fn(this: *mut IMTConFeeder, pos: UINT) -> MTAPIRES,
    pub IMTConFeeder_ParameterClear: unsafe extern "C" fn(this: *mut IMTConFeeder) -> MTAPIRES,
    pub IMTConFeeder_ParameterShift: unsafe extern "C" fn(
        this: *mut IMTConFeeder,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConFeeder_ParameterTotal: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConFeeder,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConFeeder_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConFeeder,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConFeeder_SymbolAdd:
        unsafe extern "C" fn(this: *mut IMTConFeeder, path: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_SymbolUpdate:
        unsafe extern "C" fn(this: *mut IMTConFeeder, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTConFeeder, pos: UINT) -> MTAPIRES,
    pub IMTConFeeder_SymbolClear: unsafe extern "C" fn(this: *mut IMTConFeeder) -> MTAPIRES,
    pub IMTConFeeder_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTConFeeder,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConFeeder_SymbolTotal: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_SymbolNext:
        unsafe extern "C" fn(this: *const IMTConFeeder, pos: UINT) -> LPCWSTR,
    pub IMTConFeeder_TranslateAdd: unsafe extern "C" fn(
        this: *mut IMTConFeeder,
        param: *mut IMTConFeederTranslate,
    ) -> MTAPIRES,
    pub IMTConFeeder_TranslateUpdate: unsafe extern "C" fn(
        this: *mut IMTConFeeder,
        pos: UINT,
        param: *const IMTConFeederTranslate,
    ) -> MTAPIRES,
    pub IMTConFeeder_TranslateDelete:
        unsafe extern "C" fn(this: *mut IMTConFeeder, pos: UINT) -> MTAPIRES,
    pub IMTConFeeder_TranslateClear: unsafe extern "C" fn(this: *mut IMTConFeeder) -> MTAPIRES,
    pub IMTConFeeder_TranslateShift: unsafe extern "C" fn(
        this: *mut IMTConFeeder,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConFeeder_TranslateTotal: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_TranslateNext: unsafe extern "C" fn(
        this: *const IMTConFeeder,
        pos: UINT,
        param: *mut IMTConFeederTranslate,
    ) -> MTAPIRES,
    pub IMTConFeeder_TranslateGet: unsafe extern "C" fn(
        this: *const IMTConFeeder,
        symbol: LPCWSTR,
        param: *mut IMTConFeederTranslate,
    ) -> MTAPIRES,
    pub IMTConFeeder_GatewayServer1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, server: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_GatewayServer: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_GatewayLogin1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, login: UINT64) -> MTAPIRES,
    pub IMTConFeeder_GatewayLogin: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT64,
    pub IMTConFeeder_GatewayPassword1:
        unsafe extern "C" fn(this: *mut IMTConFeeder, password: LPCWSTR) -> MTAPIRES,
    pub IMTConFeeder_GatewayPassword: unsafe extern "C" fn(this: *const IMTConFeeder) -> LPCWSTR,
    pub IMTConFeeder_StateConnected: unsafe extern "C" fn(this: *const IMTConFeeder) -> bool,
    pub IMTConFeeder_StateReceivedTicks: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_StateReceivedBooks: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_StateReceivedNews: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_StateTrafficIn: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
    pub IMTConFeeder_StateTrafficOut: unsafe extern "C" fn(this: *const IMTConFeeder) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConFeeder {
    pub vtable_: *const IMTConFeeder__bindgen_vtable,
}
pub const IMTConFeeder_EnFeedersFlags_FEED_FLAG_QUOTES: IMTConFeeder_EnFeedersFlags = 1;
pub const IMTConFeeder_EnFeedersFlags_FEED_FLAG_NEWS: IMTConFeeder_EnFeedersFlags = 2;
pub const IMTConFeeder_EnFeedersFlags_FEED_FLAG_REMOTE: IMTConFeeder_EnFeedersFlags = 8;
pub const IMTConFeeder_EnFeedersFlags_FEED_FLAG_TRIAL: IMTConFeeder_EnFeedersFlags = 16;
pub const IMTConFeeder_EnFeedersFlags_FEED_FLAG_NONE: IMTConFeeder_EnFeedersFlags = 0;
pub const IMTConFeeder_EnFeedersFlags_FEED_FLAG_ALL: IMTConFeeder_EnFeedersFlags = 27;
pub type IMTConFeeder_EnFeedersFlags = ::std::os::raw::c_int;
pub const IMTConFeeder_EnFeedersMode_FEEDER_DISABLED: IMTConFeeder_EnFeedersMode = 0;
pub const IMTConFeeder_EnFeedersMode_FEEDER_ENABLED: IMTConFeeder_EnFeedersMode = 1;
pub const IMTConFeeder_EnFeedersMode_FEEDER_FIRST: IMTConFeeder_EnFeedersMode = 0;
pub const IMTConFeeder_EnFeedersMode_FEEDER_LAST: IMTConFeeder_EnFeedersMode = 1;
pub type IMTConFeeder_EnFeedersMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConFeeder"][::std::mem::size_of::<IMTConFeeder>() - 8usize];
    ["Alignment of IMTConFeeder"][::std::mem::align_of::<IMTConFeeder>() - 8usize];
};
#[repr(C)]
pub struct IMTConFeederSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConFeederSink {
    pub vtable_: *const IMTConFeederSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConFeederSink"][::std::mem::size_of::<IMTConFeederSink>() - 8usize];
    ["Alignment of IMTConFeederSink"][::std::mem::align_of::<IMTConFeederSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConGatewaySink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConGatewaySink {
    pub vtable_: *const IMTConGatewaySink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConGatewaySink"][::std::mem::size_of::<IMTConGatewaySink>() - 8usize];
    ["Alignment of IMTConGatewaySink"][::std::mem::align_of::<IMTConGatewaySink>() - 8usize];
};
#[repr(C)]
pub struct IMTConReportModule__bindgen_vtable {
    pub IMTConReportModule_Release: unsafe extern "C" fn(this: *mut IMTConReportModule),
    pub IMTConReportModule_Assign: unsafe extern "C" fn(
        this: *mut IMTConReportModule,
        param: *const IMTConReportModule,
    ) -> MTAPIRES,
    pub IMTConReportModule_Clear: unsafe extern "C" fn(this: *mut IMTConReportModule) -> MTAPIRES,
    pub IMTConReportModule_Name: unsafe extern "C" fn(this: *const IMTConReportModule) -> LPCWSTR,
    pub IMTConReportModule_Vendor: unsafe extern "C" fn(this: *const IMTConReportModule) -> LPCWSTR,
    pub IMTConReportModule_Description:
        unsafe extern "C" fn(this: *const IMTConReportModule) -> LPCWSTR,
    pub IMTConReportModule_Module: unsafe extern "C" fn(this: *const IMTConReportModule) -> LPCWSTR,
    pub IMTConReportModule_Index: unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_Server: unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT64,
    pub IMTConReportModule_Version: unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_VersionAPI:
        unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_VersionIE: unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_Types: unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_Snapshots: unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_ParameterTotal:
        unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConReportModule,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConReportModule_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConReportModule,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConReportModule_InputTotal:
        unsafe extern "C" fn(this: *const IMTConReportModule) -> UINT,
    pub IMTConReportModule_InputNext: unsafe extern "C" fn(
        this: *const IMTConReportModule,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConReportModule_InputGet: unsafe extern "C" fn(
        this: *const IMTConReportModule,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConReportModule {
    pub vtable_: *const IMTConReportModule__bindgen_vtable,
}
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_NONE: IMTConReportModule_EnSnapshots = 0;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_USERS: IMTConReportModule_EnSnapshots = 1;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_USERS_FULL: IMTConReportModule_EnSnapshots = 2;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_ACCOUNTS: IMTConReportModule_EnSnapshots = 4;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_ACCOUNTS_FULL: IMTConReportModule_EnSnapshots = 8;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_ORDERS: IMTConReportModule_EnSnapshots = 16;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_ORDERS_FULL: IMTConReportModule_EnSnapshots = 32;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_POSITIONS: IMTConReportModule_EnSnapshots = 64;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_POSITIONS_FULL: IMTConReportModule_EnSnapshots =
    128;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_FIRST: IMTConReportModule_EnSnapshots = 0;
pub const IMTConReportModule_EnSnapshots_SNAPSHOT_LAST: IMTConReportModule_EnSnapshots = 128;
pub type IMTConReportModule_EnSnapshots = ::std::os::raw::c_int;
pub const IMTConReportModule_EnTypes_TYPE_NONE: IMTConReportModule_EnTypes = 0;
pub const IMTConReportModule_EnTypes_TYPE_HTML: IMTConReportModule_EnTypes = 1;
pub const IMTConReportModule_EnTypes_TYPE_TABLE: IMTConReportModule_EnTypes = 2;
pub const IMTConReportModule_EnTypes_TYPE_FIRST: IMTConReportModule_EnTypes = 0;
pub const IMTConReportModule_EnTypes_TYPE_LAST: IMTConReportModule_EnTypes = 2;
pub const IMTConReportModule_EnTypes_TYPE_ALL: IMTConReportModule_EnTypes = 3;
pub type IMTConReportModule_EnTypes = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConReportModule"][::std::mem::size_of::<IMTConReportModule>() - 8usize];
    ["Alignment of IMTConReportModule"][::std::mem::align_of::<IMTConReportModule>() - 8usize];
};
#[repr(C)]
pub struct IMTConReport__bindgen_vtable {
    pub IMTConReport_Release: unsafe extern "C" fn(this: *mut IMTConReport),
    pub IMTConReport_Assign:
        unsafe extern "C" fn(this: *mut IMTConReport, param: *const IMTConReport) -> MTAPIRES,
    pub IMTConReport_Clear: unsafe extern "C" fn(this: *mut IMTConReport) -> MTAPIRES,
    pub IMTConReport_Name1:
        unsafe extern "C" fn(this: *mut IMTConReport, name: LPCWSTR) -> MTAPIRES,
    pub IMTConReport_Name: unsafe extern "C" fn(this: *const IMTConReport) -> LPCWSTR,
    pub IMTConReport_Server1:
        unsafe extern "C" fn(this: *mut IMTConReport, server: UINT64) -> MTAPIRES,
    pub IMTConReport_Server: unsafe extern "C" fn(this: *const IMTConReport) -> UINT64,
    pub IMTConReport_Module1:
        unsafe extern "C" fn(this: *mut IMTConReport, name: LPCWSTR) -> MTAPIRES,
    pub IMTConReport_Module: unsafe extern "C" fn(this: *const IMTConReport) -> LPCWSTR,
    pub IMTConReport_Mode1: unsafe extern "C" fn(this: *mut IMTConReport, mode: UINT) -> MTAPIRES,
    pub IMTConReport_Mode: unsafe extern "C" fn(this: *const IMTConReport) -> UINT,
    pub IMTConReport_ParameterAdd:
        unsafe extern "C" fn(this: *mut IMTConReport, param: *mut IMTConParam) -> MTAPIRES,
    pub IMTConReport_ParameterUpdate: unsafe extern "C" fn(
        this: *mut IMTConReport,
        pos: UINT,
        param: *const IMTConParam,
    ) -> MTAPIRES,
    pub IMTConReport_ParameterDelete:
        unsafe extern "C" fn(this: *mut IMTConReport, pos: UINT) -> MTAPIRES,
    pub IMTConReport_ParameterClear: unsafe extern "C" fn(this: *mut IMTConReport) -> MTAPIRES,
    pub IMTConReport_ParameterShift: unsafe extern "C" fn(
        this: *mut IMTConReport,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConReport_ParameterTotal: unsafe extern "C" fn(this: *const IMTConReport) -> UINT,
    pub IMTConReport_ParameterNext: unsafe extern "C" fn(
        this: *const IMTConReport,
        pos: UINT,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
    pub IMTConReport_ParameterGet: unsafe extern "C" fn(
        this: *const IMTConReport,
        name: LPCWSTR,
        param: *mut IMTConParam,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConReport {
    pub vtable_: *const IMTConReport__bindgen_vtable,
}
pub const IMTConReport_EnReportMode_REPORT_DISABLED: IMTConReport_EnReportMode = 0;
pub const IMTConReport_EnReportMode_REPORT_ENABLED: IMTConReport_EnReportMode = 1;
pub const IMTConReport_EnReportMode_REPORT_FIRST: IMTConReport_EnReportMode = 0;
pub const IMTConReport_EnReportMode_REPORT_LAST: IMTConReport_EnReportMode = 1;
pub type IMTConReport_EnReportMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConReport"][::std::mem::size_of::<IMTConReport>() - 8usize];
    ["Alignment of IMTConReport"][::std::mem::align_of::<IMTConReport>() - 8usize];
};
#[repr(C)]
pub struct IMTConReportSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConReportSink {
    pub vtable_: *const IMTConReportSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConReportSink"][::std::mem::size_of::<IMTConReportSink>() - 8usize];
    ["Alignment of IMTConReportSink"][::std::mem::align_of::<IMTConReportSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConCondition__bindgen_vtable {
    pub IMTConCondition_Release: unsafe extern "C" fn(this: *mut IMTConCondition),
    pub IMTConCondition_Assign: unsafe extern "C" fn(
        this: *mut IMTConCondition,
        config: *const IMTConCondition,
    ) -> MTAPIRES,
    pub IMTConCondition_Clear: unsafe extern "C" fn(this: *mut IMTConCondition) -> MTAPIRES,
    pub IMTConCondition_Condition1:
        unsafe extern "C" fn(this: *mut IMTConCondition, condition: UINT) -> MTAPIRES,
    pub IMTConCondition_Condition: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT,
    pub IMTConCondition_Rule1:
        unsafe extern "C" fn(this: *mut IMTConCondition, rule: UINT) -> MTAPIRES,
    pub IMTConCondition_Rule: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT,
    pub IMTConCondition_ValueType: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT,
    pub IMTConCondition_ValueInt1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: INT64) -> MTAPIRES,
    pub IMTConCondition_ValueInt: unsafe extern "C" fn(this: *const IMTConCondition) -> INT64,
    pub IMTConCondition_ValueUInt1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: UINT64) -> MTAPIRES,
    pub IMTConCondition_ValueUInt: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT64,
    pub IMTConCondition_ValueDouble1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: f64) -> MTAPIRES,
    pub IMTConCondition_ValueDouble: unsafe extern "C" fn(this: *const IMTConCondition) -> f64,
    pub IMTConCondition_ValueString1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: LPCWSTR) -> MTAPIRES,
    pub IMTConCondition_ValueString: unsafe extern "C" fn(this: *const IMTConCondition) -> LPCWSTR,
    pub IMTConCondition_ValueColor1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: COLORREF) -> MTAPIRES,
    pub IMTConCondition_ValueColor: unsafe extern "C" fn(this: *const IMTConCondition) -> COLORREF,
    pub IMTConCondition_ValueMoney1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: f64) -> MTAPIRES,
    pub IMTConCondition_ValueMoney: unsafe extern "C" fn(this: *const IMTConCondition) -> f64,
    pub IMTConCondition_ValueVolume1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: UINT64) -> MTAPIRES,
    pub IMTConCondition_ValueVolume: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT64,
    pub IMTConCondition_ValueDatetime1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: INT64) -> MTAPIRES,
    pub IMTConCondition_ValueDatetime: unsafe extern "C" fn(this: *const IMTConCondition) -> INT64,
    pub IMTConCondition_ValueLeverage1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: INT64) -> MTAPIRES,
    pub IMTConCondition_ValueLeverage: unsafe extern "C" fn(this: *const IMTConCondition) -> INT64,
    pub IMTConCondition_ValueBool1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: bool) -> MTAPIRES,
    pub IMTConCondition_ValueBool: unsafe extern "C" fn(this: *const IMTConCondition) -> bool,
    pub IMTConCondition_ValueTime1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: UINT) -> MTAPIRES,
    pub IMTConCondition_ValueTime: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT,
    pub IMTConCondition_ValueWeekDay1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: UINT) -> MTAPIRES,
    pub IMTConCondition_ValueWeekDay: unsafe extern "C" fn(this: *const IMTConCondition) -> UINT,
    pub IMTConCondition_ValueVolumeExt1:
        unsafe extern "C" fn(this: *mut IMTConCondition, value: UINT64) -> MTAPIRES,
    pub IMTConCondition_ValueVolumeExt:
        unsafe extern "C" fn(this: *const IMTConCondition) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConCondition {
    pub vtable_: *const IMTConCondition__bindgen_vtable,
}
pub const IMTConCondition_EnRouteCondition_CONDITION_DATETIME: IMTConCondition_EnRouteCondition = 0;
pub const IMTConCondition_EnRouteCondition_CONDITION_SYMBOL: IMTConCondition_EnRouteCondition = 1;
pub const IMTConCondition_EnRouteCondition_CONDITION_VOLUME: IMTConCondition_EnRouteCondition = 2;
pub const IMTConCondition_EnRouteCondition_CONDITION_MARKET_DEVIATION:
    IMTConCondition_EnRouteCondition = 3;
pub const IMTConCondition_EnRouteCondition_CONDITION_TIME: IMTConCondition_EnRouteCondition = 4;
pub const IMTConCondition_EnRouteCondition_CONDITION_WEEKDAY: IMTConCondition_EnRouteCondition = 5;
pub const IMTConCondition_EnRouteCondition_CONDITION_COMMENT: IMTConCondition_EnRouteCondition = 6;
pub const IMTConCondition_EnRouteCondition_CONDITION_EXPERT: IMTConCondition_EnRouteCondition = 7;
pub const IMTConCondition_EnRouteCondition_CONDITION_SIGNAL: IMTConCondition_EnRouteCondition = 8;
pub const IMTConCondition_EnRouteCondition_CONDITION_DEALER_LOGIN:
    IMTConCondition_EnRouteCondition = 9;
pub const IMTConCondition_EnRouteCondition_CONDITION_SOURCE_LOGIN:
    IMTConCondition_EnRouteCondition = 10;
pub const IMTConCondition_EnRouteCondition_CONDITION_MARKET_DEVIATION_SPR:
    IMTConCondition_EnRouteCondition = 11;
pub const IMTConCondition_EnRouteCondition_CONDITION_GAP: IMTConCondition_EnRouteCondition = 12;
pub const IMTConCondition_EnRouteCondition_CONDITION_PRICE: IMTConCondition_EnRouteCondition = 13;
pub const IMTConCondition_EnRouteCondition_CONDITION_LOGIN: IMTConCondition_EnRouteCondition = 1000;
pub const IMTConCondition_EnRouteCondition_CONDITION_GROUP: IMTConCondition_EnRouteCondition = 1001;
pub const IMTConCondition_EnRouteCondition_CONDITION_COUNTRY: IMTConCondition_EnRouteCondition =
    1002;
pub const IMTConCondition_EnRouteCondition_CONDITION_CITY: IMTConCondition_EnRouteCondition = 1003;
pub const IMTConCondition_EnRouteCondition_CONDITION_COLOR: IMTConCondition_EnRouteCondition = 1004;
pub const IMTConCondition_EnRouteCondition_CONDITION_LEVERAGE: IMTConCondition_EnRouteCondition =
    1005;
pub const IMTConCondition_EnRouteCondition_CONDITION_COMMENT_CLIENT:
    IMTConCondition_EnRouteCondition = 1006;
pub const IMTConCondition_EnRouteCondition_CONDITION_ZIPCODE: IMTConCondition_EnRouteCondition =
    1007;
pub const IMTConCondition_EnRouteCondition_CONDITION_MARGIN: IMTConCondition_EnRouteCondition =
    2000;
pub const IMTConCondition_EnRouteCondition_CONDITION_MARGIN_LEVEL:
    IMTConCondition_EnRouteCondition = 2001;
pub const IMTConCondition_EnRouteCondition_CONDITION_MARGIN_FREE: IMTConCondition_EnRouteCondition =
    2002;
pub const IMTConCondition_EnRouteCondition_CONDITION_EQUITY: IMTConCondition_EnRouteCondition =
    2003;
pub const IMTConCondition_EnRouteCondition_CONDITION_BALANCE: IMTConCondition_EnRouteCondition =
    2004;
pub const IMTConCondition_EnRouteCondition_CONDITION_PROFIT: IMTConCondition_EnRouteCondition =
    2005;
pub const IMTConCondition_EnRouteCondition_CONDITION_DAILY_DEALS: IMTConCondition_EnRouteCondition =
    3000;
pub const IMTConCondition_EnRouteCondition_CONDITION_DAILY_DEALS_PERIOD:
    IMTConCondition_EnRouteCondition = 3001;
pub const IMTConCondition_EnRouteCondition_CONDITION_DAILY_PROFIT:
    IMTConCondition_EnRouteCondition = 3002;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_VOLUME:
    IMTConCondition_EnRouteCondition = 4000;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_PROFIT:
    IMTConCondition_EnRouteCondition = 4001;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_AGE:
    IMTConCondition_EnRouteCondition = 4002;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_MODIFY_TIME:
    IMTConCondition_EnRouteCondition = 4003;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_AVERAGE_TIME:
    IMTConCondition_EnRouteCondition = 4004;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_TOTAL:
    IMTConCondition_EnRouteCondition = 4005;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_TOTAL_SYMBOL:
    IMTConCondition_EnRouteCondition = 4006;
pub const IMTConCondition_EnRouteCondition_CONDITION_ORDER_TOTAL: IMTConCondition_EnRouteCondition =
    4007;
pub const IMTConCondition_EnRouteCondition_CONDITION_ORDER_TOTAL_SYMBOL:
    IMTConCondition_EnRouteCondition = 4008;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_SL_TOUCHED:
    IMTConCondition_EnRouteCondition = 4009;
pub const IMTConCondition_EnRouteCondition_CONDITION_POSITION_TP_TOUCHED:
    IMTConCondition_EnRouteCondition = 4010;
pub const IMTConCondition_EnRouteCondition_CONDITION_ORDER_SL_TOUCHED:
    IMTConCondition_EnRouteCondition = 4011;
pub const IMTConCondition_EnRouteCondition_CONDITION_ORDER_TP_TOUCHED:
    IMTConCondition_EnRouteCondition = 4012;
pub const IMTConCondition_EnRouteCondition_CONDITION_ORDER_ENTRY_IN:
    IMTConCondition_EnRouteCondition = 4013;
pub const IMTConCondition_EnRouteCondition_CONDITION_ORDER_ENTRY_OUT:
    IMTConCondition_EnRouteCondition = 4014;
pub const IMTConCondition_EnRouteCondition_CONDITION_SYMBOL_SPREAD:
    IMTConCondition_EnRouteCondition = 5000;
pub const IMTConCondition_EnRouteCondition_CONDITION_FIRST: IMTConCondition_EnRouteCondition = 0;
pub const IMTConCondition_EnRouteCondition_CONDITION_LAST: IMTConCondition_EnRouteCondition = 5000;
pub type IMTConCondition_EnRouteCondition = ::std::os::raw::c_int;
pub const IMTConCondition_EnConditionRule_RULE_EQ: IMTConCondition_EnConditionRule = 0;
pub const IMTConCondition_EnConditionRule_RULE_NOT_EQ: IMTConCondition_EnConditionRule = 1;
pub const IMTConCondition_EnConditionRule_RULE_GREATER: IMTConCondition_EnConditionRule = 2;
pub const IMTConCondition_EnConditionRule_RULE_NOT_LESS: IMTConCondition_EnConditionRule = 3;
pub const IMTConCondition_EnConditionRule_RULE_LESS: IMTConCondition_EnConditionRule = 4;
pub const IMTConCondition_EnConditionRule_RULE_NOT_GREATER: IMTConCondition_EnConditionRule = 5;
pub const IMTConCondition_EnConditionRule_RULE_FIRST: IMTConCondition_EnConditionRule = 0;
pub const IMTConCondition_EnConditionRule_RULE_LAST: IMTConCondition_EnConditionRule = 5;
pub type IMTConCondition_EnConditionRule = ::std::os::raw::c_int;
pub const IMTConCondition_EnConditionType_TYPE_NONE: IMTConCondition_EnConditionType = 0;
pub const IMTConCondition_EnConditionType_TYPE_STRING: IMTConCondition_EnConditionType = 1;
pub const IMTConCondition_EnConditionType_TYPE_INT: IMTConCondition_EnConditionType = 2;
pub const IMTConCondition_EnConditionType_TYPE_UINT: IMTConCondition_EnConditionType = 3;
pub const IMTConCondition_EnConditionType_TYPE_DOUBLE: IMTConCondition_EnConditionType = 4;
pub const IMTConCondition_EnConditionType_TYPE_COLOR: IMTConCondition_EnConditionType = 5;
pub const IMTConCondition_EnConditionType_TYPE_MONEY: IMTConCondition_EnConditionType = 6;
pub const IMTConCondition_EnConditionType_TYPE_VOLUME: IMTConCondition_EnConditionType = 7;
pub const IMTConCondition_EnConditionType_TYPE_DATETIME: IMTConCondition_EnConditionType = 8;
pub const IMTConCondition_EnConditionType_TYPE_LEVERAGE: IMTConCondition_EnConditionType = 9;
pub const IMTConCondition_EnConditionType_TYPE_BOOL: IMTConCondition_EnConditionType = 10;
pub const IMTConCondition_EnConditionType_TYPE_TIME: IMTConCondition_EnConditionType = 11;
pub const IMTConCondition_EnConditionType_TYPE_WEEKDAY: IMTConCondition_EnConditionType = 12;
pub const IMTConCondition_EnConditionType_TYPE_FIRST: IMTConCondition_EnConditionType = 0;
pub const IMTConCondition_EnConditionType_TYPE_LAST: IMTConCondition_EnConditionType = 12;
pub type IMTConCondition_EnConditionType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConCondition"][::std::mem::size_of::<IMTConCondition>() - 8usize];
    ["Alignment of IMTConCondition"][::std::mem::align_of::<IMTConCondition>() - 8usize];
};
#[repr(C)]
pub struct IMTConRouteDealer__bindgen_vtable {
    pub IMTConRouteDealer_Release: unsafe extern "C" fn(this: *mut IMTConRouteDealer),
    pub IMTConRouteDealer_Assign: unsafe extern "C" fn(
        this: *mut IMTConRouteDealer,
        config: *const IMTConRouteDealer,
    ) -> MTAPIRES,
    pub IMTConRouteDealer_Clear: unsafe extern "C" fn(this: *mut IMTConRouteDealer) -> MTAPIRES,
    pub IMTConRouteDealer_Login1:
        unsafe extern "C" fn(this: *mut IMTConRouteDealer, login: UINT64) -> MTAPIRES,
    pub IMTConRouteDealer_Login: unsafe extern "C" fn(this: *const IMTConRouteDealer) -> UINT64,
    pub IMTConRouteDealer_Name: unsafe extern "C" fn(this: *const IMTConRouteDealer) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConRouteDealer {
    pub vtable_: *const IMTConRouteDealer__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConRouteDealer"][::std::mem::size_of::<IMTConRouteDealer>() - 8usize];
    ["Alignment of IMTConRouteDealer"][::std::mem::align_of::<IMTConRouteDealer>() - 8usize];
};
#[repr(C)]
pub struct IMTConRoute__bindgen_vtable {
    pub IMTConRoute_Release: unsafe extern "C" fn(this: *mut IMTConRoute),
    pub IMTConRoute_Assign:
        unsafe extern "C" fn(this: *mut IMTConRoute, config: *const IMTConRoute) -> MTAPIRES,
    pub IMTConRoute_Clear: unsafe extern "C" fn(this: *mut IMTConRoute) -> MTAPIRES,
    pub IMTConRoute_Name1: unsafe extern "C" fn(this: *mut IMTConRoute, name: LPCWSTR) -> MTAPIRES,
    pub IMTConRoute_Name: unsafe extern "C" fn(this: *const IMTConRoute) -> LPCWSTR,
    pub IMTConRoute_Mode1: unsafe extern "C" fn(this: *mut IMTConRoute, mode: UINT) -> MTAPIRES,
    pub IMTConRoute_Mode: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_Request1:
        unsafe extern "C" fn(this: *mut IMTConRoute, request: UINT) -> MTAPIRES,
    pub IMTConRoute_Request: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_Type1: unsafe extern "C" fn(this: *mut IMTConRoute, type_: UINT) -> MTAPIRES,
    pub IMTConRoute_Type: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_Action1: unsafe extern "C" fn(this: *mut IMTConRoute, action: UINT) -> MTAPIRES,
    pub IMTConRoute_Action: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_ParamType: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_ParamInt1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: INT64) -> MTAPIRES,
    pub IMTConRoute_ParamInt: unsafe extern "C" fn(this: *const IMTConRoute) -> INT64,
    pub IMTConRoute_ParamUInt1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: UINT64) -> MTAPIRES,
    pub IMTConRoute_ParamUInt: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT64,
    pub IMTConRoute_ParamDouble1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: f64) -> MTAPIRES,
    pub IMTConRoute_ParamDouble: unsafe extern "C" fn(this: *const IMTConRoute) -> f64,
    pub IMTConRoute_ParamString1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: LPCWSTR) -> MTAPIRES,
    pub IMTConRoute_ParamString: unsafe extern "C" fn(this: *const IMTConRoute) -> LPCWSTR,
    pub IMTConRoute_ParamColor1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: COLORREF) -> MTAPIRES,
    pub IMTConRoute_ParamColor: unsafe extern "C" fn(this: *const IMTConRoute) -> COLORREF,
    pub IMTConRoute_ParamMoney1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: f64) -> MTAPIRES,
    pub IMTConRoute_ParamMoney: unsafe extern "C" fn(this: *const IMTConRoute) -> f64,
    pub IMTConRoute_ParamVolume1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: UINT64) -> MTAPIRES,
    pub IMTConRoute_ParamVolume: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT64,
    pub IMTConRoute_ParamDatetime1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: INT64) -> MTAPIRES,
    pub IMTConRoute_ParamDatetime: unsafe extern "C" fn(this: *const IMTConRoute) -> INT64,
    pub IMTConRoute_ParamLeverage1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: INT64) -> MTAPIRES,
    pub IMTConRoute_ParamLeverage: unsafe extern "C" fn(this: *const IMTConRoute) -> INT64,
    pub IMTConRoute_ParamBool1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: bool) -> MTAPIRES,
    pub IMTConRoute_ParamBool: unsafe extern "C" fn(this: *const IMTConRoute) -> bool,
    pub IMTConRoute_ParamTime1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: UINT) -> MTAPIRES,
    pub IMTConRoute_ParamTime: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_ConditionAdd:
        unsafe extern "C" fn(this: *mut IMTConRoute, condition: *mut IMTConCondition) -> MTAPIRES,
    pub IMTConRoute_ConditionUpdate: unsafe extern "C" fn(
        this: *mut IMTConRoute,
        pos: UINT,
        condition: *const IMTConCondition,
    ) -> MTAPIRES,
    pub IMTConRoute_ConditionDelete:
        unsafe extern "C" fn(this: *mut IMTConRoute, pos: UINT) -> MTAPIRES,
    pub IMTConRoute_ConditionClear: unsafe extern "C" fn(this: *mut IMTConRoute) -> MTAPIRES,
    pub IMTConRoute_ConditionShift: unsafe extern "C" fn(
        this: *mut IMTConRoute,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConRoute_ConditionTotal: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_ConditionNext: unsafe extern "C" fn(
        this: *const IMTConRoute,
        pos: UINT,
        condition: *mut IMTConCondition,
    ) -> MTAPIRES,
    pub IMTConRoute_DealerAdd:
        unsafe extern "C" fn(this: *mut IMTConRoute, dealer: *mut IMTConRouteDealer) -> MTAPIRES,
    pub IMTConRoute_DealerUpdate: unsafe extern "C" fn(
        this: *mut IMTConRoute,
        pos: UINT,
        dealer: *const IMTConRouteDealer,
    ) -> MTAPIRES,
    pub IMTConRoute_DealerDelete:
        unsafe extern "C" fn(this: *mut IMTConRoute, pos: UINT) -> MTAPIRES,
    pub IMTConRoute_DealerClear: unsafe extern "C" fn(this: *mut IMTConRoute) -> MTAPIRES,
    pub IMTConRoute_DealerShift: unsafe extern "C" fn(
        this: *mut IMTConRoute,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConRoute_DealerTotal: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT,
    pub IMTConRoute_DealerNext: unsafe extern "C" fn(
        this: *const IMTConRoute,
        pos: UINT,
        dealer: *mut IMTConRouteDealer,
    ) -> MTAPIRES,
    pub IMTConRoute_DealerGet: unsafe extern "C" fn(
        this: *const IMTConRoute,
        login: UINT64,
        dealer: *mut IMTConRouteDealer,
    ) -> MTAPIRES,
    pub IMTConRoute_ParamVolumeExt1:
        unsafe extern "C" fn(this: *mut IMTConRoute, value: UINT64) -> MTAPIRES,
    pub IMTConRoute_ParamVolumeExt: unsafe extern "C" fn(this: *const IMTConRoute) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConRoute {
    pub vtable_: *const IMTConRoute__bindgen_vtable,
}
pub const IMTConRoute_EnRouteMode_MODE_DISABLED: IMTConRoute_EnRouteMode = 0;
pub const IMTConRoute_EnRouteMode_MODE_ENABLED: IMTConRoute_EnRouteMode = 1;
pub const IMTConRoute_EnRouteMode_MODE_FIRST: IMTConRoute_EnRouteMode = 0;
pub const IMTConRoute_EnRouteMode_MODE_LAST: IMTConRoute_EnRouteMode = 1;
pub type IMTConRoute_EnRouteMode = ::std::os::raw::c_int;
pub const IMTConRoute_EnRouteFlags_REQUEST_NONE: IMTConRoute_EnRouteFlags = 0;
pub const IMTConRoute_EnRouteFlags_REQUEST_PRICE: IMTConRoute_EnRouteFlags = 1;
pub const IMTConRoute_EnRouteFlags_REQUEST_REQUEST: IMTConRoute_EnRouteFlags = 2;
pub const IMTConRoute_EnRouteFlags_REQUEST_INSTANT: IMTConRoute_EnRouteFlags = 4;
pub const IMTConRoute_EnRouteFlags_REQUEST_MARKET: IMTConRoute_EnRouteFlags = 8;
pub const IMTConRoute_EnRouteFlags_REQUEST_EXCHANGE: IMTConRoute_EnRouteFlags = 16;
pub const IMTConRoute_EnRouteFlags_REQUEST_PENDING: IMTConRoute_EnRouteFlags = 32;
pub const IMTConRoute_EnRouteFlags_REQUEST_SLTP: IMTConRoute_EnRouteFlags = 64;
pub const IMTConRoute_EnRouteFlags_REQUEST_MODIFY: IMTConRoute_EnRouteFlags = 128;
pub const IMTConRoute_EnRouteFlags_REQUEST_REMOVE: IMTConRoute_EnRouteFlags = 256;
pub const IMTConRoute_EnRouteFlags_REQUEST_ACTIVATE: IMTConRoute_EnRouteFlags = 512;
pub const IMTConRoute_EnRouteFlags_REQUEST_STOPLIMIT: IMTConRoute_EnRouteFlags = 1024;
pub const IMTConRoute_EnRouteFlags_REQUEST_SL: IMTConRoute_EnRouteFlags = 2048;
pub const IMTConRoute_EnRouteFlags_REQUEST_TP: IMTConRoute_EnRouteFlags = 4096;
pub const IMTConRoute_EnRouteFlags_REQUEST_STOPOUT_ORDER: IMTConRoute_EnRouteFlags = 8192;
pub const IMTConRoute_EnRouteFlags_REQUEST_STOPOUT_POSITION: IMTConRoute_EnRouteFlags = 16384;
pub const IMTConRoute_EnRouteFlags_REQUEST_EXPIRATION: IMTConRoute_EnRouteFlags = 32768;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_POS_EXECUTE: IMTConRoute_EnRouteFlags = 65536;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_ORD_PENDING: IMTConRoute_EnRouteFlags = 131072;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_POS_MODIFY: IMTConRoute_EnRouteFlags = 262144;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_ORD_MODIFY: IMTConRoute_EnRouteFlags = 524288;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_ORD_REMOVE: IMTConRoute_EnRouteFlags = 1048576;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_ORD_ACTIVATE: IMTConRoute_EnRouteFlags = 2097152;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_ORD_SLIMIT: IMTConRoute_EnRouteFlags = 4194304;
pub const IMTConRoute_EnRouteFlags_REQUEST_DEALER_CLOSE_BY: IMTConRoute_EnRouteFlags = 8388608;
pub const IMTConRoute_EnRouteFlags_REQUEST_CLOSE_BY: IMTConRoute_EnRouteFlags = 16777216;
pub const IMTConRoute_EnRouteFlags_REQUEST_ALL: IMTConRoute_EnRouteFlags = 33554431;
pub const IMTConRoute_EnRouteFlags_REQUEST_FIRST: IMTConRoute_EnRouteFlags = 0;
pub const IMTConRoute_EnRouteFlags_REQUEST_LAST: IMTConRoute_EnRouteFlags = 33554431;
pub type IMTConRoute_EnRouteFlags = ::std::os::raw::c_int;
pub const IMTConRoute_EnTypeFlags_TYPE_NONE: IMTConRoute_EnTypeFlags = 0;
pub const IMTConRoute_EnTypeFlags_TYPE_BUY: IMTConRoute_EnTypeFlags = 1;
pub const IMTConRoute_EnTypeFlags_TYPE_SELL: IMTConRoute_EnTypeFlags = 2;
pub const IMTConRoute_EnTypeFlags_TYPE_BUY_LIMIT: IMTConRoute_EnTypeFlags = 4;
pub const IMTConRoute_EnTypeFlags_TYPE_SELL_LIMIT: IMTConRoute_EnTypeFlags = 8;
pub const IMTConRoute_EnTypeFlags_TYPE_BUY_STOP: IMTConRoute_EnTypeFlags = 16;
pub const IMTConRoute_EnTypeFlags_TYPE_SELL_STOP: IMTConRoute_EnTypeFlags = 32;
pub const IMTConRoute_EnTypeFlags_TYPE_BUY_STOP_LIMIT: IMTConRoute_EnTypeFlags = 64;
pub const IMTConRoute_EnTypeFlags_TYPE_SELL_STOP_LIMIT: IMTConRoute_EnTypeFlags = 128;
pub const IMTConRoute_EnTypeFlags_TYPE_ALL: IMTConRoute_EnTypeFlags = 255;
pub const IMTConRoute_EnTypeFlags_TYPE_FIRST: IMTConRoute_EnTypeFlags = 0;
pub const IMTConRoute_EnTypeFlags_TYPE_LAST: IMTConRoute_EnTypeFlags = 255;
pub type IMTConRoute_EnTypeFlags = ::std::os::raw::c_int;
pub const IMTConRoute_EnRouteAction_ACTION_DELAY_TIME: IMTConRoute_EnRouteAction = 0;
pub const IMTConRoute_EnRouteAction_ACTION_DELAY_TICK: IMTConRoute_EnRouteAction = 1;
pub const IMTConRoute_EnRouteAction_ACTION_CLEAR_TP: IMTConRoute_EnRouteAction = 2;
pub const IMTConRoute_EnRouteAction_ACTION_CLEAR_SL: IMTConRoute_EnRouteAction = 3;
pub const IMTConRoute_EnRouteAction_ACTION_CLEAR_SLTP: IMTConRoute_EnRouteAction = 4;
pub const IMTConRoute_EnRouteAction_ACTION_DEALER: IMTConRoute_EnRouteAction = 1001;
pub const IMTConRoute_EnRouteAction_ACTION_DEALER_ONLINE: IMTConRoute_EnRouteAction = 1002;
pub const IMTConRoute_EnRouteAction_ACTION_REJECT: IMTConRoute_EnRouteAction = 1003;
pub const IMTConRoute_EnRouteAction_ACTION_REQUOTE: IMTConRoute_EnRouteAction = 1004;
pub const IMTConRoute_EnRouteAction_ACTION_CONFIRM_CLIENT: IMTConRoute_EnRouteAction = 1005;
pub const IMTConRoute_EnRouteAction_ACTION_CONFIRM_MARKET: IMTConRoute_EnRouteAction = 1006;
pub const IMTConRoute_EnRouteAction_ACTION_CANCEL_ORDER: IMTConRoute_EnRouteAction = 1007;
pub const IMTConRoute_EnRouteAction_ACTION_FIRST: IMTConRoute_EnRouteAction = 0;
pub const IMTConRoute_EnRouteAction_ACTION_LAST: IMTConRoute_EnRouteAction = 1007;
pub type IMTConRoute_EnRouteAction = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConRoute"][::std::mem::size_of::<IMTConRoute>() - 8usize];
    ["Alignment of IMTConRoute"][::std::mem::align_of::<IMTConRoute>() - 8usize];
};
#[repr(C)]
pub struct IMTConRouteSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConRouteSink {
    pub vtable_: *const IMTConRouteSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConRouteSink"][::std::mem::size_of::<IMTConRouteSink>() - 8usize];
    ["Alignment of IMTConRouteSink"][::std::mem::align_of::<IMTConRouteSink>() - 8usize];
};
#[repr(C)]
pub struct IMTUser__bindgen_vtable {
    pub IMTUser_Release: unsafe extern "C" fn(this: *mut IMTUser),
    pub IMTUser_Assign: unsafe extern "C" fn(this: *mut IMTUser, user: *const IMTUser) -> MTAPIRES,
    pub IMTUser_Clear: unsafe extern "C" fn(this: *mut IMTUser) -> MTAPIRES,
    pub IMTUser_Login1: unsafe extern "C" fn(this: *mut IMTUser, login: UINT64) -> MTAPIRES,
    pub IMTUser_Login: unsafe extern "C" fn(this: *const IMTUser) -> UINT64,
    pub IMTUser_Group1: unsafe extern "C" fn(this: *mut IMTUser, group: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Group: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_CertSerialNumber: unsafe extern "C" fn(this: *const IMTUser) -> UINT64,
    pub IMTUser_Rights1: unsafe extern "C" fn(this: *mut IMTUser, rights: UINT64) -> MTAPIRES,
    pub IMTUser_Rights: unsafe extern "C" fn(this: *const IMTUser) -> UINT64,
    pub IMTUser_Registration: unsafe extern "C" fn(this: *const IMTUser) -> INT64,
    pub IMTUser_LastAccess: unsafe extern "C" fn(this: *const IMTUser) -> INT64,
    pub IMTUser_LastIP: unsafe extern "C" fn(this: *const IMTUser, ip: *mut MTAPISTR) -> LPCWSTR,
    pub IMTUser_Name1: unsafe extern "C" fn(this: *mut IMTUser, name: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Name: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Company1: unsafe extern "C" fn(this: *mut IMTUser, id: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Company: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Account1: unsafe extern "C" fn(this: *mut IMTUser, account: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Account: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Country1: unsafe extern "C" fn(this: *mut IMTUser, account: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Country: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Language1: unsafe extern "C" fn(this: *mut IMTUser, language: UINT) -> MTAPIRES,
    pub IMTUser_Language: unsafe extern "C" fn(this: *const IMTUser) -> UINT,
    pub IMTUser_City1: unsafe extern "C" fn(this: *mut IMTUser, city: LPCWSTR) -> MTAPIRES,
    pub IMTUser_City: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_State1: unsafe extern "C" fn(this: *mut IMTUser, state: LPCWSTR) -> MTAPIRES,
    pub IMTUser_State: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_ZIPCode1: unsafe extern "C" fn(this: *mut IMTUser, code: LPCWSTR) -> MTAPIRES,
    pub IMTUser_ZIPCode: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Address1: unsafe extern "C" fn(this: *mut IMTUser, code: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Address: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Phone1: unsafe extern "C" fn(this: *mut IMTUser, phone: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Phone: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_EMail1: unsafe extern "C" fn(this: *mut IMTUser, email: LPCWSTR) -> MTAPIRES,
    pub IMTUser_EMail: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_ID1: unsafe extern "C" fn(this: *mut IMTUser, email: LPCWSTR) -> MTAPIRES,
    pub IMTUser_ID: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Status1: unsafe extern "C" fn(this: *mut IMTUser, id: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Status: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Comment1: unsafe extern "C" fn(this: *mut IMTUser, comment: LPCWSTR) -> MTAPIRES,
    pub IMTUser_Comment: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Color1: unsafe extern "C" fn(this: *mut IMTUser, color: COLORREF) -> MTAPIRES,
    pub IMTUser_Color: unsafe extern "C" fn(this: *const IMTUser) -> COLORREF,
    pub IMTUser_PhonePassword1:
        unsafe extern "C" fn(this: *mut IMTUser, password: LPCWSTR) -> MTAPIRES,
    pub IMTUser_PhonePassword: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_Leverage1: unsafe extern "C" fn(this: *mut IMTUser, leverage: UINT) -> MTAPIRES,
    pub IMTUser_Leverage: unsafe extern "C" fn(this: *const IMTUser) -> UINT,
    pub IMTUser_Agent1: unsafe extern "C" fn(this: *mut IMTUser, agent: UINT64) -> MTAPIRES,
    pub IMTUser_Agent: unsafe extern "C" fn(this: *const IMTUser) -> UINT64,
    pub IMTUser_Balance: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_Credit: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_InterestRate: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_CommissionDaily: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_CommissionMonthly: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_CommissionAgentDaily: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_CommissionAgentMonthly: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_BalancePrevDay: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_BalancePrevMonth: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_EquityPrevDay: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_EquityPrevMonth: unsafe extern "C" fn(this: *const IMTUser) -> f64,
    pub IMTUser_ApiDataSet2:
        unsafe extern "C" fn(this: *mut IMTUser, app_id: USHORT, id: UCHAR, value: f64) -> MTAPIRES,
    pub IMTUser_ApiDataSet1: unsafe extern "C" fn(
        this: *mut IMTUser,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataSet: unsafe extern "C" fn(
        this: *mut IMTUser,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataGet2: unsafe extern "C" fn(
        this: *const IMTUser,
        app_id: USHORT,
        id: UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataGet1: unsafe extern "C" fn(
        this: *const IMTUser,
        app_id: USHORT,
        id: UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataGet: unsafe extern "C" fn(
        this: *const IMTUser,
        app_id: USHORT,
        id: UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataClear: unsafe extern "C" fn(this: *mut IMTUser, app_id: USHORT) -> MTAPIRES,
    pub IMTUser_ApiDataClearAll: unsafe extern "C" fn(this: *mut IMTUser) -> MTAPIRES,
    pub IMTUser_ExternalAccountAdd:
        unsafe extern "C" fn(this: *mut IMTUser, gateway_id: UINT64, account: LPCWSTR) -> MTAPIRES,
    pub IMTUser_ExternalAccountUpdate: unsafe extern "C" fn(
        this: *mut IMTUser,
        pos: UINT,
        gateway_id: UINT64,
        account: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTUser_ExternalAccountDelete:
        unsafe extern "C" fn(this: *mut IMTUser, pos: UINT) -> MTAPIRES,
    pub IMTUser_ExternalAccountClear: unsafe extern "C" fn(this: *mut IMTUser) -> MTAPIRES,
    pub IMTUser_ExternalAccountTotal: unsafe extern "C" fn(this: *const IMTUser) -> UINT,
    pub IMTUser_ExternalAccountNext: unsafe extern "C" fn(
        this: *const IMTUser,
        pos: UINT,
        gateway_id: *mut UINT64,
        account: *mut MTAPISTR,
    ) -> MTAPIRES,
    pub IMTUser_ExternalAccountGet: unsafe extern "C" fn(
        this: *const IMTUser,
        gateway_id: UINT64,
        account: *mut MTAPISTR,
    ) -> MTAPIRES,
    pub IMTUser_LastPassChange: unsafe extern "C" fn(this: *const IMTUser) -> INT64,
    pub IMTUser_MQID: unsafe extern "C" fn(this: *const IMTUser, mqid: *mut MTAPISTR) -> LPCWSTR,
    pub IMTUser_ApiDataUpdate2: unsafe extern "C" fn(
        this: *mut IMTUser,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataUpdate1: unsafe extern "C" fn(
        this: *mut IMTUser,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataUpdate: unsafe extern "C" fn(
        this: *mut IMTUser,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataNext2: unsafe extern "C" fn(
        this: *const IMTUser,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataNext1: unsafe extern "C" fn(
        this: *const IMTUser,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTUser_ApiDataNext: unsafe extern "C" fn(
        this: *const IMTUser,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTUser_PasswordHash: unsafe extern "C" fn(
        this: *const IMTUser,
        type_: UINT,
        password_hash: *mut MTAPISTR,
    ) -> MTAPIRES,
    pub IMTUser_LeadCampaign1:
        unsafe extern "C" fn(this: *mut IMTUser, lead_campaign: LPCWSTR) -> MTAPIRES,
    pub IMTUser_LeadCampaign: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_LeadSource1:
        unsafe extern "C" fn(this: *mut IMTUser, lead_source: LPCWSTR) -> MTAPIRES,
    pub IMTUser_LeadSource: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_ClientID1: unsafe extern "C" fn(this: *mut IMTUser, id: UINT64) -> MTAPIRES,
    pub IMTUser_ClientID: unsafe extern "C" fn(this: *const IMTUser) -> UINT64,
    pub IMTUser_FirstName1:
        unsafe extern "C" fn(this: *mut IMTUser, first_name: LPCWSTR) -> MTAPIRES,
    pub IMTUser_FirstName: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_LastName1: unsafe extern "C" fn(this: *mut IMTUser, last_name: LPCWSTR) -> MTAPIRES,
    pub IMTUser_LastName: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_MiddleName1:
        unsafe extern "C" fn(this: *mut IMTUser, middle_name: LPCWSTR) -> MTAPIRES,
    pub IMTUser_MiddleName: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_RegistrationSet:
        unsafe extern "C" fn(this: *mut IMTUser, datetime: INT64) -> MTAPIRES,
    pub IMTUser_OTPSecret1:
        unsafe extern "C" fn(this: *mut IMTUser, otp_secret: LPCWSTR) -> MTAPIRES,
    pub IMTUser_OTPSecret: unsafe extern "C" fn(this: *const IMTUser) -> LPCWSTR,
    pub IMTUser_LimitOrders1: unsafe extern "C" fn(this: *mut IMTUser, id: UINT) -> MTAPIRES,
    pub IMTUser_LimitOrders: unsafe extern "C" fn(this: *const IMTUser) -> UINT,
    pub IMTUser_LimitPositionsValue1:
        unsafe extern "C" fn(this: *mut IMTUser, value: f64) -> MTAPIRES,
    pub IMTUser_LimitPositionsValue: unsafe extern "C" fn(this: *const IMTUser) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTUser {
    pub vtable_: *const IMTUser__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::mt_user::MTUser,
}
pub const IMTUser_EnUsersRights_USER_RIGHT_NONE: IMTUser_EnUsersRights = 0;
pub const IMTUser_EnUsersRights_USER_RIGHT_ENABLED: IMTUser_EnUsersRights = 1;
pub const IMTUser_EnUsersRights_USER_RIGHT_PASSWORD: IMTUser_EnUsersRights = 2;
pub const IMTUser_EnUsersRights_USER_RIGHT_TRADE_DISABLED: IMTUser_EnUsersRights = 4;
pub const IMTUser_EnUsersRights_USER_RIGHT_INVESTOR: IMTUser_EnUsersRights = 8;
pub const IMTUser_EnUsersRights_USER_RIGHT_CONFIRMED: IMTUser_EnUsersRights = 16;
pub const IMTUser_EnUsersRights_USER_RIGHT_TRAILING: IMTUser_EnUsersRights = 32;
pub const IMTUser_EnUsersRights_USER_RIGHT_EXPERT: IMTUser_EnUsersRights = 64;
pub const IMTUser_EnUsersRights_USER_RIGHT_OBSOLETE: IMTUser_EnUsersRights = 128;
pub const IMTUser_EnUsersRights_USER_RIGHT_REPORTS: IMTUser_EnUsersRights = 256;
pub const IMTUser_EnUsersRights_USER_RIGHT_READONLY: IMTUser_EnUsersRights = 512;
pub const IMTUser_EnUsersRights_USER_RIGHT_RESET_PASS: IMTUser_EnUsersRights = 1024;
pub const IMTUser_EnUsersRights_USER_RIGHT_OTP_ENABLED: IMTUser_EnUsersRights = 2048;
pub const IMTUser_EnUsersRights_USER_RIGHT_SPONSORED_HOSTING: IMTUser_EnUsersRights = 8192;
pub const IMTUser_EnUsersRights_USER_RIGHT_API_ENABLED: IMTUser_EnUsersRights = 16384;
pub const IMTUser_EnUsersRights_USER_RIGHT_PUSH_NOTIFICATION: IMTUser_EnUsersRights = 32768;
pub const IMTUser_EnUsersRights_USER_RIGHT_TECHNICAL: IMTUser_EnUsersRights = 65536;
pub const IMTUser_EnUsersRights_USER_RIGHT_EXCLUDE_REPORTS: IMTUser_EnUsersRights = 131072;
pub const IMTUser_EnUsersRights_USER_RIGHT_DEFAULT: IMTUser_EnUsersRights = 355;
pub const IMTUser_EnUsersRights_USER_RIGHT_ALL: IMTUser_EnUsersRights = 257919;
pub type IMTUser_EnUsersRights = ::std::os::raw::c_int;
pub const IMTUser_EnUsersPasswords_USER_PASS_MAIN: IMTUser_EnUsersPasswords = 0;
pub const IMTUser_EnUsersPasswords_USER_PASS_INVESTOR: IMTUser_EnUsersPasswords = 1;
pub const IMTUser_EnUsersPasswords_USER_PASS_API: IMTUser_EnUsersPasswords = 2;
pub const IMTUser_EnUsersPasswords_USER_PASS_FIRST: IMTUser_EnUsersPasswords = 0;
pub const IMTUser_EnUsersPasswords_USER_PASS_LAST: IMTUser_EnUsersPasswords = 2;
pub type IMTUser_EnUsersPasswords = ::std::os::raw::c_int;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT: IMTUser_EnUsersConnectionTypes = 0;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_WINMOBILE:
    IMTUser_EnUsersConnectionTypes = 1;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_WINPHONE: IMTUser_EnUsersConnectionTypes =
    2;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_API_WEB: IMTUser_EnUsersConnectionTypes =
    3;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_IPHONE: IMTUser_EnUsersConnectionTypes =
    4;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_ANDROID: IMTUser_EnUsersConnectionTypes =
    5;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_BLACKBERRY:
    IMTUser_EnUsersConnectionTypes = 6;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_CLIENT_WEB: IMTUser_EnUsersConnectionTypes = 11;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_ADMIN: IMTUser_EnUsersConnectionTypes = 32;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_MANAGER: IMTUser_EnUsersConnectionTypes = 33;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_MANAGER_API: IMTUser_EnUsersConnectionTypes = 34;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_ADMIN_API: IMTUser_EnUsersConnectionTypes = 36;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_MANAGER_API_WEB: IMTUser_EnUsersConnectionTypes =
    37;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_FIRST: IMTUser_EnUsersConnectionTypes = 0;
pub const IMTUser_EnUsersConnectionTypes_USER_TYPE_LAST: IMTUser_EnUsersConnectionTypes = 37;
pub type IMTUser_EnUsersConnectionTypes = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTUser"][::std::mem::size_of::<IMTUser>() - 24usize];
    ["Alignment of IMTUser"][::std::mem::align_of::<IMTUser>() - 8usize];
};
#[repr(C)]
pub struct IMTUserArray__bindgen_vtable {
    pub IMTUserArray_Release: unsafe extern "C" fn(this: *mut IMTUserArray),
    pub IMTUserArray_Assign:
        unsafe extern "C" fn(this: *mut IMTUserArray, array: *const IMTUserArray) -> MTAPIRES,
    pub IMTUserArray_Clear: unsafe extern "C" fn(this: *mut IMTUserArray) -> MTAPIRES,
    pub IMTUserArray_Add1:
        unsafe extern "C" fn(this: *mut IMTUserArray, array: *mut IMTUserArray) -> MTAPIRES,
    pub IMTUserArray_Add:
        unsafe extern "C" fn(this: *mut IMTUserArray, user: *mut IMTUser) -> MTAPIRES,
    pub IMTUserArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTUserArray, array: *const IMTUserArray) -> MTAPIRES,
    pub IMTUserArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTUserArray, user: *const IMTUser) -> MTAPIRES,
    pub IMTUserArray_Delete: unsafe extern "C" fn(this: *mut IMTUserArray, pos: UINT) -> MTAPIRES,
    pub IMTUserArray_Detach:
        unsafe extern "C" fn(this: *mut IMTUserArray, pos: UINT) -> *mut IMTUser,
    pub IMTUserArray_Update:
        unsafe extern "C" fn(this: *mut IMTUserArray, pos: UINT, account: *mut IMTUser) -> MTAPIRES,
    pub IMTUserArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTUserArray,
        pos: UINT,
        account: *const IMTUser,
    ) -> MTAPIRES,
    pub IMTUserArray_Shift: unsafe extern "C" fn(
        this: *mut IMTUserArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTUserArray_Total: unsafe extern "C" fn(this: *const IMTUserArray) -> UINT,
    pub IMTUserArray_Next:
        unsafe extern "C" fn(this: *const IMTUserArray, index: UINT) -> *mut IMTUser,
    pub IMTUserArray_Sort:
        unsafe extern "C" fn(this: *mut IMTUserArray, sort_function: MTSortFunctionPtr) -> MTAPIRES,
    pub IMTUserArray_Search: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTUserArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTUserArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTUserArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTUserArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTUserArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTUserArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTUserArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTUserArray {
    pub vtable_: *const IMTUserArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTUserArray"][::std::mem::size_of::<IMTUserArray>() - 8usize];
    ["Alignment of IMTUserArray"][::std::mem::align_of::<IMTUserArray>() - 8usize];
};
#[repr(C)]
pub struct IMTOnline__bindgen_vtable {
    pub IMTOnline_Release: unsafe extern "C" fn(this: *mut IMTOnline),
    pub IMTOnline_Assign:
        unsafe extern "C" fn(this: *mut IMTOnline, online: *const IMTOnline) -> MTAPIRES,
    pub IMTOnline_Clear: unsafe extern "C" fn(this: *mut IMTOnline) -> MTAPIRES,
    pub IMTOnline_SessionID: unsafe extern "C" fn(this: *const IMTOnline) -> UINT64,
    pub IMTOnline_Login: unsafe extern "C" fn(this: *const IMTOnline) -> UINT64,
    pub IMTOnline_Group: unsafe extern "C" fn(this: *const IMTOnline) -> LPCWSTR,
    pub IMTOnline_Address:
        unsafe extern "C" fn(this: *const IMTOnline, ip: *mut MTAPISTR) -> LPCWSTR,
    pub IMTOnline_Type: unsafe extern "C" fn(this: *const IMTOnline) -> UINT,
    pub IMTOnline_Build: unsafe extern "C" fn(this: *const IMTOnline) -> UINT,
    pub IMTOnline_Time: unsafe extern "C" fn(this: *const IMTOnline) -> INT64,
    pub IMTOnline_ComputerID:
        unsafe extern "C" fn(this: *const IMTOnline, cid: *mut MTAPISTR) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTOnline {
    pub vtable_: *const IMTOnline__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTOnline"][::std::mem::size_of::<IMTOnline>() - 8usize];
    ["Alignment of IMTOnline"][::std::mem::align_of::<IMTOnline>() - 8usize];
};
#[repr(C)]
pub struct IMTAccount__bindgen_vtable {
    pub IMTAccount_Release: unsafe extern "C" fn(this: *mut IMTAccount),
    pub IMTAccount_Assign:
        unsafe extern "C" fn(this: *mut IMTAccount, user: *const IMTAccount) -> MTAPIRES,
    pub IMTAccount_Clear: unsafe extern "C" fn(this: *mut IMTAccount) -> MTAPIRES,
    pub IMTAccount_Login1: unsafe extern "C" fn(this: *mut IMTAccount, login: UINT64) -> MTAPIRES,
    pub IMTAccount_Login: unsafe extern "C" fn(this: *const IMTAccount) -> UINT64,
    pub IMTAccount_CurrencyDigits1:
        unsafe extern "C" fn(this: *mut IMTAccount, digits: UINT) -> MTAPIRES,
    pub IMTAccount_CurrencyDigits: unsafe extern "C" fn(this: *const IMTAccount) -> UINT,
    pub IMTAccount_Balance1: unsafe extern "C" fn(this: *mut IMTAccount, balance: f64) -> MTAPIRES,
    pub IMTAccount_Balance: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Credit1: unsafe extern "C" fn(this: *mut IMTAccount, credit: f64) -> MTAPIRES,
    pub IMTAccount_Credit: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Margin1: unsafe extern "C" fn(this: *mut IMTAccount, margin: f64) -> MTAPIRES,
    pub IMTAccount_Margin: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_MarginFree1:
        unsafe extern "C" fn(this: *mut IMTAccount, margin_free: f64) -> MTAPIRES,
    pub IMTAccount_MarginFree: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_MarginLevel1:
        unsafe extern "C" fn(this: *mut IMTAccount, margin_level: f64) -> MTAPIRES,
    pub IMTAccount_MarginLevel: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_MarginLeverage1:
        unsafe extern "C" fn(this: *mut IMTAccount, leverage: UINT) -> MTAPIRES,
    pub IMTAccount_MarginLeverage: unsafe extern "C" fn(this: *const IMTAccount) -> UINT,
    pub IMTAccount_Profit1: unsafe extern "C" fn(this: *mut IMTAccount, profit: f64) -> MTAPIRES,
    pub IMTAccount_Profit: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Storage1: unsafe extern "C" fn(this: *mut IMTAccount, storage: f64) -> MTAPIRES,
    pub IMTAccount_Storage: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_ObsoleteValue1:
        unsafe extern "C" fn(this: *mut IMTAccount, value: f64) -> MTAPIRES,
    pub IMTAccount_ObsoleteValue: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Floating1:
        unsafe extern "C" fn(this: *mut IMTAccount, floating: f64) -> MTAPIRES,
    pub IMTAccount_Floating: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Equity1: unsafe extern "C" fn(this: *mut IMTAccount, equity: f64) -> MTAPIRES,
    pub IMTAccount_Equity: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_SOActivation1:
        unsafe extern "C" fn(this: *mut IMTAccount, activation: UINT) -> MTAPIRES,
    pub IMTAccount_SOActivation: unsafe extern "C" fn(this: *const IMTAccount) -> UINT,
    pub IMTAccount_SOTime1:
        unsafe extern "C" fn(this: *mut IMTAccount, datetime: INT64) -> MTAPIRES,
    pub IMTAccount_SOTime: unsafe extern "C" fn(this: *const IMTAccount) -> INT64,
    pub IMTAccount_SOLevel1: unsafe extern "C" fn(this: *mut IMTAccount, level: f64) -> MTAPIRES,
    pub IMTAccount_SOLevel: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_SOEquity1: unsafe extern "C" fn(this: *mut IMTAccount, equity: f64) -> MTAPIRES,
    pub IMTAccount_SOEquity: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_SOMargin1: unsafe extern "C" fn(this: *mut IMTAccount, margin: f64) -> MTAPIRES,
    pub IMTAccount_SOMargin: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_BlockedCommission1:
        unsafe extern "C" fn(this: *mut IMTAccount, commission: f64) -> MTAPIRES,
    pub IMTAccount_BlockedCommission: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_BlockedProfit1:
        unsafe extern "C" fn(this: *mut IMTAccount, profit: f64) -> MTAPIRES,
    pub IMTAccount_BlockedProfit: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_MarginInitial1:
        unsafe extern "C" fn(this: *mut IMTAccount, margin: f64) -> MTAPIRES,
    pub IMTAccount_MarginInitial: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_MarginMaintenance1:
        unsafe extern "C" fn(this: *mut IMTAccount, margin: f64) -> MTAPIRES,
    pub IMTAccount_MarginMaintenance: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Assets1: unsafe extern "C" fn(this: *mut IMTAccount, assets: f64) -> MTAPIRES,
    pub IMTAccount_Assets: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
    pub IMTAccount_Liabilities1:
        unsafe extern "C" fn(this: *mut IMTAccount, liabilities: f64) -> MTAPIRES,
    pub IMTAccount_Liabilities: unsafe extern "C" fn(this: *const IMTAccount) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTAccount {
    pub vtable_: *const IMTAccount__bindgen_vtable,
}
pub const IMTAccount_EnSoActivation_ACTIVATION_NONE: IMTAccount_EnSoActivation = 0;
pub const IMTAccount_EnSoActivation_ACTIVATION_MARGIN_CALL: IMTAccount_EnSoActivation = 1;
pub const IMTAccount_EnSoActivation_ACTIVATION_STOP_OUT: IMTAccount_EnSoActivation = 2;
pub const IMTAccount_EnSoActivation_ACTIVATION_FIRST: IMTAccount_EnSoActivation = 0;
pub const IMTAccount_EnSoActivation_ACTIVATION_LAST: IMTAccount_EnSoActivation = 2;
pub type IMTAccount_EnSoActivation = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTAccount"][::std::mem::size_of::<IMTAccount>() - 8usize];
    ["Alignment of IMTAccount"][::std::mem::align_of::<IMTAccount>() - 8usize];
};
#[repr(C)]
#[derive(Debug)]
pub struct IMTUserSink__bindgen_vtable {
    pub IMTUserSink_OnUserAdd: unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser),
    pub IMTUserSink_OnUserUpdate: unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser),
    pub IMTUserSink_OnUserDelete: unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser),

    pub IMTUserSink_OnUserClean: unsafe extern "C" fn(this: *mut IMTUserSink, login: UINT64),
    pub IMTUserSink_OnUserLogin:
        unsafe extern "C" fn(this: *mut IMTUserSink, ip: LPCWSTR, user: *const IMTUser, type_: UINT),
    pub IMTUserSink_OnUserSync: unsafe extern "C" fn(this: *mut IMTUserSink),

    pub IMTUserSink_HookUserAdd:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *mut IMTUser) -> MTAPIRES,
    pub IMTUserSink_HookUserUpdate:
        unsafe extern "C" fn(this: *mut IMTUserSink, prev: *const IMTUser, user: *mut IMTUser),
    pub IMTUserSink_HookUserDelete:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser),
    pub IMTUserSink_HookUserLogin:
        unsafe extern "C" fn(this: *mut IMTUserSink, ip: LPCWSTR, user: *const IMTUser, type_: UINT),

    pub IMTUserSink_OnUserLogout:
        unsafe extern "C" fn(this: *mut IMTUserSink, ip: LPCWSTR, user: *const IMTUser, type_: UINT),
    pub IMTUserSink_OnUserArchive: unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser),
    pub IMTUserSink_OnUserRestore: unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser),

    pub IMTUserSink_HookUserArchive:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser) -> MTAPIRES,
    pub IMTUserSink_HookUserLoginExt:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser, online: *const IMTOnline) -> MTAPIRES,

    pub IMTUserSink_OnUserLoginExt:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser, online: *const IMTOnline),
    pub IMTUserSink_OnUserLogoutExt:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser, online: *const IMTOnline),
    pub IMTUserSink_OnUserAddExt:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser, master_password: LPCWSTR, investor_password: LPCWSTR),
    pub IMTUserSink_OnUserChangePassword:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser, password_type: UINT, password: LPCWSTR),

    pub IMTUserSink_HookUserAddExt:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *mut IMTUser, master_password: LPCWSTR, investor_password: LPCWSTR) -> MTAPIRES,
    pub IMTUserSink_HookUserChangePassword:
        unsafe extern "C" fn(this: *mut IMTUserSink, user: *const IMTUser, password_type: UINT, password: LPCWSTR) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTUserSink {
    pub vtable_: *const IMTUserSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTUserSink"][::std::mem::size_of::<IMTUserSink>() - 8usize];
    ["Alignment of IMTUserSink"][::std::mem::align_of::<IMTUserSink>() - 8usize];
};
#[repr(C)]
pub struct IMTPosition__bindgen_vtable {
    pub IMTPosition_Release: unsafe extern "C" fn(this: *mut IMTPosition),
    pub IMTPosition_Assign:
        unsafe extern "C" fn(this: *mut IMTPosition, position: *const IMTPosition) -> MTAPIRES,
    pub IMTPosition_Clear: unsafe extern "C" fn(this: *mut IMTPosition) -> MTAPIRES,
    pub IMTPosition_Print:
        unsafe extern "C" fn(this: *const IMTPosition, string: *mut MTAPISTR) -> LPCWSTR,
    pub IMTPosition_Login: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_Symbol1:
        unsafe extern "C" fn(this: *mut IMTPosition, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTPosition_Symbol: unsafe extern "C" fn(this: *const IMTPosition) -> LPCWSTR,
    pub IMTPosition_Action1: unsafe extern "C" fn(this: *mut IMTPosition, action: UINT) -> MTAPIRES,
    pub IMTPosition_Action: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_Digits1: unsafe extern "C" fn(this: *mut IMTPosition, digits: UINT) -> MTAPIRES,
    pub IMTPosition_Digits: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_DigitsCurrency1:
        unsafe extern "C" fn(this: *mut IMTPosition, digits: UINT) -> MTAPIRES,
    pub IMTPosition_DigitsCurrency: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_ContractSize1:
        unsafe extern "C" fn(this: *mut IMTPosition, contract_size: f64) -> MTAPIRES,
    pub IMTPosition_ContractSize: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_TimeCreate1:
        unsafe extern "C" fn(this: *mut IMTPosition, time: INT64) -> MTAPIRES,
    pub IMTPosition_TimeCreate: unsafe extern "C" fn(this: *const IMTPosition) -> INT64,
    pub IMTPosition_TimeUpdate1:
        unsafe extern "C" fn(this: *mut IMTPosition, time: INT64) -> MTAPIRES,
    pub IMTPosition_TimeUpdate: unsafe extern "C" fn(this: *const IMTPosition) -> INT64,
    pub IMTPosition_PriceOpen1:
        unsafe extern "C" fn(this: *mut IMTPosition, price: f64) -> MTAPIRES,
    pub IMTPosition_PriceOpen: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_PriceCurrent1:
        unsafe extern "C" fn(this: *mut IMTPosition, price: f64) -> MTAPIRES,
    pub IMTPosition_PriceCurrent: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_PriceSL1: unsafe extern "C" fn(this: *mut IMTPosition, price: f64) -> MTAPIRES,
    pub IMTPosition_PriceSL: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_PriceTP1: unsafe extern "C" fn(this: *mut IMTPosition, price: f64) -> MTAPIRES,
    pub IMTPosition_PriceTP: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_Volume1:
        unsafe extern "C" fn(this: *mut IMTPosition, volume: UINT64) -> MTAPIRES,
    pub IMTPosition_Volume: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_Profit1: unsafe extern "C" fn(this: *mut IMTPosition, profit: f64) -> MTAPIRES,
    pub IMTPosition_Profit: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_Storage1:
        unsafe extern "C" fn(this: *mut IMTPosition, storage: f64) -> MTAPIRES,
    pub IMTPosition_Storage: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_ObsoleteValue1:
        unsafe extern "C" fn(this: *mut IMTPosition, value: f64) -> MTAPIRES,
    pub IMTPosition_ObsoleteValue: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_RateProfit1:
        unsafe extern "C" fn(this: *mut IMTPosition, rate: f64) -> MTAPIRES,
    pub IMTPosition_RateProfit: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_RateMargin1:
        unsafe extern "C" fn(this: *mut IMTPosition, rate: f64) -> MTAPIRES,
    pub IMTPosition_RateMargin: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_ExpertID1: unsafe extern "C" fn(this: *mut IMTPosition, id: UINT64) -> MTAPIRES,
    pub IMTPosition_ExpertID: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_ExpertPositionID1:
        unsafe extern "C" fn(this: *mut IMTPosition, id: UINT64) -> MTAPIRES,
    pub IMTPosition_ExpertPositionID: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_Comment1:
        unsafe extern "C" fn(this: *mut IMTPosition, comment: LPCWSTR) -> MTAPIRES,
    pub IMTPosition_Comment: unsafe extern "C" fn(this: *const IMTPosition) -> LPCWSTR,
    pub IMTPosition_ActivationMode1:
        unsafe extern "C" fn(this: *mut IMTPosition, mode: UINT) -> MTAPIRES,
    pub IMTPosition_ActivationMode: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_ActivationTime1:
        unsafe extern "C" fn(this: *mut IMTPosition, atm: INT64) -> MTAPIRES,
    pub IMTPosition_ActivationTime: unsafe extern "C" fn(this: *const IMTPosition) -> INT64,
    pub IMTPosition_ActivationPrice1:
        unsafe extern "C" fn(this: *mut IMTPosition, price: f64) -> MTAPIRES,
    pub IMTPosition_ActivationPrice: unsafe extern "C" fn(this: *const IMTPosition) -> f64,
    pub IMTPosition_ActivationFlags1:
        unsafe extern "C" fn(this: *mut IMTPosition, flags: UINT) -> MTAPIRES,
    pub IMTPosition_ActivationFlags: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_ApiDataSet2: unsafe extern "C" fn(
        this: *mut IMTPosition,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataSet1: unsafe extern "C" fn(
        this: *mut IMTPosition,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataSet: unsafe extern "C" fn(
        this: *mut IMTPosition,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataGet2: unsafe extern "C" fn(
        this: *const IMTPosition,
        app_id: USHORT,
        id: UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataGet1: unsafe extern "C" fn(
        this: *const IMTPosition,
        app_id: USHORT,
        id: UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataGet: unsafe extern "C" fn(
        this: *const IMTPosition,
        app_id: USHORT,
        id: UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataClear:
        unsafe extern "C" fn(this: *mut IMTPosition, app_id: USHORT) -> MTAPIRES,
    pub IMTPosition_ApiDataClearAll: unsafe extern "C" fn(this: *mut IMTPosition) -> MTAPIRES,
    pub IMTPosition_TimeCreateMsc1:
        unsafe extern "C" fn(this: *mut IMTPosition, time: INT64) -> MTAPIRES,
    pub IMTPosition_TimeCreateMsc: unsafe extern "C" fn(this: *const IMTPosition) -> INT64,
    pub IMTPosition_TimeUpdateMsc1:
        unsafe extern "C" fn(this: *mut IMTPosition, time: INT64) -> MTAPIRES,
    pub IMTPosition_TimeUpdateMsc: unsafe extern "C" fn(this: *const IMTPosition) -> INT64,
    pub IMTPosition_Dealer1:
        unsafe extern "C" fn(this: *mut IMTPosition, dealer: UINT64) -> MTAPIRES,
    pub IMTPosition_Dealer: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_ApiDataUpdate2: unsafe extern "C" fn(
        this: *mut IMTPosition,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataUpdate1: unsafe extern "C" fn(
        this: *mut IMTPosition,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataUpdate: unsafe extern "C" fn(
        this: *mut IMTPosition,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataNext2: unsafe extern "C" fn(
        this: *const IMTPosition,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataNext1: unsafe extern "C" fn(
        this: *const IMTPosition,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTPosition_ApiDataNext: unsafe extern "C" fn(
        this: *const IMTPosition,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTPosition_LoginSet:
        unsafe extern "C" fn(this: *mut IMTPosition, login: UINT64) -> MTAPIRES,
    pub IMTPosition_Position: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_ExternalID1:
        unsafe extern "C" fn(this: *mut IMTPosition, id: LPCWSTR) -> MTAPIRES,
    pub IMTPosition_ExternalID: unsafe extern "C" fn(this: *const IMTPosition) -> LPCWSTR,
    pub IMTPosition_ModificationFlags: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_Reason: unsafe extern "C" fn(this: *const IMTPosition) -> UINT,
    pub IMTPosition_VolumeExt1:
        unsafe extern "C" fn(this: *mut IMTPosition, volume: UINT64) -> MTAPIRES,
    pub IMTPosition_VolumeExt: unsafe extern "C" fn(this: *const IMTPosition) -> UINT64,
    pub IMTPosition_ReasonSet:
        unsafe extern "C" fn(this: *mut IMTPosition, reason: UINT) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTPosition {
    pub vtable_: *const IMTPosition__bindgen_vtable,
}
pub const IMTPosition_EnPositionAction_POSITION_BUY: IMTPosition_EnPositionAction = 0;
pub const IMTPosition_EnPositionAction_POSITION_SELL: IMTPosition_EnPositionAction = 1;
pub const IMTPosition_EnPositionAction_POSITION_FIRST: IMTPosition_EnPositionAction = 0;
pub const IMTPosition_EnPositionAction_POSITION_LAST: IMTPosition_EnPositionAction = 1;
pub type IMTPosition_EnPositionAction = ::std::os::raw::c_int;
pub const IMTPosition_EnActivation_ACTIVATION_NONE: IMTPosition_EnActivation = 0;
pub const IMTPosition_EnActivation_ACTIVATION_SL: IMTPosition_EnActivation = 1;
pub const IMTPosition_EnActivation_ACTIVATION_TP: IMTPosition_EnActivation = 2;
pub const IMTPosition_EnActivation_ACTIVATION_STOPOUT: IMTPosition_EnActivation = 3;
pub const IMTPosition_EnActivation_ACTIVATION_FIRST: IMTPosition_EnActivation = 0;
pub const IMTPosition_EnActivation_ACTIVATION_LAST: IMTPosition_EnActivation = 3;
pub type IMTPosition_EnActivation = ::std::os::raw::c_int;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_LIMIT:
    IMTPosition_EnTradeActivationFlags = 1;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_STOP:
    IMTPosition_EnTradeActivationFlags = 2;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_SLIMIT:
    IMTPosition_EnTradeActivationFlags = 4;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_SL: IMTPosition_EnTradeActivationFlags =
    8;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_TP: IMTPosition_EnTradeActivationFlags =
    16;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_SO: IMTPosition_EnTradeActivationFlags =
    32;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NO_EXPIRATION:
    IMTPosition_EnTradeActivationFlags = 64;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_NONE: IMTPosition_EnTradeActivationFlags =
    0;
pub const IMTPosition_EnTradeActivationFlags_ACTIV_FLAGS_ALL: IMTPosition_EnTradeActivationFlags =
    127;
pub type IMTPosition_EnTradeActivationFlags = ::std::os::raw::c_int;
pub const IMTPosition_EnPositionReason_POSITION_REASON_CLIENT: IMTPosition_EnPositionReason = 0;
pub const IMTPosition_EnPositionReason_POSITION_REASON_EXPERT: IMTPosition_EnPositionReason = 1;
pub const IMTPosition_EnPositionReason_POSITION_REASON_DEALER: IMTPosition_EnPositionReason = 2;
pub const IMTPosition_EnPositionReason_POSITION_REASON_SL: IMTPosition_EnPositionReason = 3;
pub const IMTPosition_EnPositionReason_POSITION_REASON_TP: IMTPosition_EnPositionReason = 4;
pub const IMTPosition_EnPositionReason_POSITION_REASON_SO: IMTPosition_EnPositionReason = 5;
pub const IMTPosition_EnPositionReason_POSITION_REASON_ROLLOVER: IMTPosition_EnPositionReason = 6;
pub const IMTPosition_EnPositionReason_POSITION_REASON_EXTERNAL_CLIENT:
    IMTPosition_EnPositionReason = 7;
pub const IMTPosition_EnPositionReason_POSITION_REASON_VMARGIN: IMTPosition_EnPositionReason = 8;
pub const IMTPosition_EnPositionReason_POSITION_REASON_GATEWAY: IMTPosition_EnPositionReason = 9;
pub const IMTPosition_EnPositionReason_POSITION_REASON_SIGNAL: IMTPosition_EnPositionReason = 10;
pub const IMTPosition_EnPositionReason_POSITION_REASON_SETTLEMENT: IMTPosition_EnPositionReason =
    11;
pub const IMTPosition_EnPositionReason_POSITION_REASON_TRANSFER: IMTPosition_EnPositionReason = 12;
pub const IMTPosition_EnPositionReason_POSITION_REASON_SYNC: IMTPosition_EnPositionReason = 13;
pub const IMTPosition_EnPositionReason_POSITION_REASON_EXTERNAL_SERVICE:
    IMTPosition_EnPositionReason = 14;
pub const IMTPosition_EnPositionReason_POSITION_REASON_MIGRATION: IMTPosition_EnPositionReason = 15;
pub const IMTPosition_EnPositionReason_POSITION_REASON_MOBILE: IMTPosition_EnPositionReason = 16;
pub const IMTPosition_EnPositionReason_POSITION_REASON_WEB: IMTPosition_EnPositionReason = 17;
pub const IMTPosition_EnPositionReason_POSITION_REASON_SPLIT: IMTPosition_EnPositionReason = 18;
pub const IMTPosition_EnPositionReason_POSITION_REASON_FIRST: IMTPosition_EnPositionReason = 0;
pub const IMTPosition_EnPositionReason_POSITION_REASON_LAST: IMTPosition_EnPositionReason = 18;
pub type IMTPosition_EnPositionReason = ::std::os::raw::c_int;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_ADMIN: IMTPosition_EnTradeModifyFlags = 1;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_MANAGER: IMTPosition_EnTradeModifyFlags = 2;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_POSITION: IMTPosition_EnTradeModifyFlags = 4;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_RESTORE: IMTPosition_EnTradeModifyFlags = 8;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_API_ADMIN: IMTPosition_EnTradeModifyFlags =
    16;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_API_MANAGER: IMTPosition_EnTradeModifyFlags =
    32;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_API_SERVER: IMTPosition_EnTradeModifyFlags =
    64;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_API_GATEWAY: IMTPosition_EnTradeModifyFlags =
    128;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_NONE: IMTPosition_EnTradeModifyFlags = 0;
pub const IMTPosition_EnTradeModifyFlags_MODIFY_FLAGS_ALL: IMTPosition_EnTradeModifyFlags = 255;
pub type IMTPosition_EnTradeModifyFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTPosition"][::std::mem::size_of::<IMTPosition>() - 8usize];
    ["Alignment of IMTPosition"][::std::mem::align_of::<IMTPosition>() - 8usize];
};
#[repr(C)]
pub struct IMTPositionArray__bindgen_vtable {
    pub IMTPositionArray_Release: unsafe extern "C" fn(this: *mut IMTPositionArray),
    pub IMTPositionArray_Assign: unsafe extern "C" fn(
        this: *mut IMTPositionArray,
        array: *const IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTPositionArray_Clear: unsafe extern "C" fn(this: *mut IMTPositionArray) -> MTAPIRES,
    pub IMTPositionArray_Add1:
        unsafe extern "C" fn(this: *mut IMTPositionArray, array: *mut IMTPositionArray) -> MTAPIRES,
    pub IMTPositionArray_Add:
        unsafe extern "C" fn(this: *mut IMTPositionArray, position: *mut IMTPosition) -> MTAPIRES,
    pub IMTPositionArray_AddCopy1: unsafe extern "C" fn(
        this: *mut IMTPositionArray,
        array: *const IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTPositionArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTPositionArray, position: *const IMTPosition) -> MTAPIRES,
    pub IMTPositionArray_Delete:
        unsafe extern "C" fn(this: *mut IMTPositionArray, pos: UINT) -> MTAPIRES,
    pub IMTPositionArray_Detach:
        unsafe extern "C" fn(this: *mut IMTPositionArray, pos: UINT) -> *mut IMTPosition,
    pub IMTPositionArray_Update: unsafe extern "C" fn(
        this: *mut IMTPositionArray,
        pos: UINT,
        position: *mut IMTPosition,
    ) -> MTAPIRES,
    pub IMTPositionArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTPositionArray,
        pos: UINT,
        position: *const IMTPosition,
    ) -> MTAPIRES,
    pub IMTPositionArray_Shift: unsafe extern "C" fn(
        this: *mut IMTPositionArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTPositionArray_Total: unsafe extern "C" fn(this: *const IMTPositionArray) -> UINT,
    pub IMTPositionArray_Next:
        unsafe extern "C" fn(this: *const IMTPositionArray, index: UINT) -> *mut IMTPosition,
    pub IMTPositionArray_Sort: unsafe extern "C" fn(
        this: *mut IMTPositionArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTPositionArray_Search: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTPositionArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTPositionArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTPositionArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTPositionArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTPositionArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTPositionArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTPositionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTPositionArray {
    pub vtable_: *const IMTPositionArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTPositionArray"][::std::mem::size_of::<IMTPositionArray>() - 8usize];
    ["Alignment of IMTPositionArray"][::std::mem::align_of::<IMTPositionArray>() - 8usize];
};
#[repr(C)]
pub struct IMTPositionSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTPositionSink {
    pub vtable_: *const IMTPositionSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTPositionSink"][::std::mem::size_of::<IMTPositionSink>() - 8usize];
    ["Alignment of IMTPositionSink"][::std::mem::align_of::<IMTPositionSink>() - 8usize];
};
#[repr(C)]
pub struct IMTDeal__bindgen_vtable {
    pub IMTDeal_Release: unsafe extern "C" fn(this: *mut IMTDeal),
    pub IMTDeal_Assign: unsafe extern "C" fn(this: *mut IMTDeal, deal: *const IMTDeal) -> MTAPIRES,
    pub IMTDeal_Clear: unsafe extern "C" fn(this: *mut IMTDeal) -> MTAPIRES,
    pub IMTDeal_Print: unsafe extern "C" fn(this: *const IMTDeal, string: *mut MTAPISTR) -> LPCWSTR,
    pub IMTDeal_Deal: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_ExternalID1: unsafe extern "C" fn(this: *mut IMTDeal, id: LPCWSTR) -> MTAPIRES,
    pub IMTDeal_ExternalID: unsafe extern "C" fn(this: *const IMTDeal) -> LPCWSTR,
    pub IMTDeal_Login1: unsafe extern "C" fn(this: *mut IMTDeal, login: UINT64) -> MTAPIRES,
    pub IMTDeal_Login: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_Dealer1: unsafe extern "C" fn(this: *mut IMTDeal, dealer: UINT64) -> MTAPIRES,
    pub IMTDeal_Dealer: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_Order1: unsafe extern "C" fn(this: *mut IMTDeal, order: UINT64) -> MTAPIRES,
    pub IMTDeal_Order: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_Action1: unsafe extern "C" fn(this: *mut IMTDeal, action: UINT) -> MTAPIRES,
    pub IMTDeal_Action: unsafe extern "C" fn(this: *const IMTDeal) -> UINT,
    pub IMTDeal_Entry1: unsafe extern "C" fn(this: *mut IMTDeal, entry: UINT) -> MTAPIRES,
    pub IMTDeal_Entry: unsafe extern "C" fn(this: *const IMTDeal) -> UINT,
    pub IMTDeal_Digits1: unsafe extern "C" fn(this: *mut IMTDeal, digits: UINT) -> MTAPIRES,
    pub IMTDeal_Digits: unsafe extern "C" fn(this: *const IMTDeal) -> UINT,
    pub IMTDeal_DigitsCurrency1: unsafe extern "C" fn(this: *mut IMTDeal, digits: UINT) -> MTAPIRES,
    pub IMTDeal_DigitsCurrency: unsafe extern "C" fn(this: *const IMTDeal) -> UINT,
    pub IMTDeal_ContractSize1:
        unsafe extern "C" fn(this: *mut IMTDeal, contract_size: f64) -> MTAPIRES,
    pub IMTDeal_ContractSize: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_Time1: unsafe extern "C" fn(this: *mut IMTDeal, time: INT64) -> MTAPIRES,
    pub IMTDeal_Time: unsafe extern "C" fn(this: *const IMTDeal) -> INT64,
    pub IMTDeal_Symbol1: unsafe extern "C" fn(this: *mut IMTDeal, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTDeal_Symbol: unsafe extern "C" fn(this: *const IMTDeal) -> LPCWSTR,
    pub IMTDeal_Price1: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
    pub IMTDeal_Price: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_Volume1: unsafe extern "C" fn(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES,
    pub IMTDeal_Volume: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_Profit1: unsafe extern "C" fn(this: *mut IMTDeal, profit: f64) -> MTAPIRES,
    pub IMTDeal_Profit: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_Storage1: unsafe extern "C" fn(this: *mut IMTDeal, storage: f64) -> MTAPIRES,
    pub IMTDeal_Storage: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_Commission1: unsafe extern "C" fn(this: *mut IMTDeal, comm: f64) -> MTAPIRES,
    pub IMTDeal_Commission: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_ObsoleteValue1: unsafe extern "C" fn(this: *mut IMTDeal, agent: f64) -> MTAPIRES,
    pub IMTDeal_ObsoleteValue: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_RateProfit1: unsafe extern "C" fn(this: *mut IMTDeal, rate: f64) -> MTAPIRES,
    pub IMTDeal_RateProfit: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_RateMargin1: unsafe extern "C" fn(this: *mut IMTDeal, rate: f64) -> MTAPIRES,
    pub IMTDeal_RateMargin: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_ExpertID1: unsafe extern "C" fn(this: *mut IMTDeal, id: UINT64) -> MTAPIRES,
    pub IMTDeal_ExpertID: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_PositionID1: unsafe extern "C" fn(this: *mut IMTDeal, id: UINT64) -> MTAPIRES,
    pub IMTDeal_PositionID: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_Comment1: unsafe extern "C" fn(this: *mut IMTDeal, comment: LPCWSTR) -> MTAPIRES,
    pub IMTDeal_Comment: unsafe extern "C" fn(this: *const IMTDeal) -> LPCWSTR,
    pub IMTDeal_ApiDataSet2:
        unsafe extern "C" fn(this: *mut IMTDeal, app_id: USHORT, id: UCHAR, value: f64) -> MTAPIRES,
    pub IMTDeal_ApiDataSet1: unsafe extern "C" fn(
        this: *mut IMTDeal,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataSet: unsafe extern "C" fn(
        this: *mut IMTDeal,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataGet2: unsafe extern "C" fn(
        this: *const IMTDeal,
        app_id: USHORT,
        id: UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataGet1: unsafe extern "C" fn(
        this: *const IMTDeal,
        app_id: USHORT,
        id: UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataGet: unsafe extern "C" fn(
        this: *const IMTDeal,
        app_id: USHORT,
        id: UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataClear: unsafe extern "C" fn(this: *mut IMTDeal, app_id: USHORT) -> MTAPIRES,
    pub IMTDeal_ApiDataClearAll: unsafe extern "C" fn(this: *mut IMTDeal) -> MTAPIRES,
    pub IMTDeal_ProfitRaw1: unsafe extern "C" fn(this: *mut IMTDeal, profit: f64) -> MTAPIRES,
    pub IMTDeal_ProfitRaw: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_PricePosition1: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
    pub IMTDeal_PricePosition: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_VolumeClosed1: unsafe extern "C" fn(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES,
    pub IMTDeal_VolumeClosed: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_TickValue1: unsafe extern "C" fn(this: *mut IMTDeal, value: f64) -> MTAPIRES,
    pub IMTDeal_TickValue: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_TickSize1: unsafe extern "C" fn(this: *mut IMTDeal, size: f64) -> MTAPIRES,
    pub IMTDeal_TickSize: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_Flags1: unsafe extern "C" fn(this: *mut IMTDeal, flags: UINT64) -> MTAPIRES,
    pub IMTDeal_Flags: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_TimeMsc1: unsafe extern "C" fn(this: *mut IMTDeal, time: INT64) -> MTAPIRES,
    pub IMTDeal_TimeMsc: unsafe extern "C" fn(this: *const IMTDeal) -> INT64,
    pub IMTDeal_Reason: unsafe extern "C" fn(this: *const IMTDeal) -> UINT,
    pub IMTDeal_Gateway: unsafe extern "C" fn(this: *const IMTDeal) -> LPCWSTR,
    pub IMTDeal_PriceGateway: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_ApiDataUpdate2: unsafe extern "C" fn(
        this: *mut IMTDeal,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataUpdate1: unsafe extern "C" fn(
        this: *mut IMTDeal,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataUpdate: unsafe extern "C" fn(
        this: *mut IMTDeal,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataNext2: unsafe extern "C" fn(
        this: *const IMTDeal,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataNext1: unsafe extern "C" fn(
        this: *const IMTDeal,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTDeal_ApiDataNext: unsafe extern "C" fn(
        this: *const IMTDeal,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTDeal_DealSet: unsafe extern "C" fn(this: *mut IMTDeal, deal: UINT64) -> MTAPIRES,
    pub IMTDeal_ModificationFlags: unsafe extern "C" fn(this: *const IMTDeal) -> UINT,
    pub IMTDeal_ReasonSet: unsafe extern "C" fn(this: *mut IMTDeal, reason: UINT) -> MTAPIRES,
    pub IMTDeal_GatewaySet: unsafe extern "C" fn(this: *mut IMTDeal, gateway: LPCWSTR) -> MTAPIRES,
    pub IMTDeal_PriceGatewaySet:
        unsafe extern "C" fn(this: *mut IMTDeal, price_gateway: f64) -> MTAPIRES,
    pub IMTDeal_PriceSL1: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
    pub IMTDeal_PriceSL: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_PriceTP1: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
    pub IMTDeal_PriceTP: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_VolumeExt1: unsafe extern "C" fn(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES,
    pub IMTDeal_VolumeExt: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_VolumeClosedExt1:
        unsafe extern "C" fn(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES,
    pub IMTDeal_VolumeClosedExt: unsafe extern "C" fn(this: *const IMTDeal) -> UINT64,
    pub IMTDeal_Fee1: unsafe extern "C" fn(this: *mut IMTDeal, fee: f64) -> MTAPIRES,
    pub IMTDeal_Fee: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_Value1: unsafe extern "C" fn(this: *mut IMTDeal, value: f64) -> MTAPIRES,
    pub IMTDeal_Value: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_MarketBid: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_MarketAsk: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_MarketLast: unsafe extern "C" fn(this: *const IMTDeal) -> f64,
    pub IMTDeal_MarketBidSet: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
    pub IMTDeal_MarketAskSet: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
    pub IMTDeal_MarketLastSet: unsafe extern "C" fn(this: *mut IMTDeal, price: f64) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDeal {
    pub vtable_: *const IMTDeal__bindgen_vtable,
}
pub const IMTDeal_EnDealAction_DEAL_BUY: IMTDeal_EnDealAction = 0;
pub const IMTDeal_EnDealAction_DEAL_SELL: IMTDeal_EnDealAction = 1;
pub const IMTDeal_EnDealAction_DEAL_BALANCE: IMTDeal_EnDealAction = 2;
pub const IMTDeal_EnDealAction_DEAL_CREDIT: IMTDeal_EnDealAction = 3;
pub const IMTDeal_EnDealAction_DEAL_CHARGE: IMTDeal_EnDealAction = 4;
pub const IMTDeal_EnDealAction_DEAL_CORRECTION: IMTDeal_EnDealAction = 5;
pub const IMTDeal_EnDealAction_DEAL_BONUS: IMTDeal_EnDealAction = 6;
pub const IMTDeal_EnDealAction_DEAL_COMMISSION: IMTDeal_EnDealAction = 7;
pub const IMTDeal_EnDealAction_DEAL_COMMISSION_DAILY: IMTDeal_EnDealAction = 8;
pub const IMTDeal_EnDealAction_DEAL_COMMISSION_MONTHLY: IMTDeal_EnDealAction = 9;
pub const IMTDeal_EnDealAction_DEAL_AGENT_DAILY: IMTDeal_EnDealAction = 10;
pub const IMTDeal_EnDealAction_DEAL_AGENT_MONTHLY: IMTDeal_EnDealAction = 11;
pub const IMTDeal_EnDealAction_DEAL_INTERESTRATE: IMTDeal_EnDealAction = 12;
pub const IMTDeal_EnDealAction_DEAL_BUY_CANCELED: IMTDeal_EnDealAction = 13;
pub const IMTDeal_EnDealAction_DEAL_SELL_CANCELED: IMTDeal_EnDealAction = 14;
pub const IMTDeal_EnDealAction_DEAL_DIVIDEND: IMTDeal_EnDealAction = 15;
pub const IMTDeal_EnDealAction_DEAL_DIVIDEND_FRANKED: IMTDeal_EnDealAction = 16;
pub const IMTDeal_EnDealAction_DEAL_TAX: IMTDeal_EnDealAction = 17;
pub const IMTDeal_EnDealAction_DEAL_AGENT: IMTDeal_EnDealAction = 18;
pub const IMTDeal_EnDealAction_DEAL_SO_COMPENSATION: IMTDeal_EnDealAction = 19;
pub const IMTDeal_EnDealAction_DEAL_SO_COMPENSATION_CREDIT: IMTDeal_EnDealAction = 20;
pub const IMTDeal_EnDealAction_DEAL_FIRST: IMTDeal_EnDealAction = 0;
pub const IMTDeal_EnDealAction_DEAL_LAST: IMTDeal_EnDealAction = 20;
pub type IMTDeal_EnDealAction = ::std::os::raw::c_int;
pub const IMTDeal_EnDealEntry_ENTRY_IN: IMTDeal_EnDealEntry = 0;
pub const IMTDeal_EnDealEntry_ENTRY_OUT: IMTDeal_EnDealEntry = 1;
pub const IMTDeal_EnDealEntry_ENTRY_INOUT: IMTDeal_EnDealEntry = 2;
pub const IMTDeal_EnDealEntry_ENTRY_OUT_BY: IMTDeal_EnDealEntry = 3;
pub const IMTDeal_EnDealEntry_ENTRY_FIRST: IMTDeal_EnDealEntry = 0;
pub const IMTDeal_EnDealEntry_ENTRY_LAST: IMTDeal_EnDealEntry = 3;
pub type IMTDeal_EnDealEntry = ::std::os::raw::c_int;
pub const IMTDeal_EnDealReason_DEAL_REASON_CLIENT: IMTDeal_EnDealReason = 0;
pub const IMTDeal_EnDealReason_DEAL_REASON_EXPERT: IMTDeal_EnDealReason = 1;
pub const IMTDeal_EnDealReason_DEAL_REASON_DEALER: IMTDeal_EnDealReason = 2;
pub const IMTDeal_EnDealReason_DEAL_REASON_SL: IMTDeal_EnDealReason = 3;
pub const IMTDeal_EnDealReason_DEAL_REASON_TP: IMTDeal_EnDealReason = 4;
pub const IMTDeal_EnDealReason_DEAL_REASON_SO: IMTDeal_EnDealReason = 5;
pub const IMTDeal_EnDealReason_DEAL_REASON_ROLLOVER: IMTDeal_EnDealReason = 6;
pub const IMTDeal_EnDealReason_DEAL_REASON_EXTERNAL_CLIENT: IMTDeal_EnDealReason = 7;
pub const IMTDeal_EnDealReason_DEAL_REASON_VMARGIN: IMTDeal_EnDealReason = 8;
pub const IMTDeal_EnDealReason_DEAL_REASON_GATEWAY: IMTDeal_EnDealReason = 9;
pub const IMTDeal_EnDealReason_DEAL_REASON_SIGNAL: IMTDeal_EnDealReason = 10;
pub const IMTDeal_EnDealReason_DEAL_REASON_SETTLEMENT: IMTDeal_EnDealReason = 11;
pub const IMTDeal_EnDealReason_DEAL_REASON_TRANSFER: IMTDeal_EnDealReason = 12;
pub const IMTDeal_EnDealReason_DEAL_REASON_SYNC: IMTDeal_EnDealReason = 13;
pub const IMTDeal_EnDealReason_DEAL_REASON_EXTERNAL_SERVICE: IMTDeal_EnDealReason = 14;
pub const IMTDeal_EnDealReason_DEAL_REASON_MIGRATION: IMTDeal_EnDealReason = 15;
pub const IMTDeal_EnDealReason_DEAL_REASON_MOBILE: IMTDeal_EnDealReason = 16;
pub const IMTDeal_EnDealReason_DEAL_REASON_WEB: IMTDeal_EnDealReason = 17;
pub const IMTDeal_EnDealReason_DEAL_REASON_SPLIT: IMTDeal_EnDealReason = 18;
pub const IMTDeal_EnDealReason_DEAL_REASON_FIRST: IMTDeal_EnDealReason = 0;
pub const IMTDeal_EnDealReason_DEAL_REASON_LAST: IMTDeal_EnDealReason = 18;
pub type IMTDeal_EnDealReason = ::std::os::raw::c_int;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_ADMIN: IMTDeal_EnTradeModifyFlags = 1;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_MANAGER: IMTDeal_EnTradeModifyFlags = 2;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_POSITION: IMTDeal_EnTradeModifyFlags = 4;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_RESTORE: IMTDeal_EnTradeModifyFlags = 8;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_API_ADMIN: IMTDeal_EnTradeModifyFlags = 16;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_API_MANAGER: IMTDeal_EnTradeModifyFlags = 32;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_API_SERVER: IMTDeal_EnTradeModifyFlags = 64;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_API_GATEWAY: IMTDeal_EnTradeModifyFlags = 128;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_NONE: IMTDeal_EnTradeModifyFlags = 0;
pub const IMTDeal_EnTradeModifyFlags_MODIFY_FLAGS_ALL: IMTDeal_EnTradeModifyFlags = 255;
pub type IMTDeal_EnTradeModifyFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDeal"][::std::mem::size_of::<IMTDeal>() - 8usize];
    ["Alignment of IMTDeal"][::std::mem::align_of::<IMTDeal>() - 8usize];
};
#[repr(C)]
pub struct IMTDealArray__bindgen_vtable {
    pub IMTDealArray_Release: unsafe extern "C" fn(this: *mut IMTDealArray),
    pub IMTDealArray_Assign:
        unsafe extern "C" fn(this: *mut IMTDealArray, array: *const IMTDealArray) -> MTAPIRES,
    pub IMTDealArray_Clear: unsafe extern "C" fn(this: *mut IMTDealArray) -> MTAPIRES,
    pub IMTDealArray_Add1:
        unsafe extern "C" fn(this: *mut IMTDealArray, array: *mut IMTDealArray) -> MTAPIRES,
    pub IMTDealArray_Add:
        unsafe extern "C" fn(this: *mut IMTDealArray, deal: *mut IMTDeal) -> MTAPIRES,
    pub IMTDealArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTDealArray, array: *const IMTDealArray) -> MTAPIRES,
    pub IMTDealArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTDealArray, deal: *const IMTDeal) -> MTAPIRES,
    pub IMTDealArray_Delete: unsafe extern "C" fn(this: *mut IMTDealArray, pos: UINT) -> MTAPIRES,
    pub IMTDealArray_Detach:
        unsafe extern "C" fn(this: *mut IMTDealArray, pos: UINT) -> *mut IMTDeal,
    pub IMTDealArray_Update:
        unsafe extern "C" fn(this: *mut IMTDealArray, pos: UINT, deal: *mut IMTDeal) -> MTAPIRES,
    pub IMTDealArray_UpdateCopy:
        unsafe extern "C" fn(this: *mut IMTDealArray, pos: UINT, deal: *const IMTDeal) -> MTAPIRES,
    pub IMTDealArray_Shift: unsafe extern "C" fn(
        this: *mut IMTDealArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTDealArray_Total: unsafe extern "C" fn(this: *const IMTDealArray) -> UINT,
    pub IMTDealArray_Next:
        unsafe extern "C" fn(this: *const IMTDealArray, index: UINT) -> *mut IMTDeal,
    pub IMTDealArray_Sort:
        unsafe extern "C" fn(this: *mut IMTDealArray, sort_function: MTSortFunctionPtr) -> MTAPIRES,
    pub IMTDealArray_Search: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDealArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDealArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDealArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDealArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDealArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDealArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTDealArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDealArray {
    pub vtable_: *const IMTDealArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDealArray"][::std::mem::size_of::<IMTDealArray>() - 8usize];
    ["Alignment of IMTDealArray"][::std::mem::align_of::<IMTDealArray>() - 8usize];
};
#[repr(C)]
pub struct IMTDealSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTDealSink {
    pub vtable_: *const IMTDealSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDealSink"][::std::mem::size_of::<IMTDealSink>() - 8usize];
    ["Alignment of IMTDealSink"][::std::mem::align_of::<IMTDealSink>() - 8usize];
};
#[repr(C)]
pub struct IMTOrder__bindgen_vtable {
    pub IMTOrder_Release: unsafe extern "C" fn(this: *mut IMTOrder),
    pub IMTOrder_Assign:
        unsafe extern "C" fn(this: *mut IMTOrder, order: *const IMTOrder) -> MTAPIRES,
    pub IMTOrder_Clear: unsafe extern "C" fn(this: *mut IMTOrder) -> MTAPIRES,
    pub IMTOrder_Print:
        unsafe extern "C" fn(this: *const IMTOrder, string: *mut MTAPISTR) -> LPCWSTR,
    pub IMTOrder_Order: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_ExternalID1: unsafe extern "C" fn(this: *mut IMTOrder, id: LPCWSTR) -> MTAPIRES,
    pub IMTOrder_ExternalID: unsafe extern "C" fn(this: *const IMTOrder) -> LPCWSTR,
    pub IMTOrder_Login1: unsafe extern "C" fn(this: *mut IMTOrder, order: UINT64) -> MTAPIRES,
    pub IMTOrder_Login: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_Dealer1: unsafe extern "C" fn(this: *mut IMTOrder, dealer: UINT64) -> MTAPIRES,
    pub IMTOrder_Dealer: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_Symbol1: unsafe extern "C" fn(this: *mut IMTOrder, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTOrder_Symbol: unsafe extern "C" fn(this: *const IMTOrder) -> LPCWSTR,
    pub IMTOrder_Digits1: unsafe extern "C" fn(this: *mut IMTOrder, digits: UINT) -> MTAPIRES,
    pub IMTOrder_Digits: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_DigitsCurrency1:
        unsafe extern "C" fn(this: *mut IMTOrder, digits: UINT) -> MTAPIRES,
    pub IMTOrder_DigitsCurrency: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_ContractSize1:
        unsafe extern "C" fn(this: *mut IMTOrder, contract_size: f64) -> MTAPIRES,
    pub IMTOrder_ContractSize: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_State: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_Reason: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_TimeSetup1: unsafe extern "C" fn(this: *mut IMTOrder, time: INT64) -> MTAPIRES,
    pub IMTOrder_TimeSetup: unsafe extern "C" fn(this: *const IMTOrder) -> INT64,
    pub IMTOrder_TimeExpiration1:
        unsafe extern "C" fn(this: *mut IMTOrder, time: INT64) -> MTAPIRES,
    pub IMTOrder_TimeExpiration: unsafe extern "C" fn(this: *const IMTOrder) -> INT64,
    pub IMTOrder_TimeDone1: unsafe extern "C" fn(this: *mut IMTOrder, time: INT64) -> MTAPIRES,
    pub IMTOrder_TimeDone: unsafe extern "C" fn(this: *const IMTOrder) -> INT64,
    pub IMTOrder_Type1: unsafe extern "C" fn(this: *mut IMTOrder, type_: UINT) -> MTAPIRES,
    pub IMTOrder_Type: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_TypeFill1: unsafe extern "C" fn(this: *mut IMTOrder, type_: UINT) -> MTAPIRES,
    pub IMTOrder_TypeFill: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_TypeTime1: unsafe extern "C" fn(this: *mut IMTOrder, type_: UINT) -> MTAPIRES,
    pub IMTOrder_TypeTime: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_PriceOrder1: unsafe extern "C" fn(this: *mut IMTOrder, price: f64) -> MTAPIRES,
    pub IMTOrder_PriceOrder: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_PriceTrigger1: unsafe extern "C" fn(this: *mut IMTOrder, price: f64) -> MTAPIRES,
    pub IMTOrder_PriceTrigger: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_PriceCurrent1: unsafe extern "C" fn(this: *mut IMTOrder, price: f64) -> MTAPIRES,
    pub IMTOrder_PriceCurrent: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_PriceSL1: unsafe extern "C" fn(this: *mut IMTOrder, price: f64) -> MTAPIRES,
    pub IMTOrder_PriceSL: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_PriceTP1: unsafe extern "C" fn(this: *mut IMTOrder, price: f64) -> MTAPIRES,
    pub IMTOrder_PriceTP: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_VolumeInitial1:
        unsafe extern "C" fn(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES,
    pub IMTOrder_VolumeInitial: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_VolumeCurrent1:
        unsafe extern "C" fn(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES,
    pub IMTOrder_VolumeCurrent: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_ExpertID1: unsafe extern "C" fn(this: *mut IMTOrder, id: UINT64) -> MTAPIRES,
    pub IMTOrder_ExpertID: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_PositionID1: unsafe extern "C" fn(this: *mut IMTOrder, id: UINT64) -> MTAPIRES,
    pub IMTOrder_PositionID: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_Comment1: unsafe extern "C" fn(this: *mut IMTOrder, comment: LPCWSTR) -> MTAPIRES,
    pub IMTOrder_Comment: unsafe extern "C" fn(this: *const IMTOrder) -> LPCWSTR,
    pub IMTOrder_ActivationMode1: unsafe extern "C" fn(this: *mut IMTOrder, mode: UINT) -> MTAPIRES,
    pub IMTOrder_ActivationMode: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_ActivationTime1: unsafe extern "C" fn(this: *mut IMTOrder, atm: INT64) -> MTAPIRES,
    pub IMTOrder_ActivationTime: unsafe extern "C" fn(this: *const IMTOrder) -> INT64,
    pub IMTOrder_ActivationPrice1:
        unsafe extern "C" fn(this: *mut IMTOrder, price: f64) -> MTAPIRES,
    pub IMTOrder_ActivationPrice: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_ActivationFlags1:
        unsafe extern "C" fn(this: *mut IMTOrder, flags: UINT) -> MTAPIRES,
    pub IMTOrder_ActivationFlags: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_ApiDataSet2: unsafe extern "C" fn(
        this: *mut IMTOrder,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataSet1: unsafe extern "C" fn(
        this: *mut IMTOrder,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataSet: unsafe extern "C" fn(
        this: *mut IMTOrder,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataGet2: unsafe extern "C" fn(
        this: *const IMTOrder,
        app_id: USHORT,
        id: UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataGet1: unsafe extern "C" fn(
        this: *const IMTOrder,
        app_id: USHORT,
        id: UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataGet: unsafe extern "C" fn(
        this: *const IMTOrder,
        app_id: USHORT,
        id: UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataClear:
        unsafe extern "C" fn(this: *mut IMTOrder, app_id: USHORT) -> MTAPIRES,
    pub IMTOrder_ApiDataClearAll: unsafe extern "C" fn(this: *mut IMTOrder) -> MTAPIRES,
    pub IMTOrder_TimeSetupMsc1: unsafe extern "C" fn(this: *mut IMTOrder, time: INT64) -> MTAPIRES,
    pub IMTOrder_TimeSetupMsc: unsafe extern "C" fn(this: *const IMTOrder) -> INT64,
    pub IMTOrder_TimeDoneMsc1: unsafe extern "C" fn(this: *mut IMTOrder, time: INT64) -> MTAPIRES,
    pub IMTOrder_TimeDoneMsc: unsafe extern "C" fn(this: *const IMTOrder) -> INT64,
    pub IMTOrder_RateMargin1: unsafe extern "C" fn(this: *mut IMTOrder, rate: f64) -> MTAPIRES,
    pub IMTOrder_RateMargin: unsafe extern "C" fn(this: *const IMTOrder) -> f64,
    pub IMTOrder_ApiDataUpdate2: unsafe extern "C" fn(
        this: *mut IMTOrder,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataUpdate1: unsafe extern "C" fn(
        this: *mut IMTOrder,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataUpdate: unsafe extern "C" fn(
        this: *mut IMTOrder,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataNext2: unsafe extern "C" fn(
        this: *const IMTOrder,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataNext1: unsafe extern "C" fn(
        this: *const IMTOrder,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTOrder_ApiDataNext: unsafe extern "C" fn(
        this: *const IMTOrder,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTOrder_OrderSet: unsafe extern "C" fn(this: *mut IMTOrder, order: UINT64) -> MTAPIRES,
    pub IMTOrder_PositionByID1: unsafe extern "C" fn(this: *mut IMTOrder, id: UINT64) -> MTAPIRES,
    pub IMTOrder_PositionByID: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_ModificationFlags: unsafe extern "C" fn(this: *const IMTOrder) -> UINT,
    pub IMTOrder_StateSet: unsafe extern "C" fn(this: *mut IMTOrder, state: UINT) -> MTAPIRES,
    pub IMTOrder_ReasonSet: unsafe extern "C" fn(this: *mut IMTOrder, reason: UINT) -> MTAPIRES,
    pub IMTOrder_VolumeInitialExt1:
        unsafe extern "C" fn(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES,
    pub IMTOrder_VolumeInitialExt: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
    pub IMTOrder_VolumeCurrentExt1:
        unsafe extern "C" fn(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES,
    pub IMTOrder_VolumeCurrentExt: unsafe extern "C" fn(this: *const IMTOrder) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTOrder {
    pub vtable_: *const IMTOrder__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::mt_order::MTOrder,
}
pub const IMTOrder_EnOrderType_OP_BUY: IMTOrder_EnOrderType = 0;
pub const IMTOrder_EnOrderType_OP_SELL: IMTOrder_EnOrderType = 1;
pub const IMTOrder_EnOrderType_OP_BUY_LIMIT: IMTOrder_EnOrderType = 2;
pub const IMTOrder_EnOrderType_OP_SELL_LIMIT: IMTOrder_EnOrderType = 3;
pub const IMTOrder_EnOrderType_OP_BUY_STOP: IMTOrder_EnOrderType = 4;
pub const IMTOrder_EnOrderType_OP_SELL_STOP: IMTOrder_EnOrderType = 5;
pub const IMTOrder_EnOrderType_OP_BUY_STOP_LIMIT: IMTOrder_EnOrderType = 6;
pub const IMTOrder_EnOrderType_OP_SELL_STOP_LIMIT: IMTOrder_EnOrderType = 7;
pub const IMTOrder_EnOrderType_OP_CLOSE_BY: IMTOrder_EnOrderType = 8;
pub const IMTOrder_EnOrderType_OP_FIRST: IMTOrder_EnOrderType = 0;
pub const IMTOrder_EnOrderType_OP_LAST: IMTOrder_EnOrderType = 8;
pub type IMTOrder_EnOrderType = ::std::os::raw::c_int;
pub const IMTOrder_EnOrderFilling_ORDER_FILL_FOK: IMTOrder_EnOrderFilling = 0;
pub const IMTOrder_EnOrderFilling_ORDER_FILL_IOC: IMTOrder_EnOrderFilling = 1;
pub const IMTOrder_EnOrderFilling_ORDER_FILL_RETURN: IMTOrder_EnOrderFilling = 2;
pub const IMTOrder_EnOrderFilling_ORDER_FILL_FIRST: IMTOrder_EnOrderFilling = 0;
pub const IMTOrder_EnOrderFilling_ORDER_FILL_LAST: IMTOrder_EnOrderFilling = 2;
pub type IMTOrder_EnOrderFilling = ::std::os::raw::c_int;
pub const IMTOrder_EnOrderTime_ORDER_TIME_GTC: IMTOrder_EnOrderTime = 0;
pub const IMTOrder_EnOrderTime_ORDER_TIME_DAY: IMTOrder_EnOrderTime = 1;
pub const IMTOrder_EnOrderTime_ORDER_TIME_SPECIFIED: IMTOrder_EnOrderTime = 2;
pub const IMTOrder_EnOrderTime_ORDER_TIME_SPECIFIED_DAY: IMTOrder_EnOrderTime = 3;
pub const IMTOrder_EnOrderTime_ORDER_TIME_FIRST: IMTOrder_EnOrderTime = 0;
pub const IMTOrder_EnOrderTime_ORDER_TIME_LAST: IMTOrder_EnOrderTime = 3;
pub type IMTOrder_EnOrderTime = ::std::os::raw::c_int;
pub const IMTOrder_EnOrderState_ORDER_STATE_STARTED: IMTOrder_EnOrderState = 0;
pub const IMTOrder_EnOrderState_ORDER_STATE_PLACED: IMTOrder_EnOrderState = 1;
pub const IMTOrder_EnOrderState_ORDER_STATE_CANCELED: IMTOrder_EnOrderState = 2;
pub const IMTOrder_EnOrderState_ORDER_STATE_PARTIAL: IMTOrder_EnOrderState = 3;
pub const IMTOrder_EnOrderState_ORDER_STATE_FILLED: IMTOrder_EnOrderState = 4;
pub const IMTOrder_EnOrderState_ORDER_STATE_REJECTED: IMTOrder_EnOrderState = 5;
pub const IMTOrder_EnOrderState_ORDER_STATE_EXPIRED: IMTOrder_EnOrderState = 6;
pub const IMTOrder_EnOrderState_ORDER_STATE_REQUEST_ADD: IMTOrder_EnOrderState = 7;
pub const IMTOrder_EnOrderState_ORDER_STATE_REQUEST_MODIFY: IMTOrder_EnOrderState = 8;
pub const IMTOrder_EnOrderState_ORDER_STATE_REQUEST_CANCEL: IMTOrder_EnOrderState = 9;
pub const IMTOrder_EnOrderState_ORDER_STATE_FIRST: IMTOrder_EnOrderState = 0;
pub const IMTOrder_EnOrderState_ORDER_STATE_LAST: IMTOrder_EnOrderState = 9;
pub type IMTOrder_EnOrderState = ::std::os::raw::c_int;
pub const IMTOrder_EnOrderActivation_ACTIVATION_NONE: IMTOrder_EnOrderActivation = 0;
pub const IMTOrder_EnOrderActivation_ACTIVATION_PENDING: IMTOrder_EnOrderActivation = 1;
pub const IMTOrder_EnOrderActivation_ACTIVATION_STOPLIMIT: IMTOrder_EnOrderActivation = 2;
pub const IMTOrder_EnOrderActivation_ACTIVATION_EXPIRATION: IMTOrder_EnOrderActivation = 3;
pub const IMTOrder_EnOrderActivation_ACTIVATION_STOPOUT: IMTOrder_EnOrderActivation = 4;
pub const IMTOrder_EnOrderActivation_ACTIVATION_FIRST: IMTOrder_EnOrderActivation = 0;
pub const IMTOrder_EnOrderActivation_ACTIVATION_LAST: IMTOrder_EnOrderActivation = 4;
pub type IMTOrder_EnOrderActivation = ::std::os::raw::c_int;
pub const IMTOrder_EnOrderReason_ORDER_REASON_CLIENT: IMTOrder_EnOrderReason = 0;
pub const IMTOrder_EnOrderReason_ORDER_REASON_EXPERT: IMTOrder_EnOrderReason = 1;
pub const IMTOrder_EnOrderReason_ORDER_REASON_DEALER: IMTOrder_EnOrderReason = 2;
pub const IMTOrder_EnOrderReason_ORDER_REASON_SL: IMTOrder_EnOrderReason = 3;
pub const IMTOrder_EnOrderReason_ORDER_REASON_TP: IMTOrder_EnOrderReason = 4;
pub const IMTOrder_EnOrderReason_ORDER_REASON_SO: IMTOrder_EnOrderReason = 5;
pub const IMTOrder_EnOrderReason_ORDER_REASON_ROLLOVER: IMTOrder_EnOrderReason = 6;
pub const IMTOrder_EnOrderReason_ORDER_REASON_EXTERNAL_CLIENT: IMTOrder_EnOrderReason = 7;
pub const IMTOrder_EnOrderReason_ORDER_REASON_VMARGIN: IMTOrder_EnOrderReason = 8;
pub const IMTOrder_EnOrderReason_ORDER_REASON_GATEWAY: IMTOrder_EnOrderReason = 9;
pub const IMTOrder_EnOrderReason_ORDER_REASON_SIGNAL: IMTOrder_EnOrderReason = 10;
pub const IMTOrder_EnOrderReason_ORDER_REASON_SETTLEMENT: IMTOrder_EnOrderReason = 11;
pub const IMTOrder_EnOrderReason_ORDER_REASON_TRANSFER: IMTOrder_EnOrderReason = 12;
pub const IMTOrder_EnOrderReason_ORDER_REASON_SYNC: IMTOrder_EnOrderReason = 13;
pub const IMTOrder_EnOrderReason_ORDER_REASON_EXTERNAL_SERVICE: IMTOrder_EnOrderReason = 14;
pub const IMTOrder_EnOrderReason_ORDER_REASON_MIGRATION: IMTOrder_EnOrderReason = 15;
pub const IMTOrder_EnOrderReason_ORDER_REASON_MOBILE: IMTOrder_EnOrderReason = 16;
pub const IMTOrder_EnOrderReason_ORDER_REASON_WEB: IMTOrder_EnOrderReason = 17;
pub const IMTOrder_EnOrderReason_ORDER_REASON_SPLIT: IMTOrder_EnOrderReason = 18;
pub const IMTOrder_EnOrderReason_ORDER_REASON_FIRST: IMTOrder_EnOrderReason = 0;
pub const IMTOrder_EnOrderReason_ORDER_REASON_LAST: IMTOrder_EnOrderReason = 18;
pub type IMTOrder_EnOrderReason = ::std::os::raw::c_int;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_LIMIT: IMTOrder_EnTradeActivationFlags = 1;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_STOP: IMTOrder_EnTradeActivationFlags = 2;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_SLIMIT: IMTOrder_EnTradeActivationFlags =
    4;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_SL: IMTOrder_EnTradeActivationFlags = 8;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_TP: IMTOrder_EnTradeActivationFlags = 16;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_SO: IMTOrder_EnTradeActivationFlags = 32;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NO_EXPIRATION:
    IMTOrder_EnTradeActivationFlags = 64;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_NONE: IMTOrder_EnTradeActivationFlags = 0;
pub const IMTOrder_EnTradeActivationFlags_ACTIV_FLAGS_ALL: IMTOrder_EnTradeActivationFlags = 127;
pub type IMTOrder_EnTradeActivationFlags = ::std::os::raw::c_int;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_ADMIN: IMTOrder_EnTradeModifyFlags = 1;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_MANAGER: IMTOrder_EnTradeModifyFlags = 2;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_POSITION: IMTOrder_EnTradeModifyFlags = 4;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_RESTORE: IMTOrder_EnTradeModifyFlags = 8;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_API_ADMIN: IMTOrder_EnTradeModifyFlags = 16;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_API_MANAGER: IMTOrder_EnTradeModifyFlags = 32;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_API_SERVER: IMTOrder_EnTradeModifyFlags = 64;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_API_GATEWAY: IMTOrder_EnTradeModifyFlags = 128;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_NONE: IMTOrder_EnTradeModifyFlags = 0;
pub const IMTOrder_EnTradeModifyFlags_MODIFY_FLAGS_ALL: IMTOrder_EnTradeModifyFlags = 255;
pub type IMTOrder_EnTradeModifyFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTOrder"][::std::mem::size_of::<IMTOrder>() - 24usize];
    ["Alignment of IMTOrder"][::std::mem::align_of::<IMTOrder>() - 8usize];
};
#[repr(C)]
pub struct IMTOrderArray__bindgen_vtable {
    pub IMTOrderArray_Release: unsafe extern "C" fn(this: *mut IMTOrderArray),
    pub IMTOrderArray_Assign:
        unsafe extern "C" fn(this: *mut IMTOrderArray, array: *const IMTOrderArray) -> MTAPIRES,
    pub IMTOrderArray_Clear: unsafe extern "C" fn(this: *mut IMTOrderArray) -> MTAPIRES,
    pub IMTOrderArray_Add1:
        unsafe extern "C" fn(this: *mut IMTOrderArray, array: *mut IMTOrderArray) -> MTAPIRES,
    pub IMTOrderArray_Add:
        unsafe extern "C" fn(this: *mut IMTOrderArray, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTOrderArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTOrderArray, array: *const IMTOrderArray) -> MTAPIRES,
    pub IMTOrderArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTOrderArray, order: *const IMTOrder) -> MTAPIRES,
    pub IMTOrderArray_Delete: unsafe extern "C" fn(this: *mut IMTOrderArray, pos: UINT) -> MTAPIRES,
    pub IMTOrderArray_Detach:
        unsafe extern "C" fn(this: *mut IMTOrderArray, pos: UINT) -> *mut IMTOrder,
    pub IMTOrderArray_Update:
        unsafe extern "C" fn(this: *mut IMTOrderArray, pos: UINT, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTOrderArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTOrderArray,
        pos: UINT,
        order: *const IMTOrder,
    ) -> MTAPIRES,
    pub IMTOrderArray_Shift: unsafe extern "C" fn(
        this: *mut IMTOrderArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTOrderArray_Total: unsafe extern "C" fn(this: *const IMTOrderArray) -> UINT,
    pub IMTOrderArray_Next:
        unsafe extern "C" fn(this: *const IMTOrderArray, index: UINT) -> *mut IMTOrder,
    pub IMTOrderArray_Sort: unsafe extern "C" fn(
        this: *mut IMTOrderArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTOrderArray_Search: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOrderArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOrderArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOrderArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOrderArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOrderArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOrderArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTOrderArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTOrderArray {
    pub vtable_: *const IMTOrderArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTOrderArray"][::std::mem::size_of::<IMTOrderArray>() - 8usize];
    ["Alignment of IMTOrderArray"][::std::mem::align_of::<IMTOrderArray>() - 8usize];
};
#[repr(C)]
pub struct IMTOrderSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTOrderSink {
    pub vtable_: *const IMTOrderSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTOrderSink"][::std::mem::size_of::<IMTOrderSink>() - 8usize];
    ["Alignment of IMTOrderSink"][::std::mem::align_of::<IMTOrderSink>() - 8usize];
};
#[repr(C)]
pub struct IMTHistorySink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTHistorySink {
    pub vtable_: *const IMTHistorySink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTHistorySink"][::std::mem::size_of::<IMTHistorySink>() - 8usize];
    ["Alignment of IMTHistorySink"][::std::mem::align_of::<IMTHistorySink>() - 8usize];
};
#[repr(C)]
pub struct IMTDaily__bindgen_vtable {
    pub IMTDaily_Release: unsafe extern "C" fn(this: *mut IMTDaily),
    pub IMTDaily_Assign:
        unsafe extern "C" fn(this: *mut IMTDaily, exec: *const IMTDaily) -> MTAPIRES,
    pub IMTDaily_Clear: unsafe extern "C" fn(this: *mut IMTDaily) -> MTAPIRES,
    pub IMTDaily_Datetime1: unsafe extern "C" fn(this: *mut IMTDaily, datetime: INT64) -> MTAPIRES,
    pub IMTDaily_Datetime: unsafe extern "C" fn(this: *const IMTDaily) -> INT64,
    pub IMTDaily_DatetimePrev1:
        unsafe extern "C" fn(this: *mut IMTDaily, datetime: INT64) -> MTAPIRES,
    pub IMTDaily_DatetimePrev: unsafe extern "C" fn(this: *const IMTDaily) -> INT64,
    pub IMTDaily_Login1: unsafe extern "C" fn(this: *mut IMTDaily, login: UINT64) -> MTAPIRES,
    pub IMTDaily_Login: unsafe extern "C" fn(this: *const IMTDaily) -> UINT64,
    pub IMTDaily_Name1: unsafe extern "C" fn(this: *mut IMTDaily, name: LPCWSTR) -> MTAPIRES,
    pub IMTDaily_Name: unsafe extern "C" fn(this: *const IMTDaily) -> LPCWSTR,
    pub IMTDaily_Group1: unsafe extern "C" fn(this: *mut IMTDaily, group: LPCWSTR) -> MTAPIRES,
    pub IMTDaily_Group: unsafe extern "C" fn(this: *const IMTDaily) -> LPCWSTR,
    pub IMTDaily_Currency1: unsafe extern "C" fn(this: *mut IMTDaily, curr: LPCWSTR) -> MTAPIRES,
    pub IMTDaily_Currency: unsafe extern "C" fn(this: *const IMTDaily) -> LPCWSTR,
    pub IMTDaily_CurrencyDigits: unsafe extern "C" fn(this: *const IMTDaily) -> UINT,
    pub IMTDaily_Company1: unsafe extern "C" fn(this: *mut IMTDaily, company: LPCWSTR) -> MTAPIRES,
    pub IMTDaily_Company: unsafe extern "C" fn(this: *const IMTDaily) -> LPCWSTR,
    pub IMTDaily_EMail1: unsafe extern "C" fn(this: *mut IMTDaily, mail: LPCWSTR) -> MTAPIRES,
    pub IMTDaily_EMail: unsafe extern "C" fn(this: *const IMTDaily) -> LPCWSTR,
    pub IMTDaily_Balance1: unsafe extern "C" fn(this: *mut IMTDaily, balance: f64) -> MTAPIRES,
    pub IMTDaily_Balance: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_Credit1: unsafe extern "C" fn(this: *mut IMTDaily, credit: f64) -> MTAPIRES,
    pub IMTDaily_Credit: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_InterestRate1: unsafe extern "C" fn(this: *mut IMTDaily, credit: f64) -> MTAPIRES,
    pub IMTDaily_InterestRate: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_CommissionDaily1: unsafe extern "C" fn(this: *mut IMTDaily, comm: f64) -> MTAPIRES,
    pub IMTDaily_CommissionDaily: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_CommissionMonthly1:
        unsafe extern "C" fn(this: *mut IMTDaily, comm: f64) -> MTAPIRES,
    pub IMTDaily_CommissionMonthly: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_AgentDaily1: unsafe extern "C" fn(this: *mut IMTDaily, agent: f64) -> MTAPIRES,
    pub IMTDaily_AgentDaily: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_AgentMonthly1: unsafe extern "C" fn(this: *mut IMTDaily, agent: f64) -> MTAPIRES,
    pub IMTDaily_AgentMonthly: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_BalancePrevDay1:
        unsafe extern "C" fn(this: *mut IMTDaily, balance: f64) -> MTAPIRES,
    pub IMTDaily_BalancePrevDay: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_BalancePrevMonth1:
        unsafe extern "C" fn(this: *mut IMTDaily, balance: f64) -> MTAPIRES,
    pub IMTDaily_BalancePrevMonth: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_EquityPrevDay1:
        unsafe extern "C" fn(this: *mut IMTDaily, balance: f64) -> MTAPIRES,
    pub IMTDaily_EquityPrevDay: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_EquityPrevMonth1:
        unsafe extern "C" fn(this: *mut IMTDaily, balance: f64) -> MTAPIRES,
    pub IMTDaily_EquityPrevMonth: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_Margin1: unsafe extern "C" fn(this: *mut IMTDaily, margin: f64) -> MTAPIRES,
    pub IMTDaily_Margin: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_MarginFree1:
        unsafe extern "C" fn(this: *mut IMTDaily, margin_free: f64) -> MTAPIRES,
    pub IMTDaily_MarginFree: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_MarginLevel1:
        unsafe extern "C" fn(this: *mut IMTDaily, margin_level: f64) -> MTAPIRES,
    pub IMTDaily_MarginLevel: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_MarginLeverage1:
        unsafe extern "C" fn(this: *mut IMTDaily, leverage: UINT) -> MTAPIRES,
    pub IMTDaily_MarginLeverage: unsafe extern "C" fn(this: *const IMTDaily) -> UINT,
    pub IMTDaily_Profit1: unsafe extern "C" fn(this: *mut IMTDaily, profit: f64) -> MTAPIRES,
    pub IMTDaily_Profit: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_ProfitStorage1:
        unsafe extern "C" fn(this: *mut IMTDaily, storage: f64) -> MTAPIRES,
    pub IMTDaily_ProfitStorage: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_ObsoleteValue1: unsafe extern "C" fn(this: *mut IMTDaily, value: f64) -> MTAPIRES,
    pub IMTDaily_ObsoleteValue: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_ProfitEquity1: unsafe extern "C" fn(this: *mut IMTDaily, equity: f64) -> MTAPIRES,
    pub IMTDaily_ProfitEquity: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyProfit1: unsafe extern "C" fn(this: *mut IMTDaily, profit: f64) -> MTAPIRES,
    pub IMTDaily_DailyProfit: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyBalance1: unsafe extern "C" fn(this: *mut IMTDaily, balance: f64) -> MTAPIRES,
    pub IMTDaily_DailyBalance: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyCredit1: unsafe extern "C" fn(this: *mut IMTDaily, comm: f64) -> MTAPIRES,
    pub IMTDaily_DailyCredit: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyCharge1: unsafe extern "C" fn(this: *mut IMTDaily, charge: f64) -> MTAPIRES,
    pub IMTDaily_DailyCharge: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyCorrection1:
        unsafe extern "C" fn(this: *mut IMTDaily, correction: f64) -> MTAPIRES,
    pub IMTDaily_DailyCorrection: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyBonus1: unsafe extern "C" fn(this: *mut IMTDaily, bonus: f64) -> MTAPIRES,
    pub IMTDaily_DailyBonus: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyStorage1: unsafe extern "C" fn(this: *mut IMTDaily, storage: f64) -> MTAPIRES,
    pub IMTDaily_DailyStorage: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyCommInstant1:
        unsafe extern "C" fn(this: *mut IMTDaily, comm: f64) -> MTAPIRES,
    pub IMTDaily_DailyCommInstant: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyCommRound1: unsafe extern "C" fn(this: *mut IMTDaily, comm: f64) -> MTAPIRES,
    pub IMTDaily_DailyCommRound: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyAgent1: unsafe extern "C" fn(this: *mut IMTDaily, comm: f64) -> MTAPIRES,
    pub IMTDaily_DailyAgent: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyInterest1:
        unsafe extern "C" fn(this: *mut IMTDaily, interest: f64) -> MTAPIRES,
    pub IMTDaily_DailyInterest: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_PositionAdd:
        unsafe extern "C" fn(this: *mut IMTDaily, position: *mut IMTPosition) -> MTAPIRES,
    pub IMTDaily_PositionUpdate: unsafe extern "C" fn(
        this: *mut IMTDaily,
        pos: UINT,
        position: *const IMTPosition,
    ) -> MTAPIRES,
    pub IMTDaily_PositionDelete: unsafe extern "C" fn(this: *mut IMTDaily, pos: UINT) -> MTAPIRES,
    pub IMTDaily_PositionClear: unsafe extern "C" fn(this: *mut IMTDaily) -> MTAPIRES,
    pub IMTDaily_PositionShift: unsafe extern "C" fn(
        this: *mut IMTDaily,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTDaily_PositionTotal: unsafe extern "C" fn(this: *const IMTDaily) -> UINT,
    pub IMTDaily_PositionNext: unsafe extern "C" fn(
        this: *const IMTDaily,
        pos: UINT,
        position: *mut IMTPosition,
    ) -> MTAPIRES,
    pub IMTDaily_PositionGet: unsafe extern "C" fn(
        this: *const IMTDaily,
        symbol: LPCWSTR,
        position: *mut IMTPosition,
    ) -> MTAPIRES,
    pub IMTDaily_OrderAdd:
        unsafe extern "C" fn(this: *mut IMTDaily, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTDaily_OrderUpdate:
        unsafe extern "C" fn(this: *mut IMTDaily, pos: UINT, order: *const IMTOrder) -> MTAPIRES,
    pub IMTDaily_OrderDelete: unsafe extern "C" fn(this: *mut IMTDaily, pos: UINT) -> MTAPIRES,
    pub IMTDaily_OrderClear: unsafe extern "C" fn(this: *mut IMTDaily) -> MTAPIRES,
    pub IMTDaily_OrderShift: unsafe extern "C" fn(
        this: *mut IMTDaily,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTDaily_OrderTotal: unsafe extern "C" fn(this: *const IMTDaily) -> UINT,
    pub IMTDaily_OrderNext:
        unsafe extern "C" fn(this: *const IMTDaily, pos: UINT, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTDaily_OrderGet: unsafe extern "C" fn(
        this: *const IMTDaily,
        ticket: UINT64,
        order: *mut IMTOrder,
    ) -> MTAPIRES,
    pub IMTDaily_ProfitAssets1: unsafe extern "C" fn(this: *mut IMTDaily, assets: f64) -> MTAPIRES,
    pub IMTDaily_ProfitAssets: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_ProfitLiabilities1:
        unsafe extern "C" fn(this: *mut IMTDaily, liabilities: f64) -> MTAPIRES,
    pub IMTDaily_ProfitLiabilities: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyDividend1:
        unsafe extern "C" fn(this: *mut IMTDaily, dividend: f64) -> MTAPIRES,
    pub IMTDaily_DailyDividend: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyTaxes1: unsafe extern "C" fn(this: *mut IMTDaily, taxes: f64) -> MTAPIRES,
    pub IMTDaily_DailyTaxes: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailySOCompensation1:
        unsafe extern "C" fn(this: *mut IMTDaily, compensation: f64) -> MTAPIRES,
    pub IMTDaily_DailySOCompensation: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailyCommFee1: unsafe extern "C" fn(this: *mut IMTDaily, fee: f64) -> MTAPIRES,
    pub IMTDaily_DailyCommFee: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
    pub IMTDaily_DailySOCompensationCredit1:
        unsafe extern "C" fn(this: *mut IMTDaily, compensation: f64) -> MTAPIRES,
    pub IMTDaily_DailySOCompensationCredit: unsafe extern "C" fn(this: *const IMTDaily) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDaily {
    pub vtable_: *const IMTDaily__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDaily"][::std::mem::size_of::<IMTDaily>() - 8usize];
    ["Alignment of IMTDaily"][::std::mem::align_of::<IMTDaily>() - 8usize];
};
#[repr(C)]
pub struct IMTDailyArray__bindgen_vtable {
    pub IMTDailyArray_Release: unsafe extern "C" fn(this: *mut IMTDailyArray),
    pub IMTDailyArray_Assign:
        unsafe extern "C" fn(this: *mut IMTDailyArray, array: *const IMTDailyArray) -> MTAPIRES,
    pub IMTDailyArray_Clear: unsafe extern "C" fn(this: *mut IMTDailyArray) -> MTAPIRES,
    pub IMTDailyArray_Add1:
        unsafe extern "C" fn(this: *mut IMTDailyArray, array: *mut IMTDailyArray) -> MTAPIRES,
    pub IMTDailyArray_Add:
        unsafe extern "C" fn(this: *mut IMTDailyArray, daily: *mut IMTDaily) -> MTAPIRES,
    pub IMTDailyArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTDailyArray, array: *const IMTDailyArray) -> MTAPIRES,
    pub IMTDailyArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTDailyArray, daily: *const IMTDaily) -> MTAPIRES,
    pub IMTDailyArray_Delete: unsafe extern "C" fn(this: *mut IMTDailyArray, pos: UINT) -> MTAPIRES,
    pub IMTDailyArray_Detach:
        unsafe extern "C" fn(this: *mut IMTDailyArray, pos: UINT) -> *mut IMTDaily,
    pub IMTDailyArray_Update:
        unsafe extern "C" fn(this: *mut IMTDailyArray, pos: UINT, daily: *mut IMTDaily) -> MTAPIRES,
    pub IMTDailyArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTDailyArray,
        pos: UINT,
        daily: *const IMTDaily,
    ) -> MTAPIRES,
    pub IMTDailyArray_Shift: unsafe extern "C" fn(
        this: *mut IMTDailyArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTDailyArray_Total: unsafe extern "C" fn(this: *const IMTDailyArray) -> UINT,
    pub IMTDailyArray_Next:
        unsafe extern "C" fn(this: *const IMTDailyArray, index: UINT) -> *mut IMTDaily,
    pub IMTDailyArray_Sort: unsafe extern "C" fn(
        this: *mut IMTDailyArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTDailyArray_Search: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDailyArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDailyArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDailyArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDailyArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDailyArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDailyArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTDailyArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDailyArray {
    pub vtable_: *const IMTDailyArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDailyArray"][::std::mem::size_of::<IMTDailyArray>() - 8usize];
    ["Alignment of IMTDailyArray"][::std::mem::align_of::<IMTDailyArray>() - 8usize];
};
#[repr(C)]
pub struct IMTDailySink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTDailySink {
    pub vtable_: *const IMTDailySink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDailySink"][::std::mem::size_of::<IMTDailySink>() - 8usize];
    ["Alignment of IMTDailySink"][::std::mem::align_of::<IMTDailySink>() - 8usize];
};
pub const EnMTFeederConstants_MT_FEEDER_DEALER: EnMTFeederConstants = -1;
pub const EnMTFeederConstants_MT_FEEDER_OFFSET: EnMTFeederConstants = 64;
pub type EnMTFeederConstants = ::std::os::raw::c_int;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTTick {
    pub symbol: [u16; 32usize],
    pub bank: [u16; 32usize],
    pub datetime: INT64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: UINT64,
    pub datetime_msc: INT64,
    pub flags: UINT64,
    pub volume_ext: UINT64,
    pub reserved: [UINT; 26usize],
}
pub const MTTick_EnTickFlags_TICK_FLAG_BUY: MTTick_EnTickFlags = 1;
pub const MTTick_EnTickFlags_TICK_FLAG_SELL: MTTick_EnTickFlags = 2;
pub const MTTick_EnTickFlags_TICK_FLAG_NONE: MTTick_EnTickFlags = 0;
pub const MTTick_EnTickFlags_TICK_FLAG_ALL: MTTick_EnTickFlags = 3;
pub type MTTick_EnTickFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTTick"][::std::mem::size_of::<MTTick>() - 296usize];
    ["Alignment of MTTick"][::std::mem::align_of::<MTTick>() - 1usize];
    ["Offset of field: MTTick::symbol"][::std::mem::offset_of!(MTTick, symbol) - 0usize];
    ["Offset of field: MTTick::bank"][::std::mem::offset_of!(MTTick, bank) - 64usize];
    ["Offset of field: MTTick::datetime"][::std::mem::offset_of!(MTTick, datetime) - 128usize];
    ["Offset of field: MTTick::bid"][::std::mem::offset_of!(MTTick, bid) - 136usize];
    ["Offset of field: MTTick::ask"][::std::mem::offset_of!(MTTick, ask) - 144usize];
    ["Offset of field: MTTick::last"][::std::mem::offset_of!(MTTick, last) - 152usize];
    ["Offset of field: MTTick::volume"][::std::mem::offset_of!(MTTick, volume) - 160usize];
    ["Offset of field: MTTick::datetime_msc"]
        [::std::mem::offset_of!(MTTick, datetime_msc) - 168usize];
    ["Offset of field: MTTick::flags"][::std::mem::offset_of!(MTTick, flags) - 176usize];
    ["Offset of field: MTTick::volume_ext"][::std::mem::offset_of!(MTTick, volume_ext) - 184usize];
    ["Offset of field: MTTick::reserved"][::std::mem::offset_of!(MTTick, reserved) - 192usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTTickShort {
    pub datetime: INT64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub volume: UINT64,
    pub datetime_msc: INT64,
    pub flags: UINT64,
    pub volume_ext: UINT64,
    pub reserved: [UINT; 26usize],
}
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_RAW: MTTickShort_EnTickShortFlags = 1;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_BID: MTTickShort_EnTickShortFlags = 2;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_ASK: MTTickShort_EnTickShortFlags = 4;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_LAST: MTTickShort_EnTickShortFlags = 8;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_VOLUME: MTTickShort_EnTickShortFlags = 16;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_BUY: MTTickShort_EnTickShortFlags = 32;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_SELL: MTTickShort_EnTickShortFlags = 64;
pub const MTTickShort_EnTickShortFlags_TICK_SHORT_FLAG_NONE: MTTickShort_EnTickShortFlags = 0;
pub type MTTickShort_EnTickShortFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTTickShort"][::std::mem::size_of::<MTTickShort>() - 168usize];
    ["Alignment of MTTickShort"][::std::mem::align_of::<MTTickShort>() - 1usize];
    ["Offset of field: MTTickShort::datetime"]
        [::std::mem::offset_of!(MTTickShort, datetime) - 0usize];
    ["Offset of field: MTTickShort::bid"][::std::mem::offset_of!(MTTickShort, bid) - 8usize];
    ["Offset of field: MTTickShort::ask"][::std::mem::offset_of!(MTTickShort, ask) - 16usize];
    ["Offset of field: MTTickShort::last"][::std::mem::offset_of!(MTTickShort, last) - 24usize];
    ["Offset of field: MTTickShort::volume"][::std::mem::offset_of!(MTTickShort, volume) - 32usize];
    ["Offset of field: MTTickShort::datetime_msc"]
        [::std::mem::offset_of!(MTTickShort, datetime_msc) - 40usize];
    ["Offset of field: MTTickShort::flags"][::std::mem::offset_of!(MTTickShort, flags) - 48usize];
    ["Offset of field: MTTickShort::volume_ext"]
        [::std::mem::offset_of!(MTTickShort, volume_ext) - 56usize];
    ["Offset of field: MTTickShort::reserved"]
        [::std::mem::offset_of!(MTTickShort, reserved) - 64usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTTickRate {
    pub datetime_msc: INT64,
    pub bid: f64,
    pub ask: f64,
    pub last: f64,
    pub flags: UINT64,
    pub volume_ext: UINT64,
    pub reserved: [INT64; 2usize],
}
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_RAW: MTTickRate_EnTickShortFlags = 1;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_BID: MTTickRate_EnTickShortFlags = 2;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_ASK: MTTickRate_EnTickShortFlags = 4;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_LAST: MTTickRate_EnTickShortFlags = 8;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_VOLUME: MTTickRate_EnTickShortFlags = 16;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_BUY: MTTickRate_EnTickShortFlags = 32;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_SELL: MTTickRate_EnTickShortFlags = 64;
pub const MTTickRate_EnTickShortFlags_TICK_SHORT_FLAG_NONE: MTTickRate_EnTickShortFlags = 0;
pub type MTTickRate_EnTickShortFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTTickRate"][::std::mem::size_of::<MTTickRate>() - 64usize];
    ["Alignment of MTTickRate"][::std::mem::align_of::<MTTickRate>() - 1usize];
    ["Offset of field: MTTickRate::datetime_msc"]
        [::std::mem::offset_of!(MTTickRate, datetime_msc) - 0usize];
    ["Offset of field: MTTickRate::bid"][::std::mem::offset_of!(MTTickRate, bid) - 8usize];
    ["Offset of field: MTTickRate::ask"][::std::mem::offset_of!(MTTickRate, ask) - 16usize];
    ["Offset of field: MTTickRate::last"][::std::mem::offset_of!(MTTickRate, last) - 24usize];
    ["Offset of field: MTTickRate::flags"][::std::mem::offset_of!(MTTickRate, flags) - 32usize];
    ["Offset of field: MTTickRate::volume_ext"]
        [::std::mem::offset_of!(MTTickRate, volume_ext) - 40usize];
    ["Offset of field: MTTickRate::reserved"]
        [::std::mem::offset_of!(MTTickRate, reserved) - 48usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTTickStat {
    pub symbol: [u16; 32usize],
    pub datetime: INT64,
    pub bid_high: f64,
    pub bid_low: f64,
    pub ask_high: f64,
    pub ask_low: f64,
    pub last_high: f64,
    pub last_low: f64,
    pub vol_high: UINT64,
    pub vol_low: UINT64,
    pub trade_deals: UINT64,
    pub trade_volume: UINT64,
    pub trade_turnover: UINT64,
    pub trade_interest: UINT64,
    pub trade_buy_orders: UINT64,
    pub trade_buy_volume: UINT64,
    pub trade_sell_orders: UINT64,
    pub trade_sell_volume: UINT64,
    pub trade_volume_ext: UINT64,
    pub trade_buy_volume_ext: UINT64,
    pub trade_sell_volume_ext: UINT64,
    pub vol_high_ext: UINT64,
    pub vol_low_ext: UINT64,
    pub trade_reserved: [::std::os::raw::c_int; 20usize],
    pub datetime_msc: INT64,
    pub price_open: f64,
    pub price_close: f64,
    pub price_aw: f64,
    pub price_obsolete: f64,
    pub price_volatility: f64,
    pub price_theoretical: f64,
    pub price_greeks_delta: f64,
    pub price_greeks_theta: f64,
    pub price_greeks_gamma: f64,
    pub price_greeks_vega: f64,
    pub price_greeks_rho: f64,
    pub price_greeks_omega: f64,
    pub price_sensitivity: f64,
    pub price_reserved: [::std::os::raw::c_int; 14usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTTickStat"][::std::mem::size_of::<MTTickStat>() - 488usize];
    ["Alignment of MTTickStat"][::std::mem::align_of::<MTTickStat>() - 1usize];
    ["Offset of field: MTTickStat::symbol"][::std::mem::offset_of!(MTTickStat, symbol) - 0usize];
    ["Offset of field: MTTickStat::datetime"]
        [::std::mem::offset_of!(MTTickStat, datetime) - 64usize];
    ["Offset of field: MTTickStat::bid_high"]
        [::std::mem::offset_of!(MTTickStat, bid_high) - 72usize];
    ["Offset of field: MTTickStat::bid_low"][::std::mem::offset_of!(MTTickStat, bid_low) - 80usize];
    ["Offset of field: MTTickStat::ask_high"]
        [::std::mem::offset_of!(MTTickStat, ask_high) - 88usize];
    ["Offset of field: MTTickStat::ask_low"][::std::mem::offset_of!(MTTickStat, ask_low) - 96usize];
    ["Offset of field: MTTickStat::last_high"]
        [::std::mem::offset_of!(MTTickStat, last_high) - 104usize];
    ["Offset of field: MTTickStat::last_low"]
        [::std::mem::offset_of!(MTTickStat, last_low) - 112usize];
    ["Offset of field: MTTickStat::vol_high"]
        [::std::mem::offset_of!(MTTickStat, vol_high) - 120usize];
    ["Offset of field: MTTickStat::vol_low"]
        [::std::mem::offset_of!(MTTickStat, vol_low) - 128usize];
    ["Offset of field: MTTickStat::trade_deals"]
        [::std::mem::offset_of!(MTTickStat, trade_deals) - 136usize];
    ["Offset of field: MTTickStat::trade_volume"]
        [::std::mem::offset_of!(MTTickStat, trade_volume) - 144usize];
    ["Offset of field: MTTickStat::trade_turnover"]
        [::std::mem::offset_of!(MTTickStat, trade_turnover) - 152usize];
    ["Offset of field: MTTickStat::trade_interest"]
        [::std::mem::offset_of!(MTTickStat, trade_interest) - 160usize];
    ["Offset of field: MTTickStat::trade_buy_orders"]
        [::std::mem::offset_of!(MTTickStat, trade_buy_orders) - 168usize];
    ["Offset of field: MTTickStat::trade_buy_volume"]
        [::std::mem::offset_of!(MTTickStat, trade_buy_volume) - 176usize];
    ["Offset of field: MTTickStat::trade_sell_orders"]
        [::std::mem::offset_of!(MTTickStat, trade_sell_orders) - 184usize];
    ["Offset of field: MTTickStat::trade_sell_volume"]
        [::std::mem::offset_of!(MTTickStat, trade_sell_volume) - 192usize];
    ["Offset of field: MTTickStat::trade_volume_ext"]
        [::std::mem::offset_of!(MTTickStat, trade_volume_ext) - 200usize];
    ["Offset of field: MTTickStat::trade_buy_volume_ext"]
        [::std::mem::offset_of!(MTTickStat, trade_buy_volume_ext) - 208usize];
    ["Offset of field: MTTickStat::trade_sell_volume_ext"]
        [::std::mem::offset_of!(MTTickStat, trade_sell_volume_ext) - 216usize];
    ["Offset of field: MTTickStat::vol_high_ext"]
        [::std::mem::offset_of!(MTTickStat, vol_high_ext) - 224usize];
    ["Offset of field: MTTickStat::vol_low_ext"]
        [::std::mem::offset_of!(MTTickStat, vol_low_ext) - 232usize];
    ["Offset of field: MTTickStat::trade_reserved"]
        [::std::mem::offset_of!(MTTickStat, trade_reserved) - 240usize];
    ["Offset of field: MTTickStat::datetime_msc"]
        [::std::mem::offset_of!(MTTickStat, datetime_msc) - 320usize];
    ["Offset of field: MTTickStat::price_open"]
        [::std::mem::offset_of!(MTTickStat, price_open) - 328usize];
    ["Offset of field: MTTickStat::price_close"]
        [::std::mem::offset_of!(MTTickStat, price_close) - 336usize];
    ["Offset of field: MTTickStat::price_aw"]
        [::std::mem::offset_of!(MTTickStat, price_aw) - 344usize];
    ["Offset of field: MTTickStat::price_obsolete"]
        [::std::mem::offset_of!(MTTickStat, price_obsolete) - 352usize];
    ["Offset of field: MTTickStat::price_volatility"]
        [::std::mem::offset_of!(MTTickStat, price_volatility) - 360usize];
    ["Offset of field: MTTickStat::price_theoretical"]
        [::std::mem::offset_of!(MTTickStat, price_theoretical) - 368usize];
    ["Offset of field: MTTickStat::price_greeks_delta"]
        [::std::mem::offset_of!(MTTickStat, price_greeks_delta) - 376usize];
    ["Offset of field: MTTickStat::price_greeks_theta"]
        [::std::mem::offset_of!(MTTickStat, price_greeks_theta) - 384usize];
    ["Offset of field: MTTickStat::price_greeks_gamma"]
        [::std::mem::offset_of!(MTTickStat, price_greeks_gamma) - 392usize];
    ["Offset of field: MTTickStat::price_greeks_vega"]
        [::std::mem::offset_of!(MTTickStat, price_greeks_vega) - 400usize];
    ["Offset of field: MTTickStat::price_greeks_rho"]
        [::std::mem::offset_of!(MTTickStat, price_greeks_rho) - 408usize];
    ["Offset of field: MTTickStat::price_greeks_omega"]
        [::std::mem::offset_of!(MTTickStat, price_greeks_omega) - 416usize];
    ["Offset of field: MTTickStat::price_sensitivity"]
        [::std::mem::offset_of!(MTTickStat, price_sensitivity) - 424usize];
    ["Offset of field: MTTickStat::price_reserved"]
        [::std::mem::offset_of!(MTTickStat, price_reserved) - 432usize];
};
#[repr(C)]
pub struct IMTTickSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTTickSink {
    pub vtable_: *const IMTTickSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTTickSink"][::std::mem::size_of::<IMTTickSink>() - 8usize];
    ["Alignment of IMTTickSink"][::std::mem::align_of::<IMTTickSink>() - 8usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTMailRange {
    pub first_login: UINT64,
    pub last_login: UINT64,
    pub reserved: [UINT; 4usize],
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTMailRange"][::std::mem::size_of::<MTMailRange>() - 32usize];
    ["Alignment of MTMailRange"][::std::mem::align_of::<MTMailRange>() - 1usize];
    ["Offset of field: MTMailRange::first_login"]
        [::std::mem::offset_of!(MTMailRange, first_login) - 0usize];
    ["Offset of field: MTMailRange::last_login"]
        [::std::mem::offset_of!(MTMailRange, last_login) - 8usize];
    ["Offset of field: MTMailRange::reserved"]
        [::std::mem::offset_of!(MTMailRange, reserved) - 16usize];
};
#[repr(C)]
pub struct IMTMail__bindgen_vtable {
    pub IMTMail_Release: unsafe extern "C" fn(this: *mut IMTMail),
    pub IMTMail_Assign: unsafe extern "C" fn(this: *mut IMTMail, mail: *const IMTMail) -> MTAPIRES,
    pub IMTMail_Clear: unsafe extern "C" fn(this: *mut IMTMail) -> MTAPIRES,
    pub IMTMail_ID: unsafe extern "C" fn(this: *const IMTMail) -> UINT64,
    pub IMTMail_Parent: unsafe extern "C" fn(this: *const IMTMail) -> UINT64,
    pub IMTMail_Subject1: unsafe extern "C" fn(this: *mut IMTMail, subject: LPCWSTR) -> MTAPIRES,
    pub IMTMail_Subject: unsafe extern "C" fn(this: *const IMTMail) -> LPCWSTR,
    pub IMTMail_From1: unsafe extern "C" fn(this: *mut IMTMail, id: UINT64) -> MTAPIRES,
    pub IMTMail_From: unsafe extern "C" fn(this: *const IMTMail) -> UINT64,
    pub IMTMail_FromName1: unsafe extern "C" fn(this: *mut IMTMail, name: LPCWSTR) -> MTAPIRES,
    pub IMTMail_FromName: unsafe extern "C" fn(this: *const IMTMail) -> LPCWSTR,
    pub IMTMail_To1: unsafe extern "C" fn(this: *mut IMTMail, id: UINT64) -> MTAPIRES,
    pub IMTMail_To: unsafe extern "C" fn(this: *const IMTMail) -> UINT64,
    pub IMTMail_ToName1: unsafe extern "C" fn(this: *mut IMTMail, name: LPCWSTR) -> MTAPIRES,
    pub IMTMail_ToName: unsafe extern "C" fn(this: *const IMTMail) -> LPCWSTR,
    pub IMTMail_ToRangesAdd:
        unsafe extern "C" fn(this: *mut IMTMail, range: *mut MTMailRange) -> MTAPIRES,
    pub IMTMail_ToRangesDelete: unsafe extern "C" fn(this: *mut IMTMail, pos: UINT) -> MTAPIRES,
    pub IMTMail_ToRangesClear: unsafe extern "C" fn(this: *mut IMTMail) -> MTAPIRES,
    pub IMTMail_ToRangesTotal: unsafe extern "C" fn(this: *const IMTMail) -> UINT,
    pub IMTMail_ToRangesNext:
        unsafe extern "C" fn(this: *const IMTMail, pos: UINT, range: *mut MTMailRange) -> MTAPIRES,
    pub IMTMail_Time: unsafe extern "C" fn(this: *const IMTMail) -> INT64,
    pub IMTMail_Body1:
        unsafe extern "C" fn(this: *mut IMTMail, body: LPCVOID, body_size: UINT) -> MTAPIRES,
    pub IMTMail_Body: unsafe extern "C" fn(this: *const IMTMail) -> LPCVOID,
    pub IMTMail_BodySize: unsafe extern "C" fn(this: *const IMTMail) -> UINT,
    pub IMTMail_AttachmentsAdd: unsafe extern "C" fn(
        this: *mut IMTMail,
        filename: LPCWSTR,
        attachment: LPCVOID,
        attachment_size: UINT,
    ) -> MTAPIRES,
    pub IMTMail_AttachmentsClear: unsafe extern "C" fn(this: *mut IMTMail) -> MTAPIRES,
    pub IMTMail_AttachmentsTotal: unsafe extern "C" fn(this: *const IMTMail) -> UINT,
    pub IMTMail_AttachmentsBody: unsafe extern "C" fn(this: *const IMTMail, pos: UINT) -> LPVOID,
    pub IMTMail_AttachmentsSize: unsafe extern "C" fn(this: *const IMTMail, pos: UINT) -> UINT,
    pub IMTMail_AttachmentsName: unsafe extern "C" fn(this: *const IMTMail, pos: UINT) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTMail {
    pub vtable_: *const IMTMail__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTMail"][::std::mem::size_of::<IMTMail>() - 8usize];
    ["Alignment of IMTMail"][::std::mem::align_of::<IMTMail>() - 8usize];
};
#[repr(C)]
pub struct IMTMailSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTMailSink {
    pub vtable_: *const IMTMailSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTMailSink"][::std::mem::size_of::<IMTMailSink>() - 8usize];
    ["Alignment of IMTMailSink"][::std::mem::align_of::<IMTMailSink>() - 8usize];
};
#[repr(C)]
pub struct IMTNews__bindgen_vtable {
    pub IMTNews_Release: unsafe extern "C" fn(this: *mut IMTNews),
    pub IMTNews_Assign: unsafe extern "C" fn(this: *mut IMTNews, news: *const IMTNews) -> MTAPIRES,
    pub IMTNews_Clear: unsafe extern "C" fn(this: *mut IMTNews) -> MTAPIRES,
    pub IMTNews_ID: unsafe extern "C" fn(this: *const IMTNews) -> UINT64,
    pub IMTNews_Subject1: unsafe extern "C" fn(this: *mut IMTNews, subject: LPCWSTR) -> MTAPIRES,
    pub IMTNews_Subject: unsafe extern "C" fn(this: *const IMTNews) -> LPCWSTR,
    pub IMTNews_Category1: unsafe extern "C" fn(this: *mut IMTNews, category: LPCWSTR) -> MTAPIRES,
    pub IMTNews_Category: unsafe extern "C" fn(this: *const IMTNews) -> LPCWSTR,
    pub IMTNews_Time1: unsafe extern "C" fn(this: *mut IMTNews, datetime: INT64) -> MTAPIRES,
    pub IMTNews_Time: unsafe extern "C" fn(this: *const IMTNews) -> INT64,
    pub IMTNews_Language1: unsafe extern "C" fn(this: *mut IMTNews, language: UINT) -> MTAPIRES,
    pub IMTNews_Language: unsafe extern "C" fn(this: *const IMTNews) -> UINT,
    pub IMTNews_Flags1: unsafe extern "C" fn(this: *mut IMTNews, flags: UINT) -> MTAPIRES,
    pub IMTNews_Flags: unsafe extern "C" fn(this: *const IMTNews) -> UINT,
    pub IMTNews_Body1:
        unsafe extern "C" fn(this: *mut IMTNews, body: LPCVOID, body_size: UINT) -> MTAPIRES,
    pub IMTNews_Body: unsafe extern "C" fn(this: *const IMTNews) -> LPCVOID,
    pub IMTNews_BodySize: unsafe extern "C" fn(this: *const IMTNews) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTNews {
    pub vtable_: *const IMTNews__bindgen_vtable,
}
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_NONE: IMTNews_EnNewsFlags = 0;
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_PRIORITY: IMTNews_EnNewsFlags = 1;
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_READ: IMTNews_EnNewsFlags = 2;
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_NOBODY: IMTNews_EnNewsFlags = 4;
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_CALENDAR: IMTNews_EnNewsFlags = 8;
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_FIRST: IMTNews_EnNewsFlags = 1;
pub const IMTNews_EnNewsFlags_NEWS_FLAGS_ALL: IMTNews_EnNewsFlags = 15;
pub type IMTNews_EnNewsFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTNews"][::std::mem::size_of::<IMTNews>() - 8usize];
    ["Alignment of IMTNews"][::std::mem::align_of::<IMTNews>() - 8usize];
};
#[repr(C)]
pub struct IMTNewsSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTNewsSink {
    pub vtable_: *const IMTNewsSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTNewsSink"][::std::mem::size_of::<IMTNewsSink>() - 8usize];
    ["Alignment of IMTNewsSink"][::std::mem::align_of::<IMTNewsSink>() - 8usize];
};
#[repr(C)]
pub struct IMTByteStream__bindgen_vtable {
    pub IMTByteStream_Release: unsafe extern "C" fn(this: *mut IMTByteStream),
    pub IMTByteStream_Assign:
        unsafe extern "C" fn(this: *mut IMTByteStream, stream: *const IMTByteStream) -> MTAPIRES,
    pub IMTByteStream_Clear: unsafe extern "C" fn(this: *mut IMTByteStream) -> MTAPIRES,
    pub IMTByteStream_Len: unsafe extern "C" fn(this: *mut IMTByteStream) -> UINT,
    pub IMTByteStream_ReadLen: unsafe extern "C" fn(this: *mut IMTByteStream) -> UINT,
    pub IMTByteStream_Add: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        buf: *const ::std::os::raw::c_void,
        len: UINT,
    ) -> MTAPIRES,
    pub IMTByteStream_AddChar:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: ::std::os::raw::c_char) -> MTAPIRES,
    pub IMTByteStream_AddUChar:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: UCHAR) -> MTAPIRES,
    pub IMTByteStream_AddShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: ::std::os::raw::c_short) -> MTAPIRES,
    pub IMTByteStream_AddUShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: USHORT) -> MTAPIRES,
    pub IMTByteStream_AddInt:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: ::std::os::raw::c_int) -> MTAPIRES,
    pub IMTByteStream_AddUInt:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: UINT) -> MTAPIRES,
    pub IMTByteStream_AddInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: INT64) -> MTAPIRES,
    pub IMTByteStream_AddUInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: UINT64) -> MTAPIRES,
    pub IMTByteStream_AddFloat:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: f32) -> MTAPIRES,
    pub IMTByteStream_AddDouble:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: f64) -> MTAPIRES,
    pub IMTByteStream_AddResult:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: MTAPIRES) -> MTAPIRES,
    pub IMTByteStream_AddStr:
        unsafe extern "C" fn(this: *mut IMTByteStream, buf: LPCWSTR) -> MTAPIRES,
    pub IMTByteStream_ReadReset: unsafe extern "C" fn(this: *mut IMTByteStream),
    pub IMTByteStream_Read: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        buf: *mut ::std::os::raw::c_void,
        len: UINT,
    ) -> MTAPIRES,
    pub IMTByteStream_ReadSkip:
        unsafe extern "C" fn(this: *mut IMTByteStream, len: UINT) -> MTAPIRES,
    pub IMTByteStream_ReadChar: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        data: *mut ::std::os::raw::c_char,
    ) -> MTAPIRES,
    pub IMTByteStream_ReadUChar:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut UCHAR) -> MTAPIRES,
    pub IMTByteStream_ReadShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut SHORT) -> MTAPIRES,
    pub IMTByteStream_ReadUShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut USHORT) -> MTAPIRES,
    pub IMTByteStream_ReadInt: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        data: *mut ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTByteStream_ReadUInt:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut UINT) -> MTAPIRES,
    pub IMTByteStream_ReadInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut INT64) -> MTAPIRES,
    pub IMTByteStream_ReadUInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut UINT64) -> MTAPIRES,
    pub IMTByteStream_ReadFloat:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut f32) -> MTAPIRES,
    pub IMTByteStream_ReadDouble:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut f64) -> MTAPIRES,
    pub IMTByteStream_ReadResult:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut MTAPIRES) -> MTAPIRES,
    pub IMTByteStream_ReadStr:
        unsafe extern "C" fn(this: *mut IMTByteStream, buf: *mut MTAPISTR) -> MTAPIRES,
    pub IMTByteStream_WebAddParamStr:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: LPCWSTR, value: LPCWSTR) -> MTAPIRES,
    pub IMTByteStream_WebAddParamChar: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        name: LPCWSTR,
        value: ::std::os::raw::c_char,
    ) -> MTAPIRES,
    pub IMTByteStream_WebAddParamUChar:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: LPCWSTR, value: UCHAR) -> MTAPIRES,
    pub IMTByteStream_WebAddParamShort: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        name: LPCWSTR,
        value: ::std::os::raw::c_short,
    ) -> MTAPIRES,
    pub IMTByteStream_WebAddParamUShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: LPCWSTR, value: USHORT) -> MTAPIRES,
    pub IMTByteStream_WebAddParamInt: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        name: LPCWSTR,
        value: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTByteStream_WebAddParamUInt:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: LPCWSTR, value: UINT) -> MTAPIRES,
    pub IMTByteStream_WebAddParamInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: LPCWSTR, value: INT64) -> MTAPIRES,
    pub IMTByteStream_WebAddParamUInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: LPCWSTR, value: UINT64) -> MTAPIRES,
    pub IMTByteStream_WebAddParamDouble: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        name: LPCWSTR,
        value: f64,
        digits: UINT,
    ) -> MTAPIRES,
    pub IMTByteStream_WebAddParamFinalize:
        unsafe extern "C" fn(this: *mut IMTByteStream) -> MTAPIRES,
    pub IMTByteStream_WebReadCommand:
        unsafe extern "C" fn(this: *mut IMTByteStream, cmd: *mut MTAPISTR) -> MTAPIRES,
    pub IMTByteStream_WebReadParamName:
        unsafe extern "C" fn(this: *mut IMTByteStream, name: *mut MTAPISTR) -> MTAPIRES,
    pub IMTByteStream_WebReadParamStr1:
        unsafe extern "C" fn(this: *mut IMTByteStream, value: LPWSTR, size: UINT) -> MTAPIRES,
    pub IMTByteStream_WebReadParamStr:
        unsafe extern "C" fn(this: *mut IMTByteStream, str_: *mut MTAPISTR) -> MTAPIRES,
    pub IMTByteStream_WebReadParamSkip: unsafe extern "C" fn(this: *mut IMTByteStream) -> MTAPIRES,
    pub IMTByteStream_WebReadParamChar: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        data: *mut ::std::os::raw::c_char,
    ) -> MTAPIRES,
    pub IMTByteStream_WebReadParamUChar:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut UCHAR) -> MTAPIRES,
    pub IMTByteStream_WebReadParamShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut SHORT) -> MTAPIRES,
    pub IMTByteStream_WebReadParamUShort:
        unsafe extern "C" fn(this: *mut IMTByteStream, data: *mut USHORT) -> MTAPIRES,
    pub IMTByteStream_WebReadParamInt: unsafe extern "C" fn(
        this: *mut IMTByteStream,
        value: *mut ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTByteStream_WebReadParamUInt:
        unsafe extern "C" fn(this: *mut IMTByteStream, value: *mut UINT) -> MTAPIRES,
    pub IMTByteStream_WebReadParamInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, value: *mut INT64) -> MTAPIRES,
    pub IMTByteStream_WebReadParamUInt64:
        unsafe extern "C" fn(this: *mut IMTByteStream, value: *mut UINT64) -> MTAPIRES,
    pub IMTByteStream_WebReadParamDouble:
        unsafe extern "C" fn(this: *mut IMTByteStream, value: *mut f64) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTByteStream {
    pub vtable_: *const IMTByteStream__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTByteStream"][::std::mem::size_of::<IMTByteStream>() - 8usize];
    ["Alignment of IMTByteStream"][::std::mem::align_of::<IMTByteStream>() - 8usize];
};
#[repr(C)]
pub struct IMTCustomSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTCustomSink {
    pub vtable_: *const IMTCustomSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTCustomSink"][::std::mem::size_of::<IMTCustomSink>() - 8usize];
    ["Alignment of IMTCustomSink"][::std::mem::align_of::<IMTCustomSink>() - 8usize];
};
#[repr(C)]
pub struct IMTRequest__bindgen_vtable {
    pub IMTRequest_Release: unsafe extern "C" fn(this: *mut IMTRequest),
    pub IMTRequest_Assign:
        unsafe extern "C" fn(this: *mut IMTRequest, request: *const IMTRequest) -> MTAPIRES,
    pub IMTRequest_Clear: unsafe extern "C" fn(this: *mut IMTRequest) -> MTAPIRES,
    pub IMTRequest_Print:
        unsafe extern "C" fn(this: *const IMTRequest, string: *mut MTAPISTR) -> LPCWSTR,
    pub IMTRequest_ID: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_Login1: unsafe extern "C" fn(this: *mut IMTRequest, login: UINT64) -> MTAPIRES,
    pub IMTRequest_Login: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_Group: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_Symbol1:
        unsafe extern "C" fn(this: *mut IMTRequest, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_Symbol: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_Digits: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_Action1: unsafe extern "C" fn(this: *mut IMTRequest, action: UINT) -> MTAPIRES,
    pub IMTRequest_Action: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_TimeExpiration1:
        unsafe extern "C" fn(this: *mut IMTRequest, time: INT64) -> MTAPIRES,
    pub IMTRequest_TimeExpiration: unsafe extern "C" fn(this: *const IMTRequest) -> INT64,
    pub IMTRequest_Type1: unsafe extern "C" fn(this: *mut IMTRequest, type_: UINT) -> MTAPIRES,
    pub IMTRequest_Type: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_TypeFill1: unsafe extern "C" fn(this: *mut IMTRequest, type_: UINT) -> MTAPIRES,
    pub IMTRequest_TypeFill: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_TypeTime1: unsafe extern "C" fn(this: *mut IMTRequest, type_: UINT) -> MTAPIRES,
    pub IMTRequest_TypeTime: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_Flags1: unsafe extern "C" fn(this: *mut IMTRequest, flags: UINT64) -> MTAPIRES,
    pub IMTRequest_Flags: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_Volume1: unsafe extern "C" fn(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES,
    pub IMTRequest_Volume: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_Order1: unsafe extern "C" fn(this: *mut IMTRequest, order: UINT64) -> MTAPIRES,
    pub IMTRequest_Order: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_OrderExternalID1:
        unsafe extern "C" fn(this: *mut IMTRequest, id: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_OrderExternalID: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_PriceOrder1: unsafe extern "C" fn(this: *mut IMTRequest, price: f64) -> MTAPIRES,
    pub IMTRequest_PriceOrder: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_PriceTrigger1:
        unsafe extern "C" fn(this: *mut IMTRequest, price: f64) -> MTAPIRES,
    pub IMTRequest_PriceTrigger: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_PriceSL1: unsafe extern "C" fn(this: *mut IMTRequest, price: f64) -> MTAPIRES,
    pub IMTRequest_PriceSL: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_PriceTP1: unsafe extern "C" fn(this: *mut IMTRequest, price: f64) -> MTAPIRES,
    pub IMTRequest_PriceTP: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_PriceDeviation1:
        unsafe extern "C" fn(this: *mut IMTRequest, deviation: UINT64) -> MTAPIRES,
    pub IMTRequest_PriceDeviation: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_PriceDeviationTop: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_PriceDeviationBottom: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_Comment1:
        unsafe extern "C" fn(this: *mut IMTRequest, comment: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_Comment: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_ResultRetcode: unsafe extern "C" fn(this: *const IMTRequest) -> MTAPIRES,
    pub IMTRequest_ResultDealer: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_ResultDeal: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_ResultOrder: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_ResultVolume: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_ResultPrice: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultDealerBid: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultDealerAsk: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultDealerLast: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultMarketBid: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultMarketAsk: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultMarketLast: unsafe extern "C" fn(this: *const IMTRequest) -> f64,
    pub IMTRequest_ResultComment: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_ExternalAccount1:
        unsafe extern "C" fn(this: *mut IMTRequest, account: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_ExternalAccount: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_IDClient: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_IP1: unsafe extern "C" fn(this: *mut IMTRequest, ip: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_IP: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_SourceLogin1:
        unsafe extern "C" fn(this: *mut IMTRequest, login: UINT64) -> MTAPIRES,
    pub IMTRequest_SourceLogin: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_Position1:
        unsafe extern "C" fn(this: *mut IMTRequest, position: UINT64) -> MTAPIRES,
    pub IMTRequest_Position: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_PositionBy1:
        unsafe extern "C" fn(this: *mut IMTRequest, position: UINT64) -> MTAPIRES,
    pub IMTRequest_PositionBy: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_PositionExternalID1:
        unsafe extern "C" fn(this: *mut IMTRequest, id: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_PositionExternalID: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_PositionByExternalID1:
        unsafe extern "C" fn(this: *mut IMTRequest, id: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_PositionByExternalID: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
    pub IMTRequest_VolumeExt1:
        unsafe extern "C" fn(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES,
    pub IMTRequest_VolumeExt: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_ResultVolumeExt: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_DigitsSet: unsafe extern "C" fn(this: *mut IMTRequest, digits: UINT) -> MTAPIRES,
    pub IMTRequest_ApiDataSet2: unsafe extern "C" fn(
        this: *mut IMTRequest,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataSet1: unsafe extern "C" fn(
        this: *mut IMTRequest,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataSet: unsafe extern "C" fn(
        this: *mut IMTRequest,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataGet2: unsafe extern "C" fn(
        this: *const IMTRequest,
        app_id: USHORT,
        id: UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataGet1: unsafe extern "C" fn(
        this: *const IMTRequest,
        app_id: USHORT,
        id: UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataGet: unsafe extern "C" fn(
        this: *const IMTRequest,
        app_id: USHORT,
        id: UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataUpdate2: unsafe extern "C" fn(
        this: *mut IMTRequest,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataUpdate1: unsafe extern "C" fn(
        this: *mut IMTRequest,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataUpdate: unsafe extern "C" fn(
        this: *mut IMTRequest,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataNext2: unsafe extern "C" fn(
        this: *const IMTRequest,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataNext1: unsafe extern "C" fn(
        this: *const IMTRequest,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataNext: unsafe extern "C" fn(
        this: *const IMTRequest,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTRequest_ApiDataRaw: unsafe extern "C" fn(this: *const IMTRequest) -> LPVOID,
    pub IMTRequest_ApiDataRawMax: unsafe extern "C" fn(this: *const IMTRequest) -> UINT,
    pub IMTRequest_ApiDataClear:
        unsafe extern "C" fn(this: *mut IMTRequest, app_id: USHORT) -> MTAPIRES,
    pub IMTRequest_ApiDataClearAll: unsafe extern "C" fn(this: *mut IMTRequest) -> MTAPIRES,
    pub IMTRequest_VolumeCurrent1:
        unsafe extern "C" fn(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES,
    pub IMTRequest_VolumeCurrent: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_VolumeCurrentExt1:
        unsafe extern "C" fn(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES,
    pub IMTRequest_VolumeCurrentExt: unsafe extern "C" fn(this: *const IMTRequest) -> UINT64,
    pub IMTRequest_SymbolOriginal1:
        unsafe extern "C" fn(this: *mut IMTRequest, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTRequest_SymbolOriginal: unsafe extern "C" fn(this: *const IMTRequest) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTRequest {
    pub vtable_: *const IMTRequest__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::mt_request::MTRequest,
}
pub const IMTRequest_EnTradeActions_TA_CLIENT_FIRST: IMTRequest_EnTradeActions = 0;
pub const IMTRequest_EnTradeActions_TA_PRICE: IMTRequest_EnTradeActions = 0;
pub const IMTRequest_EnTradeActions_TA_REQUEST: IMTRequest_EnTradeActions = 1;
pub const IMTRequest_EnTradeActions_TA_INSTANT: IMTRequest_EnTradeActions = 2;
pub const IMTRequest_EnTradeActions_TA_MARKET: IMTRequest_EnTradeActions = 3;
pub const IMTRequest_EnTradeActions_TA_EXCHANGE: IMTRequest_EnTradeActions = 4;
pub const IMTRequest_EnTradeActions_TA_PENDING: IMTRequest_EnTradeActions = 5;
pub const IMTRequest_EnTradeActions_TA_SLTP: IMTRequest_EnTradeActions = 6;
pub const IMTRequest_EnTradeActions_TA_MODIFY: IMTRequest_EnTradeActions = 7;
pub const IMTRequest_EnTradeActions_TA_REMOVE: IMTRequest_EnTradeActions = 8;
pub const IMTRequest_EnTradeActions_TA_TRANSFER: IMTRequest_EnTradeActions = 9;
pub const IMTRequest_EnTradeActions_TA_CLOSE_BY: IMTRequest_EnTradeActions = 10;
pub const IMTRequest_EnTradeActions_TA_CLIENT_LAST: IMTRequest_EnTradeActions = 10;
pub const IMTRequest_EnTradeActions_TA_SERVER_FIRST: IMTRequest_EnTradeActions = 100;
pub const IMTRequest_EnTradeActions_TA_ACTIVATE: IMTRequest_EnTradeActions = 100;
pub const IMTRequest_EnTradeActions_TA_ACTIVATE_SL: IMTRequest_EnTradeActions = 101;
pub const IMTRequest_EnTradeActions_TA_ACTIVATE_TP: IMTRequest_EnTradeActions = 102;
pub const IMTRequest_EnTradeActions_TA_ACTIVATE_STOPLIMIT: IMTRequest_EnTradeActions = 103;
pub const IMTRequest_EnTradeActions_TA_STOPOUT_ORDER: IMTRequest_EnTradeActions = 104;
pub const IMTRequest_EnTradeActions_TA_STOPOUT_POSITION: IMTRequest_EnTradeActions = 105;
pub const IMTRequest_EnTradeActions_TA_EXPIRATION: IMTRequest_EnTradeActions = 106;
pub const IMTRequest_EnTradeActions_TA_SERVER_LAST: IMTRequest_EnTradeActions = 106;
pub const IMTRequest_EnTradeActions_TA_DEALER_FIRST: IMTRequest_EnTradeActions = 200;
pub const IMTRequest_EnTradeActions_TA_DEALER_POS_EXECUTE: IMTRequest_EnTradeActions = 200;
pub const IMTRequest_EnTradeActions_TA_DEALER_ORD_PENDING: IMTRequest_EnTradeActions = 201;
pub const IMTRequest_EnTradeActions_TA_DEALER_POS_MODIFY: IMTRequest_EnTradeActions = 202;
pub const IMTRequest_EnTradeActions_TA_DEALER_ORD_MODIFY: IMTRequest_EnTradeActions = 203;
pub const IMTRequest_EnTradeActions_TA_DEALER_ORD_REMOVE: IMTRequest_EnTradeActions = 204;
pub const IMTRequest_EnTradeActions_TA_DEALER_ORD_ACTIVATE: IMTRequest_EnTradeActions = 205;
pub const IMTRequest_EnTradeActions_TA_DEALER_BALANCE: IMTRequest_EnTradeActions = 206;
pub const IMTRequest_EnTradeActions_TA_DEALER_ORD_SLIMIT: IMTRequest_EnTradeActions = 207;
pub const IMTRequest_EnTradeActions_TA_DEALER_CLOSE_BY: IMTRequest_EnTradeActions = 208;
pub const IMTRequest_EnTradeActions_TA_DEALER_LAST: IMTRequest_EnTradeActions = 208;
pub const IMTRequest_EnTradeActions_TA_FIRST: IMTRequest_EnTradeActions = 0;
pub const IMTRequest_EnTradeActions_TA_LAST: IMTRequest_EnTradeActions = 208;
pub const IMTRequest_EnTradeActions_TA_END: IMTRequest_EnTradeActions = 255;
pub type IMTRequest_EnTradeActions = ::std::os::raw::c_int;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CLOSE: IMTRequest_EnTradeActionFlags = 1;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_MARKET: IMTRequest_EnTradeActionFlags = 2;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CHANGED_PRICE: IMTRequest_EnTradeActionFlags = 4;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CHANGED_TRIGGER: IMTRequest_EnTradeActionFlags = 8;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CHANGED_SL: IMTRequest_EnTradeActionFlags = 16;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CHANGED_TP: IMTRequest_EnTradeActionFlags = 32;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CHANGED_EXP_TYPE: IMTRequest_EnTradeActionFlags =
    64;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_CHANGED_EXP_TIME: IMTRequest_EnTradeActionFlags =
    128;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_EXPERT: IMTRequest_EnTradeActionFlags = 256;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_SIGNAL: IMTRequest_EnTradeActionFlags = 512;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_SKIP_MARGIN_CHECK: IMTRequest_EnTradeActionFlags =
    1024;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_NONE: IMTRequest_EnTradeActionFlags = 0;
pub const IMTRequest_EnTradeActionFlags_TA_FLAG_ALL: IMTRequest_EnTradeActionFlags = 2047;
pub type IMTRequest_EnTradeActionFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTRequest"][::std::mem::size_of::<IMTRequest>() - 24usize];
    ["Alignment of IMTRequest"][::std::mem::align_of::<IMTRequest>() - 8usize];
};
#[repr(C)]
pub struct IMTConfirm__bindgen_vtable {
    pub IMTConfirm_Release: unsafe extern "C" fn(this: *mut IMTConfirm),
    pub IMTConfirm_Assign:
        unsafe extern "C" fn(this: *mut IMTConfirm, confirm: *const IMTConfirm) -> MTAPIRES,
    pub IMTConfirm_Clear: unsafe extern "C" fn(this: *mut IMTConfirm) -> MTAPIRES,
    pub IMTConfirm_Print:
        unsafe extern "C" fn(this: *const IMTConfirm, string: *mut MTAPISTR) -> LPCWSTR,
    pub IMTConfirm_ID1: unsafe extern "C" fn(this: *mut IMTConfirm, id: UINT) -> MTAPIRES,
    pub IMTConfirm_ID: unsafe extern "C" fn(this: *const IMTConfirm) -> UINT,
    pub IMTConfirm_Retcode1:
        unsafe extern "C" fn(this: *mut IMTConfirm, retcode: MTAPIRES) -> MTAPIRES,
    pub IMTConfirm_Retcode: unsafe extern "C" fn(this: *const IMTConfirm) -> MTAPIRES,
    pub IMTConfirm_Volume1: unsafe extern "C" fn(this: *mut IMTConfirm, volume: UINT64) -> MTAPIRES,
    pub IMTConfirm_Volume: unsafe extern "C" fn(this: *const IMTConfirm) -> UINT64,
    pub IMTConfirm_Price1: unsafe extern "C" fn(this: *mut IMTConfirm, price: f64) -> MTAPIRES,
    pub IMTConfirm_Price: unsafe extern "C" fn(this: *const IMTConfirm) -> f64,
    pub IMTConfirm_TickBid1: unsafe extern "C" fn(this: *mut IMTConfirm, tickbid: f64) -> MTAPIRES,
    pub IMTConfirm_TickBid: unsafe extern "C" fn(this: *const IMTConfirm) -> f64,
    pub IMTConfirm_TickAsk1: unsafe extern "C" fn(this: *mut IMTConfirm, tickask: f64) -> MTAPIRES,
    pub IMTConfirm_TickAsk: unsafe extern "C" fn(this: *const IMTConfirm) -> f64,
    pub IMTConfirm_TickLast1:
        unsafe extern "C" fn(this: *mut IMTConfirm, ticklast: f64) -> MTAPIRES,
    pub IMTConfirm_TickLast: unsafe extern "C" fn(this: *const IMTConfirm) -> f64,
    pub IMTConfirm_Comment1:
        unsafe extern "C" fn(this: *mut IMTConfirm, comment: LPCWSTR) -> MTAPIRES,
    pub IMTConfirm_Comment: unsafe extern "C" fn(this: *const IMTConfirm) -> LPCWSTR,
    pub IMTConfirm_Flags1: unsafe extern "C" fn(this: *mut IMTConfirm, flags: UINT) -> MTAPIRES,
    pub IMTConfirm_Flags: unsafe extern "C" fn(this: *const IMTConfirm) -> UINT,
    pub IMTConfirm_DealID1:
        unsafe extern "C" fn(this: *mut IMTConfirm, deal_id: LPCWSTR) -> MTAPIRES,
    pub IMTConfirm_DealID: unsafe extern "C" fn(this: *const IMTConfirm) -> LPCWSTR,
    pub IMTConfirm_OrderID1:
        unsafe extern "C" fn(this: *mut IMTConfirm, order_id: LPCWSTR) -> MTAPIRES,
    pub IMTConfirm_OrderID: unsafe extern "C" fn(this: *const IMTConfirm) -> LPCWSTR,
    pub IMTConfirm_PriceGateway1:
        unsafe extern "C" fn(this: *mut IMTConfirm, price: f64) -> MTAPIRES,
    pub IMTConfirm_PriceGateway: unsafe extern "C" fn(this: *const IMTConfirm) -> f64,
    pub IMTConfirm_PositionExternalID1:
        unsafe extern "C" fn(this: *mut IMTConfirm, id: LPCWSTR) -> MTAPIRES,
    pub IMTConfirm_PositionExternalID: unsafe extern "C" fn(this: *const IMTConfirm) -> LPCWSTR,
    pub IMTConfirm_ExternalRetcode1:
        unsafe extern "C" fn(this: *mut IMTConfirm, retcode: ::std::os::raw::c_int) -> MTAPIRES,
    pub IMTConfirm_ExternalRetcode:
        unsafe extern "C" fn(this: *const IMTConfirm) -> ::std::os::raw::c_int,
    pub IMTConfirm_VolumeExt1:
        unsafe extern "C" fn(this: *mut IMTConfirm, volume: UINT64) -> MTAPIRES,
    pub IMTConfirm_VolumeExt: unsafe extern "C" fn(this: *const IMTConfirm) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConfirm {
    pub vtable_: *const IMTConfirm__bindgen_vtable,
}
pub const IMTConfirm_EnConfirmFlags_CONFIRM_FLAG_NONE: IMTConfirm_EnConfirmFlags = 0;
pub const IMTConfirm_EnConfirmFlags_CONFIRM_FLAG_TICK: IMTConfirm_EnConfirmFlags = 1;
pub const IMTConfirm_EnConfirmFlags_CONFIRM_FLAG_ALL: IMTConfirm_EnConfirmFlags = 1;
pub type IMTConfirm_EnConfirmFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConfirm"][::std::mem::size_of::<IMTConfirm>() - 8usize];
    ["Alignment of IMTConfirm"][::std::mem::align_of::<IMTConfirm>() - 8usize];
};
#[repr(C)]
pub struct IMTExecution__bindgen_vtable {
    pub IMTExecution_Release: unsafe extern "C" fn(this: *mut IMTExecution),
    pub IMTExecution_Assign:
        unsafe extern "C" fn(this: *mut IMTExecution, exec: *const IMTExecution) -> MTAPIRES,
    pub IMTExecution_Clear: unsafe extern "C" fn(this: *mut IMTExecution) -> MTAPIRES,
    pub IMTExecution_Print:
        unsafe extern "C" fn(this: *const IMTExecution, string: *mut MTAPISTR) -> LPCWSTR,
    pub IMTExecution_ID1: unsafe extern "C" fn(this: *mut IMTExecution, id: UINT64) -> MTAPIRES,
    pub IMTExecution_ID: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_ExternalID1:
        unsafe extern "C" fn(this: *mut IMTExecution, id: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_ExternalID: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_Action1:
        unsafe extern "C" fn(this: *mut IMTExecution, action: UINT) -> MTAPIRES,
    pub IMTExecution_Action: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_Datetime1:
        unsafe extern "C" fn(this: *mut IMTExecution, datetime: INT64) -> MTAPIRES,
    pub IMTExecution_Datetime: unsafe extern "C" fn(this: *const IMTExecution) -> INT64,
    pub IMTExecution_Login1:
        unsafe extern "C" fn(this: *mut IMTExecution, login: UINT64) -> MTAPIRES,
    pub IMTExecution_Login: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_Group1:
        unsafe extern "C" fn(this: *mut IMTExecution, group: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_Group: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_Flags1:
        unsafe extern "C" fn(this: *mut IMTExecution, flags: UINT64) -> MTAPIRES,
    pub IMTExecution_Flags: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_Symbol1:
        unsafe extern "C" fn(this: *mut IMTExecution, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_Symbol: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_Digits1:
        unsafe extern "C" fn(this: *mut IMTExecution, digits: UINT) -> MTAPIRES,
    pub IMTExecution_Digits: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_Comment1:
        unsafe extern "C" fn(this: *mut IMTExecution, comment: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_Comment: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_Order1:
        unsafe extern "C" fn(this: *mut IMTExecution, order: UINT64) -> MTAPIRES,
    pub IMTExecution_Order: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_OrderExternalID1:
        unsafe extern "C" fn(this: *mut IMTExecution, id: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_OrderExternalID: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_OrderType1:
        unsafe extern "C" fn(this: *mut IMTExecution, type_: UINT) -> MTAPIRES,
    pub IMTExecution_OrderType: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_OrderVolume1:
        unsafe extern "C" fn(this: *mut IMTExecution, volume: UINT64) -> MTAPIRES,
    pub IMTExecution_OrderVolume: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_OrderPrice1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_OrderPrice: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_OrderActivationFlags1:
        unsafe extern "C" fn(this: *mut IMTExecution, activation: UINT) -> MTAPIRES,
    pub IMTExecution_OrderActivationFlags: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_DealExternalID1:
        unsafe extern "C" fn(this: *mut IMTExecution, id: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_DealExternalID: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_DealAction1:
        unsafe extern "C" fn(this: *mut IMTExecution, action: UINT) -> MTAPIRES,
    pub IMTExecution_DealAction: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_DealVolume1:
        unsafe extern "C" fn(this: *mut IMTExecution, volume: UINT64) -> MTAPIRES,
    pub IMTExecution_DealVolume: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_DealVolumeRemaind1:
        unsafe extern "C" fn(this: *mut IMTExecution, volume: UINT64) -> MTAPIRES,
    pub IMTExecution_DealVolumeRemaind: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_DealPrice1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_DealPrice: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_ExternalAccount1:
        unsafe extern "C" fn(this: *mut IMTExecution, account: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_ExternalAccount: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_OrderPriceTrigger1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_OrderPriceTrigger: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_OrderTypeTime1:
        unsafe extern "C" fn(this: *mut IMTExecution, type_: UINT) -> MTAPIRES,
    pub IMTExecution_OrderTypeTime: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_OrderTimeExpiration1:
        unsafe extern "C" fn(this: *mut IMTExecution, time: INT64) -> MTAPIRES,
    pub IMTExecution_OrderTimeExpiration: unsafe extern "C" fn(this: *const IMTExecution) -> INT64,
    pub IMTExecution_OrderTypeFill1:
        unsafe extern "C" fn(this: *mut IMTExecution, type_: UINT) -> MTAPIRES,
    pub IMTExecution_OrderTypeFill: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_EOSSessionStart1:
        unsafe extern "C" fn(this: *mut IMTExecution, start: INT64) -> MTAPIRES,
    pub IMTExecution_EOSSessionStart: unsafe extern "C" fn(this: *const IMTExecution) -> INT64,
    pub IMTExecution_EOSSessionEnd1:
        unsafe extern "C" fn(this: *mut IMTExecution, end: INT64) -> MTAPIRES,
    pub IMTExecution_EOSSessionEnd: unsafe extern "C" fn(this: *const IMTExecution) -> INT64,
    pub IMTExecution_EOSPriceSettlement1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_EOSPriceSettlement: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_EOSProfitRateBuy: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_EOSProfitRateSell: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_EOSProfitRate:
        unsafe extern "C" fn(this: *mut IMTExecution, rate_buy: f64, rate_sell: f64) -> MTAPIRES,
    pub IMTExecution_EOSTickValue1:
        unsafe extern "C" fn(this: *mut IMTExecution, value: f64) -> MTAPIRES,
    pub IMTExecution_EOSTickValue: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_OrderPriceSL1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_OrderPriceSL: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_OrderPriceTP1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_OrderPriceTP: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_PriceGateway1:
        unsafe extern "C" fn(this: *mut IMTExecution, price: f64) -> MTAPIRES,
    pub IMTExecution_PriceGateway: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_OrderActivationMode1:
        unsafe extern "C" fn(this: *mut IMTExecution, activation: UINT) -> MTAPIRES,
    pub IMTExecution_OrderActivationMode: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_DealCommission1:
        unsafe extern "C" fn(this: *mut IMTExecution, comm: f64) -> MTAPIRES,
    pub IMTExecution_DealCommission: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_DatetimeMsc1:
        unsafe extern "C" fn(this: *mut IMTExecution, datetime: INT64) -> MTAPIRES,
    pub IMTExecution_DatetimeMsc: unsafe extern "C" fn(this: *const IMTExecution) -> INT64,
    pub IMTExecution_SymbolNew1:
        unsafe extern "C" fn(this: *mut IMTExecution, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_SymbolNew: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_ApiDataUpdate2: unsafe extern "C" fn(
        this: *mut IMTExecution,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataUpdate1: unsafe extern "C" fn(
        this: *mut IMTExecution,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataUpdate: unsafe extern "C" fn(
        this: *mut IMTExecution,
        pos: UINT,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataNext2: unsafe extern "C" fn(
        this: *const IMTExecution,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataNext1: unsafe extern "C" fn(
        this: *const IMTExecution,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataNext: unsafe extern "C" fn(
        this: *const IMTExecution,
        pos: UINT,
        app_id: *mut USHORT,
        id: *mut UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTExecution_DealStorage1:
        unsafe extern "C" fn(this: *mut IMTExecution, storage: f64) -> MTAPIRES,
    pub IMTExecution_DealStorage: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_EOSRolloverValueLong: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_EOSRolloverValueShort: unsafe extern "C" fn(this: *const IMTExecution) -> f64,
    pub IMTExecution_EOSRolloverValue: unsafe extern "C" fn(
        this: *mut IMTExecution,
        value_long: f64,
        value_short: f64,
    ) -> MTAPIRES,
    pub IMTExecution_DealReason1:
        unsafe extern "C" fn(this: *mut IMTExecution, reason: UINT) -> MTAPIRES,
    pub IMTExecution_DealReason: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_GatewayID1:
        unsafe extern "C" fn(this: *mut IMTExecution, gateway_id: UINT64) -> MTAPIRES,
    pub IMTExecution_GatewayID: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_Position1:
        unsafe extern "C" fn(this: *mut IMTExecution, position: UINT64) -> MTAPIRES,
    pub IMTExecution_Position: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_PositionBy1:
        unsafe extern "C" fn(this: *mut IMTExecution, position: UINT64) -> MTAPIRES,
    pub IMTExecution_PositionBy: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_PositionExternalID1:
        unsafe extern "C" fn(this: *mut IMTExecution, id: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_PositionExternalID: unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_PositionByExternalID1:
        unsafe extern "C" fn(this: *mut IMTExecution, id: LPCWSTR) -> MTAPIRES,
    pub IMTExecution_PositionByExternalID:
        unsafe extern "C" fn(this: *const IMTExecution) -> LPCWSTR,
    pub IMTExecution_ExternalRetcode:
        unsafe extern "C" fn(this: *const IMTExecution) -> ::std::os::raw::c_int,
    pub IMTExecution_ExternalRetcode1:
        unsafe extern "C" fn(this: *mut IMTExecution, retcode: ::std::os::raw::c_int) -> MTAPIRES,
    pub IMTExecution_OrderVolumeExt1:
        unsafe extern "C" fn(this: *mut IMTExecution, volume: UINT64) -> MTAPIRES,
    pub IMTExecution_OrderVolumeExt: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_DealVolumeExt1:
        unsafe extern "C" fn(this: *mut IMTExecution, volume: UINT64) -> MTAPIRES,
    pub IMTExecution_DealVolumeExt: unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_DealVolumeRemaindExt1:
        unsafe extern "C" fn(this: *mut IMTExecution, volume: UINT64) -> MTAPIRES,
    pub IMTExecution_DealVolumeRemaindExt:
        unsafe extern "C" fn(this: *const IMTExecution) -> UINT64,
    pub IMTExecution_ApiDataSet2: unsafe extern "C" fn(
        this: *mut IMTExecution,
        app_id: USHORT,
        id: UCHAR,
        value: f64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataSet1: unsafe extern "C" fn(
        this: *mut IMTExecution,
        app_id: USHORT,
        id: UCHAR,
        value: UINT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataSet: unsafe extern "C" fn(
        this: *mut IMTExecution,
        app_id: USHORT,
        id: UCHAR,
        value: INT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataGet2: unsafe extern "C" fn(
        this: *const IMTExecution,
        app_id: USHORT,
        id: UCHAR,
        value: *mut f64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataGet1: unsafe extern "C" fn(
        this: *const IMTExecution,
        app_id: USHORT,
        id: UCHAR,
        value: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataGet: unsafe extern "C" fn(
        this: *const IMTExecution,
        app_id: USHORT,
        id: UCHAR,
        value: *mut INT64,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataRawSet: unsafe extern "C" fn(
        this: *mut IMTExecution,
        data: *const ::std::os::raw::c_void,
        datalen: UINT,
    ) -> MTAPIRES,
    pub IMTExecution_ApiDataRawGet: unsafe extern "C" fn(this: *const IMTExecution) -> LPCVOID,
    pub IMTExecution_ApiDataRawMax: unsafe extern "C" fn(this: *const IMTExecution) -> UINT,
    pub IMTExecution_ApiDataClear:
        unsafe extern "C" fn(this: *mut IMTExecution, app_id: USHORT) -> MTAPIRES,
    pub IMTExecution_ApiDataClearAll: unsafe extern "C" fn(this: *mut IMTExecution) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTExecution {
    pub vtable_: *const IMTExecution__bindgen_vtable,
}
pub const IMTExecution_EnTradeExecutions_TE_ORDER_FIRST: IMTExecution_EnTradeExecutions = 0;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_NEW_REQUEST: IMTExecution_EnTradeExecutions = 0;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_NEW: IMTExecution_EnTradeExecutions = 1;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_FILL: IMTExecution_EnTradeExecutions = 2;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_REJECT: IMTExecution_EnTradeExecutions = 3;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_MODIFY_REQUEST: IMTExecution_EnTradeExecutions =
    4;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_MODIFY: IMTExecution_EnTradeExecutions = 5;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_MODIFY_REJECT: IMTExecution_EnTradeExecutions = 6;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_CANCEL_REQUEST: IMTExecution_EnTradeExecutions =
    7;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_CANCEL: IMTExecution_EnTradeExecutions = 8;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_CANCEL_REJECT: IMTExecution_EnTradeExecutions = 9;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_CHANGE_ID: IMTExecution_EnTradeExecutions = 10;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_CLOSE_BY: IMTExecution_EnTradeExecutions = 11;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_EXPIRE: IMTExecution_EnTradeExecutions = 12;
pub const IMTExecution_EnTradeExecutions_TE_ORDER_LAST: IMTExecution_EnTradeExecutions = 12;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_FIRST: IMTExecution_EnTradeExecutions = 50;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_CANCEL: IMTExecution_EnTradeExecutions = 50;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_CORRECT: IMTExecution_EnTradeExecutions = 51;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_EXTERNAL: IMTExecution_EnTradeExecutions = 52;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_REPO: IMTExecution_EnTradeExecutions = 53;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_COMMISSION: IMTExecution_EnTradeExecutions = 54;
pub const IMTExecution_EnTradeExecutions_TE_DEAL_LAST: IMTExecution_EnTradeExecutions = 54;
pub const IMTExecution_EnTradeExecutions_TE_EOS_FIRST: IMTExecution_EnTradeExecutions = 100;
pub const IMTExecution_EnTradeExecutions_TE_EOS_CANCEL_DAILY_ORDERS:
    IMTExecution_EnTradeExecutions = 100;
pub const IMTExecution_EnTradeExecutions_TE_EOS_VARIATION_MARGIN: IMTExecution_EnTradeExecutions =
    101;
pub const IMTExecution_EnTradeExecutions_TE_EOS_RECALC_DEALS: IMTExecution_EnTradeExecutions = 102;
pub const IMTExecution_EnTradeExecutions_TE_EOS_SETTLEMENT: IMTExecution_EnTradeExecutions = 103;
pub const IMTExecution_EnTradeExecutions_TE_EOS_TRANSFER: IMTExecution_EnTradeExecutions = 104;
pub const IMTExecution_EnTradeExecutions_TE_EOS_CANCEL_ALL_ORDERS: IMTExecution_EnTradeExecutions =
    105;
pub const IMTExecution_EnTradeExecutions_TE_EOS_ROLLOVER: IMTExecution_EnTradeExecutions = 106;
pub const IMTExecution_EnTradeExecutions_TE_EOS_LAST: IMTExecution_EnTradeExecutions = 106;
pub const IMTExecution_EnTradeExecutions_TE_BALANCE_FIRST: IMTExecution_EnTradeExecutions = 150;
pub const IMTExecution_EnTradeExecutions_TE_BALANCE_CHANGE: IMTExecution_EnTradeExecutions = 150;
pub const IMTExecution_EnTradeExecutions_TE_BALANCE_CORRECT: IMTExecution_EnTradeExecutions = 151;
pub const IMTExecution_EnTradeExecutions_TE_BALANCE_UNBLOCK_PROFIT: IMTExecution_EnTradeExecutions =
    152;
pub const IMTExecution_EnTradeExecutions_TE_BALANCE_LAST: IMTExecution_EnTradeExecutions = 152;
pub const IMTExecution_EnTradeExecutions_TE_POSITION_FIRST: IMTExecution_EnTradeExecutions = 200;
pub const IMTExecution_EnTradeExecutions_TE_POSITION_CHANGE_ID: IMTExecution_EnTradeExecutions =
    201;
pub const IMTExecution_EnTradeExecutions_TE_POSITION_LAST: IMTExecution_EnTradeExecutions = 201;
pub const IMTExecution_EnTradeExecutions_TE_FIRST: IMTExecution_EnTradeExecutions = 0;
pub const IMTExecution_EnTradeExecutions_TE_LAST: IMTExecution_EnTradeExecutions = 201;
pub type IMTExecution_EnTradeExecutions = ::std::os::raw::c_int;
pub const IMTExecution_EnFlags_TE_FLAG_NONE: IMTExecution_EnFlags = 0;
pub const IMTExecution_EnFlags_TE_FLAG_BROADCAST: IMTExecution_EnFlags = 1;
pub const IMTExecution_EnFlags_TE_FLAG_MARKUP: IMTExecution_EnFlags = 2;
pub const IMTExecution_EnFlags_TE_FLAG_SETTLEMENT_COMMISSION: IMTExecution_EnFlags = 4;
pub const IMTExecution_EnFlags_TE_FLAG_ALL: IMTExecution_EnFlags = 7;
pub type IMTExecution_EnFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTExecution"][::std::mem::size_of::<IMTExecution>() - 8usize];
    ["Alignment of IMTExecution"][::std::mem::align_of::<IMTExecution>() - 8usize];
};
#[repr(C)]
pub struct IMTTradeSink__bindgen_vtable {
    //--- trade request event
    pub IMTTradeSink_OnTradeRequestAdd: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder
    ),

    //--- trade request process event
    pub IMTTradeSink_OnTradeRequestUpdate: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest
    ),

    //--- trade request process event
    pub IMTTradeSink_OnTradeRequestDelete: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest
    ),

    //--- trade request process event
    pub IMTTradeSink_OnTradeRequestProcess: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        confirm: *const IMTConfirm,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder,
        deal: *const IMTDeal,
    ),

    //--- trade request add hook
    pub IMTTradeSink_HookTradeRequestAdd: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder,
        order_new: *mut IMTDeal,
    ) -> MTAPIRES,

    //--- trade request route hook
    pub IMTTradeSink_HookTradeRequestRoute: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        confirm: *const IMTConfirm,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder
    ) -> MTAPIRES,

    //--- trade request deal hook
    pub IMTTradeSink_HookTradeRequestProcess: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        confirm: *const IMTConfirm,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder,
        deal: *mut IMTDeal
    ) -> MTAPIRES,

    //--- rollover calculation hook
    pub IMTTradeSink_HookTradeRollover: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        datetime: INT64,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        original_value: f64,
        new_value: *mut f64
    ) -> MTAPIRES,

    //--- interest calculation hook
    pub IMTTradeSink_HookTradeInterest: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        datetime: INT64,
        group: *const IMTConGroup,
        account: *const IMTAccount,
        original_value: f64,
        new_value: *mut f64
    ) -> MTAPIRES,

    //--- interest charge hook
    pub IMTTradeSink_HookTradeInterestCharge: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        datetime: INT64,
        group: *const IMTConGroup,
        user: *const IMTUser,
        original_value: f64,
        new_value: *mut f64
    ) -> MTAPIRES,

    //--- order commission calculation
    pub IMTTradeSink_HookTradeCommissionOrder: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        commission: *const IMTConCommission,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        order: *const IMTOrder,
        original_value: f64,
        new_value: *mut f64
    ) -> MTAPIRES,

    //--- final commission calculation
    pub IMTTradeSink_HookTradeCommissionCharge: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        period_start: INT64,
        period_end: INT64,
        commission: *const IMTConCommission,
        group: *const IMTConGroup,
        user: *const IMTUser,
        original_value: f64,
        new_value: *mut f64
    ) -> MTAPIRES,

    //--- order commission calculation
    pub IMTTradeSink_HookTradeCommissionDeal: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        commission: *const IMTConCommission,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        deal: *const IMTDeal,
        original_value: f64,
        new_value: *mut f64
    ) -> MTAPIRES,

    //--- trade execution event
    pub IMTTradeSink_OnTradeExecution: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        gateway: *const IMTConGateway,
        execution: *const IMTExecution,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        deal: *const IMTDeal
    ),

    //--- trade execution hook
    pub IMTTradeSink_HookTradeExecution: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        gateway: *const IMTConGateway,
        execution: *const IMTExecution,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *mut IMTPosition,
        order: *mut IMTOrder,
        deal: *mut IMTDeal
    ) -> MTAPIRES,

    //--- trade request refused on pre-trade control event
    pub IMTTradeSink_OnTradeRequestRefuse: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest
    ),

    //--- trade request process event
    pub IMTTradeSink_OnTradeRequestProcessCloseBy: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        confirm: *const IMTConfirm,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder,
        deal: *const IMTDeal,
        deal_by: *const IMTDeal
    ),

    //--- trade request deal hook
    pub IMTTradeSink_HookTradeRequestProcessCloseBy: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        confirm: *const IMTConfirm,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *mut IMTPosition,
        order: *mut IMTOrder,
        deal: *mut IMTDeal,
        deal_by: *mut IMTDeal
    ) -> MTAPIRES,

    //--- interest charge hook
    pub IMTTradeSink_HookTradeInterestChargeDeal: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        datetime: INT64,
        group: *const IMTConGroup,
        user: *const IMTUser,
        deal: *mut IMTDeal
    ) -> MTAPIRES,

    //--- trade request route hook
    pub IMTTradeSink_HookTradeRequestRuleFilter: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *mut IMTRequest,
        rule: *mut IMTConRoute,
        group: *const IMTConGroup
    ) -> MTAPIRES,

    //--- trade request route hook
    pub IMTTradeSink_HookTradeRequestRuleApply: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        request: *mut IMTRequest,
        rule: *mut IMTConRoute,
        group: *const IMTConGroup
    ) -> MTAPIRES,

    //--- trade execution event for close by
    pub IMTTradeSink_OnTradeExecutionCloseBy: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        execution: *const IMTExecution,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *mut IMTPosition,
        order: *mut IMTOrder,
        deal: *mut IMTDeal,
        deal_by: *mut IMTDeal
    ),

    //--- trade execution hook for close by
    pub IMTTradeSink_HookTradeExecutionCloseBy: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        execution: *const IMTExecution,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *mut IMTPosition,
        order: *mut IMTOrder,
        deal: *mut IMTDeal,
        deal_by: *mut IMTDeal
    ) -> MTAPIRES,

    //--- position split event
    pub IMTTradeSink_OnTradeSplit: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        shares_new: UINT,
        shares_old: UINT,
        round_prices: UINT,
        round_volumes: UINT,
        flags: UINT,
        adjustment: f64,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position_old: *const IMTPosition,
        position_new: *const IMTPosition,
    ),

    pub IMTTradeSink_HookTradeSplit: unsafe extern "C" fn(
        this: *mut IMTTradeSink,
        shares_new: UINT,
        shares_old: UINT,
        round_prices: UINT,
        round_volumes: UINT,
        flags: UINT,
        adjustment: f64,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position_old: *const IMTPosition,
        position_new: *const IMTPosition,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTTradeSink {
    pub vtable_: *const IMTTradeSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTTradeSink"][::std::mem::size_of::<IMTTradeSink>() - 8usize];
    ["Alignment of IMTTradeSink"][::std::mem::align_of::<IMTTradeSink>() - 8usize];
};
#[repr(C)]
pub struct IMTEndOfDaySink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTEndOfDaySink {
    pub vtable_: *const IMTEndOfDaySink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTEndOfDaySink"][::std::mem::size_of::<IMTEndOfDaySink>() - 8usize];
    ["Alignment of IMTEndOfDaySink"][::std::mem::align_of::<IMTEndOfDaySink>() - 8usize];
};
#[repr(C)]
pub struct IMTRequestArray__bindgen_vtable {
    pub IMTRequestArray_Release: unsafe extern "C" fn(this: *mut IMTRequestArray),
    pub IMTRequestArray_Assign:
        unsafe extern "C" fn(this: *mut IMTRequestArray, array: *const IMTRequestArray) -> MTAPIRES,
    pub IMTRequestArray_Clear: unsafe extern "C" fn(this: *mut IMTRequestArray) -> MTAPIRES,
    pub IMTRequestArray_Add1:
        unsafe extern "C" fn(this: *mut IMTRequestArray, array: *mut IMTRequestArray) -> MTAPIRES,
    pub IMTRequestArray_Add:
        unsafe extern "C" fn(this: *mut IMTRequestArray, request: *mut IMTRequest) -> MTAPIRES,
    pub IMTRequestArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTRequestArray, array: *const IMTRequestArray) -> MTAPIRES,
    pub IMTRequestArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTRequestArray, request: *const IMTRequest) -> MTAPIRES,
    pub IMTRequestArray_Delete:
        unsafe extern "C" fn(this: *mut IMTRequestArray, pos: UINT) -> MTAPIRES,
    pub IMTRequestArray_Detach:
        unsafe extern "C" fn(this: *mut IMTRequestArray, pos: UINT) -> *mut IMTRequest,
    pub IMTRequestArray_Update: unsafe extern "C" fn(
        this: *mut IMTRequestArray,
        pos: UINT,
        request: *mut IMTRequest,
    ) -> MTAPIRES,
    pub IMTRequestArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTRequestArray,
        pos: UINT,
        request: *const IMTRequest,
    ) -> MTAPIRES,
    pub IMTRequestArray_Shift: unsafe extern "C" fn(
        this: *mut IMTRequestArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTRequestArray_Total: unsafe extern "C" fn(this: *const IMTRequestArray) -> UINT,
    pub IMTRequestArray_Next:
        unsafe extern "C" fn(this: *const IMTRequestArray, index: UINT) -> *mut IMTRequest,
    pub IMTRequestArray_Sort: unsafe extern "C" fn(
        this: *mut IMTRequestArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTRequestArray_Search: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTRequestArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTRequestArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTRequestArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTRequestArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTRequestArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTRequestArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTRequestArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTRequestArray {
    pub vtable_: *const IMTRequestArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTRequestArray"][::std::mem::size_of::<IMTRequestArray>() - 8usize];
    ["Alignment of IMTRequestArray"][::std::mem::align_of::<IMTRequestArray>() - 8usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTBookItem {
    pub type_: UINT,
    pub price: f64,
    pub volume: INT64,
    pub volume_ext: INT64,
    pub reserved: [UINT; 6usize],
}
pub const MTBookItem_EnBookItem_ItemReset: MTBookItem_EnBookItem = 0;
pub const MTBookItem_EnBookItem_ItemSell: MTBookItem_EnBookItem = 1;
pub const MTBookItem_EnBookItem_ItemBuy: MTBookItem_EnBookItem = 2;
pub const MTBookItem_EnBookItem_ItemSellMarket: MTBookItem_EnBookItem = 3;
pub const MTBookItem_EnBookItem_ItemBuyMarket: MTBookItem_EnBookItem = 4;
pub type MTBookItem_EnBookItem = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTBookItem"][::std::mem::size_of::<MTBookItem>() - 52usize];
    ["Alignment of MTBookItem"][::std::mem::align_of::<MTBookItem>() - 1usize];
    ["Offset of field: MTBookItem::type_"][::std::mem::offset_of!(MTBookItem, type_) - 0usize];
    ["Offset of field: MTBookItem::price"][::std::mem::offset_of!(MTBookItem, price) - 4usize];
    ["Offset of field: MTBookItem::volume"][::std::mem::offset_of!(MTBookItem, volume) - 12usize];
    ["Offset of field: MTBookItem::volume_ext"]
        [::std::mem::offset_of!(MTBookItem, volume_ext) - 20usize];
    ["Offset of field: MTBookItem::reserved"]
        [::std::mem::offset_of!(MTBookItem, reserved) - 28usize];
};
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTBook {
    pub symbol: [u16; 32usize],
    pub items: [MTBookItem; 128usize],
    pub items_total: UINT,
    pub flags: UINT64,
    pub datetime: INT64,
    pub datetime_msc: INT64,
    pub reserved: [UINT; 58usize],
}
pub const MTBook_EnBookFlags_FLAG_PRE_AUCTION: MTBook_EnBookFlags = 1;
pub const MTBook_EnBookFlags_FLAG_SNAPSHOT: MTBook_EnBookFlags = 2;
pub const MTBook_EnBookFlags_FLAG_NONE: MTBook_EnBookFlags = 0;
pub const MTBook_EnBookFlags_FLAG_ALL: MTBook_EnBookFlags = 3;
pub type MTBook_EnBookFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTBook"][::std::mem::size_of::<MTBook>() - 6980usize];
    ["Alignment of MTBook"][::std::mem::align_of::<MTBook>() - 1usize];
    ["Offset of field: MTBook::symbol"][::std::mem::offset_of!(MTBook, symbol) - 0usize];
    ["Offset of field: MTBook::items"][::std::mem::offset_of!(MTBook, items) - 64usize];
    ["Offset of field: MTBook::items_total"]
        [::std::mem::offset_of!(MTBook, items_total) - 6720usize];
    ["Offset of field: MTBook::flags"][::std::mem::offset_of!(MTBook, flags) - 6724usize];
    ["Offset of field: MTBook::datetime"][::std::mem::offset_of!(MTBook, datetime) - 6732usize];
    ["Offset of field: MTBook::datetime_msc"]
        [::std::mem::offset_of!(MTBook, datetime_msc) - 6740usize];
    ["Offset of field: MTBook::reserved"][::std::mem::offset_of!(MTBook, reserved) - 6748usize];
};
pub type MTBookDiff = MTBook;
#[repr(C)]
pub struct IMTBookSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTBookSink {
    pub vtable_: *const IMTBookSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTBookSink"][::std::mem::size_of::<IMTBookSink>() - 8usize];
    ["Alignment of IMTBookSink"][::std::mem::align_of::<IMTBookSink>() - 8usize];
};
pub const EnSplitRounding_SPLIT_ROUNDING_STANDARD: EnSplitRounding = 0;
pub const EnSplitRounding_SPLIT_ROUNDING_DOWN: EnSplitRounding = 1;
pub const EnSplitRounding_SPLIT_ROUNDING_UP: EnSplitRounding = 2;
pub const EnSplitRounding_SPLIT_ROUNDING_FIRST: EnSplitRounding = 0;
pub const EnSplitRounding_SPLIT_ROUNDING_LAST: EnSplitRounding = 2;
pub type EnSplitRounding = ::std::os::raw::c_int;
#[repr(C, packed)]
#[derive(Debug, Copy, Clone)]
pub struct MTChartBar {
    pub datetime: INT64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub tick_volume: UINT64,
    pub spread: INT32,
    pub volume: UINT64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of MTChartBar"][::std::mem::size_of::<MTChartBar>() - 60usize];
    ["Alignment of MTChartBar"][::std::mem::align_of::<MTChartBar>() - 1usize];
    ["Offset of field: MTChartBar::datetime"]
        [::std::mem::offset_of!(MTChartBar, datetime) - 0usize];
    ["Offset of field: MTChartBar::open"][::std::mem::offset_of!(MTChartBar, open) - 8usize];
    ["Offset of field: MTChartBar::high"][::std::mem::offset_of!(MTChartBar, high) - 16usize];
    ["Offset of field: MTChartBar::low"][::std::mem::offset_of!(MTChartBar, low) - 24usize];
    ["Offset of field: MTChartBar::close"][::std::mem::offset_of!(MTChartBar, close) - 32usize];
    ["Offset of field: MTChartBar::tick_volume"]
        [::std::mem::offset_of!(MTChartBar, tick_volume) - 40usize];
    ["Offset of field: MTChartBar::spread"][::std::mem::offset_of!(MTChartBar, spread) - 48usize];
    ["Offset of field: MTChartBar::volume"][::std::mem::offset_of!(MTChartBar, volume) - 52usize];
};
#[repr(C)]
pub struct IMTCertificate__bindgen_vtable {
    pub IMTCertificate_Release: unsafe extern "C" fn(this: *mut IMTCertificate),
    pub IMTCertificate_Assign: unsafe extern "C" fn(
        this: *mut IMTCertificate,
        certificate: *const IMTCertificate,
    ) -> MTAPIRES,
    pub IMTCertificate_Clear: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_Open:
        unsafe extern "C" fn(this: *mut IMTCertificate, filename: LPCWSTR) -> MTAPIRES,
    pub IMTCertificate_OpenMemory: unsafe extern "C" fn(
        this: *mut IMTCertificate,
        data: *const ::std::os::raw::c_void,
        size: UINT,
    ) -> MTAPIRES,
    pub IMTCertificate_Save:
        unsafe extern "C" fn(this: *const IMTCertificate, filename: LPCWSTR) -> MTAPIRES,
    pub IMTCertificate_Close: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_Raw: unsafe extern "C" fn(this: *const IMTCertificate) -> LPCVOID,
    pub IMTCertificate_RawSize: unsafe extern "C" fn(this: *const IMTCertificate) -> UINT,
    pub IMTCertificate_CommonReserved1: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_CommonReserved2: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_CommonReserved3: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_CommonReserved4: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_CommonReserved5: unsafe extern "C" fn(this: *mut IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_IsOpened: unsafe extern "C" fn(this: *const IMTCertificate) -> bool,
    pub IMTCertificate_IsRoot: unsafe extern "C" fn(this: *const IMTCertificate) -> bool,
    pub IMTCertificate_IsCA: unsafe extern "C" fn(this: *const IMTCertificate) -> bool,
    pub IMTCertificate_IsEqual:
        unsafe extern "C" fn(this: *mut IMTCertificate, certificate: *const IMTCertificate) -> bool,
    pub IMTCertificate_IsReserved1: unsafe extern "C" fn(this: *mut IMTCertificate) -> bool,
    pub IMTCertificate_IsReserved2: unsafe extern "C" fn(this: *mut IMTCertificate) -> bool,
    pub IMTCertificate_IsReserved3: unsafe extern "C" fn(this: *mut IMTCertificate) -> bool,
    pub IMTCertificate_IsReserved4: unsafe extern "C" fn(this: *mut IMTCertificate) -> bool,
    pub IMTCertificate_IsReserved5: unsafe extern "C" fn(this: *mut IMTCertificate) -> bool,
    pub IMTCertificate_SerialNumber: unsafe extern "C" fn(this: *const IMTCertificate) -> UINT64,
    pub IMTCertificate_ValidFrom: unsafe extern "C" fn(this: *const IMTCertificate) -> INT64,
    pub IMTCertificate_ValidTo: unsafe extern "C" fn(this: *const IMTCertificate) -> INT64,
    pub IMTCertificate_NameCommon:
        unsafe extern "C" fn(this: *const IMTCertificate, name: *mut MTAPISTR) -> MTAPIRES,
    pub IMTCertificate_NameIssuer:
        unsafe extern "C" fn(this: *const IMTCertificate, name: *mut MTAPISTR) -> MTAPIRES,
    pub IMTCertificate_NameOrganization:
        unsafe extern "C" fn(this: *const IMTCertificate, name: *mut MTAPISTR) -> MTAPIRES,
    pub IMTCertificate_NameOrganizationUnit:
        unsafe extern "C" fn(this: *const IMTCertificate, name: *mut MTAPISTR) -> MTAPIRES,
    pub IMTCertificate_NameGiven:
        unsafe extern "C" fn(this: *const IMTCertificate, name: *mut MTAPISTR) -> MTAPIRES,
    pub IMTCertificate_NameReserved1: unsafe extern "C" fn(this: *const IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_NameReserved2: unsafe extern "C" fn(this: *const IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_NameReserved3: unsafe extern "C" fn(this: *const IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_NameReserved4: unsafe extern "C" fn(this: *const IMTCertificate) -> MTAPIRES,
    pub IMTCertificate_NameReserved5: unsafe extern "C" fn(this: *const IMTCertificate) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTCertificate {
    pub vtable_: *const IMTCertificate__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTCertificate"][::std::mem::size_of::<IMTCertificate>() - 8usize];
    ["Alignment of IMTCertificate"][::std::mem::align_of::<IMTCertificate>() - 8usize];
};
#[repr(C)]
pub struct IMTConSpreadLeg__bindgen_vtable {
    pub IMTConSpreadLeg_Release: unsafe extern "C" fn(this: *mut IMTConSpreadLeg),
    pub IMTConSpreadLeg_Assign:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, leg: *const IMTConSpreadLeg) -> MTAPIRES,
    pub IMTConSpreadLeg_Clear: unsafe extern "C" fn(this: *mut IMTConSpreadLeg) -> MTAPIRES,
    pub IMTConSpreadLeg_Mode1:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, mode: UINT) -> MTAPIRES,
    pub IMTConSpreadLeg_Mode: unsafe extern "C" fn(this: *const IMTConSpreadLeg) -> UINT,
    pub IMTConSpreadLeg_Symbol1:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTConSpreadLeg_Symbol: unsafe extern "C" fn(this: *const IMTConSpreadLeg) -> LPCWSTR,
    pub IMTConSpreadLeg_TimeFrom1:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, from: INT64) -> MTAPIRES,
    pub IMTConSpreadLeg_TimeFrom: unsafe extern "C" fn(this: *const IMTConSpreadLeg) -> INT64,
    pub IMTConSpreadLeg_TimeTo1:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, to: INT64) -> MTAPIRES,
    pub IMTConSpreadLeg_TimeTo: unsafe extern "C" fn(this: *const IMTConSpreadLeg) -> INT64,
    pub IMTConSpreadLeg_Ratio1:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, ratio: UINT64) -> MTAPIRES,
    pub IMTConSpreadLeg_Ratio: unsafe extern "C" fn(this: *const IMTConSpreadLeg) -> UINT64,
    pub IMTConSpreadLeg_RatioDbl1:
        unsafe extern "C" fn(this: *mut IMTConSpreadLeg, ratio: f64) -> MTAPIRES,
    pub IMTConSpreadLeg_RatioDbl: unsafe extern "C" fn(this: *const IMTConSpreadLeg) -> f64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSpreadLeg {
    pub vtable_: *const IMTConSpreadLeg__bindgen_vtable,
}
pub const IMTConSpreadLeg_EnLegMode_LEG_MODE_SYMBOL: IMTConSpreadLeg_EnLegMode = 0;
pub const IMTConSpreadLeg_EnLegMode_LEG_MODE_FUTURES: IMTConSpreadLeg_EnLegMode = 1;
pub const IMTConSpreadLeg_EnLegMode_LEG_MODE_FIRST: IMTConSpreadLeg_EnLegMode = 0;
pub const IMTConSpreadLeg_EnLegMode_LEG_MODE_LAST: IMTConSpreadLeg_EnLegMode = 1;
pub type IMTConSpreadLeg_EnLegMode = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSpreadLeg"][::std::mem::size_of::<IMTConSpreadLeg>() - 8usize];
    ["Alignment of IMTConSpreadLeg"][::std::mem::align_of::<IMTConSpreadLeg>() - 8usize];
};
#[repr(C)]
pub struct IMTConSpread__bindgen_vtable {
    pub IMTConSpread_Release: unsafe extern "C" fn(this: *mut IMTConSpread),
    pub IMTConSpread_Assign:
        unsafe extern "C" fn(this: *mut IMTConSpread, spread: *const IMTConSpread) -> MTAPIRES,
    pub IMTConSpread_Clear: unsafe extern "C" fn(this: *mut IMTConSpread) -> MTAPIRES,
    pub IMTConSpread_ID: unsafe extern "C" fn(this: *const IMTConSpread) -> UINT,
    pub IMTConSpread_MarginType1:
        unsafe extern "C" fn(this: *mut IMTConSpread, type_: UINT) -> MTAPIRES,
    pub IMTConSpread_MarginType: unsafe extern "C" fn(this: *const IMTConSpread) -> UINT,
    pub IMTConSpread_MarginInitial1:
        unsafe extern "C" fn(this: *mut IMTConSpread, margin: f64) -> MTAPIRES,
    pub IMTConSpread_MarginInitial: unsafe extern "C" fn(this: *const IMTConSpread) -> f64,
    pub IMTConSpread_MarginMaintenance1:
        unsafe extern "C" fn(this: *mut IMTConSpread, margin: f64) -> MTAPIRES,
    pub IMTConSpread_MarginMaintenance: unsafe extern "C" fn(this: *const IMTConSpread) -> f64,
    pub IMTConSpread_ALegAdd:
        unsafe extern "C" fn(this: *mut IMTConSpread, leg: *mut IMTConSpreadLeg) -> MTAPIRES,
    pub IMTConSpread_ALegUpdate: unsafe extern "C" fn(
        this: *mut IMTConSpread,
        pos: UINT,
        leg: *const IMTConSpreadLeg,
    ) -> MTAPIRES,
    pub IMTConSpread_ALegDelete:
        unsafe extern "C" fn(this: *mut IMTConSpread, pos: UINT) -> MTAPIRES,
    pub IMTConSpread_ALegClear: unsafe extern "C" fn(this: *mut IMTConSpread) -> MTAPIRES,
    pub IMTConSpread_ALegShift: unsafe extern "C" fn(
        this: *mut IMTConSpread,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSpread_ALegTotal: unsafe extern "C" fn(this: *const IMTConSpread) -> UINT,
    pub IMTConSpread_ALegNext: unsafe extern "C" fn(
        this: *const IMTConSpread,
        pos: UINT,
        leg: *mut IMTConSpreadLeg,
    ) -> MTAPIRES,
    pub IMTConSpread_BLegAdd:
        unsafe extern "C" fn(this: *mut IMTConSpread, leg: *mut IMTConSpreadLeg) -> MTAPIRES,
    pub IMTConSpread_BLegUpdate: unsafe extern "C" fn(
        this: *mut IMTConSpread,
        pos: UINT,
        leg: *const IMTConSpreadLeg,
    ) -> MTAPIRES,
    pub IMTConSpread_BLegDelete:
        unsafe extern "C" fn(this: *mut IMTConSpread, pos: UINT) -> MTAPIRES,
    pub IMTConSpread_BLegClear: unsafe extern "C" fn(this: *mut IMTConSpread) -> MTAPIRES,
    pub IMTConSpread_BLegShift: unsafe extern "C" fn(
        this: *mut IMTConSpread,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSpread_BLegTotal: unsafe extern "C" fn(this: *const IMTConSpread) -> UINT,
    pub IMTConSpread_BLegNext: unsafe extern "C" fn(
        this: *const IMTConSpread,
        pos: UINT,
        leg: *mut IMTConSpreadLeg,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSpread {
    pub vtable_: *const IMTConSpread__bindgen_vtable,
}
pub const IMTConSpread_EnSpreadMarginType_MARGIN_TYPE_VALUE: IMTConSpread_EnSpreadMarginType = 0;
pub const IMTConSpread_EnSpreadMarginType_MARGIN_TYPE_MAXIMAL: IMTConSpread_EnSpreadMarginType = 1;
pub const IMTConSpread_EnSpreadMarginType_MARGIN_TYPE_CME_INTER: IMTConSpread_EnSpreadMarginType =
    2;
pub const IMTConSpread_EnSpreadMarginType_MARGIN_TYPE_CME_INTRA: IMTConSpread_EnSpreadMarginType =
    3;
pub const IMTConSpread_EnSpreadMarginType_MARGIN_TYPE_FIRST: IMTConSpread_EnSpreadMarginType = 0;
pub const IMTConSpread_EnSpreadMarginType_MARGIN_TYPE_LAST: IMTConSpread_EnSpreadMarginType = 3;
pub type IMTConSpread_EnSpreadMarginType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSpread"][::std::mem::size_of::<IMTConSpread>() - 8usize];
    ["Alignment of IMTConSpread"][::std::mem::align_of::<IMTConSpread>() - 8usize];
};
#[repr(C)]
pub struct IMTConSpreadSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConSpreadSink {
    pub vtable_: *const IMTConSpreadSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSpreadSink"][::std::mem::size_of::<IMTConSpreadSink>() - 8usize];
    ["Alignment of IMTConSpreadSink"][::std::mem::align_of::<IMTConSpreadSink>() - 8usize];
};
#[repr(C)]
pub struct IMTOnlineArray__bindgen_vtable {
    pub IMTOnlineArray_Release: unsafe extern "C" fn(this: *mut IMTOnlineArray),
    pub IMTOnlineArray_Assign:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, array: *const IMTOnlineArray) -> MTAPIRES,
    pub IMTOnlineArray_Clear: unsafe extern "C" fn(this: *mut IMTOnlineArray) -> MTAPIRES,
    pub IMTOnlineArray_Add1:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, array: *mut IMTOnlineArray) -> MTAPIRES,
    pub IMTOnlineArray_Add:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, online: *mut IMTOnline) -> MTAPIRES,
    pub IMTOnlineArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, array: *const IMTOnlineArray) -> MTAPIRES,
    pub IMTOnlineArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, online: *const IMTOnline) -> MTAPIRES,
    pub IMTOnlineArray_Delete:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, pos: UINT) -> MTAPIRES,
    pub IMTOnlineArray_Detach:
        unsafe extern "C" fn(this: *mut IMTOnlineArray, pos: UINT) -> *mut IMTOnline,
    pub IMTOnlineArray_Update: unsafe extern "C" fn(
        this: *mut IMTOnlineArray,
        pos: UINT,
        online: *mut IMTOnline,
    ) -> MTAPIRES,
    pub IMTOnlineArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTOnlineArray,
        pos: UINT,
        online: *const IMTOnline,
    ) -> MTAPIRES,
    pub IMTOnlineArray_Shift: unsafe extern "C" fn(
        this: *mut IMTOnlineArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTOnlineArray_Total: unsafe extern "C" fn(this: *const IMTOnlineArray) -> UINT,
    pub IMTOnlineArray_Next:
        unsafe extern "C" fn(this: *const IMTOnlineArray, index: UINT) -> *mut IMTOnline,
    pub IMTOnlineArray_Sort: unsafe extern "C" fn(
        this: *mut IMTOnlineArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTOnlineArray_Search: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOnlineArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOnlineArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOnlineArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOnlineArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOnlineArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTOnlineArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTOnlineArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTOnlineArray {
    pub vtable_: *const IMTOnlineArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTOnlineArray"][::std::mem::size_of::<IMTOnlineArray>() - 8usize];
    ["Alignment of IMTOnlineArray"][::std::mem::align_of::<IMTOnlineArray>() - 8usize];
};
#[repr(C)]
pub struct IMTRequestSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTRequestSink {
    pub vtable_: *const IMTRequestSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTRequestSink"][::std::mem::size_of::<IMTRequestSink>() - 8usize];
    ["Alignment of IMTRequestSink"][::std::mem::align_of::<IMTRequestSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConEmail__bindgen_vtable {
    pub IMTConEmail_Release: unsafe extern "C" fn(this: *mut IMTConEmail),
    pub IMTConEmail_Assign:
        unsafe extern "C" fn(this: *mut IMTConEmail, email: *const IMTConEmail) -> MTAPIRES,
    pub IMTConEmail_Clear: unsafe extern "C" fn(this: *mut IMTConEmail) -> MTAPIRES,
    pub IMTConEmail_Name1: unsafe extern "C" fn(this: *mut IMTConEmail, name: LPCWSTR) -> MTAPIRES,
    pub IMTConEmail_Name: unsafe extern "C" fn(this: *const IMTConEmail) -> LPCWSTR,
    pub IMTConEmail_SenderMail1:
        unsafe extern "C" fn(this: *mut IMTConEmail, mail: LPCWSTR) -> MTAPIRES,
    pub IMTConEmail_SenderMail: unsafe extern "C" fn(this: *const IMTConEmail) -> LPCWSTR,
    pub IMTConEmail_SenderName1:
        unsafe extern "C" fn(this: *mut IMTConEmail, name: LPCWSTR) -> MTAPIRES,
    pub IMTConEmail_SenderName: unsafe extern "C" fn(this: *const IMTConEmail) -> LPCWSTR,
    pub IMTConEmail_Server1:
        unsafe extern "C" fn(this: *mut IMTConEmail, server: LPCWSTR) -> MTAPIRES,
    pub IMTConEmail_Server: unsafe extern "C" fn(this: *const IMTConEmail) -> LPCWSTR,
    pub IMTConEmail_Login1:
        unsafe extern "C" fn(this: *mut IMTConEmail, login: LPCWSTR) -> MTAPIRES,
    pub IMTConEmail_Login: unsafe extern "C" fn(this: *const IMTConEmail) -> LPCWSTR,
    pub IMTConEmail_Password1:
        unsafe extern "C" fn(this: *mut IMTConEmail, password: LPCWSTR) -> MTAPIRES,
    pub IMTConEmail_Password: unsafe extern "C" fn(this: *const IMTConEmail) -> LPCWSTR,
    pub IMTConEmail_Flags1: unsafe extern "C" fn(this: *mut IMTConEmail, flags: UINT64) -> MTAPIRES,
    pub IMTConEmail_Flags: unsafe extern "C" fn(this: *const IMTConEmail) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConEmail {
    pub vtable_: *const IMTConEmail__bindgen_vtable,
}
pub const IMTConEmail_EnFlags_FLAG_NONE: IMTConEmail_EnFlags = 0;
pub const IMTConEmail_EnFlags_FLAG_ENABLED: IMTConEmail_EnFlags = 1;
pub const IMTConEmail_EnFlags_FLAG_DEFAULT: IMTConEmail_EnFlags = 2;
pub const IMTConEmail_EnFlags_FLAG_FIRST: IMTConEmail_EnFlags = 1;
pub const IMTConEmail_EnFlags_FLAG_ALL: IMTConEmail_EnFlags = 3;
pub type IMTConEmail_EnFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConEmail"][::std::mem::size_of::<IMTConEmail>() - 8usize];
    ["Alignment of IMTConEmail"][::std::mem::align_of::<IMTConEmail>() - 8usize];
};
#[repr(C)]
pub struct IMTConEmailSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConEmailSink {
    pub vtable_: *const IMTConEmailSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConEmailSink"][::std::mem::size_of::<IMTConEmailSink>() - 8usize];
    ["Alignment of IMTConEmailSink"][::std::mem::align_of::<IMTConEmailSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConMessengerCountry__bindgen_vtable {
    pub IMTConMessengerCountry_Release: unsafe extern "C" fn(this: *mut IMTConMessengerCountry),
    pub IMTConMessengerCountry_Assign: unsafe extern "C" fn(
        this: *mut IMTConMessengerCountry,
        country: *const IMTConMessengerCountry,
    ) -> MTAPIRES,
    pub IMTConMessengerCountry_Clear:
        unsafe extern "C" fn(this: *mut IMTConMessengerCountry) -> MTAPIRES,
    pub IMTConMessengerCountry_PhoneCode:
        unsafe extern "C" fn(this: *const IMTConMessengerCountry) -> LPCWSTR,
    pub IMTConMessengerCountry_PhoneCode1:
        unsafe extern "C" fn(this: *mut IMTConMessengerCountry, code: LPCWSTR) -> MTAPIRES,
    pub IMTConMessengerCountry_MessageTemplate1:
        unsafe extern "C" fn(this: *mut IMTConMessengerCountry, msg_template: LPCWSTR) -> MTAPIRES,
    pub IMTConMessengerCountry_MessageTemplate:
        unsafe extern "C" fn(this: *const IMTConMessengerCountry) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConMessengerCountry {
    pub vtable_: *const IMTConMessengerCountry__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConMessengerCountry"][::std::mem::size_of::<IMTConMessengerCountry>() - 8usize];
    ["Alignment of IMTConMessengerCountry"]
        [::std::mem::align_of::<IMTConMessengerCountry>() - 8usize];
};
#[repr(C)]
pub struct IMTConMessengerGroup__bindgen_vtable {
    pub IMTConMessengerGroup_Release: unsafe extern "C" fn(this: *mut IMTConMessengerGroup),
    pub IMTConMessengerGroup_Assign: unsafe extern "C" fn(
        this: *mut IMTConMessengerGroup,
        group: *const IMTConMessengerGroup,
    ) -> MTAPIRES,
    pub IMTConMessengerGroup_Clear:
        unsafe extern "C" fn(this: *mut IMTConMessengerGroup) -> MTAPIRES,
    pub IMTConMessengerGroup_Group1:
        unsafe extern "C" fn(this: *mut IMTConMessengerGroup, group: LPCWSTR) -> MTAPIRES,
    pub IMTConMessengerGroup_Group:
        unsafe extern "C" fn(this: *const IMTConMessengerGroup) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConMessengerGroup {
    pub vtable_: *const IMTConMessengerGroup__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConMessengerGroup"][::std::mem::size_of::<IMTConMessengerGroup>() - 8usize];
    ["Alignment of IMTConMessengerGroup"][::std::mem::align_of::<IMTConMessengerGroup>() - 8usize];
};
#[repr(C)]
pub struct IMTConMessenger__bindgen_vtable {
    pub IMTConMessenger_Release: unsafe extern "C" fn(this: *mut IMTConMessenger),
    pub IMTConMessenger_Assign: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        messenger: *const IMTConMessenger,
    ) -> MTAPIRES,
    pub IMTConMessenger_Clear: unsafe extern "C" fn(this: *mut IMTConMessenger) -> MTAPIRES,
    pub IMTConMessenger_Name1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, name: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_Name: unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_Sender1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, sender: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_Sender: unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderType1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, provider: UINT) -> MTAPIRES,
    pub IMTConMessenger_ProviderType: unsafe extern "C" fn(this: *const IMTConMessenger) -> UINT,
    pub IMTConMessenger_ProviderAddress1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, address: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_ProviderAddress:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderLogin1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, login: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_ProviderLogin:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderPassword1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, password: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_ProviderPassword:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderToken1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, token: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_ProviderToken:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderSubId1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, subid: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_ProviderSubId:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderCurrency1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_ProviderCurrency:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_ProviderCurrencyRate1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, rate: f64) -> MTAPIRES,
    pub IMTConMessenger_ProviderCurrencyRate:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> f64,
    pub IMTConMessenger_Flags1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, flags: UINT64) -> MTAPIRES,
    pub IMTConMessenger_Flags: unsafe extern "C" fn(this: *const IMTConMessenger) -> UINT64,
    pub IMTConMessenger_MessageTemplate:
        unsafe extern "C" fn(this: *const IMTConMessenger) -> LPCWSTR,
    pub IMTConMessenger_MessageTemplate1:
        unsafe extern "C" fn(this: *mut IMTConMessenger, msg_template: LPCWSTR) -> MTAPIRES,
    pub IMTConMessenger_CountryAdd: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        country: *mut IMTConMessengerCountry,
    ) -> MTAPIRES,
    pub IMTConMessenger_CountryUpdate: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        pos: UINT,
        country: *const IMTConMessengerCountry,
    ) -> MTAPIRES,
    pub IMTConMessenger_CountryDelete:
        unsafe extern "C" fn(this: *mut IMTConMessenger, pos: UINT) -> MTAPIRES,
    pub IMTConMessenger_CountryClear: unsafe extern "C" fn(this: *mut IMTConMessenger) -> MTAPIRES,
    pub IMTConMessenger_CountryShift: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConMessenger_CountryTotal: unsafe extern "C" fn(this: *const IMTConMessenger) -> UINT,
    pub IMTConMessenger_CountryNext: unsafe extern "C" fn(
        this: *const IMTConMessenger,
        pos: UINT,
        country: *mut IMTConMessengerCountry,
    ) -> MTAPIRES,
    pub IMTConMessenger_GroupAdd: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        group: *mut IMTConMessengerGroup,
    ) -> MTAPIRES,
    pub IMTConMessenger_GroupUpdate: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        pos: UINT,
        group: *const IMTConMessengerGroup,
    ) -> MTAPIRES,
    pub IMTConMessenger_GroupDelete:
        unsafe extern "C" fn(this: *mut IMTConMessenger, pos: UINT) -> MTAPIRES,
    pub IMTConMessenger_GroupClear: unsafe extern "C" fn(this: *mut IMTConMessenger) -> MTAPIRES,
    pub IMTConMessenger_GroupShift: unsafe extern "C" fn(
        this: *mut IMTConMessenger,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConMessenger_GroupTotal: unsafe extern "C" fn(this: *const IMTConMessenger) -> UINT,
    pub IMTConMessenger_GroupNext: unsafe extern "C" fn(
        this: *const IMTConMessenger,
        pos: UINT,
        group: *mut IMTConMessengerGroup,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConMessenger {
    pub vtable_: *const IMTConMessenger__bindgen_vtable,
}
pub const IMTConMessenger_EnFlags_FLAG_NONE: IMTConMessenger_EnFlags = 0;
pub const IMTConMessenger_EnFlags_FLAG_ENABLED: IMTConMessenger_EnFlags = 1;
pub const IMTConMessenger_EnFlags_FLAG_DEFAULT: IMTConMessenger_EnFlags = 2;
pub const IMTConMessenger_EnFlags_FLAG_NOTIFY_CLIENTS: IMTConMessenger_EnFlags = 4;
pub const IMTConMessenger_EnFlags_FLAG_NOTIFY_SERVICES: IMTConMessenger_EnFlags = 8;
pub const IMTConMessenger_EnFlags_FLAG_FIRST: IMTConMessenger_EnFlags = 1;
pub const IMTConMessenger_EnFlags_FLAG_ALL: IMTConMessenger_EnFlags = 15;
pub type IMTConMessenger_EnFlags = ::std::os::raw::c_int;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_BULKSMS: IMTConMessenger_EnProviderType = 0;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_CLICKATELL: IMTConMessenger_EnProviderType =
    1;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_WEBSMS: IMTConMessenger_EnProviderType = 2;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_TWILIO: IMTConMessenger_EnProviderType = 3;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_CMCOM: IMTConMessenger_EnProviderType = 4;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_VONAGE: IMTConMessenger_EnProviderType = 5;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_INFOBIP: IMTConMessenger_EnProviderType = 6;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_FONIVA: IMTConMessenger_EnProviderType = 7;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_FIRST: IMTConMessenger_EnProviderType = 0;
pub const IMTConMessenger_EnProviderType_PROVIDER_SMS_LAST: IMTConMessenger_EnProviderType = 99;
pub const IMTConMessenger_EnProviderType_PROVIDER_IM_TELEGRAM: IMTConMessenger_EnProviderType = 100;
pub const IMTConMessenger_EnProviderType_PROVIDER_IM_SLACK: IMTConMessenger_EnProviderType = 101;
pub const IMTConMessenger_EnProviderType_PROVIDER_IM_MICROSOFT_TEAMS:
    IMTConMessenger_EnProviderType = 102;
pub const IMTConMessenger_EnProviderType_PROVIDER_IM_FIRST: IMTConMessenger_EnProviderType = 100;
pub const IMTConMessenger_EnProviderType_PROVIDER_IM_LAST: IMTConMessenger_EnProviderType = 199;
pub const IMTConMessenger_EnProviderType_PROVIDER_PUSH_METAQUOTES: IMTConMessenger_EnProviderType =
    200;
pub const IMTConMessenger_EnProviderType_PROVIDER_PUSH_UNIVERSAL: IMTConMessenger_EnProviderType =
    201;
pub const IMTConMessenger_EnProviderType_PROVIDER_PUSH_FIRST: IMTConMessenger_EnProviderType = 200;
pub const IMTConMessenger_EnProviderType_PROVIDER_PUSH_LAST: IMTConMessenger_EnProviderType = 299;
pub type IMTConMessenger_EnProviderType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConMessenger"][::std::mem::size_of::<IMTConMessenger>() - 8usize];
    ["Alignment of IMTConMessenger"][::std::mem::align_of::<IMTConMessenger>() - 8usize];
};
#[repr(C)]
pub struct IMTConMessengerSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConMessengerSink {
    pub vtable_: *const IMTConMessengerSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConMessengerSink"][::std::mem::size_of::<IMTConMessengerSink>() - 8usize];
    ["Alignment of IMTConMessengerSink"][::std::mem::align_of::<IMTConMessengerSink>() - 8usize];
};
#[repr(C)]
pub struct IMTClient__bindgen_vtable {
    pub IMTClient_Release: unsafe extern "C" fn(this: *mut IMTClient),
    pub IMTClient_Assign:
        unsafe extern "C" fn(this: *mut IMTClient, client: *const IMTClient) -> MTAPIRES,
    pub IMTClient_Clear: unsafe extern "C" fn(this: *mut IMTClient) -> MTAPIRES,
    pub IMTClient_RecordID1:
        unsafe extern "C" fn(this: *mut IMTClient, record_id: UINT64) -> MTAPIRES,
    pub IMTClient_RecordID: unsafe extern "C" fn(this: *const IMTClient) -> UINT64,
    pub IMTClient_ClientType1: unsafe extern "C" fn(this: *mut IMTClient, type_: UINT) -> MTAPIRES,
    pub IMTClient_ClientType: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_ClientStatus1:
        unsafe extern "C" fn(this: *mut IMTClient, status: UINT) -> MTAPIRES,
    pub IMTClient_ClientStatus: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_AssignedManager1:
        unsafe extern "C" fn(this: *mut IMTClient, manager: UINT64) -> MTAPIRES,
    pub IMTClient_AssignedManager: unsafe extern "C" fn(this: *const IMTClient) -> UINT64,
    pub IMTClient_ComplianceApprovedBy1:
        unsafe extern "C" fn(this: *mut IMTClient, manager: UINT64) -> MTAPIRES,
    pub IMTClient_ComplianceApprovedBy: unsafe extern "C" fn(this: *const IMTClient) -> UINT64,
    pub IMTClient_ComplianceClientCategory1:
        unsafe extern "C" fn(this: *mut IMTClient, category: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ComplianceClientCategory: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ComplianceDateApproval1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_ComplianceDateApproval: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_ComplianceDateTermination1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_ComplianceDateTermination: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_LeadCampaign1:
        unsafe extern "C" fn(this: *mut IMTClient, campaign: LPCWSTR) -> MTAPIRES,
    pub IMTClient_LeadCampaign: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_LeadSource1:
        unsafe extern "C" fn(this: *mut IMTClient, source: LPCWSTR) -> MTAPIRES,
    pub IMTClient_LeadSource: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_Introducer1:
        unsafe extern "C" fn(this: *mut IMTClient, introducer: LPCWSTR) -> MTAPIRES,
    pub IMTClient_Introducer: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonTitle1:
        unsafe extern "C" fn(this: *mut IMTClient, title: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonTitle: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonName1:
        unsafe extern "C" fn(this: *mut IMTClient, name: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonName: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonMiddleName1:
        unsafe extern "C" fn(this: *mut IMTClient, middle_name: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonMiddleName: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonLastName1:
        unsafe extern "C" fn(this: *mut IMTClient, last_name: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonLastName: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonBirthDate1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_PersonBirthDate: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_PersonCitizenship1:
        unsafe extern "C" fn(this: *mut IMTClient, citizenship: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonCitizenship: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonGender1:
        unsafe extern "C" fn(this: *mut IMTClient, gender: UINT) -> MTAPIRES,
    pub IMTClient_PersonGender: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_PersonTaxID1:
        unsafe extern "C" fn(this: *mut IMTClient, taxid: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonTaxID: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonDocumentType1:
        unsafe extern "C" fn(this: *mut IMTClient, type_: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonDocumentType: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonDocumentNumber1:
        unsafe extern "C" fn(this: *mut IMTClient, number: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonDocumentNumber: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonDocumentDate1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_PersonDocumentDate: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_PersonDocumentExtra1:
        unsafe extern "C" fn(this: *mut IMTClient, extra: LPCWSTR) -> MTAPIRES,
    pub IMTClient_PersonDocumentExtra: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonEmployment1:
        unsafe extern "C" fn(this: *mut IMTClient, employment: UINT) -> MTAPIRES,
    pub IMTClient_PersonEmployment: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_PersonIndustry1:
        unsafe extern "C" fn(this: *mut IMTClient, industry: UINT) -> MTAPIRES,
    pub IMTClient_PersonIndustry: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_PersonEducation1:
        unsafe extern "C" fn(this: *mut IMTClient, education: UINT) -> MTAPIRES,
    pub IMTClient_PersonEducation: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_PersonWealthSource1:
        unsafe extern "C" fn(this: *mut IMTClient, source: UINT) -> MTAPIRES,
    pub IMTClient_PersonWealthSource: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_PersonAnnualIncome1:
        unsafe extern "C" fn(this: *mut IMTClient, income: f64) -> MTAPIRES,
    pub IMTClient_PersonAnnualIncome: unsafe extern "C" fn(this: *const IMTClient) -> f64,
    pub IMTClient_PersonNetWorth1:
        unsafe extern "C" fn(this: *mut IMTClient, worth: f64) -> MTAPIRES,
    pub IMTClient_PersonNetWorth: unsafe extern "C" fn(this: *const IMTClient) -> f64,
    pub IMTClient_PersonAnnualDeposit1:
        unsafe extern "C" fn(this: *mut IMTClient, deposit: f64) -> MTAPIRES,
    pub IMTClient_PersonAnnualDeposit: unsafe extern "C" fn(this: *const IMTClient) -> f64,
    pub IMTClient_CompanyName1:
        unsafe extern "C" fn(this: *mut IMTClient, name: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyName: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyRegNumber1:
        unsafe extern "C" fn(this: *mut IMTClient, number: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyRegNumber: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyRegDate1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_CompanyRegDate: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_CompanyRegAuthority1:
        unsafe extern "C" fn(this: *mut IMTClient, authority: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyRegAuthority: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyVat1: unsafe extern "C" fn(this: *mut IMTClient, vat: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyVat: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyLei1: unsafe extern "C" fn(this: *mut IMTClient, lei: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyLei: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyLicenseNumber1:
        unsafe extern "C" fn(this: *mut IMTClient, number: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyLicenseNumber: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyLicenseAuthority1:
        unsafe extern "C" fn(this: *mut IMTClient, authority: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyLicenseAuthority: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyCountry1:
        unsafe extern "C" fn(this: *mut IMTClient, country: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyCountry: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyAddress1:
        unsafe extern "C" fn(this: *mut IMTClient, address: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyAddress: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_CompanyWebsite1:
        unsafe extern "C" fn(this: *mut IMTClient, website: LPCWSTR) -> MTAPIRES,
    pub IMTClient_CompanyWebsite: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ContactPreferred1:
        unsafe extern "C" fn(this: *mut IMTClient, preferred: UINT) -> MTAPIRES,
    pub IMTClient_ContactPreferred: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_ContactLanguage1:
        unsafe extern "C" fn(this: *mut IMTClient, language: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ContactLanguage: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ContactEmail1:
        unsafe extern "C" fn(this: *mut IMTClient, email: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ContactEmail: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ContactPhone1:
        unsafe extern "C" fn(this: *mut IMTClient, phone: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ContactPhone: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ContactMessengers1:
        unsafe extern "C" fn(this: *mut IMTClient, messengers: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ContactMessengers: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ContactSocialNetworks1:
        unsafe extern "C" fn(this: *mut IMTClient, social_networks: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ContactSocialNetworks: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ContactLastDate1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_ContactLastDate: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_AddressCountry1:
        unsafe extern "C" fn(this: *mut IMTClient, country: LPCWSTR) -> MTAPIRES,
    pub IMTClient_AddressCountry: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_AddressPostcode1:
        unsafe extern "C" fn(this: *mut IMTClient, postcode: LPCWSTR) -> MTAPIRES,
    pub IMTClient_AddressPostcode: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_AddressStreet1:
        unsafe extern "C" fn(this: *mut IMTClient, street: LPCWSTR) -> MTAPIRES,
    pub IMTClient_AddressStreet: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_AddressState1:
        unsafe extern "C" fn(this: *mut IMTClient, state: LPCWSTR) -> MTAPIRES,
    pub IMTClient_AddressState: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_AddressCity1:
        unsafe extern "C" fn(this: *mut IMTClient, city: LPCWSTR) -> MTAPIRES,
    pub IMTClient_AddressCity: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ExperienceFX1:
        unsafe extern "C" fn(this: *mut IMTClient, experience: UINT) -> MTAPIRES,
    pub IMTClient_ExperienceFX: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_ExperienceCFD1:
        unsafe extern "C" fn(this: *mut IMTClient, experience: UINT) -> MTAPIRES,
    pub IMTClient_ExperienceCFD: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_ExperienceFutures1:
        unsafe extern "C" fn(this: *mut IMTClient, experience: UINT) -> MTAPIRES,
    pub IMTClient_ExperienceFutures: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_ExperienceStocks1:
        unsafe extern "C" fn(this: *mut IMTClient, experience: UINT) -> MTAPIRES,
    pub IMTClient_ExperienceStocks: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_TradingGroup1:
        unsafe extern "C" fn(this: *mut IMTClient, group: LPCWSTR) -> MTAPIRES,
    pub IMTClient_TradingGroup: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_ClientOrigin1:
        unsafe extern "C" fn(this: *mut IMTClient, origin: UINT) -> MTAPIRES,
    pub IMTClient_ClientOrigin: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
    pub IMTClient_ClientOriginLogin1:
        unsafe extern "C" fn(this: *mut IMTClient, login: UINT64) -> MTAPIRES,
    pub IMTClient_ClientOriginLogin: unsafe extern "C" fn(this: *const IMTClient) -> UINT64,
    pub IMTClient_ClientExternalID1:
        unsafe extern "C" fn(this: *mut IMTClient, external_id: LPCWSTR) -> MTAPIRES,
    pub IMTClient_ClientExternalID: unsafe extern "C" fn(this: *const IMTClient) -> LPCWSTR,
    pub IMTClient_PersonDocumentExpiration1:
        unsafe extern "C" fn(this: *mut IMTClient, date: INT64) -> MTAPIRES,
    pub IMTClient_PersonDocumentExpiration: unsafe extern "C" fn(this: *const IMTClient) -> INT64,
    pub IMTClient_KYCStatus1:
        unsafe extern "C" fn(this: *mut IMTClient, kyc_status: UINT) -> MTAPIRES,
    pub IMTClient_KYCStatus: unsafe extern "C" fn(this: *const IMTClient) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTClient {
    pub vtable_: *const IMTClient__bindgen_vtable,
}
pub const IMTClient_EnGender_GENDER_UNSPECIFIED: IMTClient_EnGender = 0;
pub const IMTClient_EnGender_GENDER_MALE: IMTClient_EnGender = 1;
pub const IMTClient_EnGender_GENDER_FEMALE: IMTClient_EnGender = 2;
pub const IMTClient_EnGender_GENDER_FIRST: IMTClient_EnGender = 0;
pub const IMTClient_EnGender_GENDER_LAST: IMTClient_EnGender = 2;
pub type IMTClient_EnGender = ::std::os::raw::c_int;
pub const IMTClient_EnClientType_CLIENT_TYPE_UNDEFINED: IMTClient_EnClientType = 0;
pub const IMTClient_EnClientType_CLIENT_TYPE_INDIVIDUAL: IMTClient_EnClientType = 1;
pub const IMTClient_EnClientType_CLIENT_TYPE_CORPORATE: IMTClient_EnClientType = 2;
pub const IMTClient_EnClientType_CLIENT_TYPE_FUND: IMTClient_EnClientType = 3;
pub const IMTClient_EnClientType_CLIENT_TYPE_FIRST: IMTClient_EnClientType = 0;
pub const IMTClient_EnClientType_CLIENT_TYPE_LAST: IMTClient_EnClientType = 3;
pub type IMTClient_EnClientType = ::std::os::raw::c_int;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_UNREGISTERED: IMTClient_EnClientStatus = 0;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_REGISTERED: IMTClient_EnClientStatus = 100;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_NOTINTERESTED: IMTClient_EnClientStatus = 200;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_APPLICATION_INCOMPLETED: IMTClient_EnClientStatus =
    300;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_APPLICATION_COMPLETED: IMTClient_EnClientStatus =
    400;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_APPLICATION_INFORMATION: IMTClient_EnClientStatus =
    500;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_APPLICATION_REJECTED: IMTClient_EnClientStatus =
    600;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_APPROVED: IMTClient_EnClientStatus = 700;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_FUNDED: IMTClient_EnClientStatus = 800;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_ACTIVE: IMTClient_EnClientStatus = 900;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_INACTIVE: IMTClient_EnClientStatus = 1000;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_SUSPENDED: IMTClient_EnClientStatus = 1100;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_CLOSED: IMTClient_EnClientStatus = 1200;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_TERMINATED: IMTClient_EnClientStatus = 1300;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_FIRST: IMTClient_EnClientStatus = 0;
pub const IMTClient_EnClientStatus_CLIENT_STATUS_LAST: IMTClient_EnClientStatus = 1300;
pub type IMTClient_EnClientStatus = ::std::os::raw::c_int;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_UNDEFINED:
    IMTClient_EnPreferredCommunication = 0;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_EMAIL:
    IMTClient_EnPreferredCommunication = 1;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_PHONE:
    IMTClient_EnPreferredCommunication = 2;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_PHONE_SMS:
    IMTClient_EnPreferredCommunication = 3;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_MESSENGER:
    IMTClient_EnPreferredCommunication = 4;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_FIRST:
    IMTClient_EnPreferredCommunication = 0;
pub const IMTClient_EnPreferredCommunication_PREFERRED_COMMUNICATION_LAST:
    IMTClient_EnPreferredCommunication = 4;
pub type IMTClient_EnPreferredCommunication = ::std::os::raw::c_int;
pub const IMTClient_EnTradingExperience_EXPERIENCE_LESS_1_YEAR: IMTClient_EnTradingExperience = 0;
pub const IMTClient_EnTradingExperience_EXPERIENCE_1_3_YEAR: IMTClient_EnTradingExperience = 1;
pub const IMTClient_EnTradingExperience_EXPERIENCE_ABOVE_3_YEAR: IMTClient_EnTradingExperience = 2;
pub const IMTClient_EnTradingExperience_EXPERIENCE_FIRST: IMTClient_EnTradingExperience = 0;
pub const IMTClient_EnTradingExperience_EXPERIENCE_LAST: IMTClient_EnTradingExperience = 2;
pub type IMTClient_EnTradingExperience = ::std::os::raw::c_int;
pub const IMTClient_EnEmployment_EMPLOYMENT_UNEMPLOYED: IMTClient_EnEmployment = 0;
pub const IMTClient_EnEmployment_EMPLOYMENT_EMPLOYED: IMTClient_EnEmployment = 1;
pub const IMTClient_EnEmployment_EMPLOYMENT_SELF_EMPLOYED: IMTClient_EnEmployment = 2;
pub const IMTClient_EnEmployment_EMPLOYMENT_RETIRED: IMTClient_EnEmployment = 3;
pub const IMTClient_EnEmployment_EMPLOYMENT_STUDENT: IMTClient_EnEmployment = 4;
pub const IMTClient_EnEmployment_EMPLOYMENT_OTHER: IMTClient_EnEmployment = 5;
pub const IMTClient_EnEmployment_EMPLOYMENT_FIRST: IMTClient_EnEmployment = 0;
pub const IMTClient_EnEmployment_EMPLOYMENT_LAST: IMTClient_EnEmployment = 5;
pub type IMTClient_EnEmployment = ::std::os::raw::c_int;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_NONE: IMTClient_EnEmploymentIndustry = 0;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_AGRICULTURE: IMTClient_EnEmploymentIndustry = 1;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_CONSTRUCTION: IMTClient_EnEmploymentIndustry = 2;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_MANAGEMENT: IMTClient_EnEmploymentIndustry = 3;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_COMMUNICATION: IMTClient_EnEmploymentIndustry = 4;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_EDUCATION: IMTClient_EnEmploymentIndustry = 5;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_GOVERNMENT: IMTClient_EnEmploymentIndustry = 6;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_HEALTHCARE: IMTClient_EnEmploymentIndustry = 7;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_TOURISM: IMTClient_EnEmploymentIndustry = 8;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_IT: IMTClient_EnEmploymentIndustry = 9;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_SECURITY: IMTClient_EnEmploymentIndustry = 10;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_MANUFACTURING: IMTClient_EnEmploymentIndustry =
    11;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_MARKETING: IMTClient_EnEmploymentIndustry = 12;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_SCIENCE: IMTClient_EnEmploymentIndustry = 13;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_ENGINEERING: IMTClient_EnEmploymentIndustry = 14;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_TRANSPORT: IMTClient_EnEmploymentIndustry = 15;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_OTHER: IMTClient_EnEmploymentIndustry = 16;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_FIRST: IMTClient_EnEmploymentIndustry = 1;
pub const IMTClient_EnEmploymentIndustry_INDUSTRY_LAST: IMTClient_EnEmploymentIndustry = 16;
pub type IMTClient_EnEmploymentIndustry = ::std::os::raw::c_int;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_NONE: IMTClient_EnEducationLevel = 0;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_HIGH_SCHOOL: IMTClient_EnEducationLevel = 1;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_BACHELOR: IMTClient_EnEducationLevel = 2;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_MASTER: IMTClient_EnEducationLevel = 3;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_PHD: IMTClient_EnEducationLevel = 4;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_OTHER: IMTClient_EnEducationLevel = 5;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_FIRST: IMTClient_EnEducationLevel = 0;
pub const IMTClient_EnEducationLevel_EDUCATION_LEVEL_LAST: IMTClient_EnEducationLevel = 5;
pub type IMTClient_EnEducationLevel = ::std::os::raw::c_int;
pub const IMTClient_EnWealthSource_WEALTH_SOURCE_EMPLOYMENT: IMTClient_EnWealthSource = 0;
pub const IMTClient_EnWealthSource_WEALTH_SOURCE_SAVINGS: IMTClient_EnWealthSource = 1;
pub const IMTClient_EnWealthSource_WEALTH_SOURCE_INHERITANCE: IMTClient_EnWealthSource = 2;
pub const IMTClient_EnWealthSource_WEALTH_SOURCE_OTHER: IMTClient_EnWealthSource = 3;
pub const IMTClient_EnWealthSource_WEALTH_SOURCE_FIRST: IMTClient_EnWealthSource = 0;
pub const IMTClient_EnWealthSource_WEALTH_SOURCE_LAST: IMTClient_EnWealthSource = 3;
pub type IMTClient_EnWealthSource = ::std::os::raw::c_int;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_MANUAL: IMTClient_EnClientOrigin = 0;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_DEMO: IMTClient_EnClientOrigin = 1;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_CONTEST: IMTClient_EnClientOrigin = 2;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_PRELIMINARY: IMTClient_EnClientOrigin = 3;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_REAL: IMTClient_EnClientOrigin = 4;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_FIRST: IMTClient_EnClientOrigin = 0;
pub const IMTClient_EnClientOrigin_CLIENT_ORIGIN_LAST: IMTClient_EnClientOrigin = 4;
pub type IMTClient_EnClientOrigin = ::std::os::raw::c_int;
pub const IMTClient_EnKYCStatus_KYC_STATUS_UNDEFINED: IMTClient_EnKYCStatus = 0;
pub const IMTClient_EnKYCStatus_KYC_STATUS_APPROVED: IMTClient_EnKYCStatus = 1;
pub const IMTClient_EnKYCStatus_KYC_STATUS_DECLINED: IMTClient_EnKYCStatus = 2;
pub const IMTClient_EnKYCStatus_KYC_STATUS_FIRST: IMTClient_EnKYCStatus = 0;
pub const IMTClient_EnKYCStatus_KYC_STATUS_LAST: IMTClient_EnKYCStatus = 2;
pub type IMTClient_EnKYCStatus = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTClient"][::std::mem::size_of::<IMTClient>() - 8usize];
    ["Alignment of IMTClient"][::std::mem::align_of::<IMTClient>() - 8usize];
};
#[repr(C)]
pub struct IMTClientArray__bindgen_vtable {
    pub IMTClientArray_Release: unsafe extern "C" fn(this: *mut IMTClientArray),
    pub IMTClientArray_Assign:
        unsafe extern "C" fn(this: *mut IMTClientArray, array: *const IMTClientArray) -> MTAPIRES,
    pub IMTClientArray_Clear: unsafe extern "C" fn(this: *mut IMTClientArray) -> MTAPIRES,
    pub IMTClientArray_Add1:
        unsafe extern "C" fn(this: *mut IMTClientArray, array: *mut IMTClientArray) -> MTAPIRES,
    pub IMTClientArray_Add:
        unsafe extern "C" fn(this: *mut IMTClientArray, client: *mut IMTClient) -> MTAPIRES,
    pub IMTClientArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTClientArray, array: *const IMTClientArray) -> MTAPIRES,
    pub IMTClientArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTClientArray, client: *const IMTClient) -> MTAPIRES,
    pub IMTClientArray_Delete:
        unsafe extern "C" fn(this: *mut IMTClientArray, pos: UINT) -> MTAPIRES,
    pub IMTClientArray_Detach:
        unsafe extern "C" fn(this: *mut IMTClientArray, pos: UINT) -> *mut IMTClient,
    pub IMTClientArray_Update: unsafe extern "C" fn(
        this: *mut IMTClientArray,
        pos: UINT,
        client: *mut IMTClient,
    ) -> MTAPIRES,
    pub IMTClientArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTClientArray,
        pos: UINT,
        client: *const IMTClient,
    ) -> MTAPIRES,
    pub IMTClientArray_Shift: unsafe extern "C" fn(
        this: *mut IMTClientArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTClientArray_Total: unsafe extern "C" fn(this: *const IMTClientArray) -> UINT,
    pub IMTClientArray_Next:
        unsafe extern "C" fn(this: *const IMTClientArray, index: UINT) -> *mut IMTClient,
    pub IMTClientArray_Sort: unsafe extern "C" fn(
        this: *mut IMTClientArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTClientArray_Search: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTClientArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTClientArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTClientArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTClientArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTClientArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTClientArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTClientArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTClientArray {
    pub vtable_: *const IMTClientArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTClientArray"][::std::mem::size_of::<IMTClientArray>() - 8usize];
    ["Alignment of IMTClientArray"][::std::mem::align_of::<IMTClientArray>() - 8usize];
};
#[repr(C)]
pub struct IMTClientSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTClientSink {
    pub vtable_: *const IMTClientSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTClientSink"][::std::mem::size_of::<IMTClientSink>() - 8usize];
    ["Alignment of IMTClientSink"][::std::mem::align_of::<IMTClientSink>() - 8usize];
};
#[repr(C)]
pub struct IMTAttachment__bindgen_vtable {
    pub IMTAttachment_Release: unsafe extern "C" fn(this: *mut IMTAttachment),
    pub IMTAttachment_Assign:
        unsafe extern "C" fn(this: *mut IMTAttachment, file: *const IMTAttachment) -> MTAPIRES,
    pub IMTAttachment_Clear: unsafe extern "C" fn(this: *mut IMTAttachment) -> MTAPIRES,
    pub IMTAttachment_RecordID1:
        unsafe extern "C" fn(this: *mut IMTAttachment, record_id: UINT64) -> MTAPIRES,
    pub IMTAttachment_RecordID: unsafe extern "C" fn(this: *const IMTAttachment) -> UINT64,
    pub IMTAttachment_RelatedClient1:
        unsafe extern "C" fn(this: *mut IMTAttachment, record_id: UINT64) -> MTAPIRES,
    pub IMTAttachment_RelatedClient: unsafe extern "C" fn(this: *const IMTAttachment) -> UINT64,
    pub IMTAttachment_FileType1:
        unsafe extern "C" fn(this: *mut IMTAttachment, type_: UINT) -> MTAPIRES,
    pub IMTAttachment_FileType: unsafe extern "C" fn(this: *const IMTAttachment) -> UINT,
    pub IMTAttachment_FileName1:
        unsafe extern "C" fn(this: *mut IMTAttachment, name: LPCWSTR) -> MTAPIRES,
    pub IMTAttachment_FileName: unsafe extern "C" fn(this: *const IMTAttachment) -> LPCWSTR,
    pub IMTAttachment_FileContent1: unsafe extern "C" fn(
        this: *mut IMTAttachment,
        content: *const ::std::os::raw::c_void,
        content_size: UINT,
    ) -> MTAPIRES,
    pub IMTAttachment_FileContent: unsafe extern "C" fn(
        this: *const IMTAttachment,
        content_size: *mut UINT,
    ) -> *const ::std::os::raw::c_void,
    pub IMTAttachment_FileSize: unsafe extern "C" fn(this: *mut IMTAttachment) -> UINT,
    pub IMTAttachment_FileFlags1:
        unsafe extern "C" fn(this: *mut IMTAttachment, flags: UINT) -> MTAPIRES,
    pub IMTAttachment_FileFlags: unsafe extern "C" fn(this: *const IMTAttachment) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTAttachment {
    pub vtable_: *const IMTAttachment__bindgen_vtable,
}
pub const IMTAttachment_EnFileType_FILE_TYPE_OTHER: IMTAttachment_EnFileType = 0;
pub const IMTAttachment_EnFileType_FILE_TYPE_TXT: IMTAttachment_EnFileType = 1;
pub const IMTAttachment_EnFileType_FILE_TYPE_DOC: IMTAttachment_EnFileType = 2;
pub const IMTAttachment_EnFileType_FILE_TYPE_PDF: IMTAttachment_EnFileType = 3;
pub const IMTAttachment_EnFileType_FILE_TYPE_JPG: IMTAttachment_EnFileType = 4;
pub const IMTAttachment_EnFileType_FILE_TYPE_PNG: IMTAttachment_EnFileType = 5;
pub const IMTAttachment_EnFileType_FILE_TYPE_BMP: IMTAttachment_EnFileType = 6;
pub const IMTAttachment_EnFileType_FILE_TYPE_ZIP: IMTAttachment_EnFileType = 7;
pub const IMTAttachment_EnFileType_FILE_TYPE_FIRST: IMTAttachment_EnFileType = 0;
pub const IMTAttachment_EnFileType_FILE_TYPE_LAST: IMTAttachment_EnFileType = 7;
pub type IMTAttachment_EnFileType = ::std::os::raw::c_int;
pub const IMTAttachment_EnFileFlags_FILE_FLAG_EMBEDDED: IMTAttachment_EnFileFlags = 1;
pub const IMTAttachment_EnFileFlags_FILE_FLAG_NONE: IMTAttachment_EnFileFlags = 0;
pub const IMTAttachment_EnFileFlags_FILE_FLAG_ALL: IMTAttachment_EnFileFlags = 1;
pub type IMTAttachment_EnFileFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTAttachment"][::std::mem::size_of::<IMTAttachment>() - 8usize];
    ["Alignment of IMTAttachment"][::std::mem::align_of::<IMTAttachment>() - 8usize];
};
#[repr(C)]
pub struct IMTAttachmentArray__bindgen_vtable {
    pub IMTAttachmentArray_Release: unsafe extern "C" fn(this: *mut IMTAttachmentArray),
    pub IMTAttachmentArray_Assign: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        array: *const IMTAttachmentArray,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_Clear: unsafe extern "C" fn(this: *mut IMTAttachmentArray) -> MTAPIRES,
    pub IMTAttachmentArray_Add1: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        array: *mut IMTAttachmentArray,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_Add: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        attachment: *mut IMTAttachment,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_AddCopy1: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        array: *const IMTAttachmentArray,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_AddCopy: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        attachment: *const IMTAttachment,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_Delete:
        unsafe extern "C" fn(this: *mut IMTAttachmentArray, pos: UINT) -> MTAPIRES,
    pub IMTAttachmentArray_Detach:
        unsafe extern "C" fn(this: *mut IMTAttachmentArray, pos: UINT) -> *mut IMTAttachment,
    pub IMTAttachmentArray_Update: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        pos: UINT,
        attachment: *mut IMTAttachment,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        pos: UINT,
        attachment: *const IMTAttachment,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_Shift: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_Total: unsafe extern "C" fn(this: *const IMTAttachmentArray) -> UINT,
    pub IMTAttachmentArray_Next:
        unsafe extern "C" fn(this: *const IMTAttachmentArray, index: UINT) -> *mut IMTAttachment,
    pub IMTAttachmentArray_Sort: unsafe extern "C" fn(
        this: *mut IMTAttachmentArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTAttachmentArray_Search: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTAttachmentArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTAttachmentArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTAttachmentArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTAttachmentArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTAttachmentArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTAttachmentArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTAttachmentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTAttachmentArray {
    pub vtable_: *const IMTAttachmentArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTAttachmentArray"][::std::mem::size_of::<IMTAttachmentArray>() - 8usize];
    ["Alignment of IMTAttachmentArray"][::std::mem::align_of::<IMTAttachmentArray>() - 8usize];
};
#[repr(C)]
pub struct IMTDocument__bindgen_vtable {
    pub IMTDocument_Release: unsafe extern "C" fn(this: *mut IMTDocument),
    pub IMTDocument_Assign:
        unsafe extern "C" fn(this: *mut IMTDocument, document: *const IMTDocument) -> MTAPIRES,
    pub IMTDocument_Clear: unsafe extern "C" fn(this: *mut IMTDocument) -> MTAPIRES,
    pub IMTDocument_RecordID1:
        unsafe extern "C" fn(this: *mut IMTDocument, record_id: UINT64) -> MTAPIRES,
    pub IMTDocument_RecordID: unsafe extern "C" fn(this: *const IMTDocument) -> UINT64,
    pub IMTDocument_RelatedClient1:
        unsafe extern "C" fn(this: *mut IMTDocument, record_id: UINT64) -> MTAPIRES,
    pub IMTDocument_RelatedClient: unsafe extern "C" fn(this: *const IMTDocument) -> UINT64,
    pub IMTDocument_ApprovedDate1:
        unsafe extern "C" fn(this: *mut IMTDocument, date: INT64) -> MTAPIRES,
    pub IMTDocument_ApprovedDate: unsafe extern "C" fn(this: *const IMTDocument) -> INT64,
    pub IMTDocument_ApprovedBy1:
        unsafe extern "C" fn(this: *mut IMTDocument, manager: UINT64) -> MTAPIRES,
    pub IMTDocument_ApprovedBy: unsafe extern "C" fn(this: *const IMTDocument) -> UINT64,
    pub IMTDocument_DateIssue1:
        unsafe extern "C" fn(this: *mut IMTDocument, date: INT64) -> MTAPIRES,
    pub IMTDocument_DateIssue: unsafe extern "C" fn(this: *const IMTDocument) -> INT64,
    pub IMTDocument_DateExpiration1:
        unsafe extern "C" fn(this: *mut IMTDocument, date: INT64) -> MTAPIRES,
    pub IMTDocument_DateExpiration: unsafe extern "C" fn(this: *const IMTDocument) -> INT64,
    pub IMTDocument_DocumentType1:
        unsafe extern "C" fn(this: *mut IMTDocument, type_: UINT) -> MTAPIRES,
    pub IMTDocument_DocumentType: unsafe extern "C" fn(this: *const IMTDocument) -> UINT,
    pub IMTDocument_DocumentName1:
        unsafe extern "C" fn(this: *mut IMTDocument, name: LPCWSTR) -> MTAPIRES,
    pub IMTDocument_DocumentName: unsafe extern "C" fn(this: *const IMTDocument) -> LPCWSTR,
    pub IMTDocument_DocumentComment1:
        unsafe extern "C" fn(this: *mut IMTDocument, comment: LPCWSTR) -> MTAPIRES,
    pub IMTDocument_DocumentComment: unsafe extern "C" fn(this: *const IMTDocument) -> LPCWSTR,
    pub IMTDocument_DocumentStatus: unsafe extern "C" fn(this: *const IMTDocument) -> UINT,
    pub IMTDocument_DocumentStatus1:
        unsafe extern "C" fn(this: *mut IMTDocument, status: UINT) -> MTAPIRES,
    pub IMTDocument_AttachmentsAdd:
        unsafe extern "C" fn(this: *mut IMTDocument, attachment: *const IMTAttachment) -> MTAPIRES,
    pub IMTDocument_AttachmentsClear: unsafe extern "C" fn(this: *mut IMTDocument) -> MTAPIRES,
    pub IMTDocument_AttachmentsTotal: unsafe extern "C" fn(this: *const IMTDocument) -> UINT,
    pub IMTDocument_AttachmentsNext: unsafe extern "C" fn(
        this: *const IMTDocument,
        pos: UINT,
        attachment_id: *mut UINT64,
        attachment_name: *mut MTAPISTR,
        attachment_size: *mut UINT,
    ) -> MTAPIRES,
    pub IMTDocument_DocumentSubtype1:
        unsafe extern "C" fn(this: *mut IMTDocument, subtype: UINT) -> MTAPIRES,
    pub IMTDocument_DocumentSubtype: unsafe extern "C" fn(this: *const IMTDocument) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDocument {
    pub vtable_: *const IMTDocument__bindgen_vtable,
}
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_OTHER: IMTDocument_EnDocumentTypes = 0;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_PERSONAL_IDENTITY: IMTDocument_EnDocumentTypes =
    1;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_PERSONAL_ADDRESS: IMTDocument_EnDocumentTypes =
    2;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_REGISTERED_ADDRESS:
    IMTDocument_EnDocumentTypes = 1000;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_DIRECTORS_PASSPORT:
    IMTDocument_EnDocumentTypes = 1001;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_CERTIFICATE_OF_INCORPORATION:
    IMTDocument_EnDocumentTypes = 1002;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_CERTIFICATE_OF_DIRECTORS:
    IMTDocument_EnDocumentTypes = 1003;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_CERTIFICATE_OF_GOOD_STANDING:
    IMTDocument_EnDocumentTypes = 1004;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_FIRST: IMTDocument_EnDocumentTypes = 0;
pub const IMTDocument_EnDocumentTypes_DOCUMENT_TYPE_LAST: IMTDocument_EnDocumentTypes = 1004;
pub type IMTDocument_EnDocumentTypes = ::std::os::raw::c_int;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_OTHER: IMTDocument_EnDocumentSubtype = 0;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_ID_CARD: IMTDocument_EnDocumentSubtype = 1;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_PASSPORT: IMTDocument_EnDocumentSubtype =
    2;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_DRIVERS: IMTDocument_EnDocumentSubtype = 3;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_BANK_CARD: IMTDocument_EnDocumentSubtype =
    4;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_UTILITY_BILL:
    IMTDocument_EnDocumentSubtype = 5;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_BANK_STATEMENT:
    IMTDocument_EnDocumentSubtype = 6;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_TAX_STATEMENT:
    IMTDocument_EnDocumentSubtype = 7;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_SELFIE: IMTDocument_EnDocumentSubtype = 8;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_PROFILE_IMAGE:
    IMTDocument_EnDocumentSubtype = 9;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_ID_DOC_PHOTO:
    IMTDocument_EnDocumentSubtype = 10;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_AGREEMENT: IMTDocument_EnDocumentSubtype =
    11;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_CONTRACT: IMTDocument_EnDocumentSubtype =
    12;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_RESIDENCE_PERMIT:
    IMTDocument_EnDocumentSubtype = 13;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_EMPLOYMENT_CERTIFICATE:
    IMTDocument_EnDocumentSubtype = 14;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_DRIVERS_TRANSLATION:
    IMTDocument_EnDocumentSubtype = 15;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_INVESTOR_DOC:
    IMTDocument_EnDocumentSubtype = 16;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_VEHICLE_REG_CERTIFICATE:
    IMTDocument_EnDocumentSubtype = 17;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_INCOME_SOURCE:
    IMTDocument_EnDocumentSubtype = 18;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_PAYMENT_METHOD:
    IMTDocument_EnDocumentSubtype = 19;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_FIRST: IMTDocument_EnDocumentSubtype = 0;
pub const IMTDocument_EnDocumentSubtype_DOCUMENT_SUBTYPE_LAST: IMTDocument_EnDocumentSubtype = 19;
pub type IMTDocument_EnDocumentSubtype = ::std::os::raw::c_int;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_NEW: IMTDocument_EnDocumentStatus = 0;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_APPROVED: IMTDocument_EnDocumentStatus = 100;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_REJECTED: IMTDocument_EnDocumentStatus = 200;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_ARCHIVED: IMTDocument_EnDocumentStatus = 300;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_DELETED: IMTDocument_EnDocumentStatus = 400;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_FIRST: IMTDocument_EnDocumentStatus = 0;
pub const IMTDocument_EnDocumentStatus_DOCUMENT_STATUS_LAST: IMTDocument_EnDocumentStatus = 400;
pub type IMTDocument_EnDocumentStatus = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDocument"][::std::mem::size_of::<IMTDocument>() - 8usize];
    ["Alignment of IMTDocument"][::std::mem::align_of::<IMTDocument>() - 8usize];
};
#[repr(C)]
pub struct IMTDocumentArray__bindgen_vtable {
    pub IMTDocumentArray_Release: unsafe extern "C" fn(this: *mut IMTDocumentArray),
    pub IMTDocumentArray_Assign: unsafe extern "C" fn(
        this: *mut IMTDocumentArray,
        array: *const IMTDocumentArray,
    ) -> MTAPIRES,
    pub IMTDocumentArray_Clear: unsafe extern "C" fn(this: *mut IMTDocumentArray) -> MTAPIRES,
    pub IMTDocumentArray_Add:
        unsafe extern "C" fn(this: *mut IMTDocumentArray, document: *mut IMTDocument) -> MTAPIRES,
    pub IMTDocumentArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTDocumentArray, document: *const IMTDocument) -> MTAPIRES,
    pub IMTDocumentArray_Add1:
        unsafe extern "C" fn(this: *mut IMTDocumentArray, array: *mut IMTDocumentArray) -> MTAPIRES,
    pub IMTDocumentArray_AddCopy1: unsafe extern "C" fn(
        this: *mut IMTDocumentArray,
        array: *const IMTDocumentArray,
    ) -> MTAPIRES,
    pub IMTDocumentArray_Delete:
        unsafe extern "C" fn(this: *mut IMTDocumentArray, pos: UINT) -> MTAPIRES,
    pub IMTDocumentArray_Detach:
        unsafe extern "C" fn(this: *mut IMTDocumentArray, pos: UINT) -> *mut IMTDocument,
    pub IMTDocumentArray_Update: unsafe extern "C" fn(
        this: *mut IMTDocumentArray,
        pos: UINT,
        document: *mut IMTDocument,
    ) -> MTAPIRES,
    pub IMTDocumentArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTDocumentArray,
        pos: UINT,
        document: *const IMTDocument,
    ) -> MTAPIRES,
    pub IMTDocumentArray_Shift: unsafe extern "C" fn(
        this: *mut IMTDocumentArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTDocumentArray_Total: unsafe extern "C" fn(this: *const IMTDocumentArray) -> UINT,
    pub IMTDocumentArray_Next:
        unsafe extern "C" fn(this: *const IMTDocumentArray, index: UINT) -> *mut IMTDocument,
    pub IMTDocumentArray_Sort: unsafe extern "C" fn(
        this: *mut IMTDocumentArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTDocumentArray_Search: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDocumentArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDocumentArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDocumentArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDocumentArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDocumentArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTDocumentArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTDocumentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTDocumentArray {
    pub vtable_: *const IMTDocumentArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDocumentArray"][::std::mem::size_of::<IMTDocumentArray>() - 8usize];
    ["Alignment of IMTDocumentArray"][::std::mem::align_of::<IMTDocumentArray>() - 8usize];
};
#[repr(C)]
pub struct IMTDocumentSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTDocumentSink {
    pub vtable_: *const IMTDocumentSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTDocumentSink"][::std::mem::size_of::<IMTDocumentSink>() - 8usize];
    ["Alignment of IMTDocumentSink"][::std::mem::align_of::<IMTDocumentSink>() - 8usize];
};
#[repr(C)]
pub struct IMTComment__bindgen_vtable {
    pub IMTComment_Release: unsafe extern "C" fn(this: *mut IMTComment),
    pub IMTComment_Assign:
        unsafe extern "C" fn(this: *mut IMTComment, comment: *const IMTComment) -> MTAPIRES,
    pub IMTComment_Clear: unsafe extern "C" fn(this: *mut IMTComment) -> MTAPIRES,
    pub IMTComment_RecordID1:
        unsafe extern "C" fn(this: *mut IMTComment, record_id: UINT64) -> MTAPIRES,
    pub IMTComment_RecordID: unsafe extern "C" fn(this: *const IMTComment) -> UINT64,
    pub IMTComment_RelatedClient1:
        unsafe extern "C" fn(this: *mut IMTComment, record_id: UINT64) -> MTAPIRES,
    pub IMTComment_RelatedClient: unsafe extern "C" fn(this: *const IMTComment) -> UINT64,
    pub IMTComment_RelatedDocument1:
        unsafe extern "C" fn(this: *mut IMTComment, record_id: UINT64) -> MTAPIRES,
    pub IMTComment_RelatedDocument: unsafe extern "C" fn(this: *const IMTComment) -> UINT64,
    pub IMTComment_Flags1: unsafe extern "C" fn(this: *mut IMTComment, flags: UINT) -> MTAPIRES,
    pub IMTComment_Flags: unsafe extern "C" fn(this: *const IMTComment) -> UINT,
    pub IMTComment_Extra1: unsafe extern "C" fn(this: *mut IMTComment, extra: LPCWSTR) -> MTAPIRES,
    pub IMTComment_Extra: unsafe extern "C" fn(this: *const IMTComment) -> LPCWSTR,
    pub IMTComment_Text1: unsafe extern "C" fn(this: *mut IMTComment, text: LPCWSTR) -> MTAPIRES,
    pub IMTComment_Text: unsafe extern "C" fn(this: *const IMTComment) -> LPCWSTR,
    pub IMTComment_CommentType: unsafe extern "C" fn(this: *const IMTComment) -> UINT,
    pub IMTComment_CommentType1:
        unsafe extern "C" fn(this: *mut IMTComment, type_: UINT) -> MTAPIRES,
    pub IMTComment_CommentResult: unsafe extern "C" fn(this: *const IMTComment) -> UINT,
    pub IMTComment_CommentResult1:
        unsafe extern "C" fn(this: *mut IMTComment, result: UINT) -> MTAPIRES,
    pub IMTComment_AttachmentsAdd:
        unsafe extern "C" fn(this: *mut IMTComment, attachment: *const IMTAttachment) -> MTAPIRES,
    pub IMTComment_AttachmentsClear: unsafe extern "C" fn(this: *mut IMTComment) -> MTAPIRES,
    pub IMTComment_AttachmentsTotal: unsafe extern "C" fn(this: *const IMTComment) -> UINT,
    pub IMTComment_AttachmentsNext: unsafe extern "C" fn(
        this: *const IMTComment,
        pos: UINT,
        attachment_id: *mut UINT64,
        attachment_name: *mut MTAPISTR,
        attachment_size: *mut UINT,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTComment {
    pub vtable_: *const IMTComment__bindgen_vtable,
}
pub const IMTComment_EnCommentFlags_COMMENT_FLAG_DELETED: IMTComment_EnCommentFlags = 1;
pub const IMTComment_EnCommentFlags_COMMENT_FLAG_IMPORTANT: IMTComment_EnCommentFlags = 2;
pub const IMTComment_EnCommentFlags_COMMENT_FLAG_NONE: IMTComment_EnCommentFlags = 0;
pub const IMTComment_EnCommentFlags_COMMENT_FLAG_ALL: IMTComment_EnCommentFlags = 3;
pub type IMTComment_EnCommentFlags = ::std::os::raw::c_int;
pub const IMTComment_EnCommentType_COMMENT_TYPE_UNDEFINED: IMTComment_EnCommentType = 0;
pub const IMTComment_EnCommentType_COMMENT_TYPE_LOGRECORD: IMTComment_EnCommentType = 1;
pub const IMTComment_EnCommentType_COMMENT_TYPE_CALLRECORD: IMTComment_EnCommentType = 2;
pub const IMTComment_EnCommentType_COMMENT_TYPE_ROBOTRECORD: IMTComment_EnCommentType = 3;
pub const IMTComment_EnCommentType_COMMENT_TYPE_FIRST: IMTComment_EnCommentType = 0;
pub const IMTComment_EnCommentType_COMMENT_TYPE_LAST: IMTComment_EnCommentType = 3;
pub type IMTComment_EnCommentType = ::std::os::raw::c_int;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_UNDEFINED: IMTComment_EnCommentResult = 0;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_CALL_NO_ANSWER: IMTComment_EnCommentResult = 1;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_CALL_WRONG_NUMBER: IMTComment_EnCommentResult =
    2;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_CALL_NOT_INTERESTED:
    IMTComment_EnCommentResult = 3;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_CALL_SUCCESSFUL: IMTComment_EnCommentResult = 4;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_FIRST: IMTComment_EnCommentResult = 0;
pub const IMTComment_EnCommentResult_COMMENT_RESULT_LAST: IMTComment_EnCommentResult = 4;
pub type IMTComment_EnCommentResult = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTComment"][::std::mem::size_of::<IMTComment>() - 8usize];
    ["Alignment of IMTComment"][::std::mem::align_of::<IMTComment>() - 8usize];
};
#[repr(C)]
pub struct IMTCommentArray__bindgen_vtable {
    pub IMTCommentArray_Release: unsafe extern "C" fn(this: *mut IMTCommentArray),
    pub IMTCommentArray_Assign:
        unsafe extern "C" fn(this: *mut IMTCommentArray, array: *const IMTCommentArray) -> MTAPIRES,
    pub IMTCommentArray_Clear: unsafe extern "C" fn(this: *mut IMTCommentArray) -> MTAPIRES,
    pub IMTCommentArray_Add1:
        unsafe extern "C" fn(this: *mut IMTCommentArray, array: *mut IMTCommentArray) -> MTAPIRES,
    pub IMTCommentArray_Add:
        unsafe extern "C" fn(this: *mut IMTCommentArray, comment: *mut IMTComment) -> MTAPIRES,
    pub IMTCommentArray_AddCopy1:
        unsafe extern "C" fn(this: *mut IMTCommentArray, array: *const IMTCommentArray) -> MTAPIRES,
    pub IMTCommentArray_AddCopy:
        unsafe extern "C" fn(this: *mut IMTCommentArray, comment: *const IMTComment) -> MTAPIRES,
    pub IMTCommentArray_Delete:
        unsafe extern "C" fn(this: *mut IMTCommentArray, pos: UINT) -> MTAPIRES,
    pub IMTCommentArray_Detach:
        unsafe extern "C" fn(this: *mut IMTCommentArray, pos: UINT) -> *mut IMTComment,
    pub IMTCommentArray_Update: unsafe extern "C" fn(
        this: *mut IMTCommentArray,
        pos: UINT,
        comment: *mut IMTComment,
    ) -> MTAPIRES,
    pub IMTCommentArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTCommentArray,
        pos: UINT,
        comment: *const IMTComment,
    ) -> MTAPIRES,
    pub IMTCommentArray_Shift: unsafe extern "C" fn(
        this: *mut IMTCommentArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTCommentArray_Total: unsafe extern "C" fn(this: *const IMTCommentArray) -> UINT,
    pub IMTCommentArray_Next:
        unsafe extern "C" fn(this: *const IMTCommentArray, index: UINT) -> *mut IMTComment,
    pub IMTCommentArray_Sort: unsafe extern "C" fn(
        this: *mut IMTCommentArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTCommentArray_Search: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTCommentArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTCommentArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTCommentArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTCommentArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTCommentArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTCommentArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTCommentArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTCommentArray {
    pub vtable_: *const IMTCommentArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTCommentArray"][::std::mem::size_of::<IMTCommentArray>() - 8usize];
    ["Alignment of IMTCommentArray"][::std::mem::align_of::<IMTCommentArray>() - 8usize];
};
#[repr(C)]
pub struct IMTCommentSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTCommentSink {
    pub vtable_: *const IMTCommentSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTCommentSink"][::std::mem::size_of::<IMTCommentSink>() - 8usize];
    ["Alignment of IMTCommentSink"][::std::mem::align_of::<IMTCommentSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConAutoCondition__bindgen_vtable {
    pub IMTConAutoCondition_Release: unsafe extern "C" fn(this: *mut IMTConAutoCondition),
    pub IMTConAutoCondition_Assign: unsafe extern "C" fn(
        this: *mut IMTConAutoCondition,
        condition: *const IMTConAutoCondition,
    ) -> MTAPIRES,
    pub IMTConAutoCondition_Clear: unsafe extern "C" fn(this: *mut IMTConAutoCondition) -> MTAPIRES,
    pub IMTConAutoCondition_Condition1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, condition: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_Condition:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_Rule1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, rule: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_Rule: unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueType:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueInt1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: INT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueInt:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> INT64,
    pub IMTConAutoCondition_ValueUInt1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueUInt:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT64,
    pub IMTConAutoCondition_ValueDouble1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: f64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueDouble:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> f64,
    pub IMTConAutoCondition_ValueString1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: LPCWSTR) -> MTAPIRES,
    pub IMTConAutoCondition_ValueString:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> LPCWSTR,
    pub IMTConAutoCondition_ValueColor1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: COLORREF) -> MTAPIRES,
    pub IMTConAutoCondition_ValueColor:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> COLORREF,
    pub IMTConAutoCondition_ValueMoney1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: f64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueMoney:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> f64,
    pub IMTConAutoCondition_ValueVolume1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueVolume:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT64,
    pub IMTConAutoCondition_ValueDatetime1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: INT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueDatetime:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> INT64,
    pub IMTConAutoCondition_ValueLeverage1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: INT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueLeverage:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> INT64,
    pub IMTConAutoCondition_ValueBool1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: bool) -> MTAPIRES,
    pub IMTConAutoCondition_ValueBool:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> bool,
    pub IMTConAutoCondition_ValueTime1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueTime:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueDate1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: INT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueDate:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> INT64,
    pub IMTConAutoCondition_ValuePercent1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValuePercent:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueLanguage1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueLanguage:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueServer1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT64) -> MTAPIRES,
    pub IMTConAutoCondition_ValueServer:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT64,
    pub IMTConAutoCondition_ValuePositionType1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValuePositionType:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueReason1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueReason:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueDealType1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueDealType:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueDealEntry1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueDealEntry:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueOrderType1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueOrderType:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
    pub IMTConAutoCondition_ValueOrderState1:
        unsafe extern "C" fn(this: *mut IMTConAutoCondition, value: UINT) -> MTAPIRES,
    pub IMTConAutoCondition_ValueOrderState:
        unsafe extern "C" fn(this: *const IMTConAutoCondition) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConAutoCondition {
    pub vtable_: *const IMTConAutoCondition__bindgen_vtable,
}
pub const IMTConAutoCondition_EnConditions_CONDITION_DATETIME_DATETIME:
    IMTConAutoCondition_EnConditions = 0;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATETIME_DATE:
    IMTConAutoCondition_EnConditions = 1;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATETIME_TIME:
    IMTConAutoCondition_EnConditions = 2;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATETIME_FIRST:
    IMTConAutoCondition_EnConditions = 0;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATETIME_LAST:
    IMTConAutoCondition_EnConditions = 1000;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LOGIN:
    IMTConAutoCondition_EnConditions = 1001;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_GROUP:
    IMTConAutoCondition_EnConditions = 1002;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_COUNTRY:
    IMTConAutoCondition_EnConditions = 1003;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_CITY:
    IMTConAutoCondition_EnConditions = 1004;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LANGUAGE:
    IMTConAutoCondition_EnConditions = 1005;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_PHONE:
    IMTConAutoCondition_EnConditions = 1006;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_EMAIL:
    IMTConAutoCondition_EnConditions = 1007;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_COLOR:
    IMTConAutoCondition_EnConditions = 1008;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_COMMENT:
    IMTConAutoCondition_EnConditions = 1009;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_REGISTRATION:
    IMTConAutoCondition_EnConditions = 1010;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LASTTIME:
    IMTConAutoCondition_EnConditions = 1011;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LEVERAGE:
    IMTConAutoCondition_EnConditions = 1012;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_BALANCE:
    IMTConAutoCondition_EnConditions = 1013;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_CREDIT:
    IMTConAutoCondition_EnConditions = 1014;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_POSITIONS:
    IMTConAutoCondition_EnConditions = 1015;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_ORDERS:
    IMTConAutoCondition_EnConditions = 1016;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_PROFIT:
    IMTConAutoCondition_EnConditions = 1017;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_EQUITY:
    IMTConAutoCondition_EnConditions = 1018;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_MARGIN:
    IMTConAutoCondition_EnConditions = 1019;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_MARGIN_FREE:
    IMTConAutoCondition_EnConditions = 1020;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_MARGIN_LEVEL:
    IMTConAutoCondition_EnConditions = 1021;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_CURRENCY:
    IMTConAutoCondition_EnConditions = 1022;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_REGISTRATION_ELAPSED:
    IMTConAutoCondition_EnConditions = 1023;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LASTTIME_ELAPSED:
    IMTConAutoCondition_EnConditions = 1024;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LEAD_SOURCE:
    IMTConAutoCondition_EnConditions = 1025;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LEAD_CAMPAIGN:
    IMTConAutoCondition_EnConditions = 1026;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_ONLINE:
    IMTConAutoCondition_EnConditions = 1027;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_ACTIVITY_TRADE_ELAPSED:
    IMTConAutoCondition_EnConditions = 1028;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_ENABLED:
    IMTConAutoCondition_EnConditions = 1029;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_TRADING_ENABLED:
    IMTConAutoCondition_EnConditions = 1030;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_EXPERT_ENABLED:
    IMTConAutoCondition_EnConditions = 1031;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_OWN_FUNDS_PERCENT:
    IMTConAutoCondition_EnConditions = 1032;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_OWN_FUNDS_VOLUME:
    IMTConAutoCondition_EnConditions = 1033;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_FIRST:
    IMTConAutoCondition_EnConditions = 1001;
pub const IMTConAutoCondition_EnConditions_CONDITION_ACCOUNT_LAST:
    IMTConAutoCondition_EnConditions = 2000;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_CPU: IMTConAutoCondition_EnConditions =
    2001;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_CPU_PROCESS:
    IMTConAutoCondition_EnConditions = 2002;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_CPU_PROCESS_THREADS:
    IMTConAutoCondition_EnConditions = 2003;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_MEMORY_FREE:
    IMTConAutoCondition_EnConditions = 2004;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_MEMORY_PROCESS:
    IMTConAutoCondition_EnConditions = 2005;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_DISK_FREE:
    IMTConAutoCondition_EnConditions = 2006;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_DISK_SPEED_READ:
    IMTConAutoCondition_EnConditions = 2007;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_DISK_SPEED_WRITE:
    IMTConAutoCondition_EnConditions = 2008;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_DISK_QUEUE_LENGTH:
    IMTConAutoCondition_EnConditions = 2009;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_NETWORK_CONNECTIONS:
    IMTConAutoCondition_EnConditions = 2010;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_NETWORK_BLOCKED:
    IMTConAutoCondition_EnConditions = 2011;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_NETWORK_SOCKETS:
    IMTConAutoCondition_EnConditions = 2012;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_NETWORK_TRAFFIC_IN:
    IMTConAutoCondition_EnConditions = 2013;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_NETWORK_TRAFFIC_OUT:
    IMTConAutoCondition_EnConditions = 2014;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_NETWORK_RETRANSMIT:
    IMTConAutoCondition_EnConditions = 2015;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_HANDLES_PROCESS:
    IMTConAutoCondition_EnConditions = 2016;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_CPU_DPC:
    IMTConAutoCondition_EnConditions = 2017;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_CPU_INTERRUPTS:
    IMTConAutoCondition_EnConditions = 2018;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_FIRST:
    IMTConAutoCondition_EnConditions = 2001;
pub const IMTConAutoCondition_EnConditions_CONDITION_MONITOR_LAST:
    IMTConAutoCondition_EnConditions = 3000;
pub const IMTConAutoCondition_EnConditions_CONDITION_SERVER_ID: IMTConAutoCondition_EnConditions =
    3001;
pub const IMTConAutoCondition_EnConditions_CONDITION_SERVER_TYPE: IMTConAutoCondition_EnConditions =
    3002;
pub const IMTConAutoCondition_EnConditions_CONDITION_SERVER_CONNECTED:
    IMTConAutoCondition_EnConditions = 3003;
pub const IMTConAutoCondition_EnConditions_CONDITION_SERVER_FIRST:
    IMTConAutoCondition_EnConditions = 3001;
pub const IMTConAutoCondition_EnConditions_CONDITION_SERVER_LAST: IMTConAutoCondition_EnConditions =
    4000;
pub const IMTConAutoCondition_EnConditions_CONDITION_FINANCE_AMOUNT:
    IMTConAutoCondition_EnConditions = 4001;
pub const IMTConAutoCondition_EnConditions_CONDITION_FINANCE_CURRENCY:
    IMTConAutoCondition_EnConditions = 4002;
pub const IMTConAutoCondition_EnConditions_CONDITION_FINANCE_FIRST:
    IMTConAutoCondition_EnConditions = 4001;
pub const IMTConAutoCondition_EnConditions_CONDITION_FINANCE_LAST:
    IMTConAutoCondition_EnConditions = 5000;
pub const IMTConAutoCondition_EnConditions_CONDITION_PRICES_SYMBOL:
    IMTConAutoCondition_EnConditions = 5001;
pub const IMTConAutoCondition_EnConditions_CONDITION_PRICES_LASTTIME:
    IMTConAutoCondition_EnConditions = 5002;
pub const IMTConAutoCondition_EnConditions_CONDITION_PRICES_FIRST:
    IMTConAutoCondition_EnConditions = 5001;
pub const IMTConAutoCondition_EnConditions_CONDITION_PRICES_LAST: IMTConAutoCondition_EnConditions =
    6000;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_LOGIN:
    IMTConAutoCondition_EnConditions = 6001;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_TICKET:
    IMTConAutoCondition_EnConditions = 6002;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_SYMBOL:
    IMTConAutoCondition_EnConditions = 6003;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_VOLUME:
    IMTConAutoCondition_EnConditions = 6004;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_TYPE:
    IMTConAutoCondition_EnConditions = 6005;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_PRICE_OPEN:
    IMTConAutoCondition_EnConditions = 6006;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_PRICE_CURRENT:
    IMTConAutoCondition_EnConditions = 6007;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_PROFIT:
    IMTConAutoCondition_EnConditions = 6008;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_REASON:
    IMTConAutoCondition_EnConditions = 6009;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_TIME_CREATE:
    IMTConAutoCondition_EnConditions = 6010;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_TIME_UPDATE:
    IMTConAutoCondition_EnConditions = 6011;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_EXPERT:
    IMTConAutoCondition_EnConditions = 6012;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_TIME_CREATE_ELAPSED:
    IMTConAutoCondition_EnConditions = 6013;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_TIME_UPDATE_ELAPSED:
    IMTConAutoCondition_EnConditions = 6014;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_FIRST:
    IMTConAutoCondition_EnConditions = 6001;
pub const IMTConAutoCondition_EnConditions_CONDITION_POSITION_LAST:
    IMTConAutoCondition_EnConditions = 7000;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_LOGIN: IMTConAutoCondition_EnConditions =
    7001;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_TICKET: IMTConAutoCondition_EnConditions =
    7002;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_SYMBOL: IMTConAutoCondition_EnConditions =
    7003;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_VOLUME: IMTConAutoCondition_EnConditions =
    7004;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_TYPE: IMTConAutoCondition_EnConditions =
    7005;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_ENTRY: IMTConAutoCondition_EnConditions =
    7006;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_PRICE: IMTConAutoCondition_EnConditions =
    7007;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_PROFIT: IMTConAutoCondition_EnConditions =
    7008;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_REASON: IMTConAutoCondition_EnConditions =
    7009;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_TIME: IMTConAutoCondition_EnConditions =
    7010;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_EXPERT: IMTConAutoCondition_EnConditions =
    7011;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_FIRST: IMTConAutoCondition_EnConditions =
    7001;
pub const IMTConAutoCondition_EnConditions_CONDITION_DEAL_LAST: IMTConAutoCondition_EnConditions =
    8000;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_FROM:
    IMTConAutoCondition_EnConditions = 8001;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_FROM_NAME:
    IMTConAutoCondition_EnConditions = 8002;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_TO: IMTConAutoCondition_EnConditions =
    8003;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_TO_NAME:
    IMTConAutoCondition_EnConditions = 8004;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_SUBJECT:
    IMTConAutoCondition_EnConditions = 8005;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_BODY:
    IMTConAutoCondition_EnConditions = 8006;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_FIRST:
    IMTConAutoCondition_EnConditions = 8001;
pub const IMTConAutoCondition_EnConditions_CONDITION_MESSAGE_LAST:
    IMTConAutoCondition_EnConditions = 9000;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_TICKET:
    IMTConAutoCondition_EnConditions = 9001;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_ORDER_ID:
    IMTConAutoCondition_EnConditions = 9002;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_LOGIN: IMTConAutoCondition_EnConditions =
    9003;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_SYMBOL:
    IMTConAutoCondition_EnConditions = 9004;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_TIME_SETUP:
    IMTConAutoCondition_EnConditions = 9005;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_TIME_SETUP_ELAPSED:
    IMTConAutoCondition_EnConditions = 9006;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_TIME_EXPIRATION:
    IMTConAutoCondition_EnConditions = 9007;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_TYPE: IMTConAutoCondition_EnConditions =
    9008;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_PRICE_ORDER:
    IMTConAutoCondition_EnConditions = 9009;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_PRICE_TRIGGER:
    IMTConAutoCondition_EnConditions = 9010;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_PRICE_CURRENT:
    IMTConAutoCondition_EnConditions = 9011;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_PRICE_SL:
    IMTConAutoCondition_EnConditions = 9012;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_PRICE_TP:
    IMTConAutoCondition_EnConditions = 9013;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_VOLUME_INITIAL:
    IMTConAutoCondition_EnConditions = 9014;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_VOLUME_CURRENT:
    IMTConAutoCondition_EnConditions = 9015;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_STATE: IMTConAutoCondition_EnConditions =
    9016;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_EXPERT:
    IMTConAutoCondition_EnConditions = 9017;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_POSITION_ID:
    IMTConAutoCondition_EnConditions = 9018;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_COMMENT:
    IMTConAutoCondition_EnConditions = 9019;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_CONTRACT_SIZE:
    IMTConAutoCondition_EnConditions = 9020;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_CURRENCY:
    IMTConAutoCondition_EnConditions = 9021;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_FIRST: IMTConAutoCondition_EnConditions =
    9001;
pub const IMTConAutoCondition_EnConditions_CONDITION_ORDER_LAST: IMTConAutoCondition_EnConditions =
    10000;
pub const IMTConAutoCondition_EnConditions_CONDITION_GATEWAY_NAME:
    IMTConAutoCondition_EnConditions = 10001;
pub const IMTConAutoCondition_EnConditions_CONDITION_GATEWAY_ID: IMTConAutoCondition_EnConditions =
    10002;
pub const IMTConAutoCondition_EnConditions_CONDITION_GATEWAY_CONNECTED:
    IMTConAutoCondition_EnConditions = 10003;
pub const IMTConAutoCondition_EnConditions_CONDITION_GATEWAY_FIRST:
    IMTConAutoCondition_EnConditions = 10001;
pub const IMTConAutoCondition_EnConditions_CONDITION_GATEWAY_LAST:
    IMTConAutoCondition_EnConditions = 11000;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATAFEED_NAME:
    IMTConAutoCondition_EnConditions = 11001;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATAFEED_CONNECTED:
    IMTConAutoCondition_EnConditions = 11002;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATAFEED_FIRST:
    IMTConAutoCondition_EnConditions = 11001;
pub const IMTConAutoCondition_EnConditions_CONDITION_DATAFEED_LAST:
    IMTConAutoCondition_EnConditions = 12000;
pub const IMTConAutoCondition_EnConditions_CONDITION_KYC_STATE_CODE:
    IMTConAutoCondition_EnConditions = 12001;
pub const IMTConAutoCondition_EnConditions_CONDITION_KYC_STATE_DESC:
    IMTConAutoCondition_EnConditions = 12002;
pub const IMTConAutoCondition_EnConditions_CONDITION_KYC_REJECT_REASON:
    IMTConAutoCondition_EnConditions = 12003;
pub const IMTConAutoCondition_EnConditions_CONDITION_KYC_FIRST: IMTConAutoCondition_EnConditions =
    12001;
pub const IMTConAutoCondition_EnConditions_CONDITION_KYC_LAST: IMTConAutoCondition_EnConditions =
    13000;
pub const IMTConAutoCondition_EnConditions_CONDITION_FIRST: IMTConAutoCondition_EnConditions = 0;
pub const IMTConAutoCondition_EnConditions_CONDITION_LAST: IMTConAutoCondition_EnConditions = 13000;
pub type IMTConAutoCondition_EnConditions = ::std::os::raw::c_int;
pub const IMTConAutoCondition_EnConditionRule_RULE_EQ: IMTConAutoCondition_EnConditionRule = 0;
pub const IMTConAutoCondition_EnConditionRule_RULE_NOT_EQ: IMTConAutoCondition_EnConditionRule = 1;
pub const IMTConAutoCondition_EnConditionRule_RULE_GREATER: IMTConAutoCondition_EnConditionRule = 2;
pub const IMTConAutoCondition_EnConditionRule_RULE_NOT_LESS: IMTConAutoCondition_EnConditionRule =
    3;
pub const IMTConAutoCondition_EnConditionRule_RULE_LESS: IMTConAutoCondition_EnConditionRule = 4;
pub const IMTConAutoCondition_EnConditionRule_RULE_NOT_GREATER:
    IMTConAutoCondition_EnConditionRule = 5;
pub const IMTConAutoCondition_EnConditionRule_RULE_MATCH_MASK: IMTConAutoCondition_EnConditionRule =
    6;
pub const IMTConAutoCondition_EnConditionRule_RULE_FIRST: IMTConAutoCondition_EnConditionRule = 0;
pub const IMTConAutoCondition_EnConditionRule_RULE_LAST: IMTConAutoCondition_EnConditionRule = 6;
pub type IMTConAutoCondition_EnConditionRule = ::std::os::raw::c_int;
pub const IMTConAutoCondition_EnConditionType_TYPE_NONE: IMTConAutoCondition_EnConditionType = 0;
pub const IMTConAutoCondition_EnConditionType_TYPE_STRING: IMTConAutoCondition_EnConditionType = 1;
pub const IMTConAutoCondition_EnConditionType_TYPE_INT: IMTConAutoCondition_EnConditionType = 2;
pub const IMTConAutoCondition_EnConditionType_TYPE_UINT: IMTConAutoCondition_EnConditionType = 3;
pub const IMTConAutoCondition_EnConditionType_TYPE_DOUBLE: IMTConAutoCondition_EnConditionType = 4;
pub const IMTConAutoCondition_EnConditionType_TYPE_COLOR: IMTConAutoCondition_EnConditionType = 5;
pub const IMTConAutoCondition_EnConditionType_TYPE_MONEY: IMTConAutoCondition_EnConditionType = 6;
pub const IMTConAutoCondition_EnConditionType_TYPE_VOLUME: IMTConAutoCondition_EnConditionType = 7;
pub const IMTConAutoCondition_EnConditionType_TYPE_DATETIME: IMTConAutoCondition_EnConditionType =
    8;
pub const IMTConAutoCondition_EnConditionType_TYPE_LEVERAGE: IMTConAutoCondition_EnConditionType =
    9;
pub const IMTConAutoCondition_EnConditionType_TYPE_BOOL: IMTConAutoCondition_EnConditionType = 10;
pub const IMTConAutoCondition_EnConditionType_TYPE_TIME: IMTConAutoCondition_EnConditionType = 11;
pub const IMTConAutoCondition_EnConditionType_TYPE_DATE: IMTConAutoCondition_EnConditionType = 12;
pub const IMTConAutoCondition_EnConditionType_TYPE_PERCENT: IMTConAutoCondition_EnConditionType =
    13;
pub const IMTConAutoCondition_EnConditionType_TYPE_LANGUAGE: IMTConAutoCondition_EnConditionType =
    14;
pub const IMTConAutoCondition_EnConditionType_TYPE_SERVER: IMTConAutoCondition_EnConditionType = 15;
pub const IMTConAutoCondition_EnConditionType_TYPE_POSITION: IMTConAutoCondition_EnConditionType =
    16;
pub const IMTConAutoCondition_EnConditionType_TYPE_REASON: IMTConAutoCondition_EnConditionType = 17;
pub const IMTConAutoCondition_EnConditionType_TYPE_DEAL: IMTConAutoCondition_EnConditionType = 18;
pub const IMTConAutoCondition_EnConditionType_TYPE_DEAL_ENTRY: IMTConAutoCondition_EnConditionType =
    19;
pub const IMTConAutoCondition_EnConditionType_TYPE_ORDER: IMTConAutoCondition_EnConditionType = 20;
pub const IMTConAutoCondition_EnConditionType_TYPE_ORDER_STATE:
    IMTConAutoCondition_EnConditionType = 21;
pub const IMTConAutoCondition_EnConditionType_TYPE_KYC_STATE: IMTConAutoCondition_EnConditionType =
    22;
pub const IMTConAutoCondition_EnConditionType_TYPE_FIRST: IMTConAutoCondition_EnConditionType = 0;
pub const IMTConAutoCondition_EnConditionType_TYPE_LAST: IMTConAutoCondition_EnConditionType = 22;
pub type IMTConAutoCondition_EnConditionType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConAutoCondition"][::std::mem::size_of::<IMTConAutoCondition>() - 8usize];
    ["Alignment of IMTConAutoCondition"][::std::mem::align_of::<IMTConAutoCondition>() - 8usize];
};
#[repr(C)]
pub struct IMTConAutoParam__bindgen_vtable {
    pub IMTConAutoParam_Release: unsafe extern "C" fn(this: *mut IMTConAutoParam),
    pub IMTConAutoParam_Assign:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, param: *const IMTConAutoParam) -> MTAPIRES,
    pub IMTConAutoParam_Clear: unsafe extern "C" fn(this: *mut IMTConAutoParam) -> MTAPIRES,
    pub IMTConAutoParam_Param1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, param: UINT) -> MTAPIRES,
    pub IMTConAutoParam_Param: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT,
    pub IMTConAutoParam_ValueType: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT,
    pub IMTConAutoParam_ValueInt1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: INT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueInt: unsafe extern "C" fn(this: *const IMTConAutoParam) -> INT64,
    pub IMTConAutoParam_ValueUInt1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: UINT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueUInt: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT64,
    pub IMTConAutoParam_ValueDouble1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: f64) -> MTAPIRES,
    pub IMTConAutoParam_ValueDouble: unsafe extern "C" fn(this: *const IMTConAutoParam) -> f64,
    pub IMTConAutoParam_ValueString1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: LPCWSTR) -> MTAPIRES,
    pub IMTConAutoParam_ValueString: unsafe extern "C" fn(this: *const IMTConAutoParam) -> LPCWSTR,
    pub IMTConAutoParam_ValueColor1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: COLORREF) -> MTAPIRES,
    pub IMTConAutoParam_ValueColor: unsafe extern "C" fn(this: *const IMTConAutoParam) -> COLORREF,
    pub IMTConAutoParam_ValueMoney1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: f64) -> MTAPIRES,
    pub IMTConAutoParam_ValueMoney: unsafe extern "C" fn(this: *const IMTConAutoParam) -> f64,
    pub IMTConAutoParam_ValueVolume1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: UINT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueVolume: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT64,
    pub IMTConAutoParam_ValueDatetime1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: INT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueDatetime: unsafe extern "C" fn(this: *const IMTConAutoParam) -> INT64,
    pub IMTConAutoParam_ValueLeverage1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: INT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueLeverage: unsafe extern "C" fn(this: *const IMTConAutoParam) -> INT64,
    pub IMTConAutoParam_ValueBool1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: bool) -> MTAPIRES,
    pub IMTConAutoParam_ValueBool: unsafe extern "C" fn(this: *const IMTConAutoParam) -> bool,
    pub IMTConAutoParam_ValueTime1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: UINT) -> MTAPIRES,
    pub IMTConAutoParam_ValueTime: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT,
    pub IMTConAutoParam_ValueDate1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: INT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueDate: unsafe extern "C" fn(this: *const IMTConAutoParam) -> INT64,
    pub IMTConAutoParam_ValuePercent1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: UINT) -> MTAPIRES,
    pub IMTConAutoParam_ValuePercent: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT,
    pub IMTConAutoParam_ValueLanguage1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: UINT) -> MTAPIRES,
    pub IMTConAutoParam_ValueLanguage: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT,
    pub IMTConAutoParam_ValueServer1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: UINT64) -> MTAPIRES,
    pub IMTConAutoParam_ValueServer: unsafe extern "C" fn(this: *const IMTConAutoParam) -> UINT64,
    pub IMTConAutoParam_ValueHTML1:
        unsafe extern "C" fn(this: *mut IMTConAutoParam, value: LPCWSTR) -> MTAPIRES,
    pub IMTConAutoParam_ValueHTML: unsafe extern "C" fn(this: *const IMTConAutoParam) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConAutoParam {
    pub vtable_: *const IMTConAutoParam__bindgen_vtable,
}
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_PUSH_TO_LOGINS: IMTConAutoParam_EnParams =
    0;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_PUSH_TO_GROUPS: IMTConAutoParam_EnParams =
    1;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_PUSH_TEXT: IMTConAutoParam_EnParams = 2;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_PUSH_FIRST: IMTConAutoParam_EnParams = 0;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_PUSH_LAST: IMTConAutoParam_EnParams = 50;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_TO_LOGINS:
    IMTConAutoParam_EnParams = 51;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_TO_GROUPS:
    IMTConAutoParam_EnParams = 52;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_SUBJ: IMTConAutoParam_EnParams =
    53;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_TEXT: IMTConAutoParam_EnParams =
    54;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_FROM: IMTConAutoParam_EnParams =
    55;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_FIRST: IMTConAutoParam_EnParams =
    51;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_INTERNAL_LAST: IMTConAutoParam_EnParams =
    100;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_SMS_TO_LOGINS: IMTConAutoParam_EnParams =
    101;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_SMS_TO_GROUPS: IMTConAutoParam_EnParams =
    102;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_SMS_TO_PHONES: IMTConAutoParam_EnParams =
    103;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_SMS_TEXT: IMTConAutoParam_EnParams = 104;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_SMS_FIRST: IMTConAutoParam_EnParams = 101;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_SMS_LAST: IMTConAutoParam_EnParams = 150;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_TO_LOGINS: IMTConAutoParam_EnParams =
    151;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_TO_GROUPS: IMTConAutoParam_EnParams =
    152;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_TO_EMAILS: IMTConAutoParam_EnParams =
    153;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_SERVER: IMTConAutoParam_EnParams =
    154;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_SUBJ: IMTConAutoParam_EnParams = 155;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_TEXT: IMTConAutoParam_EnParams = 156;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_FIRST: IMTConAutoParam_EnParams = 151;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_MESSAGE_EMAIL_LAST: IMTConAutoParam_EnParams = 200;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_GROUP: IMTConAutoParam_EnParams =
    201;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_GROUP_LOGINS:
    IMTConAutoParam_EnParams = 202;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_GROUP_GROUPS:
    IMTConAutoParam_EnParams = 203;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_GROUP_FIRST:
    IMTConAutoParam_EnParams = 201;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_GROUP_LAST:
    IMTConAutoParam_EnParams = 250;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_COLOR: IMTConAutoParam_EnParams =
    251;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_COLOR_LOGINS:
    IMTConAutoParam_EnParams = 252;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_COLOR_GROUPS:
    IMTConAutoParam_EnParams = 253;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_COLOR_FIRST:
    IMTConAutoParam_EnParams = 251;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CHANGE_COLOR_LAST:
    IMTConAutoParam_EnParams = 300;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_COMMENT: IMTConAutoParam_EnParams = 301;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_COMMENT_LOGINS: IMTConAutoParam_EnParams =
    302;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_COMMENT_GROUPS: IMTConAutoParam_EnParams =
    303;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_COMMENT_FIRST: IMTConAutoParam_EnParams =
    301;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_COMMENT_LAST: IMTConAutoParam_EnParams =
    350;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CLIENT_COMMENT: IMTConAutoParam_EnParams =
    351;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CLIENT_COMMENT_LOGINS:
    IMTConAutoParam_EnParams = 352;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_CLIENT_COMMENT_GROUPS:
    IMTConAutoParam_EnParams = 353;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CLIENT_COMMENT_FIRST: IMTConAutoParam_EnParams =
    351;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CLIENT_COMMENT_LAST: IMTConAutoParam_EnParams = 400;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_AMOUNT: IMTConAutoParam_EnParams =
    401;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_COMMENT: IMTConAutoParam_EnParams =
    402;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_LOGINS: IMTConAutoParam_EnParams =
    403;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_GROUPS: IMTConAutoParam_EnParams =
    404;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_ACTION: IMTConAutoParam_EnParams =
    405;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_FIRST: IMTConAutoParam_EnParams =
    401;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_LAST: IMTConAutoParam_EnParams =
    450;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_BONUS_AMOUNT: IMTConAutoParam_EnParams =
    451;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_BONUS_COMMENT: IMTConAutoParam_EnParams =
    452;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_BONUS_LOGINS: IMTConAutoParam_EnParams =
    453;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_BONUS_GROUPS: IMTConAutoParam_EnParams =
    454;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_BONUS_FIRST: IMTConAutoParam_EnParams = 451;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_BONUS_LAST: IMTConAutoParam_EnParams = 500;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_AMOUNT: IMTConAutoParam_EnParams =
    501;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_COMMENT: IMTConAutoParam_EnParams =
    502;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_LOGINS: IMTConAutoParam_EnParams =
    503;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_GROUPS: IMTConAutoParam_EnParams =
    504;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_FIRST: IMTConAutoParam_EnParams =
    501;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_LAST: IMTConAutoParam_EnParams = 550;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_TYPES:
    IMTConAutoParam_EnParams = 551;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_SYMBOLS:
    IMTConAutoParam_EnParams = 552;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_PRICE:
    IMTConAutoParam_EnParams = 553;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_COMMENT:
    IMTConAutoParam_EnParams = 554;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_LOGINS:
    IMTConAutoParam_EnParams = 555;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_GROUPS:
    IMTConAutoParam_EnParams = 556;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_REASON:
    IMTConAutoParam_EnParams = 557;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_TRIGGER_LOGIN:
    IMTConAutoParam_EnParams = 558;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_TRIGGER_POSITION:
    IMTConAutoParam_EnParams = 559;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_FIRST:
    IMTConAutoParam_EnParams = 551;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_POSITIONS_LAST:
    IMTConAutoParam_EnParams = 600;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_TYPES: IMTConAutoParam_EnParams =
    601;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_SYMBOLS:
    IMTConAutoParam_EnParams = 602;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_COMMENT:
    IMTConAutoParam_EnParams = 603;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_LOGINS:
    IMTConAutoParam_EnParams = 604;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_GROUPS:
    IMTConAutoParam_EnParams = 605;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_TRIGGER_LOGIN:
    IMTConAutoParam_EnParams = 606;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_TRIGGER_ORDER:
    IMTConAutoParam_EnParams = 607;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_FIRST: IMTConAutoParam_EnParams =
    601;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_TRADE_CLOSE_ORDERS_LAST: IMTConAutoParam_EnParams =
    650;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_SYMBOL_MOVE_PATH: IMTConAutoParam_EnParams =
    651;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_SYMBOL_MOVE_FIRST: IMTConAutoParam_EnParams =
    651;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_SYMBOL_MOVE_LAST: IMTConAutoParam_EnParams =
    700;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_SERVER_LOGIN:
    IMTConAutoParam_EnParams = 701;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_SERVER_FIRST:
    IMTConAutoParam_EnParams = 701;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_SERVER_LAST:
    IMTConAutoParam_EnParams = 750;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_FEED_NAME:
    IMTConAutoParam_EnParams = 751;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_FEED_FIRST:
    IMTConAutoParam_EnParams = 751;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_FEED_LAST:
    IMTConAutoParam_EnParams = 800;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_GATEWAY_NAME:
    IMTConAutoParam_EnParams = 801;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_GATEWAY_FIRST:
    IMTConAutoParam_EnParams = 801;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_PLATFORM_RESTART_GATEWAY_LAST:
    IMTConAutoParam_EnParams = 850;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_GROUP_UPDATE_JSON: IMTConAutoParam_EnParams =
    851;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_GROUP_UPDATE_FIRST:
    IMTConAutoParam_EnParams = 851;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_GROUP_UPDATE_LAST: IMTConAutoParam_EnParams =
    900;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_SYMBOL_UPDATE_JSON:
    IMTConAutoParam_EnParams = 901;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_SYMBOL_UPDATE_FIRST:
    IMTConAutoParam_EnParams = 901;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_SYMBOL_UPDATE_LAST:
    IMTConAutoParam_EnParams = 950;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_ROUTING_UPDATE_JSON:
    IMTConAutoParam_EnParams = 951;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_ROUTING_UPDATE_FIRST:
    IMTConAutoParam_EnParams = 951;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_CONFIG_ROUTING_UPDATE_LAST:
    IMTConAutoParam_EnParams = 1000;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_LOGINS: IMTConAutoParam_EnParams =
    1001;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_GROUPS: IMTConAutoParam_EnParams =
    1002;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_FIRST: IMTConAutoParam_EnParams =
    1001;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_LAST: IMTConAutoParam_EnParams =
    1050;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_LOGINS: IMTConAutoParam_EnParams =
    1051;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_GROUPS: IMTConAutoParam_EnParams =
    1052;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_FIRST: IMTConAutoParam_EnParams =
    1051;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_LAST: IMTConAutoParam_EnParams =
    1110;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRADE_LOGINS:
    IMTConAutoParam_EnParams = 1101;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRADE_GROUPS:
    IMTConAutoParam_EnParams = 1102;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRADE_FIRST:
    IMTConAutoParam_EnParams = 1101;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRADE_LAST:
    IMTConAutoParam_EnParams = 1150;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRADE_LOGINS:
    IMTConAutoParam_EnParams = 1151;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRADE_GROUPS:
    IMTConAutoParam_EnParams = 1152;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRADE_FIRST:
    IMTConAutoParam_EnParams = 1151;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRADE_LAST:
    IMTConAutoParam_EnParams = 1200;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_LOGINS:
    IMTConAutoParam_EnParams = 1201;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_GROUPS:
    IMTConAutoParam_EnParams = 1202;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_FIRST:
    IMTConAutoParam_EnParams = 1201;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_EXPERT_LAST:
    IMTConAutoParam_EnParams = 1250;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_LOGINS:
    IMTConAutoParam_EnParams = 1251;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_GROUPS:
    IMTConAutoParam_EnParams = 1252;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_FIRST:
    IMTConAutoParam_EnParams = 1251;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_EXPERT_LAST:
    IMTConAutoParam_EnParams = 1300;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_LOGINS:
    IMTConAutoParam_EnParams = 1301;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_GROUPS:
    IMTConAutoParam_EnParams = 1302;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_FIRST:
    IMTConAutoParam_EnParams = 1301;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_DISABLE_TRAILING_LAST:
    IMTConAutoParam_EnParams = 1350;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_LOGINS:
    IMTConAutoParam_EnParams = 1351;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_GROUPS:
    IMTConAutoParam_EnParams = 1352;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_FIRST:
    IMTConAutoParam_EnParams = 1351;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ENABLE_TRAILING_LAST:
    IMTConAutoParam_EnParams = 1400;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_LOGINS:
    IMTConAutoParam_EnParams = 1401;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_GROUPS:
    IMTConAutoParam_EnParams = 1402;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_FIRST:
    IMTConAutoParam_EnParams = 1401;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_FORCE_CHANGE_PASS_LAST:
    IMTConAutoParam_EnParams = 1450;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ARCHIVE_LOGINS: IMTConAutoParam_EnParams =
    1451;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ARCHIVE_GROUPS: IMTConAutoParam_EnParams =
    1452;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ARCHIVE_FIRST: IMTConAutoParam_EnParams =
    1451;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_ARCHIVE_LAST: IMTConAutoParam_EnParams =
    1500;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_LEVERAGE: IMTConAutoParam_EnParams = 1501;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_LEVERAGE_LOGINS: IMTConAutoParam_EnParams =
    1502;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_LEVERAGE_GROUPS: IMTConAutoParam_EnParams =
    1503;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_LEVERAGE_FIRST: IMTConAutoParam_EnParams =
    1501;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_ACCOUNT_LEVERAGE_LAST: IMTConAutoParam_EnParams =
    1550;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_LOGINS:
    IMTConAutoParam_EnParams = 1551;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_GROUPS:
    IMTConAutoParam_EnParams = 1552;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_COMMENT:
    IMTConAutoParam_EnParams = 1553;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_ACTION:
    IMTConAutoParam_EnParams = 1554;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_FIRST:
    IMTConAutoParam_EnParams = 1551;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_DEPOSIT_PAYOFF_LAST:
    IMTConAutoParam_EnParams = 1600;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_PAYOFF_LOGINS:
    IMTConAutoParam_EnParams = 1601;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_PAYOFF_GROUPS:
    IMTConAutoParam_EnParams = 1602;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_PAYOFF_COMMENT:
    IMTConAutoParam_EnParams = 1603;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_PAYOFF_FIRST:
    IMTConAutoParam_EnParams = 1601;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_FINANCE_CREDIT_PAYOFF_LAST:
    IMTConAutoParam_EnParams = 1650;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_HTTP_REQUEST_METHOD:
    IMTConAutoParam_EnParams = 1651;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_HTTP_REQUEST_URL:
    IMTConAutoParam_EnParams = 1652;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_HTTP_REQUEST_HEADERS:
    IMTConAutoParam_EnParams = 1653;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_HTTP_REQUEST_DATA:
    IMTConAutoParam_EnParams = 1654;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_HTTP_REQUEST_FIRST:
    IMTConAutoParam_EnParams = 1651;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_HTTP_REQUEST_LAST:
    IMTConAutoParam_EnParams = 1700;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_NAME:
    IMTConAutoParam_EnParams = 1701;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_VALUE:
    IMTConAutoParam_EnParams = 1702;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_UNIT:
    IMTConAutoParam_EnParams = 1703;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_FIRST:
    IMTConAutoParam_EnParams = 1701;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_FINTEZA_EVENT_LAST:
    IMTConAutoParam_EnParams = 1750;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_APPLICATION_PATH:
    IMTConAutoParam_EnParams = 1751;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_APPLICATION_CMD: IMTConAutoParam_EnParams =
    1752;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_APPLICATION_FIRST:
    IMTConAutoParam_EnParams = 1751;
pub const IMTConAutoParam_EnParams_PARAM_ACTION_EXTERNAL_APPLICATION_LAST:
    IMTConAutoParam_EnParams = 1752;
pub const IMTConAutoParam_EnParams_PARAM_FIRST: IMTConAutoParam_EnParams = 0;
pub const IMTConAutoParam_EnParams_PARAM_LAST: IMTConAutoParam_EnParams = 1752;
pub type IMTConAutoParam_EnParams = ::std::os::raw::c_int;
pub const IMTConAutoParam_EnParamType_TYPE_NONE: IMTConAutoParam_EnParamType = 0;
pub const IMTConAutoParam_EnParamType_TYPE_STRING: IMTConAutoParam_EnParamType = 1;
pub const IMTConAutoParam_EnParamType_TYPE_INT: IMTConAutoParam_EnParamType = 2;
pub const IMTConAutoParam_EnParamType_TYPE_UINT: IMTConAutoParam_EnParamType = 3;
pub const IMTConAutoParam_EnParamType_TYPE_FLOAT: IMTConAutoParam_EnParamType = 4;
pub const IMTConAutoParam_EnParamType_TYPE_COLOR: IMTConAutoParam_EnParamType = 5;
pub const IMTConAutoParam_EnParamType_TYPE_MONEY: IMTConAutoParam_EnParamType = 6;
pub const IMTConAutoParam_EnParamType_TYPE_VOLUME: IMTConAutoParam_EnParamType = 7;
pub const IMTConAutoParam_EnParamType_TYPE_DATETIME: IMTConAutoParam_EnParamType = 8;
pub const IMTConAutoParam_EnParamType_TYPE_LEVERAGE: IMTConAutoParam_EnParamType = 9;
pub const IMTConAutoParam_EnParamType_TYPE_BOOL: IMTConAutoParam_EnParamType = 10;
pub const IMTConAutoParam_EnParamType_TYPE_TIME: IMTConAutoParam_EnParamType = 11;
pub const IMTConAutoParam_EnParamType_TYPE_DATE: IMTConAutoParam_EnParamType = 12;
pub const IMTConAutoParam_EnParamType_TYPE_PERCENT: IMTConAutoParam_EnParamType = 13;
pub const IMTConAutoParam_EnParamType_TYPE_LANGUAGE: IMTConAutoParam_EnParamType = 14;
pub const IMTConAutoParam_EnParamType_TYPE_SERVER: IMTConAutoParam_EnParamType = 15;
pub const IMTConAutoParam_EnParamType_TYPE_HTML: IMTConAutoParam_EnParamType = 16;
pub const IMTConAutoParam_EnParamType_TYPE_FIRST: IMTConAutoParam_EnParamType = 0;
pub const IMTConAutoParam_EnParamType_TYPE_LAST: IMTConAutoParam_EnParamType = 16;
pub type IMTConAutoParam_EnParamType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConAutoParam"][::std::mem::size_of::<IMTConAutoParam>() - 8usize];
    ["Alignment of IMTConAutoParam"][::std::mem::align_of::<IMTConAutoParam>() - 8usize];
};
#[repr(C)]
pub struct IMTConAutoAction__bindgen_vtable {
    pub IMTConAutoAction_Release: unsafe extern "C" fn(this: *mut IMTConAutoAction),
    pub IMTConAutoAction_Assign: unsafe extern "C" fn(
        this: *mut IMTConAutoAction,
        action: *const IMTConAutoAction,
    ) -> MTAPIRES,
    pub IMTConAutoAction_Clear: unsafe extern "C" fn(this: *mut IMTConAutoAction) -> MTAPIRES,
    pub IMTConAutoAction_Action1:
        unsafe extern "C" fn(this: *mut IMTConAutoAction, condition: UINT) -> MTAPIRES,
    pub IMTConAutoAction_Action: unsafe extern "C" fn(this: *const IMTConAutoAction) -> UINT,
    pub IMTConAutoAction_Name1:
        unsafe extern "C" fn(this: *mut IMTConAutoAction, name: LPCWSTR) -> MTAPIRES,
    pub IMTConAutoAction_Name: unsafe extern "C" fn(this: *const IMTConAutoAction) -> LPCWSTR,
    pub IMTConAutoAction_ParamAdd:
        unsafe extern "C" fn(this: *mut IMTConAutoAction, param: *mut IMTConAutoParam) -> MTAPIRES,
    pub IMTConAutoAction_ParamUpdate: unsafe extern "C" fn(
        this: *mut IMTConAutoAction,
        pos: UINT,
        param: *const IMTConAutoParam,
    ) -> MTAPIRES,
    pub IMTConAutoAction_ParamDelete:
        unsafe extern "C" fn(this: *mut IMTConAutoAction, pos: UINT) -> MTAPIRES,
    pub IMTConAutoAction_ParamClear: unsafe extern "C" fn(this: *mut IMTConAutoAction) -> MTAPIRES,
    pub IMTConAutoAction_ParamShift: unsafe extern "C" fn(
        this: *mut IMTConAutoAction,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConAutoAction_ParamTotal: unsafe extern "C" fn(this: *const IMTConAutoAction) -> UINT,
    pub IMTConAutoAction_ParamNext: unsafe extern "C" fn(
        this: *const IMTConAutoAction,
        pos: UINT,
        param: *mut IMTConAutoParam,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConAutoAction {
    pub vtable_: *const IMTConAutoAction__bindgen_vtable,
}
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_PUSH: IMTConAutoAction_EnActions = 0;
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_INTERNAL: IMTConAutoAction_EnActions = 1;
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_SMS: IMTConAutoAction_EnActions = 2;
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_EMAIL: IMTConAutoAction_EnActions = 3;
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_CHANNEL_TELEGRAM: IMTConAutoAction_EnActions =
    4;
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_FIRST: IMTConAutoAction_EnActions = 0;
pub const IMTConAutoAction_EnActions_ACTION_MESSAGE_LAST: IMTConAutoAction_EnActions = 1000;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_DISABLE: IMTConAutoAction_EnActions = 1001;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_ENABLE: IMTConAutoAction_EnActions = 1002;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_DISABLE_TRADE: IMTConAutoAction_EnActions =
    1003;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_ENABLE_TRADE: IMTConAutoAction_EnActions = 1004;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_DISABLE_EXPERT: IMTConAutoAction_EnActions =
    1005;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_ENABLE_EXPERT: IMTConAutoAction_EnActions =
    1006;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_DISABLE_TRAILING: IMTConAutoAction_EnActions =
    1007;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_ENABLE_TRAILING: IMTConAutoAction_EnActions =
    1008;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_FORCE_CHANGE_PASS: IMTConAutoAction_EnActions =
    1009;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_CHANGE_GROUP: IMTConAutoAction_EnActions = 1010;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_CHANGE_COLOR: IMTConAutoAction_EnActions = 1011;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_ARCHIVE: IMTConAutoAction_EnActions = 1012;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_COMMENT: IMTConAutoAction_EnActions = 1013;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_CLIENT_COMMENT: IMTConAutoAction_EnActions =
    1014;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_LEVERAGE: IMTConAutoAction_EnActions = 1015;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_FIRST: IMTConAutoAction_EnActions = 1001;
pub const IMTConAutoAction_EnActions_ACTION_ACCOUNT_LAST: IMTConAutoAction_EnActions = 2000;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_DEPOSIT: IMTConAutoAction_EnActions = 2001;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_BONUS: IMTConAutoAction_EnActions = 2002;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_CREDIT: IMTConAutoAction_EnActions = 2003;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_DEPOSIT_PAYOFF: IMTConAutoAction_EnActions =
    2004;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_CREDIT_PAYOFF: IMTConAutoAction_EnActions =
    2005;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_BONUS_PERCENT: IMTConAutoAction_EnActions =
    2006;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_FIRST: IMTConAutoAction_EnActions = 2001;
pub const IMTConAutoAction_EnActions_ACTION_FINANCE_LAST: IMTConAutoAction_EnActions = 3000;
pub const IMTConAutoAction_EnActions_ACTION_TRADE_CLOSE_POSITIONS: IMTConAutoAction_EnActions =
    3001;
pub const IMTConAutoAction_EnActions_ACTION_TRADE_CLOSE_ORDERS: IMTConAutoAction_EnActions = 4000;
pub const IMTConAutoAction_EnActions_ACTION_TRADE_FIRST: IMTConAutoAction_EnActions = 3001;
pub const IMTConAutoAction_EnActions_ACTION_TRADE_LAST: IMTConAutoAction_EnActions = 5000;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_GROUP_UPDATE: IMTConAutoAction_EnActions = 5001;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_SYMBOL_UPDATE: IMTConAutoAction_EnActions = 5002;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_SYMBOL_MOVE: IMTConAutoAction_EnActions = 5003;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_ROUTING_UPDATE: IMTConAutoAction_EnActions =
    5004;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_SERVER_UPDATE: IMTConAutoAction_EnActions = 5005;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_GATEWAY_UPDATE: IMTConAutoAction_EnActions =
    5006;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_DATAFEED_UPDATE: IMTConAutoAction_EnActions =
    5007;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_FIRST: IMTConAutoAction_EnActions = 5001;
pub const IMTConAutoAction_EnActions_ACTION_CONFIG_LAST: IMTConAutoAction_EnActions = 6000;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_RESTART_SERVER: IMTConAutoAction_EnActions =
    6001;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_RESTART_FEED: IMTConAutoAction_EnActions =
    6002;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_RESTART_GATEWAY: IMTConAutoAction_EnActions =
    6003;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_SYNC_HISTORY: IMTConAutoAction_EnActions =
    6004;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_LIVE_UPDATE: IMTConAutoAction_EnActions = 6005;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_FIRST: IMTConAutoAction_EnActions = 6001;
pub const IMTConAutoAction_EnActions_ACTION_PLATFORM_LAST: IMTConAutoAction_EnActions = 7000;
pub const IMTConAutoAction_EnActions_ACTION_EXTERNAL_APPLICATION: IMTConAutoAction_EnActions = 7001;
pub const IMTConAutoAction_EnActions_ACTION_EXTERNAL_PLUGIN: IMTConAutoAction_EnActions = 7002;
pub const IMTConAutoAction_EnActions_ACTION_EXTERNAL_HTTP_REQUEST: IMTConAutoAction_EnActions =
    7003;
pub const IMTConAutoAction_EnActions_ACTION_EXTERNAL_FINTEZA_EVENT: IMTConAutoAction_EnActions =
    7004;
pub const IMTConAutoAction_EnActions_ACTION_EXTERNAL_FIRST: IMTConAutoAction_EnActions = 7001;
pub const IMTConAutoAction_EnActions_ACTION_EXTERNAL_LAST: IMTConAutoAction_EnActions = 8000;
pub const IMTConAutoAction_EnActions_ACTION_FIRST: IMTConAutoAction_EnActions = 0;
pub const IMTConAutoAction_EnActions_ACTION_LAST: IMTConAutoAction_EnActions = 8000;
pub type IMTConAutoAction_EnActions = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConAutoAction"][::std::mem::size_of::<IMTConAutoAction>() - 8usize];
    ["Alignment of IMTConAutoAction"][::std::mem::align_of::<IMTConAutoAction>() - 8usize];
};
#[repr(C)]
pub struct IMTConAutomation__bindgen_vtable {
    pub IMTConAutomation_Release: unsafe extern "C" fn(this: *mut IMTConAutomation),
    pub IMTConAutomation_Assign: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        automation: *const IMTConAutomation,
    ) -> MTAPIRES,
    pub IMTConAutomation_Clear: unsafe extern "C" fn(this: *mut IMTConAutomation) -> MTAPIRES,
    pub IMTConAutomation_ID: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT64,
    pub IMTConAutomation_ParentID: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT64,
    pub IMTConAutomation_Name1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, name: LPCWSTR) -> MTAPIRES,
    pub IMTConAutomation_Name: unsafe extern "C" fn(this: *const IMTConAutomation) -> LPCWSTR,
    pub IMTConAutomation_Trigger1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, trigger: UINT) -> MTAPIRES,
    pub IMTConAutomation_Trigger: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_Flags1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, flags: UINT64) -> MTAPIRES,
    pub IMTConAutomation_Flags: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT64,
    pub IMTConAutomation_TimeStart1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, start: INT64) -> MTAPIRES,
    pub IMTConAutomation_TimeStart: unsafe extern "C" fn(this: *const IMTConAutomation) -> INT64,
    pub IMTConAutomation_TimeExpire1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, expire: INT64) -> MTAPIRES,
    pub IMTConAutomation_TimeExpire: unsafe extern "C" fn(this: *const IMTConAutomation) -> INT64,
    pub IMTConAutomation_TimeWeekdays1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, weekdays: UINT) -> MTAPIRES,
    pub IMTConAutomation_TimeWeekdays: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_TimeMonths1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, months: UINT) -> MTAPIRES,
    pub IMTConAutomation_TimeMonths: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_TimeMonthdays1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, monthdays: UINT) -> MTAPIRES,
    pub IMTConAutomation_TimeMonthdays: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_EventPauseMinutes1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, minutes: UINT) -> MTAPIRES,
    pub IMTConAutomation_EventPauseMinutes:
        unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_EventPauseHours1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, hours: UINT) -> MTAPIRES,
    pub IMTConAutomation_EventPauseHours:
        unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_EventPauseDays1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, days: UINT) -> MTAPIRES,
    pub IMTConAutomation_EventPauseDays:
        unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_EventRepeats: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_EventRepeats1:
        unsafe extern "C" fn(this: *mut IMTConAutomation, repeats: UINT) -> MTAPIRES,
    pub IMTConAutomation_ConditionAdd: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        condition: *mut IMTConAutoCondition,
    ) -> MTAPIRES,
    pub IMTConAutomation_ConditionUpdate: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        pos: UINT,
        condition: *const IMTConAutoCondition,
    ) -> MTAPIRES,
    pub IMTConAutomation_ConditionDelete:
        unsafe extern "C" fn(this: *mut IMTConAutomation, pos: UINT) -> MTAPIRES,
    pub IMTConAutomation_ConditionClear:
        unsafe extern "C" fn(this: *mut IMTConAutomation) -> MTAPIRES,
    pub IMTConAutomation_ConditionShift: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConAutomation_ConditionTotal:
        unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_ConditionNext: unsafe extern "C" fn(
        this: *const IMTConAutomation,
        pos: UINT,
        condition: *mut IMTConAutoCondition,
    ) -> MTAPIRES,
    pub IMTConAutomation_ActionAdd: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        action: *mut IMTConAutoAction,
    ) -> MTAPIRES,
    pub IMTConAutomation_ActionUpdate: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        pos: UINT,
        action: *const IMTConAutoAction,
    ) -> MTAPIRES,
    pub IMTConAutomation_ActionDelete:
        unsafe extern "C" fn(this: *mut IMTConAutomation, pos: UINT) -> MTAPIRES,
    pub IMTConAutomation_ActionClear: unsafe extern "C" fn(this: *mut IMTConAutomation) -> MTAPIRES,
    pub IMTConAutomation_ActionShift: unsafe extern "C" fn(
        this: *mut IMTConAutomation,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConAutomation_ActionTotal: unsafe extern "C" fn(this: *const IMTConAutomation) -> UINT,
    pub IMTConAutomation_ActionNext: unsafe extern "C" fn(
        this: *const IMTConAutomation,
        pos: UINT,
        action: *mut IMTConAutoAction,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConAutomation {
    pub vtable_: *const IMTConAutomation__bindgen_vtable,
}
pub const IMTConAutomation_EnFlags_FLAG_NONE: IMTConAutomation_EnFlags = 0;
pub const IMTConAutomation_EnFlags_FLAG_ENABLED: IMTConAutomation_EnFlags = 1;
pub const IMTConAutomation_EnFlags_FLAG_FOLDER: IMTConAutomation_EnFlags = 2;
pub const IMTConAutomation_EnFlags_FLAG_ALL: IMTConAutomation_EnFlags = 3;
pub type IMTConAutomation_EnFlags = ::std::os::raw::c_int;
pub const IMTConAutomation_EnTriggers_TRIGGER_SCHEDULE_SCHEDULE: IMTConAutomation_EnTriggers = 0;
pub const IMTConAutomation_EnTriggers_TRIGGER_SCHEDULE_ACCOUNTS: IMTConAutomation_EnTriggers = 1;
pub const IMTConAutomation_EnTriggers_TRIGGER_SCHEDULE_POSITIONS: IMTConAutomation_EnTriggers = 2;
pub const IMTConAutomation_EnTriggers_TRIGGER_SCHEDULE_ORDERS: IMTConAutomation_EnTriggers = 3;
pub const IMTConAutomation_EnTriggers_TRIGGER_SCHEDULE_FIRST: IMTConAutomation_EnTriggers = 0;
pub const IMTConAutomation_EnTriggers_TRIGGER_SCHEDULE_LAST: IMTConAutomation_EnTriggers = 1000;
pub const IMTConAutomation_EnTriggers_TRIGGER_CONNECTIONS_LOGIN: IMTConAutomation_EnTriggers = 1001;
pub const IMTConAutomation_EnTriggers_TRIGGER_CONNECTIONS_LOGIN_FIRST: IMTConAutomation_EnTriggers =
    1002;
pub const IMTConAutomation_EnTriggers_TRIGGER_CONNECTIONS_LOGOUT: IMTConAutomation_EnTriggers =
    1003;
pub const IMTConAutomation_EnTriggers_TRIGGER_CONNECTIONS_AUTHORIZE_FAIL:
    IMTConAutomation_EnTriggers = 1004;
pub const IMTConAutomation_EnTriggers_TRIGGER_CONNECTIONS_FIRST: IMTConAutomation_EnTriggers = 1001;
pub const IMTConAutomation_EnTriggers_TRIGGER_CONNECTIONS_LAST: IMTConAutomation_EnTriggers = 2000;
pub const IMTConAutomation_EnTriggers_TRIGGER_ACCOUNT_CREATE: IMTConAutomation_EnTriggers = 2001;
pub const IMTConAutomation_EnTriggers_TRIGGER_ACCOUNT_FIRST: IMTConAutomation_EnTriggers = 2001;
pub const IMTConAutomation_EnTriggers_TRIGGER_ACCOUNT_LAST: IMTConAutomation_EnTriggers = 3000;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_DEPOSIT_IN: IMTConAutomation_EnTriggers =
    3001;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_DEPOSIT_IN_FIRST:
    IMTConAutomation_EnTriggers = 3002;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_DEPOSIT_OUT: IMTConAutomation_EnTriggers =
    3003;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_DEPOSIT_OUT_FIRST:
    IMTConAutomation_EnTriggers = 3004;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_CREDIT_IN: IMTConAutomation_EnTriggers = 3005;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_CREDIT_IN_FIRST: IMTConAutomation_EnTriggers =
    3006;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_CREDIT_OUT: IMTConAutomation_EnTriggers =
    3007;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_FIRST: IMTConAutomation_EnTriggers = 3001;
pub const IMTConAutomation_EnTriggers_TRIGGER_FINANCE_LAST: IMTConAutomation_EnTriggers = 4000;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_MARGIN_CALL: IMTConAutomation_EnTriggers = 4001;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_STOP_OUT: IMTConAutomation_EnTriggers = 4002;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_POSITION_OPEN: IMTConAutomation_EnTriggers =
    4003;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_POSITION_INCREASE: IMTConAutomation_EnTriggers =
    4004;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_POSITION_DECREASE: IMTConAutomation_EnTriggers =
    4005;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_POSITION_CLOSE: IMTConAutomation_EnTriggers =
    4006;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_POSITION_REVERSE: IMTConAutomation_EnTriggers =
    4007;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_FIRST: IMTConAutomation_EnTriggers = 4001;
pub const IMTConAutomation_EnTriggers_TRIGGER_TRADE_LAST: IMTConAutomation_EnTriggers = 5000;
pub const IMTConAutomation_EnTriggers_TRIGGER_PRICES_GAP_STARTED: IMTConAutomation_EnTriggers =
    5001;
pub const IMTConAutomation_EnTriggers_TRIGGER_PRICES_GAP_FINISHED: IMTConAutomation_EnTriggers =
    5002;
pub const IMTConAutomation_EnTriggers_TRIGGER_PRICES_DELAYED: IMTConAutomation_EnTriggers = 5003;
pub const IMTConAutomation_EnTriggers_TRIGGER_PRICES_RESUMED: IMTConAutomation_EnTriggers = 5004;
pub const IMTConAutomation_EnTriggers_TRIGGER_PRICES_FIRST: IMTConAutomation_EnTriggers = 5001;
pub const IMTConAutomation_EnTriggers_TRIGGER_PRICES_LAST: IMTConAutomation_EnTriggers = 6000;
pub const IMTConAutomation_EnTriggers_TRIGGER_PLATFORM_MONITOR: IMTConAutomation_EnTriggers = 6001;
pub const IMTConAutomation_EnTriggers_TRIGGER_PLATFORM_CONNECT: IMTConAutomation_EnTriggers = 6002;
pub const IMTConAutomation_EnTriggers_TRIGGER_PLATFORM_DISCONNECT: IMTConAutomation_EnTriggers =
    6003;
pub const IMTConAutomation_EnTriggers_TRIGGER_PLATFORM_FAILOVER: IMTConAutomation_EnTriggers = 6004;
pub const IMTConAutomation_EnTriggers_TRIGGER_PLATFORM_FIRST: IMTConAutomation_EnTriggers = 6001;
pub const IMTConAutomation_EnTriggers_TRIGGER_PLATFORM_LAST: IMTConAutomation_EnTriggers = 7000;
pub const IMTConAutomation_EnTriggers_TRIGGER_EXTERNAL_API: IMTConAutomation_EnTriggers = 7001;
pub const IMTConAutomation_EnTriggers_TRIGGER_EXTERNAL_FIRST: IMTConAutomation_EnTriggers = 7001;
pub const IMTConAutomation_EnTriggers_TRIGGER_EXTERNAL_LAST: IMTConAutomation_EnTriggers = 8000;
pub const IMTConAutomation_EnTriggers_TRIGGER_MESSAGE_CLIENT_SEND: IMTConAutomation_EnTriggers =
    8001;
pub const IMTConAutomation_EnTriggers_TRIGGER_MESSAGE_CLIENT_READ: IMTConAutomation_EnTriggers =
    8002;
pub const IMTConAutomation_EnTriggers_TRIGGER_MESSAGE_FIRST: IMTConAutomation_EnTriggers = 8001;
pub const IMTConAutomation_EnTriggers_TRIGGER_MESSAGE_LAST: IMTConAutomation_EnTriggers = 9000;
pub const IMTConAutomation_EnTriggers_TRIGGER_GATEWAY_CONNECT: IMTConAutomation_EnTriggers = 9001;
pub const IMTConAutomation_EnTriggers_TRIGGER_GATEWAY_DISCONNECT: IMTConAutomation_EnTriggers =
    9002;
pub const IMTConAutomation_EnTriggers_TRIGGER_GATEWAY_FIRST: IMTConAutomation_EnTriggers = 9001;
pub const IMTConAutomation_EnTriggers_TRIGGER_GATEWAY_LAST: IMTConAutomation_EnTriggers = 10000;
pub const IMTConAutomation_EnTriggers_TRIGGER_DATAFEED_CONNECT: IMTConAutomation_EnTriggers = 10001;
pub const IMTConAutomation_EnTriggers_TRIGGER_DATAFEED_DISCONNECT: IMTConAutomation_EnTriggers =
    10002;
pub const IMTConAutomation_EnTriggers_TRIGGER_DATAFEED_FIRST: IMTConAutomation_EnTriggers = 10001;
pub const IMTConAutomation_EnTriggers_TRIGGER_DATAFEED_LAST: IMTConAutomation_EnTriggers = 11000;
pub const IMTConAutomation_EnTriggers_TRIGGER_KYC_START: IMTConAutomation_EnTriggers = 11001;
pub const IMTConAutomation_EnTriggers_TRIGGER_KYC_CONFIRM: IMTConAutomation_EnTriggers = 11002;
pub const IMTConAutomation_EnTriggers_TRIGGER_KYC_REJECT: IMTConAutomation_EnTriggers = 11003;
pub const IMTConAutomation_EnTriggers_TRIGGER_KYC_FIRST: IMTConAutomation_EnTriggers = 11001;
pub const IMTConAutomation_EnTriggers_TRIGGER_KYC_LAST: IMTConAutomation_EnTriggers = 12000;
pub const IMTConAutomation_EnTriggers_TRIGGER_FIRST: IMTConAutomation_EnTriggers = 0;
pub const IMTConAutomation_EnTriggers_TRIGGER_LAST: IMTConAutomation_EnTriggers = 12000;
pub type IMTConAutomation_EnTriggers = ::std::os::raw::c_int;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_SUN:
    IMTConAutomation_EnTriggerWeekdays = 1;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_MON:
    IMTConAutomation_EnTriggerWeekdays = 2;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_TUE:
    IMTConAutomation_EnTriggerWeekdays = 4;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_WED:
    IMTConAutomation_EnTriggerWeekdays = 8;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_THU:
    IMTConAutomation_EnTriggerWeekdays = 16;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_FRI:
    IMTConAutomation_EnTriggerWeekdays = 32;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_SAT:
    IMTConAutomation_EnTriggerWeekdays = 64;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_NONE:
    IMTConAutomation_EnTriggerWeekdays = 0;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_ALL:
    IMTConAutomation_EnTriggerWeekdays = 127;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_FIRST:
    IMTConAutomation_EnTriggerWeekdays = 1;
pub const IMTConAutomation_EnTriggerWeekdays_TRIGGER_WEEKDAY_LAST:
    IMTConAutomation_EnTriggerWeekdays = 64;
pub type IMTConAutomation_EnTriggerWeekdays = ::std::os::raw::c_int;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_JAN: IMTConAutomation_EnTriggerMonths = 1;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_FEB: IMTConAutomation_EnTriggerMonths = 2;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_MAR: IMTConAutomation_EnTriggerMonths = 4;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_APR: IMTConAutomation_EnTriggerMonths = 8;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_MAY: IMTConAutomation_EnTriggerMonths =
    16;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_JUN: IMTConAutomation_EnTriggerMonths =
    32;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_JUL: IMTConAutomation_EnTriggerMonths =
    64;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_AUG: IMTConAutomation_EnTriggerMonths =
    128;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_SEP: IMTConAutomation_EnTriggerMonths =
    256;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_OCT: IMTConAutomation_EnTriggerMonths =
    512;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_NOV: IMTConAutomation_EnTriggerMonths =
    1024;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_DEC: IMTConAutomation_EnTriggerMonths =
    2048;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_NONE: IMTConAutomation_EnTriggerMonths =
    0;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_ALL: IMTConAutomation_EnTriggerMonths =
    4095;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_FIRST: IMTConAutomation_EnTriggerMonths =
    1;
pub const IMTConAutomation_EnTriggerMonths_TRIGGER_MONTHS_LAST: IMTConAutomation_EnTriggerMonths =
    2048;
pub type IMTConAutomation_EnTriggerMonths = ::std::os::raw::c_int;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_1:
    IMTConAutomation_EnTriggerMonthDays = 1;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_2:
    IMTConAutomation_EnTriggerMonthDays = 2;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_3:
    IMTConAutomation_EnTriggerMonthDays = 4;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_4:
    IMTConAutomation_EnTriggerMonthDays = 8;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_5:
    IMTConAutomation_EnTriggerMonthDays = 16;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_6:
    IMTConAutomation_EnTriggerMonthDays = 32;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_7:
    IMTConAutomation_EnTriggerMonthDays = 64;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_8:
    IMTConAutomation_EnTriggerMonthDays = 128;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_9:
    IMTConAutomation_EnTriggerMonthDays = 256;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_10:
    IMTConAutomation_EnTriggerMonthDays = 512;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_11:
    IMTConAutomation_EnTriggerMonthDays = 1024;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_12:
    IMTConAutomation_EnTriggerMonthDays = 2048;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_13:
    IMTConAutomation_EnTriggerMonthDays = 4096;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_14:
    IMTConAutomation_EnTriggerMonthDays = 8192;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_15:
    IMTConAutomation_EnTriggerMonthDays = 16384;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_16:
    IMTConAutomation_EnTriggerMonthDays = 32768;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_17:
    IMTConAutomation_EnTriggerMonthDays = 65536;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_18:
    IMTConAutomation_EnTriggerMonthDays = 131072;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_19:
    IMTConAutomation_EnTriggerMonthDays = 262144;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_20:
    IMTConAutomation_EnTriggerMonthDays = 524288;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_21:
    IMTConAutomation_EnTriggerMonthDays = 1048576;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_22:
    IMTConAutomation_EnTriggerMonthDays = 2097152;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_23:
    IMTConAutomation_EnTriggerMonthDays = 4194304;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_24:
    IMTConAutomation_EnTriggerMonthDays = 8388608;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_25:
    IMTConAutomation_EnTriggerMonthDays = 16777216;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_26:
    IMTConAutomation_EnTriggerMonthDays = 33554432;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_27:
    IMTConAutomation_EnTriggerMonthDays = 67108864;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_28:
    IMTConAutomation_EnTriggerMonthDays = 134217728;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_29:
    IMTConAutomation_EnTriggerMonthDays = 268435456;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_30:
    IMTConAutomation_EnTriggerMonthDays = 536870912;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_31:
    IMTConAutomation_EnTriggerMonthDays = 1073741824;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_NONE:
    IMTConAutomation_EnTriggerMonthDays = 0;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_ALL:
    IMTConAutomation_EnTriggerMonthDays = 2147483647;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_FIRST:
    IMTConAutomation_EnTriggerMonthDays = 1;
pub const IMTConAutomation_EnTriggerMonthDays_TRIGGER_MONTHDAY_LAST:
    IMTConAutomation_EnTriggerMonthDays = 1073741824;
pub type IMTConAutomation_EnTriggerMonthDays = UINT;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConAutomation"][::std::mem::size_of::<IMTConAutomation>() - 8usize];
    ["Alignment of IMTConAutomation"][::std::mem::align_of::<IMTConAutomation>() - 8usize];
};
#[repr(C)]
pub struct IMTConAutomationSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConAutomationSink {
    pub vtable_: *const IMTConAutomationSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConAutomationSink"][::std::mem::size_of::<IMTConAutomationSink>() - 8usize];
    ["Alignment of IMTConAutomationSink"][::std::mem::align_of::<IMTConAutomationSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConSubscriptionSymbol__bindgen_vtable {
    pub IMTConSubscriptionSymbol_Release: unsafe extern "C" fn(this: *mut IMTConSubscriptionSymbol),
    pub IMTConSubscriptionSymbol_Assign: unsafe extern "C" fn(
        this: *mut IMTConSubscriptionSymbol,
        symbol: *const IMTConSubscriptionSymbol,
    ) -> MTAPIRES,
    pub IMTConSubscriptionSymbol_Clear:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionSymbol) -> MTAPIRES,
    pub IMTConSubscriptionSymbol_Symbol1:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionSymbol, symbols: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscriptionSymbol_Symbol:
        unsafe extern "C" fn(this: *const IMTConSubscriptionSymbol) -> LPCWSTR,
    pub IMTConSubscriptionSymbol_Level1:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionSymbol, level: UINT) -> MTAPIRES,
    pub IMTConSubscriptionSymbol_Level:
        unsafe extern "C" fn(this: *const IMTConSubscriptionSymbol) -> UINT,
    pub IMTConSubscriptionSymbol_TickHistory1:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionSymbol, mode: UINT) -> MTAPIRES,
    pub IMTConSubscriptionSymbol_TickHistory:
        unsafe extern "C" fn(this: *const IMTConSubscriptionSymbol) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSubscriptionSymbol {
    pub vtable_: *const IMTConSubscriptionSymbol__bindgen_vtable,
}
pub const IMTConSubscriptionSymbol_EnLevel_LEVEL_DELAYED: IMTConSubscriptionSymbol_EnLevel = 0;
pub const IMTConSubscriptionSymbol_EnLevel_LEVEL_REALTIME_LEVEL_1:
    IMTConSubscriptionSymbol_EnLevel = 1;
pub const IMTConSubscriptionSymbol_EnLevel_LEVEL_REALTIME_LEVEL_2:
    IMTConSubscriptionSymbol_EnLevel = 2;
pub const IMTConSubscriptionSymbol_EnLevel_LEVEL_FIRST: IMTConSubscriptionSymbol_EnLevel = 0;
pub const IMTConSubscriptionSymbol_EnLevel_LEVEL_LAST: IMTConSubscriptionSymbol_EnLevel = 2;
pub type IMTConSubscriptionSymbol_EnLevel = ::std::os::raw::c_int;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_NONE:
    IMTConSubscriptionSymbol_EnTickHistory = 0;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_DAY:
    IMTConSubscriptionSymbol_EnTickHistory = 1;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_WEEK:
    IMTConSubscriptionSymbol_EnTickHistory = 2;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_MONTH:
    IMTConSubscriptionSymbol_EnTickHistory = 3;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_3MONTHS:
    IMTConSubscriptionSymbol_EnTickHistory = 4;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_6MONTHS:
    IMTConSubscriptionSymbol_EnTickHistory = 5;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_YEAR:
    IMTConSubscriptionSymbol_EnTickHistory = 6;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_2YEARS:
    IMTConSubscriptionSymbol_EnTickHistory = 7;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_3YEARS:
    IMTConSubscriptionSymbol_EnTickHistory = 8;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST_5YEARS:
    IMTConSubscriptionSymbol_EnTickHistory = 9;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_ALL:
    IMTConSubscriptionSymbol_EnTickHistory = 10;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_FIRST:
    IMTConSubscriptionSymbol_EnTickHistory = 0;
pub const IMTConSubscriptionSymbol_EnTickHistory_TICK_HISTORY_LAST:
    IMTConSubscriptionSymbol_EnTickHistory = 10;
pub type IMTConSubscriptionSymbol_EnTickHistory = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSubscriptionSymbol"]
        [::std::mem::size_of::<IMTConSubscriptionSymbol>() - 8usize];
    ["Alignment of IMTConSubscriptionSymbol"]
        [::std::mem::align_of::<IMTConSubscriptionSymbol>() - 8usize];
};
#[repr(C)]
pub struct IMTConSubscriptionNews__bindgen_vtable {
    pub IMTConSubscriptionNews_Release: unsafe extern "C" fn(this: *mut IMTConSubscriptionNews),
    pub IMTConSubscriptionNews_Assign: unsafe extern "C" fn(
        this: *mut IMTConSubscriptionNews,
        news: *const IMTConSubscriptionNews,
    ) -> MTAPIRES,
    pub IMTConSubscriptionNews_Clear:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionNews) -> MTAPIRES,
    pub IMTConSubscriptionNews_Category1:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionNews, category: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscriptionNews_Category:
        unsafe extern "C" fn(this: *const IMTConSubscriptionNews) -> LPCWSTR,
    pub IMTConSubscriptionNews_Language1:
        unsafe extern "C" fn(this: *mut IMTConSubscriptionNews, language: UINT) -> MTAPIRES,
    pub IMTConSubscriptionNews_Language:
        unsafe extern "C" fn(this: *const IMTConSubscriptionNews) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSubscriptionNews {
    pub vtable_: *const IMTConSubscriptionNews__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSubscriptionNews"][::std::mem::size_of::<IMTConSubscriptionNews>() - 8usize];
    ["Alignment of IMTConSubscriptionNews"]
        [::std::mem::align_of::<IMTConSubscriptionNews>() - 8usize];
};
#[repr(C)]
pub struct IMTConSubscription__bindgen_vtable {
    pub IMTConSubscription_Release: unsafe extern "C" fn(this: *mut IMTConSubscription),
    pub IMTConSubscription_Assign: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        iface: *const IMTConSubscription,
    ) -> MTAPIRES,
    pub IMTConSubscription_Clear: unsafe extern "C" fn(this: *mut IMTConSubscription) -> MTAPIRES,
    pub IMTConSubscription_ID: unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT64,
    pub IMTConSubscription_ParentID:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT64,
    pub IMTConSubscription_DependsID1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, depends_id: UINT64) -> MTAPIRES,
    pub IMTConSubscription_DependsID:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT64,
    pub IMTConSubscription_Name1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, name: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_Name: unsafe extern "C" fn(this: *const IMTConSubscription) -> LPCWSTR,
    pub IMTConSubscription_Type1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, type_: UINT) -> MTAPIRES,
    pub IMTConSubscription_Type: unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_Image1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, image: UINT) -> MTAPIRES,
    pub IMTConSubscription_Image: unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_Description1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, description: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_Description:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> LPCWSTR,
    pub IMTConSubscription_URLDescription1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, url: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_URLDescription:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> LPCWSTR,
    pub IMTConSubscription_URLAgreement1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, url: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_URLAgreement:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> LPCWSTR,
    pub IMTConSubscription_ControlMode1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, mode: UINT) -> MTAPIRES,
    pub IMTConSubscription_ControlMode:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_PeriodMode1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, mode: UINT) -> MTAPIRES,
    pub IMTConSubscription_PeriodMode:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_PeriodCustom1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, days: UINT) -> MTAPIRES,
    pub IMTConSubscription_PeriodCustom:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_PeriodFreeMode1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, mode: UINT) -> MTAPIRES,
    pub IMTConSubscription_PeriodFreeMode:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_PeriodFreeCustom1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, days: UINT) -> MTAPIRES,
    pub IMTConSubscription_PeriodFreeCustom:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_Flags1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, flags: UINT) -> MTAPIRES,
    pub IMTConSubscription_Flags: unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_Price1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, price: f64) -> MTAPIRES,
    pub IMTConSubscription_Price: unsafe extern "C" fn(this: *const IMTConSubscription) -> f64,
    pub IMTConSubscription_PricePro1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, price: f64) -> MTAPIRES,
    pub IMTConSubscription_PricePro: unsafe extern "C" fn(this: *const IMTConSubscription) -> f64,
    pub IMTConSubscription_PriceCost1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, price: f64) -> MTAPIRES,
    pub IMTConSubscription_PriceCost: unsafe extern "C" fn(this: *const IMTConSubscription) -> f64,
    pub IMTConSubscription_PriceCurrency1:
        unsafe extern "C" fn(this: *mut IMTConSubscription, currency: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_PriceCurrency:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> LPCWSTR,
    pub IMTConSubscription_CountryAdd:
        unsafe extern "C" fn(this: *mut IMTConSubscription, path: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_CountryUpdate:
        unsafe extern "C" fn(this: *mut IMTConSubscription, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_CountryShift: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSubscription_CountryDelete:
        unsafe extern "C" fn(this: *mut IMTConSubscription, pos: UINT) -> MTAPIRES,
    pub IMTConSubscription_CountryClear:
        unsafe extern "C" fn(this: *mut IMTConSubscription) -> MTAPIRES,
    pub IMTConSubscription_CountryTotal:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_CountryNext:
        unsafe extern "C" fn(this: *const IMTConSubscription, pos: UINT) -> LPCWSTR,
    pub IMTConSubscription_GroupAdd:
        unsafe extern "C" fn(this: *mut IMTConSubscription, path: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_GroupUpdate:
        unsafe extern "C" fn(this: *mut IMTConSubscription, pos: UINT, path: LPCWSTR) -> MTAPIRES,
    pub IMTConSubscription_GroupDelete:
        unsafe extern "C" fn(this: *mut IMTConSubscription, pos: UINT) -> MTAPIRES,
    pub IMTConSubscription_GroupClear:
        unsafe extern "C" fn(this: *mut IMTConSubscription) -> MTAPIRES,
    pub IMTConSubscription_GroupShift: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSubscription_GroupTotal:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_GroupNext:
        unsafe extern "C" fn(this: *const IMTConSubscription, pos: UINT) -> LPCWSTR,
    pub IMTConSubscription_SymbolAdd: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        symbol: *mut IMTConSubscriptionSymbol,
    ) -> MTAPIRES,
    pub IMTConSubscription_SymbolUpdate: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        pos: UINT,
        symbol: *const IMTConSubscriptionSymbol,
    ) -> MTAPIRES,
    pub IMTConSubscription_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTConSubscription, pos: UINT) -> MTAPIRES,
    pub IMTConSubscription_SymbolClear:
        unsafe extern "C" fn(this: *mut IMTConSubscription) -> MTAPIRES,
    pub IMTConSubscription_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSubscription_SymbolTotal:
        unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_SymbolNext: unsafe extern "C" fn(
        this: *const IMTConSubscription,
        pos: UINT,
        symbol: *mut IMTConSubscriptionSymbol,
    ) -> MTAPIRES,
    pub IMTConSubscription_NewsAdd: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        news: *mut IMTConSubscriptionNews,
    ) -> MTAPIRES,
    pub IMTConSubscription_NewsUpdate: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        pos: UINT,
        news: *const IMTConSubscriptionNews,
    ) -> MTAPIRES,
    pub IMTConSubscription_NewsDelete:
        unsafe extern "C" fn(this: *mut IMTConSubscription, pos: UINT) -> MTAPIRES,
    pub IMTConSubscription_NewsClear:
        unsafe extern "C" fn(this: *mut IMTConSubscription) -> MTAPIRES,
    pub IMTConSubscription_NewsShift: unsafe extern "C" fn(
        this: *mut IMTConSubscription,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConSubscription_NewsTotal: unsafe extern "C" fn(this: *const IMTConSubscription) -> UINT,
    pub IMTConSubscription_NewsNext: unsafe extern "C" fn(
        this: *const IMTConSubscription,
        pos: UINT,
        news: *mut IMTConSubscriptionNews,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConSubscription {
    pub vtable_: *const IMTConSubscription__bindgen_vtable,
}
pub const IMTConSubscription_EnType_TYPE_SUBSCRIPTION: IMTConSubscription_EnType = 0;
pub const IMTConSubscription_EnType_TYPE_FOLDER: IMTConSubscription_EnType = 1;
pub const IMTConSubscription_EnType_TYPE_FIRST: IMTConSubscription_EnType = 0;
pub const IMTConSubscription_EnType_TYPE_LAST: IMTConSubscription_EnType = 1;
pub type IMTConSubscription_EnType = ::std::os::raw::c_int;
pub const IMTConSubscription_EnPeriod_PERIOD_ONCE: IMTConSubscription_EnPeriod = 0;
pub const IMTConSubscription_EnPeriod_PERIOD_DAILY: IMTConSubscription_EnPeriod = 1;
pub const IMTConSubscription_EnPeriod_PERIOD_WEEKLY: IMTConSubscription_EnPeriod = 2;
pub const IMTConSubscription_EnPeriod_PERIOD_MONTHLY: IMTConSubscription_EnPeriod = 3;
pub const IMTConSubscription_EnPeriod_PERIOD_QUARTERLY: IMTConSubscription_EnPeriod = 4;
pub const IMTConSubscription_EnPeriod_PERIOD_ANNUAL: IMTConSubscription_EnPeriod = 5;
pub const IMTConSubscription_EnPeriod_PERIOD_CUSTOM: IMTConSubscription_EnPeriod = 100;
pub const IMTConSubscription_EnPeriod_PERIOD_FIRST: IMTConSubscription_EnPeriod = 0;
pub const IMTConSubscription_EnPeriod_PERIOD_LAST: IMTConSubscription_EnPeriod = 100;
pub type IMTConSubscription_EnPeriod = ::std::os::raw::c_int;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_NONE: IMTConSubscription_EnFreePeriod = 0;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_DAY: IMTConSubscription_EnFreePeriod = 1;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_WEEK: IMTConSubscription_EnFreePeriod = 2;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_MONTH: IMTConSubscription_EnFreePeriod = 3;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_QUARTER: IMTConSubscription_EnFreePeriod = 4;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_YEAR: IMTConSubscription_EnFreePeriod = 5;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_CUSTOM: IMTConSubscription_EnFreePeriod = 100;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_FIRST: IMTConSubscription_EnFreePeriod = 0;
pub const IMTConSubscription_EnFreePeriod_FREE_PERIOD_LAST: IMTConSubscription_EnFreePeriod = 100;
pub type IMTConSubscription_EnFreePeriod = ::std::os::raw::c_int;
pub const IMTConSubscription_EnFlags_FLAG_NONE: IMTConSubscription_EnFlags = 0;
pub const IMTConSubscription_EnFlags_FLAG_ENABLE: IMTConSubscription_EnFlags = 1;
pub const IMTConSubscription_EnFlags_FLAG_AUTO_PROLONG: IMTConSubscription_EnFlags = 2;
pub const IMTConSubscription_EnFlags_FLAG_ALL: IMTConSubscription_EnFlags = 3;
pub type IMTConSubscription_EnFlags = ::std::os::raw::c_int;
pub const IMTConSubscription_EnControlMode_CONTROL_FULL: IMTConSubscription_EnControlMode = 0;
pub const IMTConSubscription_EnControlMode_CONTROL_UNSUBCRIBE: IMTConSubscription_EnControlMode = 1;
pub const IMTConSubscription_EnControlMode_CONTROL_VIEW: IMTConSubscription_EnControlMode = 2;
pub const IMTConSubscription_EnControlMode_CONTROL_HIDDEN: IMTConSubscription_EnControlMode = 3;
pub const IMTConSubscription_EnControlMode_CONTROL_FIRST: IMTConSubscription_EnControlMode = 0;
pub const IMTConSubscription_EnControlMode_CONTROL_LAST: IMTConSubscription_EnControlMode = 3;
pub type IMTConSubscription_EnControlMode = ::std::os::raw::c_int;
pub const IMTConSubscription_EnImageType_IMAGE_CUSTOM: IMTConSubscription_EnImageType = 0;
pub const IMTConSubscription_EnImageType_IMAGE_DEFAULT: IMTConSubscription_EnImageType = 1;
pub const IMTConSubscription_EnImageType_IMAGE_STOCK_EXCHANGE: IMTConSubscription_EnImageType = 2;
pub const IMTConSubscription_EnImageType_IMAGE_AUDIT: IMTConSubscription_EnImageType = 3;
pub const IMTConSubscription_EnImageType_IMAGE_REPORT: IMTConSubscription_EnImageType = 4;
pub const IMTConSubscription_EnImageType_IMAGE_PERSONAL_MANAGER: IMTConSubscription_EnImageType = 5;
pub const IMTConSubscription_EnImageType_IMAGE_DOCUMENTS_DELIVERY: IMTConSubscription_EnImageType =
    6;
pub const IMTConSubscription_EnImageType_IMAGE_DOCUMENTS_STORING: IMTConSubscription_EnImageType =
    7;
pub const IMTConSubscription_EnImageType_IMAGE_TRANSFER: IMTConSubscription_EnImageType = 8;
pub const IMTConSubscription_EnImageType_IMAGE_CONVERSION: IMTConSubscription_EnImageType = 9;
pub const IMTConSubscription_EnImageType_IMAGE_NASDAQ: IMTConSubscription_EnImageType = 1000;
pub const IMTConSubscription_EnImageType_IMAGE_TMX_GROUP: IMTConSubscription_EnImageType = 1001;
pub const IMTConSubscription_EnImageType_IMAGE_CANADIAN_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1002;
pub const IMTConSubscription_EnImageType_IMAGE_CBOE: IMTConSubscription_EnImageType = 1003;
pub const IMTConSubscription_EnImageType_IMAGE_CBOE_FUTURES_EXCHANGE:
    IMTConSubscription_EnImageType = 1004;
pub const IMTConSubscription_EnImageType_IMAGE_MEXICAN_DERIVATIVES: IMTConSubscription_EnImageType =
    1005;
pub const IMTConSubscription_EnImageType_IMAGE_FUND_SERV: IMTConSubscription_EnImageType = 1006;
pub const IMTConSubscription_EnImageType_IMAGE_CME_CBOT: IMTConSubscription_EnImageType = 1007;
pub const IMTConSubscription_EnImageType_IMAGE_CME: IMTConSubscription_EnImageType = 1008;
pub const IMTConSubscription_EnImageType_IMAGE_GLOBAL_OTC: IMTConSubscription_EnImageType = 1009;
pub const IMTConSubscription_EnImageType_IMAGE_ICE_FUTURES: IMTConSubscription_EnImageType = 1010;
pub const IMTConSubscription_EnImageType_IMAGE_IEX_GROUP: IMTConSubscription_EnImageType = 1011;
pub const IMTConSubscription_EnImageType_IMAGE_ISE_OPTIONS: IMTConSubscription_EnImageType = 1012;
pub const IMTConSubscription_EnImageType_IMAGE_NYSE: IMTConSubscription_EnImageType = 1013;
pub const IMTConSubscription_EnImageType_IMAGE_ONE_CHICAGO: IMTConSubscription_EnImageType = 1014;
pub const IMTConSubscription_EnImageType_IMAGE_OTC_MARKETS: IMTConSubscription_EnImageType = 1015;
pub const IMTConSubscription_EnImageType_IMAGE_BOND_RATINGS: IMTConSubscription_EnImageType = 1016;
pub const IMTConSubscription_EnImageType_IMAGE_US_REG_NMS: IMTConSubscription_EnImageType = 1017;
pub const IMTConSubscription_EnImageType_IMAGE_BOVESPA: IMTConSubscription_EnImageType = 1018;
pub const IMTConSubscription_EnImageType_IMAGE_VIENNA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1019;
pub const IMTConSubscription_EnImageType_IMAGE_EURONEXT: IMTConSubscription_EnImageType = 1020;
pub const IMTConSubscription_EnImageType_IMAGE_GERMAN_ETFS: IMTConSubscription_EnImageType = 1021;
pub const IMTConSubscription_EnImageType_IMAGE_BOLSA_DE_MADRID: IMTConSubscription_EnImageType =
    1022;
pub const IMTConSubscription_EnImageType_IMAGE_STOXX: IMTConSubscription_EnImageType = 1023;
pub const IMTConSubscription_EnImageType_IMAGE_MEFF: IMTConSubscription_EnImageType = 1024;
pub const IMTConSubscription_EnImageType_IMAGE_BORSA_ITALIANA: IMTConSubscription_EnImageType =
    1025;
pub const IMTConSubscription_EnImageType_IMAGE_EUREX: IMTConSubscription_EnImageType = 1026;
pub const IMTConSubscription_EnImageType_IMAGE_MOSCOW_EXCHANGE: IMTConSubscription_EnImageType =
    1027;
pub const IMTConSubscription_EnImageType_IMAGE_NORDIC_DERIVATIVES_EXCHANGE:
    IMTConSubscription_EnImageType = 1028;
pub const IMTConSubscription_EnImageType_IMAGE_OSLO_STOCK_EXCHANGE: IMTConSubscription_EnImageType =
    1029;
pub const IMTConSubscription_EnImageType_IMAGE_PRAGUE_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1030;
pub const IMTConSubscription_EnImageType_IMAGE_SIX_GROUP: IMTConSubscription_EnImageType = 1031;
pub const IMTConSubscription_EnImageType_IMAGE_XETRA: IMTConSubscription_EnImageType = 1032;
pub const IMTConSubscription_EnImageType_IMAGE_BOERSE_STUTTGART: IMTConSubscription_EnImageType =
    1033;
pub const IMTConSubscription_EnImageType_IMAGE_TURQUOISE: IMTConSubscription_EnImageType = 1034;
pub const IMTConSubscription_EnImageType_IMAGE_LONDON_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1035;
pub const IMTConSubscription_EnImageType_IMAGE_WARSAW_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1036;
pub const IMTConSubscription_EnImageType_IMAGE_BUDAPEST_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1037;
pub const IMTConSubscription_EnImageType_IMAGE_BATS_CHI_X_EUROPE: IMTConSubscription_EnImageType =
    1038;
pub const IMTConSubscription_EnImageType_IMAGE_LONDON_METAL_EXCHANGE:
    IMTConSubscription_EnImageType = 1039;
pub const IMTConSubscription_EnImageType_IMAGE_EUROPEAN_MUTUAL_FUNDS:
    IMTConSubscription_EnImageType = 1040;
pub const IMTConSubscription_EnImageType_IMAGE_TEL_AVIV_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1041;
pub const IMTConSubscription_EnImageType_IMAGE_JOHANNESBURG_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1042;
pub const IMTConSubscription_EnImageType_IMAGE_HANG_SENG_BANK: IMTConSubscription_EnImageType =
    1043;
pub const IMTConSubscription_EnImageType_IMAGE_CHI_X_AUSTRALIA: IMTConSubscription_EnImageType =
    1044;
pub const IMTConSubscription_EnImageType_IMAGE_HKEX: IMTConSubscription_EnImageType = 1045;
pub const IMTConSubscription_EnImageType_IMAGE_JPX: IMTConSubscription_EnImageType = 1046;
pub const IMTConSubscription_EnImageType_IMAGE_SHANGHAI_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1047;
pub const IMTConSubscription_EnImageType_IMAGE_SHENZHEN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1048;
pub const IMTConSubscription_EnImageType_IMAGE_SINGAPORE_EXCHANGE: IMTConSubscription_EnImageType =
    1049;
pub const IMTConSubscription_EnImageType_IMAGE_AUSTRALIAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1050;
pub const IMTConSubscription_EnImageType_IMAGE_KOREA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1051;
pub const IMTConSubscription_EnImageType_IMAGE_A2X_MARKETS: IMTConSubscription_EnImageType = 1052;
pub const IMTConSubscription_EnImageType_IMAGE_ASCE: IMTConSubscription_EnImageType = 1053;
pub const IMTConSubscription_EnImageType_IMAGE_ALTX_EAST_AFRICA_EXCHANGE:
    IMTConSubscription_EnImageType = 1054;
pub const IMTConSubscription_EnImageType_IMAGE_AMMAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1055;
pub const IMTConSubscription_EnImageType_IMAGE_ARMENIA_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1056;
pub const IMTConSubscription_EnImageType_IMAGE_ATHENS_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1057;
pub const IMTConSubscription_EnImageType_IMAGE_BAKU_STOCK_EXCHANGE: IMTConSubscription_EnImageType =
    1058;
pub const IMTConSubscription_EnImageType_IMAGE_BANJA_LUKA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1059;
pub const IMTConSubscription_EnImageType_IMAGE_BERMUDA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1060;
pub const IMTConSubscription_EnImageType_IMAGE_BOLIVIA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1061;
pub const IMTConSubscription_EnImageType_IMAGE_BVRD: IMTConSubscription_EnImageType = 1062;
pub const IMTConSubscription_EnImageType_IMAGE_BOLSAS_Y_MERCADOS_ESPANOLES:
    IMTConSubscription_EnImageType = 1063;
pub const IMTConSubscription_EnImageType_IMAGE_BOMBAY_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1064;
pub const IMTConSubscription_EnImageType_IMAGE_BORSA_ISTANBUL: IMTConSubscription_EnImageType =
    1065;
pub const IMTConSubscription_EnImageType_IMAGE_BOTSWANA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1067;
pub const IMTConSubscription_EnImageType_IMAGE_BOURSA_KUWAIT: IMTConSubscription_EnImageType = 1068;
pub const IMTConSubscription_EnImageType_IMAGE_BVMAC: IMTConSubscription_EnImageType = 1069;
pub const IMTConSubscription_EnImageType_IMAGE_BOURSE_DE_TUNIS: IMTConSubscription_EnImageType =
    1070;
pub const IMTConSubscription_EnImageType_IMAGE_BRVM: IMTConSubscription_EnImageType = 1071;
pub const IMTConSubscription_EnImageType_IMAGE_BRAZIL_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1072;
pub const IMTConSubscription_EnImageType_IMAGE_BUCHAREST_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1073;
pub const IMTConSubscription_EnImageType_IMAGE_BUENOS_AIRES_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1074;
pub const IMTConSubscription_EnImageType_IMAGE_BULGARIAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1075;
pub const IMTConSubscription_EnImageType_IMAGE_BURSA_MALAYSIA: IMTConSubscription_EnImageType =
    1076;
pub const IMTConSubscription_EnImageType_IMAGE_CALCUTTA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1077;
pub const IMTConSubscription_EnImageType_IMAGE_CARACAS_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1078;
pub const IMTConSubscription_EnImageType_IMAGE_CASABLANCA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1079;
pub const IMTConSubscription_EnImageType_IMAGE_COLOMBIA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1080;
pub const IMTConSubscription_EnImageType_IMAGE_COLOMBO_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1081;
pub const IMTConSubscription_EnImageType_IMAGE_CYPRUS_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1082;
pub const IMTConSubscription_EnImageType_IMAGE_DALIAN_COMMODITY_EXCHANGE:
    IMTConSubscription_EnImageType = 1083;
pub const IMTConSubscription_EnImageType_IMAGE_DAMASCUS_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1084;
pub const IMTConSubscription_EnImageType_IMAGE_DAR_ES_SALAAM_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1085;
pub const IMTConSubscription_EnImageType_IMAGE_DEUTSCHE_BOERSE: IMTConSubscription_EnImageType =
    1086;
pub const IMTConSubscription_EnImageType_IMAGE_DHAKA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1087;
pub const IMTConSubscription_EnImageType_IMAGE_DOHA_SECURITIES_MARKET:
    IMTConSubscription_EnImageType = 1088;
pub const IMTConSubscription_EnImageType_IMAGE_DOUALA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1089;
pub const IMTConSubscription_EnImageType_IMAGE_DUBAI_FINANCIAL_MARKET:
    IMTConSubscription_EnImageType = 1090;
pub const IMTConSubscription_EnImageType_IMAGE_DUTCH_CARIBBEAN_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1091;
pub const IMTConSubscription_EnImageType_IMAGE_EGYPTIAN_EXCHANGE: IMTConSubscription_EnImageType =
    1092;
pub const IMTConSubscription_EnImageType_IMAGE_ESWATINI_STOCK_MARKET:
    IMTConSubscription_EnImageType = 1093;
pub const IMTConSubscription_EnImageType_IMAGE_FRANKFURT_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1094;
pub const IMTConSubscription_EnImageType_IMAGE_GEORGIAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1095;
pub const IMTConSubscription_EnImageType_IMAGE_GHANA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1096;
pub const IMTConSubscription_EnImageType_IMAGE_INDONESIA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1097;
pub const IMTConSubscription_EnImageType_IMAGE_IRAN_FARA_BOURSE: IMTConSubscription_EnImageType =
    1098;
pub const IMTConSubscription_EnImageType_IMAGE_IRAN_MERCANTILE_EXCHANGE:
    IMTConSubscription_EnImageType = 1099;
pub const IMTConSubscription_EnImageType_IMAGE_ISLAMABAD_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1100;
pub const IMTConSubscription_EnImageType_IMAGE_JAMAICA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1101;
pub const IMTConSubscription_EnImageType_IMAGE_KAZAKHSTAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1102;
pub const IMTConSubscription_EnImageType_IMAGE_KHARTOUM_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1103;
pub const IMTConSubscription_EnImageType_IMAGE_LAHORE_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1104;
pub const IMTConSubscription_EnImageType_IMAGE_LIBYAN_STOCK_MARKET: IMTConSubscription_EnImageType =
    1105;
pub const IMTConSubscription_EnImageType_IMAGE_LJUBLJANA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1106;
pub const IMTConSubscription_EnImageType_IMAGE_LUSAKA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1107;
pub const IMTConSubscription_EnImageType_IMAGE_LUXEMBOURG_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1108;
pub const IMTConSubscription_EnImageType_IMAGE_MALAWI_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1109;
pub const IMTConSubscription_EnImageType_IMAGE_MIAMI_INTERNATIONAL_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1110;
pub const IMTConSubscription_EnImageType_IMAGE_MONGOLIAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1111;
pub const IMTConSubscription_EnImageType_IMAGE_MULTI_COMMODITY_EXCHANGE:
    IMTConSubscription_EnImageType = 1112;
pub const IMTConSubscription_EnImageType_IMAGE_MUSCAT_SECURITIES_MARKET:
    IMTConSubscription_EnImageType = 1113;
pub const IMTConSubscription_EnImageType_IMAGE_NAIROBI_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1114;
pub const IMTConSubscription_EnImageType_IMAGE_NCDEX: IMTConSubscription_EnImageType = 1115;
pub const IMTConSubscription_EnImageType_IMAGE_NATIONAL_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1116;
pub const IMTConSubscription_EnImageType_IMAGE_NEO_EXCHANGE: IMTConSubscription_EnImageType = 1117;
pub const IMTConSubscription_EnImageType_IMAGE_NEPAL_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1118;
pub const IMTConSubscription_EnImageType_IMAGE_NEW_ZEALAND_EXCHANGE:
    IMTConSubscription_EnImageType = 1119;
pub const IMTConSubscription_EnImageType_IMAGE_NIGERIAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1120;
pub const IMTConSubscription_EnImageType_IMAGE_NXCHANGE: IMTConSubscription_EnImageType = 1121;
pub const IMTConSubscription_EnImageType_IMAGE_PAKISTAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1122;
pub const IMTConSubscription_EnImageType_IMAGE_PALESTINE_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1123;
pub const IMTConSubscription_EnImageType_IMAGE_PFTS_UKRAINE_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1124;
pub const IMTConSubscription_EnImageType_IMAGE_PHILIPPINE_DEALING_EXCHANGE:
    IMTConSubscription_EnImageType = 1125;
pub const IMTConSubscription_EnImageType_IMAGE_PHILIPPINE_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1126;
pub const IMTConSubscription_EnImageType_IMAGE_SAINT_PETERSBURG_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1127;
pub const IMTConSubscription_EnImageType_IMAGE_SVGEX: IMTConSubscription_EnImageType = 1128;
pub const IMTConSubscription_EnImageType_IMAGE_SANTIAGO_STOCK_EXCHANGE_MILA:
    IMTConSubscription_EnImageType = 1129;
pub const IMTConSubscription_EnImageType_IMAGE_STOCK_EXCHANGE_OF_MAURITIUS:
    IMTConSubscription_EnImageType = 1130;
pub const IMTConSubscription_EnImageType_IMAGE_STOCK_EXCHANGE_OF_THAILAND:
    IMTConSubscription_EnImageType = 1131;
pub const IMTConSubscription_EnImageType_IMAGE_TADAWUL: IMTConSubscription_EnImageType = 1132;
pub const IMTConSubscription_EnImageType_IMAGE_TAIWAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1133;
pub const IMTConSubscription_EnImageType_IMAGE_TEHRAN_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1134;
pub const IMTConSubscription_EnImageType_IMAGE_TIRANA_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1135;
pub const IMTConSubscription_EnImageType_IMAGE_TOKYO_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1136;
pub const IMTConSubscription_EnImageType_IMAGE_UGANDA_SECURITIES_EXCHANGE:
    IMTConSubscription_EnImageType = 1137;
pub const IMTConSubscription_EnImageType_IMAGE_UKRAINIAN_EXCHANGE: IMTConSubscription_EnImageType =
    1138;
pub const IMTConSubscription_EnImageType_IMAGE_ZAGREB_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1139;
pub const IMTConSubscription_EnImageType_IMAGE_ZIMBABWE_STOCK_EXCHANGE:
    IMTConSubscription_EnImageType = 1140;
pub const IMTConSubscription_EnImageType_IMAGE_BALTIC_EXCHANGE: IMTConSubscription_EnImageType =
    1141;
pub const IMTConSubscription_EnImageType_IMAGE_FIRST: IMTConSubscription_EnImageType = 0;
pub const IMTConSubscription_EnImageType_IMAGE_LAST: IMTConSubscription_EnImageType = 1141;
pub type IMTConSubscription_EnImageType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSubscription"][::std::mem::size_of::<IMTConSubscription>() - 8usize];
    ["Alignment of IMTConSubscription"][::std::mem::align_of::<IMTConSubscription>() - 8usize];
};
#[repr(C)]
pub struct IMTConSubscriptionSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConSubscriptionSink {
    pub vtable_: *const IMTConSubscriptionSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConSubscriptionSink"][::std::mem::size_of::<IMTConSubscriptionSink>() - 8usize];
    ["Alignment of IMTConSubscriptionSink"]
        [::std::mem::align_of::<IMTConSubscriptionSink>() - 8usize];
};
#[repr(C)]
pub struct IMTSubscription__bindgen_vtable {
    pub IMTSubscription_Release: unsafe extern "C" fn(this: *mut IMTSubscription),
    pub IMTSubscription_Assign:
        unsafe extern "C" fn(this: *mut IMTSubscription, obj: *const IMTSubscription) -> MTAPIRES,
    pub IMTSubscription_Clear: unsafe extern "C" fn(this: *mut IMTSubscription) -> MTAPIRES,
    pub IMTSubscription_ID: unsafe extern "C" fn(this: *const IMTSubscription) -> UINT64,
    pub IMTSubscription_Login1:
        unsafe extern "C" fn(this: *mut IMTSubscription, login: UINT64) -> MTAPIRES,
    pub IMTSubscription_Login: unsafe extern "C" fn(this: *const IMTSubscription) -> UINT64,
    pub IMTSubscription_Subscription1:
        unsafe extern "C" fn(this: *mut IMTSubscription, subscription_id: UINT64) -> MTAPIRES,
    pub IMTSubscription_Subscription: unsafe extern "C" fn(this: *const IMTSubscription) -> UINT64,
    pub IMTSubscription_Status1:
        unsafe extern "C" fn(this: *mut IMTSubscription, status: UINT) -> MTAPIRES,
    pub IMTSubscription_Status: unsafe extern "C" fn(this: *const IMTSubscription) -> UINT,
    pub IMTSubscription_TimeSubscribe1:
        unsafe extern "C" fn(this: *mut IMTSubscription, time: INT64) -> MTAPIRES,
    pub IMTSubscription_TimeSubscribe: unsafe extern "C" fn(this: *const IMTSubscription) -> INT64,
    pub IMTSubscription_TimeRenewal1:
        unsafe extern "C" fn(this: *mut IMTSubscription, time: INT64) -> MTAPIRES,
    pub IMTSubscription_TimeRenewal: unsafe extern "C" fn(this: *const IMTSubscription) -> INT64,
    pub IMTSubscription_TimeExpire1:
        unsafe extern "C" fn(this: *mut IMTSubscription, time: INT64) -> MTAPIRES,
    pub IMTSubscription_TimeExpire: unsafe extern "C" fn(this: *const IMTSubscription) -> INT64,
    pub IMTSubscription_Flags1:
        unsafe extern "C" fn(this: *mut IMTSubscription, flags: UINT) -> MTAPIRES,
    pub IMTSubscription_Flags: unsafe extern "C" fn(this: *const IMTSubscription) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTSubscription {
    pub vtable_: *const IMTSubscription__bindgen_vtable,
}
pub const IMTSubscription_EnStatus_STATUS_ACTIVE: IMTSubscription_EnStatus = 0;
pub const IMTSubscription_EnStatus_STATUS_SUSPENDED: IMTSubscription_EnStatus = 1;
pub const IMTSubscription_EnStatus_STATUS_CANCELED: IMTSubscription_EnStatus = 2;
pub const IMTSubscription_EnStatus_STATUS_FIRST: IMTSubscription_EnStatus = 0;
pub const IMTSubscription_EnStatus_STATUS_LAST: IMTSubscription_EnStatus = 2;
pub type IMTSubscription_EnStatus = ::std::os::raw::c_int;
pub const IMTSubscription_EnFlags_FLAG_FREE_PERIOD: IMTSubscription_EnFlags = 1;
pub const IMTSubscription_EnFlags_FLAG_NONE: IMTSubscription_EnFlags = 0;
pub const IMTSubscription_EnFlags_FLAG_ALL: IMTSubscription_EnFlags = 1;
pub type IMTSubscription_EnFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTSubscription"][::std::mem::size_of::<IMTSubscription>() - 8usize];
    ["Alignment of IMTSubscription"][::std::mem::align_of::<IMTSubscription>() - 8usize];
};
#[repr(C)]
pub struct IMTSubscriptionArray__bindgen_vtable {
    pub IMTSubscriptionArray_Release: unsafe extern "C" fn(this: *mut IMTSubscriptionArray),
    pub IMTSubscriptionArray_Assign: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        array: *const IMTSubscriptionArray,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_Clear:
        unsafe extern "C" fn(this: *mut IMTSubscriptionArray) -> MTAPIRES,
    pub IMTSubscriptionArray_Add1: unsafe extern "C" fn(
            this: *mut IMTSubscriptionArray,
            array: *mut IMTSubscriptionArray,
        ) -> MTAPIRES,
    pub IMTSubscriptionArray_Add: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        record: *mut IMTSubscription,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_AddCopy1: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        array: *const IMTSubscriptionArray,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_AddCopy: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        record: *const IMTSubscription,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_Delete:
        unsafe extern "C" fn(this: *mut IMTSubscriptionArray, pos: UINT) -> MTAPIRES,
    pub IMTSubscriptionArray_Detach:
        unsafe extern "C" fn(this: *mut IMTSubscriptionArray, pos: UINT) -> *mut IMTSubscription,
    pub IMTSubscriptionArray_Update: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        pos: UINT,
        record: *mut IMTSubscription,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        pos: UINT,
        record: *const IMTSubscription,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_Shift: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_Total: unsafe extern "C" fn(this: *const IMTSubscriptionArray) -> UINT,
    pub IMTSubscriptionArray_Next: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        index: UINT,
    ) -> *mut IMTSubscription,
    pub IMTSubscriptionArray_Sort: unsafe extern "C" fn(
        this: *mut IMTSubscriptionArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTSubscriptionArray_Search: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTSubscriptionArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTSubscriptionArray {
    pub vtable_: *const IMTSubscriptionArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTSubscriptionArray"][::std::mem::size_of::<IMTSubscriptionArray>() - 8usize];
    ["Alignment of IMTSubscriptionArray"][::std::mem::align_of::<IMTSubscriptionArray>() - 8usize];
};
#[repr(C)]
pub struct IMTSubscriptionHistory__bindgen_vtable {
    pub IMTSubscriptionHistory_Release: unsafe extern "C" fn(this: *mut IMTSubscriptionHistory),
    pub IMTSubscriptionHistory_Assign: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistory,
        obj: *const IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistory_Clear:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory) -> MTAPIRES,
    pub IMTSubscriptionHistory_ID:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT64,
    pub IMTSubscriptionHistory_Login1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, login: UINT64) -> MTAPIRES,
    pub IMTSubscriptionHistory_Login:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT64,
    pub IMTSubscriptionHistory_Subscription1: unsafe extern "C" fn(
            this: *mut IMTSubscriptionHistory,
            subscription_id: UINT64,
        ) -> MTAPIRES,
    pub IMTSubscriptionHistory_Subscription:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT64,
    pub IMTSubscriptionHistory_Record1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, record_id: UINT64) -> MTAPIRES,
    pub IMTSubscriptionHistory_Record:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT64,
    pub IMTSubscriptionHistory_Action1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, status: UINT) -> MTAPIRES,
    pub IMTSubscriptionHistory_Action:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT,
    pub IMTSubscriptionHistory_Time1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, time: INT64) -> MTAPIRES,
    pub IMTSubscriptionHistory_Time:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> INT64,
    pub IMTSubscriptionHistory_Amount1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, ammount: f64) -> MTAPIRES,
    pub IMTSubscriptionHistory_Amount:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> f64,
    pub IMTSubscriptionHistory_AmountDigits1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, digits: UINT) -> MTAPIRES,
    pub IMTSubscriptionHistory_AmountDigits:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT,
    pub IMTSubscriptionHistory_AmountDeal1:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistory, deal: UINT64) -> MTAPIRES,
    pub IMTSubscriptionHistory_AmountDeal:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistory) -> UINT64,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTSubscriptionHistory {
    pub vtable_: *const IMTSubscriptionHistory__bindgen_vtable,
}
pub const IMTSubscriptionHistory_EnAction_ACTION_SUBSCRIBE: IMTSubscriptionHistory_EnAction = 0;
pub const IMTSubscriptionHistory_EnAction_ACTION_RENEWAL: IMTSubscriptionHistory_EnAction = 1;
pub const IMTSubscriptionHistory_EnAction_ACTION_SUSPEND: IMTSubscriptionHistory_EnAction = 2;
pub const IMTSubscriptionHistory_EnAction_ACTION_CANCEL: IMTSubscriptionHistory_EnAction = 3;
pub const IMTSubscriptionHistory_EnAction_ACTION_DELETED: IMTSubscriptionHistory_EnAction = 4;
pub const IMTSubscriptionHistory_EnAction_ACTION_FIRST: IMTSubscriptionHistory_EnAction = 0;
pub const IMTSubscriptionHistory_EnAction_ACTION_LAST: IMTSubscriptionHistory_EnAction = 4;
pub type IMTSubscriptionHistory_EnAction = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTSubscriptionHistory"][::std::mem::size_of::<IMTSubscriptionHistory>() - 8usize];
    ["Alignment of IMTSubscriptionHistory"]
        [::std::mem::align_of::<IMTSubscriptionHistory>() - 8usize];
};
#[repr(C)]
pub struct IMTSubscriptionHistoryArray__bindgen_vtable {
    pub IMTSubscriptionHistoryArray_Release:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistoryArray),
    pub IMTSubscriptionHistoryArray_Assign: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        array: *const IMTSubscriptionHistoryArray,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Clear:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistoryArray) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Add: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        record: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_AddCopy: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        record: *const IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Add1: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        array: *mut IMTSubscriptionHistoryArray,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_AddCopy1: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        array: *const IMTSubscriptionHistoryArray,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Delete:
        unsafe extern "C" fn(this: *mut IMTSubscriptionHistoryArray, pos: UINT) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Detach: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        pos: UINT,
    )
        -> *mut IMTSubscriptionHistory,
    pub IMTSubscriptionHistoryArray_Update: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        pos: UINT,
        record: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_UpdateCopy: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        pos: UINT,
        record: *const IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Shift: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Total:
        unsafe extern "C" fn(this: *const IMTSubscriptionHistoryArray) -> UINT,
    pub IMTSubscriptionHistoryArray_Next: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        index: UINT,
    ) -> *mut IMTSubscriptionHistory,
    pub IMTSubscriptionHistoryArray_Sort: unsafe extern "C" fn(
        this: *mut IMTSubscriptionHistoryArray,
        sort_function: MTSortFunctionPtr,
    ) -> MTAPIRES,
    pub IMTSubscriptionHistoryArray_Search: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionHistoryArray_SearchGreatOrEq: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    )
        -> ::std::os::raw::c_int,
    pub IMTSubscriptionHistoryArray_SearchGreater: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    )
        -> ::std::os::raw::c_int,
    pub IMTSubscriptionHistoryArray_SearchLessOrEq: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    )
        -> ::std::os::raw::c_int,
    pub IMTSubscriptionHistoryArray_SearchLess: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionHistoryArray_SearchLeft: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
    pub IMTSubscriptionHistoryArray_SearchRight: unsafe extern "C" fn(
        this: *const IMTSubscriptionHistoryArray,
        key: *const ::std::os::raw::c_void,
        sort_function: MTSortFunctionPtr,
    ) -> ::std::os::raw::c_int,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTSubscriptionHistoryArray {
    pub vtable_: *const IMTSubscriptionHistoryArray__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTSubscriptionHistoryArray"]
        [::std::mem::size_of::<IMTSubscriptionHistoryArray>() - 8usize];
    ["Alignment of IMTSubscriptionHistoryArray"]
        [::std::mem::align_of::<IMTSubscriptionHistoryArray>() - 8usize];
};
#[repr(C)]
pub struct IMTSubscriptionSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTSubscriptionSink {
    pub vtable_: *const IMTSubscriptionSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTSubscriptionSink"][::std::mem::size_of::<IMTSubscriptionSink>() - 8usize];
    ["Alignment of IMTSubscriptionSink"][::std::mem::align_of::<IMTSubscriptionSink>() - 8usize];
};
#[repr(C)]
pub struct IMTSubscriptionHistorySink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTSubscriptionHistorySink {
    pub vtable_: *const IMTSubscriptionHistorySink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTSubscriptionHistorySink"]
        [::std::mem::size_of::<IMTSubscriptionHistorySink>() - 8usize];
    ["Alignment of IMTSubscriptionHistorySink"]
        [::std::mem::align_of::<IMTSubscriptionHistorySink>() - 8usize];
};
#[repr(C)]
pub struct IMTConVPSGroup__bindgen_vtable {
    pub IMTConVPSGroup_Release: unsafe extern "C" fn(this: *mut IMTConVPSGroup),
    pub IMTConVPSGroup_Assign:
        unsafe extern "C" fn(this: *mut IMTConVPSGroup, param: *mut IMTConVPSGroup) -> MTAPIRES,
    pub IMTConVPSGroup_Clear: unsafe extern "C" fn(this: *mut IMTConVPSGroup) -> MTAPIRES,
    pub IMTConVPSGroup_Group1:
        unsafe extern "C" fn(this: *mut IMTConVPSGroup, group: LPCWSTR) -> MTAPIRES,
    pub IMTConVPSGroup_Group: unsafe extern "C" fn(this: *const IMTConVPSGroup) -> LPCWSTR,
    pub IMTConVPSGroup_MinBalance1:
        unsafe extern "C" fn(this: *mut IMTConVPSGroup, balance: f64) -> MTAPIRES,
    pub IMTConVPSGroup_MinBalance: unsafe extern "C" fn(this: *const IMTConVPSGroup) -> f64,
    pub IMTConVPSGroup_InactiveDays1:
        unsafe extern "C" fn(this: *mut IMTConVPSGroup, days: UINT) -> MTAPIRES,
    pub IMTConVPSGroup_InactiveDays: unsafe extern "C" fn(this: *const IMTConVPSGroup) -> UINT,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConVPSGroup {
    pub vtable_: *const IMTConVPSGroup__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConVPSGroup"][::std::mem::size_of::<IMTConVPSGroup>() - 8usize];
    ["Alignment of IMTConVPSGroup"][::std::mem::align_of::<IMTConVPSGroup>() - 8usize];
};
#[repr(C)]
pub struct IMTConVPS__bindgen_vtable {
    pub IMTConVPS_Release: unsafe extern "C" fn(this: *mut IMTConVPS),
    pub IMTConVPS_Assign:
        unsafe extern "C" fn(this: *mut IMTConVPS, param: *mut IMTConVPS) -> MTAPIRES,
    pub IMTConVPS_Clear: unsafe extern "C" fn(this: *mut IMTConVPS) -> MTAPIRES,
    pub IMTConVPS_Flags1: unsafe extern "C" fn(this: *mut IMTConVPS, flags: UINT64) -> MTAPIRES,
    pub IMTConVPS_Flags: unsafe extern "C" fn(this: *const IMTConVPS) -> UINT64,
    pub IMTConVPS_MQL5Login1:
        unsafe extern "C" fn(this: *mut IMTConVPS, login: LPCWSTR) -> MTAPIRES,
    pub IMTConVPS_MQL5Login: unsafe extern "C" fn(this: *const IMTConVPS) -> LPCWSTR,
    pub IMTConVPS_MQL5Password1:
        unsafe extern "C" fn(this: *mut IMTConVPS, password: LPCWSTR) -> MTAPIRES,
    pub IMTConVPS_MQL5Password: unsafe extern "C" fn(this: *const IMTConVPS) -> LPCWSTR,
    pub IMTConVPS_GroupAdd:
        unsafe extern "C" fn(this: *mut IMTConVPS, group: *mut IMTConVPSGroup) -> MTAPIRES,
    pub IMTConVPS_GroupUpdate: unsafe extern "C" fn(
        this: *mut IMTConVPS,
        pos: UINT,
        group: *const IMTConVPSGroup,
    ) -> MTAPIRES,
    pub IMTConVPS_GroupDelete: unsafe extern "C" fn(this: *mut IMTConVPS, pos: UINT) -> MTAPIRES,
    pub IMTConVPS_GroupClear: unsafe extern "C" fn(this: *mut IMTConVPS) -> MTAPIRES,
    pub IMTConVPS_GroupShift: unsafe extern "C" fn(
        this: *mut IMTConVPS,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConVPS_GroupTotal: unsafe extern "C" fn(this: *const IMTConVPS) -> UINT,
    pub IMTConVPS_GroupNext: unsafe extern "C" fn(
        this: *const IMTConVPS,
        pos: UINT,
        group: *mut IMTConVPSGroup,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConVPS {
    pub vtable_: *const IMTConVPS__bindgen_vtable,
}
pub const IMTConVPS_EnFlags_VPS_NONE: IMTConVPS_EnFlags = 0;
pub const IMTConVPS_EnFlags_VPS_SPONSOR_ACTIVE: IMTConVPS_EnFlags = 1;
pub type IMTConVPS_EnFlags = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConVPS"][::std::mem::size_of::<IMTConVPS>() - 8usize];
    ["Alignment of IMTConVPS"][::std::mem::align_of::<IMTConVPS>() - 8usize];
};
#[repr(C)]
pub struct IMTConVPSSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConVPSSink {
    pub vtable_: *const IMTConVPSSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConVPSSink"][::std::mem::size_of::<IMTConVPSSink>() - 8usize];
    ["Alignment of IMTConVPSSink"][::std::mem::align_of::<IMTConVPSSink>() - 8usize];
};
#[repr(C)]
pub struct IMTConKYCCountry__bindgen_vtable {
    pub IMTConKYCCountry_Release: unsafe extern "C" fn(this: *mut IMTConKYCCountry),
    pub IMTConKYCCountry_Assign: unsafe extern "C" fn(
        this: *mut IMTConKYCCountry,
        country: *const IMTConKYCCountry,
    ) -> MTAPIRES,
    pub IMTConKYCCountry_Clear: unsafe extern "C" fn(this: *mut IMTConKYCCountry) -> MTAPIRES,
    pub IMTConKYCCountry_CountryCode1:
        unsafe extern "C" fn(this: *mut IMTConKYCCountry, code: LPCWSTR) -> MTAPIRES,
    pub IMTConKYCCountry_CountryCode:
        unsafe extern "C" fn(this: *const IMTConKYCCountry) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConKYCCountry {
    pub vtable_: *const IMTConKYCCountry__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConKYCCountry"][::std::mem::size_of::<IMTConKYCCountry>() - 8usize];
    ["Alignment of IMTConKYCCountry"][::std::mem::align_of::<IMTConKYCCountry>() - 8usize];
};
#[repr(C)]
pub struct IMTConKYCGroup__bindgen_vtable {
    pub IMTConKYCGroup_Release: unsafe extern "C" fn(this: *mut IMTConKYCGroup),
    pub IMTConKYCGroup_Assign:
        unsafe extern "C" fn(this: *mut IMTConKYCGroup, group: *const IMTConKYCGroup) -> MTAPIRES,
    pub IMTConKYCGroup_Clear: unsafe extern "C" fn(this: *mut IMTConKYCGroup) -> MTAPIRES,
    pub IMTConKYCGroup_Group1:
        unsafe extern "C" fn(this: *mut IMTConKYCGroup, group: LPCWSTR) -> MTAPIRES,
    pub IMTConKYCGroup_Group: unsafe extern "C" fn(this: *const IMTConKYCGroup) -> LPCWSTR,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConKYCGroup {
    pub vtable_: *const IMTConKYCGroup__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConKYCGroup"][::std::mem::size_of::<IMTConKYCGroup>() - 8usize];
    ["Alignment of IMTConKYCGroup"][::std::mem::align_of::<IMTConKYCGroup>() - 8usize];
};
#[repr(C)]
pub struct IMTConKYC__bindgen_vtable {
    pub IMTConKYC_Release: unsafe extern "C" fn(this: *mut IMTConKYC),
    pub IMTConKYC_Assign:
        unsafe extern "C" fn(this: *mut IMTConKYC, config: *const IMTConKYC) -> MTAPIRES,
    pub IMTConKYC_Clear: unsafe extern "C" fn(this: *mut IMTConKYC) -> MTAPIRES,
    pub IMTConKYC_Name1: unsafe extern "C" fn(this: *mut IMTConKYC, name: LPCWSTR) -> MTAPIRES,
    pub IMTConKYC_Name: unsafe extern "C" fn(this: *const IMTConKYC) -> LPCWSTR,
    pub IMTConKYC_ProviderType1:
        unsafe extern "C" fn(this: *mut IMTConKYC, provider: UINT) -> MTAPIRES,
    pub IMTConKYC_ProviderType: unsafe extern "C" fn(this: *const IMTConKYC) -> UINT,
    pub IMTConKYC_ProviderAddress1:
        unsafe extern "C" fn(this: *mut IMTConKYC, address: LPCWSTR) -> MTAPIRES,
    pub IMTConKYC_ProviderAddress: unsafe extern "C" fn(this: *const IMTConKYC) -> LPCWSTR,
    pub IMTConKYC_ProviderLogin1:
        unsafe extern "C" fn(this: *mut IMTConKYC, login: LPCWSTR) -> MTAPIRES,
    pub IMTConKYC_ProviderLogin: unsafe extern "C" fn(this: *const IMTConKYC) -> LPCWSTR,
    pub IMTConKYC_ProviderPassword1:
        unsafe extern "C" fn(this: *mut IMTConKYC, password: LPCWSTR) -> MTAPIRES,
    pub IMTConKYC_ProviderPassword: unsafe extern "C" fn(this: *const IMTConKYC) -> LPCWSTR,
    pub IMTConKYC_ProviderToken1:
        unsafe extern "C" fn(this: *mut IMTConKYC, token: LPCWSTR) -> MTAPIRES,
    pub IMTConKYC_ProviderToken: unsafe extern "C" fn(this: *const IMTConKYC) -> LPCWSTR,
    pub IMTConKYC_Flags1: unsafe extern "C" fn(this: *mut IMTConKYC, flags: UINT64) -> MTAPIRES,
    pub IMTConKYC_Flags: unsafe extern "C" fn(this: *const IMTConKYC) -> UINT64,
    pub IMTConKYC_CountryAdd:
        unsafe extern "C" fn(this: *mut IMTConKYC, country: *mut IMTConKYCCountry) -> MTAPIRES,
    pub IMTConKYC_CountryUpdate: unsafe extern "C" fn(
        this: *mut IMTConKYC,
        pos: UINT,
        country: *const IMTConKYCCountry,
    ) -> MTAPIRES,
    pub IMTConKYC_CountryDelete: unsafe extern "C" fn(this: *mut IMTConKYC, pos: UINT) -> MTAPIRES,
    pub IMTConKYC_CountryClear: unsafe extern "C" fn(this: *mut IMTConKYC) -> MTAPIRES,
    pub IMTConKYC_CountryShift: unsafe extern "C" fn(
        this: *mut IMTConKYC,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConKYC_CountryTotal: unsafe extern "C" fn(this: *const IMTConKYC) -> UINT,
    pub IMTConKYC_CountryNext: unsafe extern "C" fn(
        this: *const IMTConKYC,
        pos: UINT,
        country: *mut IMTConKYCCountry,
    ) -> MTAPIRES,
    pub IMTConKYC_GroupAdd:
        unsafe extern "C" fn(this: *mut IMTConKYC, group: *mut IMTConKYCGroup) -> MTAPIRES,
    pub IMTConKYC_GroupUpdate: unsafe extern "C" fn(
        this: *mut IMTConKYC,
        pos: UINT,
        group: *const IMTConKYCGroup,
    ) -> MTAPIRES,
    pub IMTConKYC_GroupDelete: unsafe extern "C" fn(this: *mut IMTConKYC, pos: UINT) -> MTAPIRES,
    pub IMTConKYC_GroupClear: unsafe extern "C" fn(this: *mut IMTConKYC) -> MTAPIRES,
    pub IMTConKYC_GroupShift: unsafe extern "C" fn(
        this: *mut IMTConKYC,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTConKYC_GroupTotal: unsafe extern "C" fn(this: *const IMTConKYC) -> UINT,
    pub IMTConKYC_GroupNext: unsafe extern "C" fn(
        this: *const IMTConKYC,
        pos: UINT,
        group: *mut IMTConKYCGroup,
    ) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug)]
pub struct IMTConKYC {
    pub vtable_: *const IMTConKYC__bindgen_vtable,
}
pub const IMTConKYC_EnFlags_FLAG_NONE: IMTConKYC_EnFlags = 0;
pub const IMTConKYC_EnFlags_FLAG_ENABLED: IMTConKYC_EnFlags = 1;
pub const IMTConKYC_EnFlags_FLAG_DEFAULT: IMTConKYC_EnFlags = 2;
pub const IMTConKYC_EnFlags_FLAG_FIRST: IMTConKYC_EnFlags = 1;
pub const IMTConKYC_EnFlags_FLAG_ALL: IMTConKYC_EnFlags = 3;
pub type IMTConKYC_EnFlags = ::std::os::raw::c_int;
pub const IMTConKYC_EnProviderType_PROVIDER_KYC_SUMSUB: IMTConKYC_EnProviderType = 0;
pub const IMTConKYC_EnProviderType_PROVIDER_KYC_WORLD_CHECK: IMTConKYC_EnProviderType = 1;
pub const IMTConKYC_EnProviderType_PROVIDER_KYC_ESPEAR: IMTConKYC_EnProviderType = 2;
pub const IMTConKYC_EnProviderType_PROVIDER_KYC_FIRST: IMTConKYC_EnProviderType = 0;
pub const IMTConKYC_EnProviderType_PROVIDER_KYC_LAST: IMTConKYC_EnProviderType = 2;
pub type IMTConKYC_EnProviderType = ::std::os::raw::c_int;
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConKYC"][::std::mem::size_of::<IMTConKYC>() - 8usize];
    ["Alignment of IMTConKYC"][::std::mem::align_of::<IMTConKYC>() - 8usize];
};
#[repr(C)]
pub struct IMTConKYCSink__bindgen_vtable {}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTConKYCSink {
    pub vtable_: *const IMTConKYCSink__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTConKYCSink"][::std::mem::size_of::<IMTConKYCSink>() - 8usize];
    ["Alignment of IMTConKYCSink"][::std::mem::align_of::<IMTConKYCSink>() - 8usize];
};
#[repr(C)]
pub struct IMTServerAPI__bindgen_vtable {
    pub IMTServerAPI_About:
        unsafe extern "C" fn(this: *mut IMTServerAPI, info: *mut MTServerInfo) -> MTAPIRES,
    pub IMTServerAPI_LicenseCheck:
        unsafe extern "C" fn(this: *mut IMTServerAPI, license_name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_Allocate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, bytes: UINT) -> *mut ::std::os::raw::c_void,
    pub IMTServerAPI_Free:
        unsafe extern "C" fn(this: *mut IMTServerAPI, ptr: *mut ::std::os::raw::c_void),
    pub IMTServerAPI_ServerRestart: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ServerSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTServerSink) -> MTAPIRES,
    pub IMTServerAPI_ServerUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTServerSink) -> MTAPIRES,
    pub IMTServerAPI_ServerRestartRemote:
        unsafe extern "C" fn(this: *mut IMTServerAPI, id: UINT64, reason: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_ServerReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_LoggerOut:
        unsafe extern "C" fn(this: *mut IMTServerAPI, code: UINT, msg: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_LoggerRequest: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        mode: UINT,
        type_: UINT,
        from: INT64,
        to: INT64,
        filter: LPCWSTR,
        records: *mut *mut MTLogRecord,
        records_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_LoggerFlush: unsafe extern "C" fn(this: *mut IMTServerAPI),
    pub IMTServerAPI_LoggerReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_LoggerReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_LoggerReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_LoggerReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_PluginCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConPlugin,
    pub IMTServerAPI_PluginModuleCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConPluginModule,
    pub IMTServerAPI_PluginParamCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConParam,
    pub IMTServerAPI_PluginSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConPluginSink) -> MTAPIRES,
    pub IMTServerAPI_PluginUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConPluginSink) -> MTAPIRES,
    pub IMTServerAPI_PluginCurrent:
        unsafe extern "C" fn(this: *mut IMTServerAPI, plugin: *mut IMTConPlugin) -> MTAPIRES,
    pub IMTServerAPI_PluginAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, plugin: *mut IMTConPlugin) -> MTAPIRES,
    pub IMTServerAPI_PluginDelete2:
        unsafe extern "C" fn(this: *mut IMTServerAPI, server: UINT64, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_PluginDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_PluginDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_PluginShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_PluginNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        plugin: *mut IMTConPlugin,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        server: UINT64,
        name: LPCWSTR,
        plugin: *mut IMTConPlugin,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        plugin: *mut IMTConPlugin,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginModuleTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_PluginModuleNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        module: *mut IMTConPluginModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginModuleGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        server: UINT64,
        name: LPCWSTR,
        module: *mut IMTConPluginModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginModuleGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        module: *mut IMTConPluginModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_PluginReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommonCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConCommon,
    pub IMTServerAPI_CommonSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConCommonSink) -> MTAPIRES,
    pub IMTServerAPI_CommonUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConCommonSink) -> MTAPIRES,
    pub IMTServerAPI_CommonGet:
        unsafe extern "C" fn(this: *mut IMTServerAPI, common: *mut IMTConCommon) -> MTAPIRES,
    pub IMTServerAPI_CommonSet:
        unsafe extern "C" fn(this: *mut IMTServerAPI, common: *const IMTConCommon) -> MTAPIRES,
    pub IMTServerAPI_CommonReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommonReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommonReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommonReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NetServerCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConServer,
    pub IMTServerAPI_NetServerRangeCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConServerRange,
    pub IMTServerAPI_NetServerSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConServerSink) -> MTAPIRES,
    pub IMTServerAPI_NetServerUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConServerSink) -> MTAPIRES,
    pub IMTServerAPI_NetServerAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConServer) -> MTAPIRES,
    pub IMTServerAPI_NetServerDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_NetServerShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_NetServerTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_NetServerNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        config: *mut IMTConServer,
    ) -> MTAPIRES,
    pub IMTServerAPI_NetServerGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        id: UINT64,
        config: *mut IMTConServer,
    ) -> MTAPIRES,
    pub IMTServerAPI_NetServerAddressRangeCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConAddressRange,
    pub IMTServerAPI_NetServerClusterStateCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConClusterState,
    pub IMTServerAPI_NetServerReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NetServerReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TimeCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConTime,
    pub IMTServerAPI_TimeSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConTimeSink) -> MTAPIRES,
    pub IMTServerAPI_TimeUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConTimeSink) -> MTAPIRES,
    pub IMTServerAPI_TimeCurrent: unsafe extern "C" fn(this: *mut IMTServerAPI) -> INT64,
    pub IMTServerAPI_TimeGet:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConTime) -> MTAPIRES,
    pub IMTServerAPI_TimeSet:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *const IMTConTime) -> MTAPIRES,
    pub IMTServerAPI_TimeCurrentMsc: unsafe extern "C" fn(this: *mut IMTServerAPI) -> INT64,
    pub IMTServerAPI_TimeReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TimeReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TimeReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HolidayCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConHoliday,
    pub IMTServerAPI_HolidaySubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConHolidaySink) -> MTAPIRES,
    pub IMTServerAPI_HolidayUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConHolidaySink) -> MTAPIRES,
    pub IMTServerAPI_HolidayAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConHoliday) -> MTAPIRES,
    pub IMTServerAPI_HolidayDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_HolidayShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_HolidayTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_HolidayNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        config: *mut IMTConHoliday,
    ) -> MTAPIRES,
    pub IMTServerAPI_HolidayReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HolidayReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HolidayReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HolidayReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FirewallCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConFirewall,
    pub IMTServerAPI_FirewallSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConFirewallSink) -> MTAPIRES,
    pub IMTServerAPI_FirewallUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConFirewallSink) -> MTAPIRES,
    pub IMTServerAPI_FirewallAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConFirewall) -> MTAPIRES,
    pub IMTServerAPI_FirewallDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_FirewallShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_FirewallTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_FirewallNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        config: *mut IMTConFirewall,
    ) -> MTAPIRES,
    pub IMTServerAPI_FirewallReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FirewallReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FirewallReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FirewallReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SymbolCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSymbol,
    pub IMTServerAPI_SymbolSessionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSymbolSession,
    pub IMTServerAPI_SymbolSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConSymbolSink) -> MTAPIRES,
    pub IMTServerAPI_SymbolUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConSymbolSink) -> MTAPIRES,
    pub IMTServerAPI_SymbolAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, symbol: *mut IMTConSymbol) -> MTAPIRES,
    pub IMTServerAPI_SymbolDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_SymbolDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_SymbolShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_SymbolTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_SymbolNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        symbol: *mut IMTConSymbol,
    ) -> MTAPIRES,
    pub IMTServerAPI_SymbolGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        group: *const IMTConGroup,
        symbol: *mut IMTConSymbol,
    ) -> MTAPIRES,
    pub IMTServerAPI_SymbolGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        symbol: *mut IMTConSymbol,
    ) -> MTAPIRES,
    pub IMTServerAPI_SymbolExist: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: *const IMTConSymbol,
        group: *const IMTConGroup,
    ) -> MTAPIRES,
    pub IMTServerAPI_SymbolReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SymbolReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SymbolReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SymbolReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GroupCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConGroup,
    pub IMTServerAPI_GroupSymbolCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConGroupSymbol,
    pub IMTServerAPI_GroupCommissionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConCommission,
    pub IMTServerAPI_GroupTierCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConCommTier,
    pub IMTServerAPI_GroupSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConGroupSink) -> MTAPIRES,
    pub IMTServerAPI_GroupUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConGroupSink) -> MTAPIRES,
    pub IMTServerAPI_GroupAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, group: *mut IMTConGroup) -> MTAPIRES,
    pub IMTServerAPI_GroupDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_GroupDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_GroupShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_GroupTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_GroupNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        group: *mut IMTConGroup,
    ) -> MTAPIRES,
    pub IMTServerAPI_GroupGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        group: *mut IMTConGroup,
    ) -> MTAPIRES,
    pub IMTServerAPI_GroupReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GroupReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GroupReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GroupReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ManagerCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConManager,
    pub IMTServerAPI_ManagerAccessCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConManagerAccess,
    pub IMTServerAPI_ManagerSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConManagerSink) -> MTAPIRES,
    pub IMTServerAPI_ManagerUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConManagerSink) -> MTAPIRES,
    pub IMTServerAPI_ManagerAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, manager: *mut IMTConManager) -> MTAPIRES,
    pub IMTServerAPI_ManagerDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_ManagerShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_ManagerTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_ManagerNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        manager: *mut IMTConManager,
    ) -> MTAPIRES,
    pub IMTServerAPI_ManagerGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        manager: *mut IMTConManager,
    ) -> MTAPIRES,
    pub IMTServerAPI_ManagerReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ManagerReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ManagerReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ManagerReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConHistorySync,
    pub IMTServerAPI_HistorySyncSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConHistorySyncSink) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConHistorySyncSink) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConHistorySync) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_HistorySyncNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        config: *mut IMTConHistorySync,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncReserved1:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncReserved2:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncReserved3:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistorySyncReserved4:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FeederCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConFeeder,
    pub IMTServerAPI_FeederModuleCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConFeederModule,
    pub IMTServerAPI_FeederParamCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConParam,
    pub IMTServerAPI_FeederTranslateCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConFeederTranslate,
    pub IMTServerAPI_FeederSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConFeederSink) -> MTAPIRES,
    pub IMTServerAPI_FeederUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConFeederSink) -> MTAPIRES,
    pub IMTServerAPI_FeederAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, feeder: *mut IMTConFeeder) -> MTAPIRES,
    pub IMTServerAPI_FeederDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_FeederDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_FeederShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_FeederTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_FeederNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        feeder: *mut IMTConFeeder,
    ) -> MTAPIRES,
    pub IMTServerAPI_FeederGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        feeder: *mut IMTConFeeder,
    ) -> MTAPIRES,
    pub IMTServerAPI_FeederModuleTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_FeederModuleNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        module: *mut IMTConFeederModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_FeederModuleGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        module: *mut IMTConFeederModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_FeederRestart:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_FeederReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FeederReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_FeederReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GatewayCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConGateway,
    pub IMTServerAPI_GatewayModuleCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConGatewayModule,
    pub IMTServerAPI_GatewayParamCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConParam,
    pub IMTServerAPI_GatewayTranslateCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConGatewayTranslate,
    pub IMTServerAPI_GatewaySubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConGatewaySink) -> MTAPIRES,
    pub IMTServerAPI_GatewayUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConGatewaySink) -> MTAPIRES,
    pub IMTServerAPI_GatewayAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, gateway: *mut IMTConGateway) -> MTAPIRES,
    pub IMTServerAPI_GatewayDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_GatewayDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_GatewayShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_GatewayTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_GatewayNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        gateway: *mut IMTConGateway,
    ) -> MTAPIRES,
    pub IMTServerAPI_GatewayGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        gateway: *mut IMTConGateway,
    ) -> MTAPIRES,
    pub IMTServerAPI_GatewayModuleTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_GatewayModuleNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        module: *mut IMTConGatewayModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_GatewayModuleGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        module: *mut IMTConGatewayModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_GatewayRestart:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_GatewayReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GatewayReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GatewayReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ReportCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConReport,
    pub IMTServerAPI_ReportModuleCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConReportModule,
    pub IMTServerAPI_ReportParamCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConParam,
    pub IMTServerAPI_ReportSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConReportSink) -> MTAPIRES,
    pub IMTServerAPI_ReportUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConReportSink) -> MTAPIRES,
    pub IMTServerAPI_ReportAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, report: *mut IMTConReport) -> MTAPIRES,
    pub IMTServerAPI_ReportDelete2:
        unsafe extern "C" fn(this: *mut IMTServerAPI, server: UINT64, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_ReportDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_ReportDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_ReportShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_ReportNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        report: *mut IMTConReport,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        server: UINT64,
        name: LPCWSTR,
        report: *mut IMTConReport,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        report: *mut IMTConReport,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportModuleTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_ReportModuleNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        module: *mut IMTConReportModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportModuleGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        server: UINT64,
        name: LPCWSTR,
        module: *mut IMTConReportModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportModuleGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        module: *mut IMTConReportModule,
    ) -> MTAPIRES,
    pub IMTServerAPI_ReportReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_RouteCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConRoute,
    pub IMTServerAPI_RouteConditionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConCondition,
    pub IMTServerAPI_RouteDealerCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConRouteDealer,
    pub IMTServerAPI_RouteSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConRouteSink) -> MTAPIRES,
    pub IMTServerAPI_RouteUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConRouteSink) -> MTAPIRES,
    pub IMTServerAPI_RouteAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, route: *mut IMTConRoute) -> MTAPIRES,
    pub IMTServerAPI_RouteDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_RouteDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_RouteShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_RouteTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_RouteNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        route: *mut IMTConRoute,
    ) -> MTAPIRES,
    pub IMTServerAPI_RouteGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        route: *mut IMTConRoute,
    ) -> MTAPIRES,
    pub IMTServerAPI_RouteReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_RouteReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_RouteReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_RouteReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_UserCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTUser,
    pub IMTServerAPI_UserCreateAccount:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTAccount,
    pub IMTServerAPI_UserSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTUserSink) -> MTAPIRES,
    pub IMTServerAPI_UserUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTUserSink) -> MTAPIRES,
    pub IMTServerAPI_UserAdd: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        user: *mut IMTUser,
        master_pass: LPCWSTR,
        investor_pass: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES,
    pub IMTServerAPI_UserUpdate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, user: *mut IMTUser) -> MTAPIRES,
    pub IMTServerAPI_UserTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_UserGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        user: *mut IMTUser,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        group: *mut MTAPISTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        logins: *mut *mut UINT64,
        logins_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserPasswordCheck: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        type_: UINT,
        login: UINT64,
        password: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserPasswordChange: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        type_: UINT,
        login: UINT64,
        password: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserCertDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES,
    pub IMTServerAPI_UserCertConfirm:
        unsafe extern "C" fn(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES,
    pub IMTServerAPI_UserDepositChangeRaw: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        value: f64,
        type_: UINT,
        comment: LPCWSTR,
        deal_id: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserDepositChange: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        value: f64,
        action: UINT,
        comment: LPCWSTR,
        deal_id: *mut UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserAccountGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        account: *mut IMTAccount,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserArchive:
        unsafe extern "C" fn(this: *mut IMTServerAPI, login: UINT64) -> MTAPIRES,
    pub IMTServerAPI_UserArchiveGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        user: *mut IMTUser,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserRestore:
        unsafe extern "C" fn(this: *mut IMTServerAPI, user: *mut IMTUser) -> MTAPIRES,
    pub IMTServerAPI_UserArchiveLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        logins: *mut *mut UINT64,
        logins_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDeal,
    pub IMTServerAPI_DealCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDealArray,
    pub IMTServerAPI_DealSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTDealSink) -> MTAPIRES,
    pub IMTServerAPI_DealUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTDealSink) -> MTAPIRES,
    pub IMTServerAPI_DealDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES,
    pub IMTServerAPI_DealUpdate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, deal: *mut IMTDeal) -> MTAPIRES,
    pub IMTServerAPI_DealGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ticket: UINT64,
        deal: *mut IMTDeal,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        from: INT64,
        to: INT64,
        login: UINT64,
        deals: *mut IMTDealArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, deal: *mut IMTDeal) -> MTAPIRES,
    pub IMTServerAPI_DealPerform:
        unsafe extern "C" fn(this: *mut IMTServerAPI, deal: *mut IMTDeal) -> MTAPIRES,
    pub IMTServerAPI_DealPerformCloseBy: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deal: *mut IMTDeal,
        dealby: *mut IMTDeal,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealDeleteBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTPosition,
    pub IMTServerAPI_PositionCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTPositionArray,
    pub IMTServerAPI_PositionSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTPositionSink) -> MTAPIRES,
    pub IMTServerAPI_PositionUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTPositionSink) -> MTAPIRES,
    pub IMTServerAPI_PositionDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, login: UINT64, symbol: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_PositionUpdate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, position: *mut IMTPosition) -> MTAPIRES,
    pub IMTServerAPI_PositionGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        symbol: LPCWSTR,
        position: *mut IMTPosition,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        position: *mut IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionDeleteByTicket:
        unsafe extern "C" fn(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES,
    pub IMTServerAPI_PositionGetByTicket: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ticket: UINT64,
        position: *mut IMTPosition,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionCheck: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        current: *mut IMTPositionArray,
        invalid: *mut IMTPositionArray,
        missed: *mut IMTPositionArray,
        nonexist: *mut IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionFix: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        current: *mut IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTOrder,
    pub IMTServerAPI_OrderCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTOrderArray,
    pub IMTServerAPI_OrderSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTOrderSink) -> MTAPIRES,
    pub IMTServerAPI_OrderUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTOrderSink) -> MTAPIRES,
    pub IMTServerAPI_OrderDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES,
    pub IMTServerAPI_OrderUpdate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTServerAPI_OrderGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ticket: UINT64,
        order: *mut IMTOrder,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTServerAPI_OrderDeleteBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderUpdateBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut IMTOrderArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderUpdateBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut *mut IMTOrder,
        orders_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistorySubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTHistorySink) -> MTAPIRES,
    pub IMTServerAPI_HistoryUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTHistorySink) -> MTAPIRES,
    pub IMTServerAPI_HistoryDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES,
    pub IMTServerAPI_HistoryUpdate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTServerAPI_HistoryGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        from: INT64,
        to: INT64,
        login: UINT64,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ticket: UINT64,
        order: *mut IMTOrder,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryReopen:
        unsafe extern "C" fn(this: *mut IMTServerAPI, ticket: UINT64) -> MTAPIRES,
    pub IMTServerAPI_HistoryAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, order: *mut IMTOrder) -> MTAPIRES,
    pub IMTServerAPI_HistoryDeleteBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryUpdateBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut IMTOrderArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DailyCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDaily,
    pub IMTServerAPI_DailyCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDailyArray,
    pub IMTServerAPI_DailySubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTDailySink) -> MTAPIRES,
    pub IMTServerAPI_DailyUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTDailySink) -> MTAPIRES,
    pub IMTServerAPI_DailyGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        from: INT64,
        to: INT64,
        login: UINT64,
        daily: *mut IMTDailyArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DailyGetLight: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        from: INT64,
        to: INT64,
        login: UINT64,
        daily: *mut IMTDailyArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DailySelectByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_DailySelectByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_DailyReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TickSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTTickSink) -> MTAPIRES,
    pub IMTServerAPI_TickUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTTickSink) -> MTAPIRES,
    pub IMTServerAPI_TickAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, tick: *mut MTTick) -> MTAPIRES,
    pub IMTServerAPI_TickAddStat: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tick: *mut MTTick,
        stat: *mut MTTickStat,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickLast1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: *const IMTConSymbol,
        tick: *mut MTTickShort,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickLast: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        tick: *mut MTTickShort,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickStat: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        stat: *mut MTTickStat,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickGet1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: *const IMTConSymbol,
        from: INT64,
        to: INT64,
        ticks: *mut *mut MTTickShort,
        ticks_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        from: INT64,
        to: INT64,
        ticks: *mut *mut MTTickShort,
        ticks_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickHistoryGetRaw: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        from: INT64,
        to: INT64,
        ticks: *mut *mut MTTickShort,
        ticks_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickHistoryGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        from: INT64,
        to: INT64,
        ticks: *mut *mut MTTickShort,
        ticks_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickAddBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ticks: *mut MTTick,
        ticks_total: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_TickReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TickReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_MailCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTMail,
    pub IMTServerAPI_MailSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTMailSink) -> MTAPIRES,
    pub IMTServerAPI_MailUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTMailSink) -> MTAPIRES,
    pub IMTServerAPI_MailSend:
        unsafe extern "C" fn(this: *mut IMTServerAPI, mail: *mut IMTMail) -> MTAPIRES,
    pub IMTServerAPI_MailReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_MailReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_MailReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_MailReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NewsCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTNews,
    pub IMTServerAPI_NewsSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTNewsSink) -> MTAPIRES,
    pub IMTServerAPI_NewsUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTNewsSink) -> MTAPIRES,
    pub IMTServerAPI_NewsSend:
        unsafe extern "C" fn(this: *mut IMTServerAPI, news: *mut IMTNews) -> MTAPIRES,
    pub IMTServerAPI_NewsReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NewsReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NewsReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NewsReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CustomSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTCustomSink) -> MTAPIRES,
    pub IMTServerAPI_CustomUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTCustomSink) -> MTAPIRES,
    pub IMTServerAPI_CustomCreateStream:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTByteStream,
    pub IMTServerAPI_CustomReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CustomReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CustomReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeRequestCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTRequest,
    pub IMTServerAPI_TradeSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTTradeSink) -> MTAPIRES,
    pub IMTServerAPI_TradeUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTTradeSink) -> MTAPIRES,
    pub IMTServerAPI_TradeRequest:
        unsafe extern "C" fn(this: *mut IMTServerAPI, request: *mut IMTRequest) -> MTAPIRES,
    pub IMTServerAPI_TradeProfit: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        symbol: LPCWSTR,
        type_: UINT,
        volume: UINT64,
        price_open: f64,
        price_close: f64,
        profit: *mut f64,
        profit_rate: *mut f64,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeRateBuy: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        base: LPCWSTR,
        currency: LPCWSTR,
        rate: *mut f64,
        group: LPCWSTR,
        symbol: LPCWSTR,
        price: f64,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeRateSell: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        base: LPCWSTR,
        currency: LPCWSTR,
        rate: *mut f64,
        group: LPCWSTR,
        symbol: LPCWSTR,
        price: f64,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeMarginCheck1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        order: *const IMTOrder,
        account_new: *mut IMTAccount,
        account_current: *mut IMTAccount,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeMarginCheck: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        symbol: LPCWSTR,
        type_: UINT,
        volume: UINT64,
        price: f64,
        account_new: *mut IMTAccount,
        account_current: *mut IMTAccount,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeBalanceCheckObsolete: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        fixflag: UINT,
        balance_user: *mut f64,
        balance_history: *mut f64,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeSubscribeEOD:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTEndOfDaySink) -> MTAPIRES,
    pub IMTServerAPI_TradeUnsubscribeEOD:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTEndOfDaySink) -> MTAPIRES,
    pub IMTServerAPI_TradeBalanceCheck: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        fixflag: UINT,
        balance_user: *mut f64,
        balance_history: *mut f64,
        credit_user: *mut f64,
        credit_history: *mut f64,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeAccountSet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        user: *const IMTUser,
        account: *const IMTAccount,
        orders: *const IMTOrderArray,
        positions: *const IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeConfirmCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConfirm,
    pub IMTServerAPI_TradeExecutionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTExecution,
    pub IMTServerAPI_TradeRequestCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTRequestArray,
    pub IMTServerAPI_TradeProfitExt: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        symbol: LPCWSTR,
        type_: UINT,
        volume: UINT64,
        price_open: f64,
        price_close: f64,
        profit: *mut f64,
        profit_rate: *mut f64,
    ) -> MTAPIRES,
    pub IMTServerAPI_BookSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTBookSink) -> MTAPIRES,
    pub IMTServerAPI_BookUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTBookSink) -> MTAPIRES,
    pub IMTServerAPI_BookGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        book: *mut MTBook,
    ) -> MTAPIRES,
    pub IMTServerAPI_BookReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_BookReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_BookReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_BookReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ChartGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        from: INT64,
        to: INT64,
        bars: *mut *mut MTChartBar,
        bars_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ChartDelete: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        bars_dates: *const INT64,
        bars_dates_total: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ChartUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        bars: *const MTChartBar,
        bars_total: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ChartSplit: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        symbol: LPCWSTR,
        new_shares: UINT,
        old_shares: UINT,
        rounding_rule: UINT,
        datetime_from: INT64,
        datetime_to: INT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_ChartReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ChartReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ChartReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_UserCertCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTCertificate,
    pub IMTServerAPI_UserCertUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        certificate: *const IMTCertificate,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserCertGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        certificate: *mut IMTCertificate,
    ) -> MTAPIRES,
    pub IMTServerAPI_UserCertReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_UserCertReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_UserCertReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_UserCertReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SpreadCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSpread,
    pub IMTServerAPI_SpreadLegCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSpreadLeg,
    pub IMTServerAPI_SpreadSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConSpreadSink) -> MTAPIRES,
    pub IMTServerAPI_SpreadUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConSpreadSink) -> MTAPIRES,
    pub IMTServerAPI_SpreadAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, spread: *mut IMTConSpread) -> MTAPIRES,
    pub IMTServerAPI_SpreadDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_SpreadShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_SpreadTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_SpreadNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        spread: *mut IMTConSpread,
    ) -> MTAPIRES,
    pub IMTServerAPI_SpreadGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        id: UINT,
        spread: *mut IMTConSpread,
    ) -> MTAPIRES,
    pub IMTServerAPI_SpreadReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SpreadReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SpreadReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SpreadReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_OnlineCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTOnline,
    pub IMTServerAPI_OnlineCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTOnlineArray,
    pub IMTServerAPI_OnlineTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_OnlineNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        online: *mut IMTOnline,
    ) -> MTAPIRES,
    pub IMTServerAPI_OnlineGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        online: *mut IMTOnlineArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_OnlineDisconnect:
        unsafe extern "C" fn(this: *mut IMTServerAPI, online: *mut IMTOnline) -> MTAPIRES,
    pub IMTServerAPI_OnlineDisconnectBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        online: *mut IMTOnlineArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_OnlineDisconnectBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        online: *mut *mut IMTOnline,
        online_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_OnlineReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NotificationsSend1: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        message: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_NotificationsSend: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        metaquotes_ids: LPCWSTR,
        message: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_NotificationsReserved1:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NotificationsReserved2:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NotificationsReserved3:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_NotificationsReserved4:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealUpdateBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deals: *mut IMTDealArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealUpdateBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deals: *mut *mut IMTDeal,
        deals_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealAddBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deals: *mut IMTDealArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealAddBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deals: *mut *mut IMTDeal,
        deals_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealPerformBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deals: *mut IMTDealArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealPerformBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        deals: *mut *mut IMTDeal,
        deals_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealSelectByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealSelectByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealGetByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        from: INT64,
        to: INT64,
        deals: *mut IMTDealArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealGetByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        from: INT64,
        to: INT64,
        deals: *mut IMTDealArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderAddBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut IMTOrderArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderAddBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut *mut IMTOrder,
        orders_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderSelectByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderSelectByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderGetByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderGetByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryUpdateBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut *mut IMTOrder,
        orders_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryAddBatch: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut IMTOrderArray,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryAddBatchArray: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        orders: *mut *mut IMTOrder,
        orders_total: UINT,
        results: *mut MTAPIRES,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistorySelectByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistorySelectByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryGetByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        from: INT64,
        to: INT64,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryGetByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        from: INT64,
        to: INT64,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerStart: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        sink: *mut IMTRequestSink,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerStop: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        sink: *mut IMTRequestSink,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        request: *mut IMTRequest,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerLock: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        id: UINT,
        request: *mut IMTRequest,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerAnswer: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        confirm: *mut IMTConfirm,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerRequestTotal:
        unsafe extern "C" fn(this: *mut IMTServerAPI, dealer: UINT64) -> UINT,
    pub IMTServerAPI_DealerRequestNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        pos: UINT,
        request: *mut IMTRequest,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerRequestGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        id: UINT,
        request: *mut IMTRequest,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerRequestGetAll: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        dealer: UINT64,
        requests: *mut IMTRequestArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerExecution: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        gateway_name: LPCWSTR,
        gateway_type: LPCWSTR,
        execution: *mut IMTExecution,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealerReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealerReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealerReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeMarginCheckExt: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        symbol: LPCWSTR,
        type_: UINT,
        volume: UINT64,
        price: f64,
        account_new: *mut IMTAccount,
        account_current: *mut IMTAccount,
    ) -> MTAPIRES,
    pub IMTServerAPI_TradeReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeReserved5: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TradeReserved6: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_EmailCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConEmail,
    pub IMTServerAPI_EmailSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConEmailSink) -> MTAPIRES,
    pub IMTServerAPI_EmailUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConEmailSink) -> MTAPIRES,
    pub IMTServerAPI_EmailAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConEmail) -> MTAPIRES,
    pub IMTServerAPI_EmailDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_EmailDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_EmailShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_EmailTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_EmailNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        email: *mut IMTConEmail,
    ) -> MTAPIRES,
    pub IMTServerAPI_EmailGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        email: *mut IMTConEmail,
    ) -> MTAPIRES,
    pub IMTServerAPI_EmailSend: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        account: LPCWSTR,
        to: LPCWSTR,
        to_name: LPCWSTR,
        subject: LPCWSTR,
        body: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_EmailReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_EmailReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_EmailReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_MessengerCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConMessenger,
    pub IMTServerAPI_MessengerSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConMessengerSink) -> MTAPIRES,
    pub IMTServerAPI_MessengerUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConMessengerSink) -> MTAPIRES,
    pub IMTServerAPI_MessengerAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConMessenger) -> MTAPIRES,
    pub IMTServerAPI_MessengerDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_MessengerDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_MessengerShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_MessengerTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_MessengerNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        messenger: *mut IMTConMessenger,
    ) -> MTAPIRES,
    pub IMTServerAPI_MessengerGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        messenger: *mut IMTConMessenger,
    ) -> MTAPIRES,
    pub IMTServerAPI_MessengerSend: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        destination: LPCWSTR,
        group: LPCWSTR,
        sender: LPCWSTR,
        text: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_MessengerVerifyPhone:
        unsafe extern "C" fn(this: *mut IMTServerAPI, phone_number: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_MessengerCountryCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConMessengerCountry,
    pub IMTServerAPI_MessengerGroupCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConMessengerGroup,
    pub IMTServerAPI_PositionSelectByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionSelectByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        from: INT64,
        to: INT64,
        request: *const IMTDatasetRequest,
        dataset: *mut IMTDataset,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionGetByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        group: LPCWSTR,
        positions: *mut IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionGetByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        positions: *mut IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionGetByTickets: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        positions: *mut IMTPositionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionSplit: unsafe extern "C" fn(
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
    ) -> MTAPIRES,
    pub IMTServerAPI_PositionReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_PositionReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_PositionReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DatasetRequestCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDatasetRequest,
    pub IMTServerAPI_DatasetCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDataset,
    pub IMTServerAPI_DatasetReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DatasetReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DatasetReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DatasetReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_OrderGetByTickets: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_OrderReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_OrderReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_OrderReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_OrderReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistoryGetByTickets: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        orders: *mut IMTOrderArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_HistoryReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistoryReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistoryReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_HistoryReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealGetByTickets: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        tickets: *const UINT64,
        tickets_total: UINT,
        deals: *mut IMTDealArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DealReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DealReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ClientCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTClient,
    pub IMTServerAPI_ClientCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTClientArray,
    pub IMTServerAPI_ClientSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTClientSink) -> MTAPIRES,
    pub IMTServerAPI_ClientUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTClientSink) -> MTAPIRES,
    pub IMTServerAPI_ClientAdd: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client: *mut IMTClient,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client: *mut IMTClient,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientDelete: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client_id: UINT64,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client_id: UINT64,
        client: *mut IMTClient,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientGetHistory: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client_id: UINT64,
        author: UINT64,
        from: INT64,
        to: INT64,
        history: *mut IMTClientArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientIdsAll: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ids: *mut *mut UINT64,
        ids_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientIdsByGroup: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        groups: LPCWSTR,
        ids: *mut *mut UINT64,
        ids_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientIdsByManager: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        manager: UINT64,
        ids: *mut *mut UINT64,
        ids_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientUserAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, client_id: UINT64, login: UINT64) -> MTAPIRES,
    pub IMTServerAPI_ClientUserDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, client_id: UINT64, login: UINT64) -> MTAPIRES,
    pub IMTServerAPI_ClientUserLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client_id: UINT64,
        logins: *mut *mut UINT64,
        logins_total: *mut UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_ClientReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ClientReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ClientReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_ClientReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DocumentCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDocument,
    pub IMTServerAPI_DocumentCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTDocumentArray,
    pub IMTServerAPI_DocumentSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTDocumentSink) -> MTAPIRES,
    pub IMTServerAPI_DocumentUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTDocumentSink) -> MTAPIRES,
    pub IMTServerAPI_DocumentAdd: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        document: *mut IMTDocument,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_DocumentUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        document: *mut IMTDocument,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_DocumentDelete: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        document_id: UINT64,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_DocumentGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        document_id: UINT64,
        document: *mut IMTDocument,
    ) -> MTAPIRES,
    pub IMTServerAPI_DocumentGetByClient: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client_id: UINT64,
        position: UINT,
        total: UINT,
        documents: *mut IMTDocumentArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DocumentGetHistory: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        document_id: UINT64,
        author: UINT64,
        from: INT64,
        to: INT64,
        history: *mut IMTDocumentArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_DocumentReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DocumentReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DocumentReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_DocumentReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommentCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTComment,
    pub IMTServerAPI_CommentCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTCommentArray,
    pub IMTServerAPI_CommentSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTCommentSink) -> MTAPIRES,
    pub IMTServerAPI_CommentUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTCommentSink) -> MTAPIRES,
    pub IMTServerAPI_CommentAdd: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        comment: *mut IMTComment,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_CommentUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        comment: *mut IMTComment,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_CommentDelete: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        comment_id: UINT64,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_CommentGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        comment_id: UINT64,
        comment: *mut IMTComment,
    ) -> MTAPIRES,
    pub IMTServerAPI_CommentGetByClient: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        client_id: UINT64,
        position: UINT,
        total: UINT,
        comments: *mut IMTCommentArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_CommentGetByDocument: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        document_id: UINT64,
        position: UINT,
        total: UINT,
        comments: *mut IMTCommentArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_CommentReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommentReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommentReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_CommentReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AttachmentCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTAttachment,
    pub IMTServerAPI_AttachmentAdd: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        attachment: *mut IMTAttachment,
        author: UINT64,
    ) -> MTAPIRES,
    pub IMTServerAPI_AttachmentGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        attachment_id: UINT64,
        attachment: *mut IMTAttachment,
    ) -> MTAPIRES,
    pub IMTServerAPI_AttachmentReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AttachmentReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AttachmentReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AttachmentReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pfx_certificate: *const ::std::os::raw::c_void,
        pfx_certificate_size: UINT,
        password: LPCWSTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_TLSCertificateNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        name: *mut MTAPISTR,
        thumbprint: *mut MTAPISTR,
    ) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificatePfx: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        pfx_certificate_size: *mut UINT,
    ) -> *mut ::std::os::raw::c_void,
    pub IMTServerAPI_TLSCertificateReserved1:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateReserved2:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateReserved3:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_TLSCertificateReserved4:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AutomationCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConAutomation,
    pub IMTServerAPI_AutomationConditionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConAutoCondition,
    pub IMTServerAPI_AutomationActionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConAutoAction,
    pub IMTServerAPI_AutomationParamCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConAutoParam,
    pub IMTServerAPI_AutomationSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConAutomationSink) -> MTAPIRES,
    pub IMTServerAPI_AutomationUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConAutomationSink) -> MTAPIRES,
    pub IMTServerAPI_AutomationAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConAutomation) -> MTAPIRES,
    pub IMTServerAPI_AutomationDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_AutomationDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_AutomationShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_AutomationTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_AutomationNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        config: *mut IMTConAutomation,
    ) -> MTAPIRES,
    pub IMTServerAPI_AutomationGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        config: *mut IMTConAutomation,
    ) -> MTAPIRES,
    pub IMTServerAPI_AutomationTrigger: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        user: *const IMTUser,
        account: *const IMTAccount,
        deal: *const IMTDeal,
        order: *const IMTOrder,
        position: *const IMTPosition,
    ) -> MTAPIRES,
    pub IMTServerAPI_AutomationReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AutomationReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_AutomationReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSubscription,
    pub IMTServerAPI_SubscriptionCfgSymbolCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSubscriptionSymbol,
    pub IMTServerAPI_SubscriptionCfgNewsCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConSubscriptionNews,
    pub IMTServerAPI_SubscriptionCfgSubscribe: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        sink: *mut IMTConSubscriptionSink,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgUnsubscribe: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        sink: *mut IMTConSubscriptionSink,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConSubscription) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgDeleteByID:
        unsafe extern "C" fn(this: *mut IMTServerAPI, id: UINT64) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_SubscriptionCfgNext: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        config: *mut IMTConSubscription,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        config: *mut IMTConSubscription,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgGetByID: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        id: UINT64,
        config: *mut IMTConSubscription,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgReserved1:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgReserved2:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgReserved3:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCfgReserved4:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTSubscription,
    pub IMTServerAPI_SubscriptionCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTSubscriptionArray,
    pub IMTServerAPI_SubscriptionSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTSubscriptionSink) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTSubscriptionSink) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionJoin: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        manager: UINT64,
        login: UINT64,
        subscription: UINT64,
        record: *mut IMTSubscription,
        history: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionCancel: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        manager: UINT64,
        login: UINT64,
        subscription: UINT64,
        record: *mut IMTSubscription,
        history: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionExist:
        unsafe extern "C" fn(this: *mut IMTServerAPI, login: UINT64, subscription: UINT64) -> bool,
    pub IMTServerAPI_SubscriptionAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, record: *mut IMTSubscription) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionUpdate:
        unsafe extern "C" fn(this: *mut IMTServerAPI, record: *mut IMTSubscription) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, id: UINT64) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        records: *mut IMTSubscriptionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionGetBySubscription: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        login: UINT64,
        subscription: UINT64,
        record: *mut IMTSubscription,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionGetByID: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        id: UINT64,
        record: *mut IMTSubscription,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionGetByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        logins: *const UINT64,
        logins_total: UINT,
        records: *mut IMTSubscriptionArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionReserved1:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionReserved2:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionReserved3:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionReserved4:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTSubscriptionHistory,
    pub IMTServerAPI_SubscriptionHistoryCreateArray:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTSubscriptionHistoryArray,
    pub IMTServerAPI_SubscriptionHistorySubscribe: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        sink: *mut IMTSubscriptionHistorySink,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryUnsubscribe: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        sink: *mut IMTSubscriptionHistorySink,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryAdd: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        record: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryUpdate: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        record: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, id: UINT64) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        from: INT64,
        to: INT64,
        login: UINT64,
        records: *mut IMTSubscriptionHistoryArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryGetByID: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        id: UINT64,
        record: *mut IMTSubscriptionHistory,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryGetByLogins: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        from: INT64,
        to: INT64,
        logins: *const UINT64,
        logins_total: UINT,
        records: *mut IMTSubscriptionHistoryArray,
    ) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryReserved1:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryReserved2:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryReserved3:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_SubscriptionHistoryReserved4:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveAny: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ip_list: LPCWSTR,
        result: LPWSTR,
        result_len: UINT,
        flags: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveIPv4: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ip: ULONG,
        result: LPWSTR,
        result_len: UINT,
        flags: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveIPv4Bulk: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ip_list: *const ULONG,
        ip_list_len: UINT,
        result: LPWSTR,
        result_len: UINT,
        flags: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveIPv6: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ip: *const IN6_ADDR,
        result: LPWSTR,
        result_len: UINT,
        flags: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveIPv6Bulk: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        ip_list: *const IN6_ADDR,
        ip_list_len: UINT,
        result: LPWSTR,
        result_len: UINT,
        flags: UINT,
    ) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_GeoResolveReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_VPSCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConVPS,
    pub IMTServerAPI_VPSCreateGroup:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConVPSGroup,
    pub IMTServerAPI_VPSSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConVPSSink) -> MTAPIRES,
    pub IMTServerAPI_VPSUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConVPSSink) -> MTAPIRES,
    pub IMTServerAPI_VPSGet:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConVPS) -> MTAPIRES,
    pub IMTServerAPI_VPSSet:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *const IMTConVPS) -> MTAPIRES,
    pub IMTServerAPI_VPSReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_VPSReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_VPSReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_VPSReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_KYCCreate: unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConKYC,
    pub IMTServerAPI_KYCCountryCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConKYCCountry,
    pub IMTServerAPI_KYCGroupCreate:
        unsafe extern "C" fn(this: *mut IMTServerAPI) -> *mut IMTConKYCGroup,
    pub IMTServerAPI_KYCSubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConKYCSink) -> MTAPIRES,
    pub IMTServerAPI_KYCUnsubscribe:
        unsafe extern "C" fn(this: *mut IMTServerAPI, sink: *mut IMTConKYCSink) -> MTAPIRES,
    pub IMTServerAPI_KYCAdd:
        unsafe extern "C" fn(this: *mut IMTServerAPI, config: *mut IMTConKYC) -> MTAPIRES,
    pub IMTServerAPI_KYCDelete1:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT) -> MTAPIRES,
    pub IMTServerAPI_KYCDelete:
        unsafe extern "C" fn(this: *mut IMTServerAPI, name: LPCWSTR) -> MTAPIRES,
    pub IMTServerAPI_KYCShift: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        pos: UINT,
        shift: ::std::os::raw::c_int,
    ) -> MTAPIRES,
    pub IMTServerAPI_KYCTotal: unsafe extern "C" fn(this: *mut IMTServerAPI) -> UINT,
    pub IMTServerAPI_KYCNext:
        unsafe extern "C" fn(this: *mut IMTServerAPI, pos: UINT, kyc: *mut IMTConKYC) -> MTAPIRES,
    pub IMTServerAPI_KYCGet: unsafe extern "C" fn(
        this: *mut IMTServerAPI,
        name: LPCWSTR,
        kyc: *mut IMTConKYC,
    ) -> MTAPIRES,
    pub IMTServerAPI_KYCStart:
        unsafe extern "C" fn(this: *mut IMTServerAPI, client_id: UINT64) -> MTAPIRES,
    pub IMTServerAPI_KYCReserved1: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_KYCReserved2: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_KYCReserved3: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerAPI_KYCReserved4: unsafe extern "C" fn(this: *mut IMTServerAPI) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTServerAPI {
    pub vtable_: *const IMTServerAPI__bindgen_vtable,
    pub impl_ptr: *mut dyn crate::interfaces::server::MT5Server,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTServerAPI"][::std::mem::size_of::<IMTServerAPI>() - 24usize];
    ["Alignment of IMTServerAPI"][::std::mem::align_of::<IMTServerAPI>() - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTServerPlugin__bindgen_vtable {
    pub IMTServerPlugin_Release: unsafe extern "C" fn(this: *mut IMTServerPlugin),
    pub IMTServerPlugin_Start:
        unsafe extern "C" fn(this: *mut IMTServerPlugin, server: *mut IMTServerAPI) -> MTAPIRES,
    pub IMTServerPlugin_Stop: unsafe extern "C" fn(this: *mut IMTServerPlugin) -> MTAPIRES,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IMTServerPlugin {
    pub vtable_: *const IMTServerPlugin__bindgen_vtable,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of IMTServerPlugin"][::std::mem::size_of::<IMTServerPlugin>() - 8usize];
    ["Alignment of IMTServerPlugin"][::std::mem::align_of::<IMTServerPlugin>() - 8usize];
};
