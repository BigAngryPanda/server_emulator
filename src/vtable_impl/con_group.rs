use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTConGroup_Release(this: *mut IMTConGroup) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTConGroup>());
}

unsafe extern "C" fn IMTConGroup_Assign(this: *mut IMTConGroup, group: *const IMTConGroup) -> MTAPIRES {
    (*(*this).impl_ptr).assign(group)
}

unsafe extern "C" fn IMTConGroup_Clear(this: *mut IMTConGroup) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTConGroup_Group1(this: *mut IMTConGroup, group: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).group1(group)
}

unsafe extern "C" fn IMTConGroup_Group(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).group()
}

unsafe extern "C" fn IMTConGroup_Server1(this: *mut IMTConGroup, server: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).server1(server)
}

unsafe extern "C" fn IMTConGroup_Server(this: *const IMTConGroup) -> UINT64 {
    (*(*this).impl_ptr).server()
}

unsafe extern "C" fn IMTConGroup_PermissionsFlags1(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).permissions_flags1(flags)
}

unsafe extern "C" fn IMTConGroup_PermissionsFlags(this: *const IMTConGroup) -> UINT64 {
    (*(*this).impl_ptr).permissions_flags()
}

unsafe extern "C" fn IMTConGroup_AuthMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).auth_mode1(mode)
}

unsafe extern "C" fn IMTConGroup_AuthMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).auth_mode()
}

unsafe extern "C" fn IMTConGroup_AuthPasswordMin1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).auth_password_min1(mode)
}

unsafe extern "C" fn IMTConGroup_AuthPasswordMin(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).auth_password_min()
}

unsafe extern "C" fn IMTConGroup_Company1(this: *mut IMTConGroup, company: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company1(company)
}

unsafe extern "C" fn IMTConGroup_Company(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company()
}

unsafe extern "C" fn IMTConGroup_CompanyPage1(this: *mut IMTConGroup, page: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_page1(page)
}

unsafe extern "C" fn IMTConGroup_CompanyPage(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_page()
}

unsafe extern "C" fn IMTConGroup_CompanyEmail1(this: *mut IMTConGroup, email: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_email1(email)
}

unsafe extern "C" fn IMTConGroup_CompanyEmail(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_email()
}

unsafe extern "C" fn IMTConGroup_CompanySupportPage1(this: *mut IMTConGroup, page: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_support_page1(page)
}

unsafe extern "C" fn IMTConGroup_CompanySupportPage(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_support_page()
}

unsafe extern "C" fn IMTConGroup_CompanySupportEmail1(this: *mut IMTConGroup, email: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_support_email1(email)
}

unsafe extern "C" fn IMTConGroup_CompanySupportEmail(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_support_email()
}

unsafe extern "C" fn IMTConGroup_CompanyCatalog1(this: *mut IMTConGroup, catalog: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_catalog1(catalog)
}

unsafe extern "C" fn IMTConGroup_CompanyCatalog(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_catalog()
}

unsafe extern "C" fn IMTConGroup_Currency1(this: *mut IMTConGroup, currency: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).currency1(currency)
}

unsafe extern "C" fn IMTConGroup_Currency(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).currency()
}

unsafe extern "C" fn IMTConGroup_CurrencyDigits(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).currency_digits()
}

unsafe extern "C" fn IMTConGroup_ReportsMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).reports_mode1(mode)
}

unsafe extern "C" fn IMTConGroup_ReportsMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).reports_mode()
}

unsafe extern "C" fn IMTConGroup_ReportsFlags1(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).reports_flags1(flags)
}

unsafe extern "C" fn IMTConGroup_ReportsFlags(this: *const IMTConGroup) -> UINT64 {
    (*(*this).impl_ptr).reports_flags()
}

unsafe extern "C" fn IMTConGroup_ReportsSMTP1(this: *mut IMTConGroup, server: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).reports_smtp1(server)
}

unsafe extern "C" fn IMTConGroup_ReportsSMTP(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).reports_smtp()
}

unsafe extern "C" fn IMTConGroup_ReportsSMTPLogin1(this: *mut IMTConGroup, login: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).reports_smtp_login1(login)
}

