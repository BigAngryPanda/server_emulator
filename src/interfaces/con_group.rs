use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MT5ConGroup {
    fn release(&mut self) {
    }

    fn assign(&mut self, group: *const IMTConGroup) -> MTAPIRES {
        0
    }

    fn clear(&mut self) -> MTAPIRES {
        0
    }

    fn group1(&mut self, group: LPCWSTR) -> MTAPIRES {
        0
    }

    fn group(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn server1(&mut self, server: UINT64) -> MTAPIRES {
        0
    }

    fn server(&self) -> UINT64 {
        0
    }

    fn permissions_flags1(&mut self, flags: UINT64) -> MTAPIRES {
        0
    }

    fn permissions_flags(&self) -> UINT64 {
        0
    }

    fn auth_mode1(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn auth_mode(&self) -> UINT {
        0
    }

    fn auth_password_min1(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn auth_password_min(&self) -> UINT {
        0
    }

    fn company1(&mut self, company: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_page1(&mut self, page: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_page(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_email1(&mut self, email: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_email(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_support_page1(&mut self, page: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_support_page(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_support_email1(&mut self, email: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_support_email(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_catalog1(&mut self, catalog: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_catalog(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn currency1(&mut self, currency: LPCWSTR) -> MTAPIRES {
        0
    }

    fn currency(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn currency_digits(&self) -> UINT {
        0
    }

    fn reports_mode1(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn reports_mode(&self) -> UINT {
        0
    }

    fn reports_flags1(&mut self, flags: UINT64) -> MTAPIRES {
        0
    }

    fn reports_flags(&self) -> UINT64 {
        0
    }

    fn reports_smtp1(&mut self, server: LPCWSTR) -> MTAPIRES {
        0
    }

    fn reports_smtp(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn reports_smtp_login1(&mut self, login: LPCWSTR) -> MTAPIRES {
        0
    }

    fn reports_smtp_login(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn reports_smtp_pass1(&mut self, password: LPCWSTR) -> MTAPIRES {
        0
    }

    fn reports_smtp_pass(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn news_mode1(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn news_mode(&self) -> UINT {
        0
    }

    fn news_category1(&mut self, category: LPCWSTR) -> MTAPIRES {
        0
    }

    fn news_category(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn news_lang_add(&mut self, language: UINT) -> MTAPIRES {
        0
    }

    fn news_lang_update(&mut self, pos: UINT, language: UINT) -> MTAPIRES {
        0
    }

    fn news_lang_delete(&mut self, pos: UINT) -> MTAPIRES {
        0
    }

    fn news_lang_clear(&mut self) -> MTAPIRES {
        0
    }

    fn news_lang_total(&self) -> UINT {
        0
    }

    fn news_lang_next(&self, pos: UINT) -> UINT {
        0
    }

    fn mail_mode_set(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn mail_mode(&self) -> UINT {
        0
    }

    fn trade_flags_set(&mut self, flags: UINT64) -> MTAPIRES {
        0
    }

    fn trade_flags(&self) -> UINT64 {
        0
    }

    fn trade_interest_rate_set(&mut self, rate: f64) -> MTAPIRES {
        0
    }

    fn trade_interest_rate(&self) -> f64 {
        0.0
    }

    fn trade_virtual_credit_set(&mut self, credit: f64) -> MTAPIRES {
        0
    }

    fn trade_virtual_credit(&self) -> f64 {
        0.0
    }

    fn margin_free_mode_set(&mut self, freemode: UINT) -> MTAPIRES {
        0
    }

    fn margin_free_mode(&self) -> UINT {
        0
    }

    fn margin_so_mode_set(&mut self, level: UINT) -> MTAPIRES {
        0
    }

    fn margin_so_mode(&self) -> UINT {
        0
    }

    fn margin_call_set(&mut self, level: f64) -> MTAPIRES {
        0
    }

    fn margin_call(&self) -> f64 {
        0.0
    }

    fn margin_stopout_set(&mut self, level: f64) -> MTAPIRES {
        0
    }

    fn margin_stopout(&self) -> f64 {
        0.0
    }

    fn demo_leverage_set(&mut self, leverage: UINT) -> MTAPIRES {
        0
    }

    fn demo_leverage(&self) -> UINT {
        0
    }

    fn demo_deposit_set(&mut self, deposit: f64) -> MTAPIRES {
        0
    }

    fn demo_deposit(&self) -> f64 {
        0.0
    }

    fn limit_history_set(&mut self, limit: UINT) -> MTAPIRES {
        0
    }

    fn limit_history(&self) -> UINT {
        0
    }

    fn limit_orders_set(&mut self, limit: UINT) -> MTAPIRES {
        0
    }

    fn limit_orders(&self) -> UINT {
        0
    }

    fn limit_symbols_set(&mut self, limit: UINT) -> MTAPIRES {
        0
    }

    fn limit_symbols(&self) -> UINT {
        0
    }

    fn commission_add(&mut self, commission: &mut IMTConCommission) -> MTAPIRES {
        0
    }

    fn commission_update(&mut self, pos: UINT, commission: &IMTConCommission) -> MTAPIRES {
        0
    }

    fn commission_delete(&mut self, pos: UINT) -> MTAPIRES {
        0
    }

    fn commission_clear(&mut self) -> MTAPIRES {
        0
    }

    fn commission_shift(&mut self, pos: UINT, shift: INT) -> MTAPIRES {
        0
    }

    fn commission_total(&self) -> UINT {
        0
    }

    fn commission_next(&self, pos: UINT, commission: &mut IMTConCommission) -> MTAPIRES {
        0
    }

    fn commission_get(&self, name: LPCWSTR, commission: &mut IMTConCommission) -> MTAPIRES {
        0
    }

    fn symbol_add(&mut self, symbol: &mut IMTConGroupSymbol) -> MTAPIRES {
        0
    }

    fn symbol_update(&mut self, pos: UINT, symbol: &IMTConGroupSymbol) -> MTAPIRES {
        0
    }

    fn symbol_delete(&mut self, pos: UINT) -> MTAPIRES {
        0
    }

    fn symbol_clear(&mut self) -> MTAPIRES {
        0
    }

    fn symbol_shift(&mut self, pos: UINT, shift: INT) -> MTAPIRES {
        0
    }

    fn symbol_total(&self) -> UINT {
        0
    }

    fn symbol_next(&self, pos: UINT, symbol: &mut IMTConGroupSymbol) -> MTAPIRES {
        0
    }

    fn symbol_get(&self, name: LPCWSTR, symbol: &mut IMTConGroupSymbol) -> MTAPIRES {
        0
    }

    fn margin_free_profit_mode_set(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn margin_free_profit_mode(&self) -> UINT {
        0
    }

    fn margin_mode_set(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn margin_mode(&self) -> UINT {
        0
    }

    fn auth_otp_mode_set(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn auth_otp_mode(&self) -> UINT {
        0
    }

    fn trade_transfer_mode_set(&mut self, mode: UINT) -> MTAPIRES {
        0
    }

    fn trade_transfer_mode(&self) -> UINT {
        0
    }

    fn margin_flags_set(&mut self, flags: UINT64) -> MTAPIRES {
        0
    }

    fn margin_flags(&self) -> UINT64 {
        0
    }

    fn limit_positions_set(&mut self, limit: UINT) -> MTAPIRES {
        0
    }

    fn limit_positions(&self) -> UINT {
        0
    }

    fn currency_digits_set(&mut self, currency_digits: UINT) -> MTAPIRES {
        0
    }

    fn reports_email_set(&mut self, email: LPCWSTR) -> MTAPIRES {
        0
    }

    fn reports_email(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_deposit_page_set(&mut self, url: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_deposit_page(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn company_withdrawal_page_set(&mut self, url: LPCWSTR) -> MTAPIRES {
        0
    }

    fn company_withdrawal_page(&self) -> LPCWSTR {
        std::ptr::null()
    }

    fn demo_inactivity_period_set(&mut self, period: UINT) -> MTAPIRES {
        0
    }

    fn demo_inactivity_period(&self) -> UINT {
        0
    }

}