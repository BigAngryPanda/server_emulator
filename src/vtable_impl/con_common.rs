use crate::{
    mt5_apiserver,
    conversion
};

use mt5_apiserver::*;

unsafe extern "C" fn IMTConCommon_Release(this: *mut IMTConCommon) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTConCommon>());
}

unsafe extern "C" fn IMTConCommon_Assign(this: *mut IMTConCommon, param: *const IMTConCommon) -> MTAPIRES {
    (*(*this).impl_ptr).assign(&*param)
}

unsafe extern "C" fn IMTConCommon_Clear(this: *mut IMTConCommon) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTConCommon_Name1(this: *mut IMTConCommon, name: LPCWSTR) -> MTAPIRES {
    let wstr = unsafe { std::slice::from_raw_parts(name, conversion::strlen(name)) };
    (*(*this).impl_ptr).name1(wstr)
}

unsafe extern "C" fn IMTConCommon_Name(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).name()
}

unsafe extern "C" fn IMTConCommon_NameFull(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).name_full()
}

unsafe extern "C" fn IMTConCommon_Owner(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).owner()
}

unsafe extern "C" fn IMTConCommon_OwnerID(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).owner_id()
}

unsafe extern "C" fn IMTConCommon_OwnerHost(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).owner_host()
}

unsafe extern "C" fn IMTConCommon_OwnerEmail(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).owner_email()
}

unsafe extern "C" fn IMTConCommon_Product(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).product()
}

unsafe extern "C" fn IMTConCommon_ExpirationLicense(this: *const IMTConCommon) -> INT64 {
    (*(*this).impl_ptr).expiration_license()
}

unsafe extern "C" fn IMTConCommon_ExpirationSupport(this: *const IMTConCommon) -> INT64 {
    (*(*this).impl_ptr).expiration_support()
}

unsafe extern "C" fn IMTConCommon_LimitTradeServers(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).limit_trade_servers()
}

unsafe extern "C" fn IMTConCommon_LimitWebServers(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).limit_web_servers()
}

unsafe extern "C" fn IMTConCommon_LimitAccounts(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).limit_accounts()
}

unsafe extern "C" fn IMTConCommon_LimitDeals(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).limit_deals()
}

unsafe extern "C" fn IMTConCommon_LimitGroups(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).limit_groups()
}

unsafe extern "C" fn IMTConCommon_LiveUpdateMode(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).live_update_mode()
}

unsafe extern "C" fn IMTConCommon_LiveUpdateMode1(this: *mut IMTConCommon, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).live_update_mode1(mode)
}

unsafe extern "C" fn IMTConCommon_TotalUsers(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).total_users()
}

unsafe extern "C" fn IMTConCommon_TotalUsersReal(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).total_users_real()
}

unsafe extern "C" fn IMTConCommon_TotalDeals(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).total_deals()
}

unsafe extern "C" fn IMTConCommon_TotalOrders(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).total_orders()
}

unsafe extern "C" fn IMTConCommon_TotalOrdersHistory(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).total_orders_history()
}

unsafe extern "C" fn IMTConCommon_TotalPositions(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).total_positions()
}

unsafe extern "C" fn IMTConCommon_LimitSymbols(this: *const IMTConCommon) -> UINT {
    (*(*this).impl_ptr).limit_symbols()
}

unsafe extern "C" fn IMTConCommon_AccountURL(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).account_url()
}

unsafe extern "C" fn IMTConCommon_AccountURL1(this: *mut IMTConCommon, url: LPCWSTR) -> MTAPIRES {
    let wstr = unsafe { std::slice::from_raw_parts(url, conversion::strlen(url)) };
    (*(*this).impl_ptr).account_url1(wstr)
}

unsafe extern "C" fn IMTConCommon_AccountDepositURL(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).account_deposit_url()
}

unsafe extern "C" fn IMTConCommon_AccountDepositURL1(this: *mut IMTConCommon, url: LPCWSTR) -> MTAPIRES {
    let wstr = unsafe { std::slice::from_raw_parts(url, conversion::strlen(url)) };
    (*(*this).impl_ptr).account_deposit_url1(wstr)
}

unsafe extern "C" fn IMTConCommon_AccountWithdrawalURL(this: *const IMTConCommon) -> LPCWSTR {
    (*(*this).impl_ptr).account_withdrawal_url()
}

unsafe extern "C" fn IMTConCommon_AccountWithdrawalURL1(this: *mut IMTConCommon, url: LPCWSTR) -> MTAPIRES {
    let wstr = unsafe { std::slice::from_raw_parts(url, conversion::strlen(url)) };
    (*(*this).impl_ptr).account_withdrawal_url1(wstr)
}

pub const fn new() -> IMTConCommon__bindgen_vtable {
    IMTConCommon__bindgen_vtable {
        IMTConCommon_Release,
        IMTConCommon_Assign,
        IMTConCommon_Clear,
        IMTConCommon_Name1,
        IMTConCommon_Name,
        IMTConCommon_NameFull,
        IMTConCommon_Owner,
        IMTConCommon_OwnerID,
        IMTConCommon_OwnerHost,
        IMTConCommon_OwnerEmail,
        IMTConCommon_Product,
        IMTConCommon_ExpirationLicense,
        IMTConCommon_ExpirationSupport,
        IMTConCommon_LimitTradeServers,
        IMTConCommon_LimitWebServers,
        IMTConCommon_LimitAccounts,
        IMTConCommon_LimitDeals,
        IMTConCommon_LimitGroups,
        IMTConCommon_LiveUpdateMode,
        IMTConCommon_LiveUpdateMode1,
        IMTConCommon_TotalUsers,
        IMTConCommon_TotalUsersReal,
        IMTConCommon_TotalDeals,
        IMTConCommon_TotalOrders,
        IMTConCommon_TotalOrdersHistory,
        IMTConCommon_TotalPositions,
        IMTConCommon_LimitSymbols,
        IMTConCommon_AccountURL,
        IMTConCommon_AccountURL1,
        IMTConCommon_AccountDepositURL,
        IMTConCommon_AccountDepositURL1,
        IMTConCommon_AccountWithdrawalURL,
        IMTConCommon_AccountWithdrawalURL1,
    }
}