unsafe extern "C" fn IMTConGroup_ReportsSMTPLogin(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).reports_smtp_login()
}

unsafe extern "C" fn IMTConGroup_ReportsSMTPPass1(this: *mut IMTConGroup, password: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).reports_smtp_pass1(password)
}

unsafe extern "C" fn IMTConGroup_ReportsSMTPPass(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).reports_smtp_pass()
}

unsafe extern "C" fn IMTConGroup_NewsMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).news_mode1(mode)
}

unsafe extern "C" fn IMTConGroup_NewsMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).news_mode()
}

unsafe extern "C" fn IMTConGroup_NewsCategory1(this: *mut IMTConGroup, category: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).news_category1(category)
}

unsafe extern "C" fn IMTConGroup_NewsCategory(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).news_category()
}

unsafe extern "C" fn IMTConGroup_NewsLangAdd(this: *mut IMTConGroup, language: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).news_lang_add(language)
}

unsafe extern "C" fn IMTConGroup_NewsLangUpdate(this: *mut IMTConGroup, pos: UINT, language: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).news_lang_update(pos, language)
}

unsafe extern "C" fn IMTConGroup_NewsLangDelete(this: *mut IMTConGroup, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).news_lang_delete(pos)
}

unsafe extern "C" fn IMTConGroup_NewsLangClear(this: *mut IMTConGroup) -> MTAPIRES {
    (*(*this).impl_ptr).news_lang_clear()
}

unsafe extern "C" fn IMTConGroup_NewsLangTotal(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).news_lang_total()
}

unsafe extern "C" fn IMTConGroup_NewsLangNext(this: *const IMTConGroup, pos: UINT) -> UINT {
    (*(*this).impl_ptr).news_lang_next(pos)
}

unsafe extern "C" fn IMTConGroup_MailMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).mail_mode_set(mode)
}

unsafe extern "C" fn IMTConGroup_MailMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).mail_mode()
}

unsafe extern "C" fn IMTConGroup_TradeFlags1(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).trade_flags_set(flags)
}

unsafe extern "C" fn IMTConGroup_TradeFlags(this: *const IMTConGroup) -> UINT64 {
    (*(*this).impl_ptr).trade_flags()
}

unsafe extern "C" fn IMTConGroup_TradeInterestrate1(this: *mut IMTConGroup, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).trade_interest_rate_set(rate)
}

unsafe extern "C" fn IMTConGroup_TradeInterestrate(this: *const IMTConGroup) -> f64 {
    (*(*this).impl_ptr).trade_interest_rate()
}

unsafe extern "C" fn IMTConGroup_TradeVirtualCredit1(this: *mut IMTConGroup, credit: f64) -> MTAPIRES {
    (*(*this).impl_ptr).trade_virtual_credit_set(credit)
}

unsafe extern "C" fn IMTConGroup_TradeVirtualCredit(this: *const IMTConGroup) -> f64 {
    (*(*this).impl_ptr).trade_virtual_credit()
}

unsafe extern "C" fn IMTConGroup_MarginFreeMode1(this: *mut IMTConGroup, freemode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).margin_free_mode_set(freemode)
}

unsafe extern "C" fn IMTConGroup_MarginFreeMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).margin_free_mode()
}

unsafe extern "C" fn IMTConGroup_MarginSOMode1(this: *mut IMTConGroup, level: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).margin_so_mode_set(level)
}

unsafe extern "C" fn IMTConGroup_MarginSOMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).margin_so_mode()
}

unsafe extern "C" fn IMTConGroup_MarginCall1(this: *mut IMTConGroup, level: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_call_set(level)
}

unsafe extern "C" fn IMTConGroup_MarginCall(this: *const IMTConGroup) -> f64 {
    (*(*this).impl_ptr).margin_call()
}

unsafe extern "C" fn IMTConGroup_MarginStopOut1(this: *mut IMTConGroup, level: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_stopout_set(level)
}

unsafe extern "C" fn IMTConGroup_MarginStopOut(this: *const IMTConGroup) -> f64 {
    (*(*this).impl_ptr).margin_stopout()
}

unsafe extern "C" fn IMTConGroup_DemoLeverage1(this: *mut IMTConGroup, leverage: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).demo_leverage_set(leverage)
}

unsafe extern "C" fn IMTConGroup_DemoLeverage(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).demo_leverage()
}

unsafe extern "C" fn IMTConGroup_DemoDeposit1(this: *mut IMTConGroup, deposit: f64) -> MTAPIRES {
    (*(*this).impl_ptr).demo_deposit_set(deposit)
}

unsafe extern "C" fn IMTConGroup_DemoDeposit(this: *const IMTConGroup) -> f64 {
    (*(*this).impl_ptr).demo_deposit()
}

unsafe extern "C" fn IMTConGroup_LimitHistory1(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).limit_history_set(limit)
}

unsafe extern "C" fn IMTConGroup_LimitHistory(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).limit_history()
}

unsafe extern "C" fn IMTConGroup_LimitOrders1(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).limit_orders_set(limit)
}

unsafe extern "C" fn IMTConGroup_LimitOrders(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).limit_orders()
}

unsafe extern "C" fn IMTConGroup_LimitSymbols1(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).limit_symbols_set(limit)
}

unsafe extern "C" fn IMTConGroup_LimitSymbols(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).limit_symbols()
}

unsafe extern "C" fn IMTConGroup_CommissionAdd(this: *mut IMTConGroup, commission: *mut IMTConCommission) -> MTAPIRES {
    (*(*this).impl_ptr).commission_add(&mut *commission)
}

unsafe extern "C" fn IMTConGroup_CommissionUpdate(
    this: *mut IMTConGroup,
    pos: UINT,
    commission: *const IMTConCommission,
) -> MTAPIRES {
    (*(*this).impl_ptr).commission_update(pos, &*commission)
}

unsafe extern "C" fn IMTConGroup_CommissionDelete(this: *mut IMTConGroup, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).commission_delete(pos)
}

unsafe extern "C" fn IMTConGroup_CommissionClear(this: *mut IMTConGroup) -> MTAPIRES {
    (*(*this).impl_ptr).commission_clear()
}

unsafe extern "C" fn IMTConGroup_CommissionShift(
    this: *mut IMTConGroup,
    pos: UINT,
    shift: ::std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).commission_shift(pos, shift)
}

unsafe extern "C" fn IMTConGroup_CommissionTotal(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).commission_total()
}

unsafe extern "C" fn IMTConGroup_CommissionNext(
    this: *const IMTConGroup,
    pos: UINT,
    commission: *mut IMTConCommission,
) -> MTAPIRES {
    (*(*this).impl_ptr).commission_next(pos, &mut *commission)
}

unsafe extern "C" fn IMTConGroup_CommissionGet(
    this: *const IMTConGroup,
    name: LPCWSTR,
    commission: *mut IMTConCommission,
) -> MTAPIRES {
    (*(*this).impl_ptr).commission_get(name, &mut *commission)
}

unsafe extern "C" fn IMTConGroup_SymbolAdd(this: *mut IMTConGroup, symbol: *mut IMTConGroupSymbol) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_add(&mut *symbol)
}

unsafe extern "C" fn IMTConGroup_SymbolUpdate(
    this: *mut IMTConGroup,
    pos: UINT,
    symbol: *const IMTConGroupSymbol,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_update(pos, &*symbol)
}

unsafe extern "C" fn IMTConGroup_SymbolDelete(this: *mut IMTConGroup, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_delete(pos)
}

unsafe extern "C" fn IMTConGroup_SymbolClear(this: *mut IMTConGroup) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_clear()
}

unsafe extern "C" fn IMTConGroup_SymbolShift(
    this: *mut IMTConGroup,
    pos: UINT,
    shift: ::std::os::raw::c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_shift(pos, shift)
}

unsafe extern "C" fn IMTConGroup_SymbolTotal(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).symbol_total()
}

unsafe extern "C" fn IMTConGroup_SymbolNext(
    this: *const IMTConGroup,
    pos: UINT,
    symbol: *mut IMTConGroupSymbol,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_next(pos, &mut *symbol)
}

unsafe extern "C" fn IMTConGroup_SymbolGet(
    this: *const IMTConGroup,
    name: LPCWSTR,
    symbol: *mut IMTConGroupSymbol,
) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_get(name, &mut *symbol)
}

unsafe extern "C" fn IMTConGroup_MarginFreeProfitMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).margin_free_profit_mode_set(mode)
}

unsafe extern "C" fn IMTConGroup_MarginFreeProfitMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).margin_free_profit_mode()
}

unsafe extern "C" fn IMTConGroup_MarginMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).margin_mode_set(mode)
}

unsafe extern "C" fn IMTConGroup_MarginMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).margin_mode()
}

unsafe extern "C" fn IMTConGroup_AuthOTPMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).auth_otp_mode_set(mode)
}

unsafe extern "C" fn IMTConGroup_AuthOTPMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).auth_otp_mode()
}

unsafe extern "C" fn IMTConGroup_TradeTransferMode1(this: *mut IMTConGroup, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).trade_transfer_mode_set(mode)
}

unsafe extern "C" fn IMTConGroup_TradeTransferMode(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).trade_transfer_mode()
}

unsafe extern "C" fn IMTConGroup_MarginFlags1(this: *mut IMTConGroup, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_flags_set(flags)
}

unsafe extern "C" fn IMTConGroup_MarginFlags(this: *const IMTConGroup) -> UINT64 {
    (*(*this).impl_ptr).margin_flags()
}

unsafe extern "C" fn IMTConGroup_LimitPositions1(this: *mut IMTConGroup, limit: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).limit_positions_set(limit)
}

unsafe extern "C" fn IMTConGroup_LimitPositions(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).limit_positions()
}

unsafe extern "C" fn IMTConGroup_CurrencyDigitsSet(this: *mut IMTConGroup, currency_digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).currency_digits_set(currency_digits)
}

unsafe extern "C" fn IMTConGroup_ReportsEmail1(this: *mut IMTConGroup, email: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).reports_email_set(email)
}

unsafe extern "C" fn IMTConGroup_ReportsEmail(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).reports_email()
}

unsafe extern "C" fn IMTConGroup_CompanyDepositPage1(this: *mut IMTConGroup, url: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_deposit_page_set(url)
}

unsafe extern "C" fn IMTConGroup_CompanyDepositPage(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_deposit_page()
}

unsafe extern "C" fn IMTConGroup_CompanyWithdrawalPage1(this: *mut IMTConGroup, url: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company_withdrawal_page_set(url)
}

unsafe extern "C" fn IMTConGroup_CompanyWithdrawalPage(this: *const IMTConGroup) -> LPCWSTR {
    (*(*this).impl_ptr).company_withdrawal_page()
}

unsafe extern "C" fn IMTConGroup_DemoInactivityPeriod1(this: *mut IMTConGroup, period: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).demo_inactivity_period_set(period)
}

unsafe extern "C" fn IMTConGroup_DemoInactivityPeriod(this: *const IMTConGroup) -> UINT {
    (*(*this).impl_ptr).demo_inactivity_period()
}

pub const fn new() -> IMTConGroup__bindgen_vtable {
    IMTConGroup__bindgen_vtable {
        IMTConGroup_Release,
        IMTConGroup_Assign,
        IMTConGroup_Clear,
        IMTConGroup_Group1,
        IMTConGroup_Group,
        IMTConGroup_Server1,
        IMTConGroup_Server,
        IMTConGroup_PermissionsFlags1,
        IMTConGroup_PermissionsFlags,
        IMTConGroup_AuthMode1,
        IMTConGroup_AuthMode,
        IMTConGroup_AuthPasswordMin1,
        IMTConGroup_AuthPasswordMin,
        IMTConGroup_Company1,
        IMTConGroup_Company,
        IMTConGroup_CompanyPage1,
        IMTConGroup_CompanyPage,
        IMTConGroup_CompanyEmail1,
        IMTConGroup_CompanyEmail,
        IMTConGroup_CompanySupportPage1,
        IMTConGroup_CompanySupportPage,
        IMTConGroup_CompanySupportEmail1,
        IMTConGroup_CompanySupportEmail,
        IMTConGroup_CompanyCatalog1,
        IMTConGroup_CompanyCatalog,
        IMTConGroup_Currency1,
        IMTConGroup_Currency,
        IMTConGroup_CurrencyDigits,
        IMTConGroup_ReportsMode1,
        IMTConGroup_ReportsMode,
        IMTConGroup_ReportsFlags1,
        IMTConGroup_ReportsFlags,
        IMTConGroup_ReportsSMTP1,
        IMTConGroup_ReportsSMTP,
        IMTConGroup_ReportsSMTPLogin1,
        IMTConGroup_ReportsSMTPLogin,
        IMTConGroup_ReportsSMTPPass1,
        IMTConGroup_ReportsSMTPPass,
        IMTConGroup_NewsMode1,
        IMTConGroup_NewsMode,
        IMTConGroup_NewsCategory1,
        IMTConGroup_NewsCategory,
        IMTConGroup_NewsLangAdd,
        IMTConGroup_NewsLangUpdate,
        IMTConGroup_NewsLangDelete,
        IMTConGroup_NewsLangClear,
        IMTConGroup_NewsLangTotal,
        IMTConGroup_NewsLangNext,
        IMTConGroup_MailMode1,
        IMTConGroup_MailMode,
        IMTConGroup_TradeFlags1,
        IMTConGroup_TradeFlags,
        IMTConGroup_TradeInterestrate1,
        IMTConGroup_TradeInterestrate,
        IMTConGroup_TradeVirtualCredit1,
        IMTConGroup_TradeVirtualCredit,
        IMTConGroup_MarginFreeMode1,
        IMTConGroup_MarginFreeMode,
        IMTConGroup_MarginSOMode1,
        IMTConGroup_MarginSOMode,
        IMTConGroup_MarginCall1,
        IMTConGroup_MarginCall,
        IMTConGroup_MarginStopOut1,
        IMTConGroup_MarginStopOut,
        IMTConGroup_DemoLeverage1,
        IMTConGroup_DemoLeverage,
        IMTConGroup_DemoDeposit1,
        IMTConGroup_DemoDeposit,
        IMTConGroup_LimitHistory1,
        IMTConGroup_LimitHistory,
        IMTConGroup_LimitOrders1,
        IMTConGroup_LimitOrders,
        IMTConGroup_LimitSymbols1,
        IMTConGroup_LimitSymbols,
        IMTConGroup_CommissionAdd,
        IMTConGroup_CommissionUpdate,
        IMTConGroup_CommissionDelete,
        IMTConGroup_CommissionClear,
        IMTConGroup_CommissionShift,
        IMTConGroup_CommissionTotal,
        IMTConGroup_CommissionNext,
        IMTConGroup_CommissionGet,
        IMTConGroup_SymbolAdd,
        IMTConGroup_SymbolUpdate,
        IMTConGroup_SymbolDelete,
        IMTConGroup_SymbolClear,
        IMTConGroup_SymbolShift,
        IMTConGroup_SymbolTotal,
        IMTConGroup_SymbolNext,
        IMTConGroup_SymbolGet,
        IMTConGroup_MarginFreeProfitMode1,
        IMTConGroup_MarginFreeProfitMode,
        IMTConGroup_MarginMode1,
        IMTConGroup_MarginMode,
        IMTConGroup_AuthOTPMode1,
        IMTConGroup_AuthOTPMode,
        IMTConGroup_TradeTransferMode1,
        IMTConGroup_TradeTransferMode,
        IMTConGroup_MarginFlags1,
        IMTConGroup_MarginFlags,
        IMTConGroup_LimitPositions1,
        IMTConGroup_LimitPositions,
        IMTConGroup_CurrencyDigitsSet,
        IMTConGroup_ReportsEmail1,
        IMTConGroup_ReportsEmail,
        IMTConGroup_CompanyDepositPage1,
        IMTConGroup_CompanyDepositPage,
        IMTConGroup_CompanyWithdrawalPage1,
        IMTConGroup_CompanyWithdrawalPage,
        IMTConGroup_DemoInactivityPeriod1,
        IMTConGroup_DemoInactivityPeriod
    }
}
