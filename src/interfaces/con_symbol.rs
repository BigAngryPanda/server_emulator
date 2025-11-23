use crate::mt5_apiserver::*;

use server_emulator_macro::log_trait_calls;

use std::os::raw::c_uint;

#[allow(unused_variables)]
#[log_trait_calls]
pub trait MTConSymbol {
    fn release(&mut self) {}

    fn assign(&mut self, symbol: *const IMTConSymbol) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn symbol(&self) -> *const u16 {
        std::ptr::null()
    }

    fn symbol1(&mut self, symbol: *const u16) -> c_uint {
        0
    }

    fn path(&self) -> *const u16 {
        std::ptr::null()
    }

    fn path1(&mut self, path: *const u16) -> c_uint {
        0
    }

    fn isin(&self) -> *const u16 {
        std::ptr::null()
    }

    fn isin1(&mut self, isin: *const u16) -> c_uint {
        0
    }

    fn description(&self) -> *const u16 {
        std::ptr::null()
    }

    fn description1(&mut self, descr: *const u16) -> c_uint {
        0
    }

    fn international(&self) -> *const u16 {
        std::ptr::null()
    }

    fn international1(&mut self, intern: *const u16) -> c_uint {
        0
    }

    fn basis(&self) -> *const u16 {
        std::ptr::null()
    }

    fn basis1(&mut self, basis: *const u16) -> c_uint {
        0
    }

    fn source(&self) -> *const u16 {
        std::ptr::null()
    }

    fn source1(&mut self, source: *const u16) -> c_uint {
        0
    }

    fn page(&self) -> *const u16 {
        std::ptr::null()
    }

    fn page1(&mut self, page: *const u16) -> c_uint {
        0
    }

    fn currency_base(&self) -> *const u16 {
        std::ptr::null()
    }

    fn currency_base1(&mut self, currency: *const u16) -> c_uint {
        0
    }

    fn currency_base_digits(&self) -> u32 {
        0
    }

    fn currency_profit(&self) -> *const u16 {
        std::ptr::null()
    }

    fn currency_profit1(&mut self, currency: *const u16) -> c_uint {
        0
    }

    fn currency_profit_digits(&self) -> u32 {
        0
    }

    fn currency_margin(&self) -> *const u16 {
        std::ptr::null()
    }

    fn currency_margin1(&mut self, currency: *const u16) -> c_uint {
        0
    }

    fn currency_margin_digits(&self) -> u32 {
        0
    }

    fn color(&self) -> u32 {
        0
    }

    fn color1(&mut self, color: u32) -> c_uint {
        0
    }

    fn color_background(&self) -> u32 {
        0
    }

    fn color_background1(&mut self, color: u32) -> c_uint {
        0
    }

    fn digits(&self) -> u32 {
        0
    }

    fn digits1(&mut self, digits: u32) -> c_uint {
        0
    }

    fn point(&self) -> f64 {
        0.0
    }

    fn multiply(&self) -> f64 {
        0.0
    }

    fn tick_flags(&self) -> u64 {
        0
    }

    fn tick_flags1(&mut self, flags: u64) -> c_uint {
        0
    }

    fn tick_book_depth(&self) -> u32 {
        0
    }

    fn tick_book_depth1(&mut self, depth: u32) -> c_uint {
        0
    }

    fn filter_soft(&self) -> u32 {
        0
    }

    fn filter_soft1(&mut self, filter: u32) -> c_uint {
        0
    }

    fn filter_soft_ticks(&self) -> u32 {
        0
    }

    fn filter_soft_ticks1(&mut self, ticks: u32) -> c_uint {
        0
    }

    fn filter_hard(&self) -> u32 {
        0
    }

    fn filter_hard1(&mut self, filter: u32) -> c_uint {
        0
    }

    fn filter_hard_ticks(&self) -> u32 {
        0
    }

    fn filter_hard_ticks1(&mut self, ticks: u32) -> c_uint {
        0
    }

    fn filter_discard(&self) -> u32 {
        0
    }

    fn filter_discard1(&mut self, ticks: u32) -> c_uint {
        0
    }

    fn filter_spread_max(&self) -> u32 {
        0
    }

    fn filter_spread_max1(&mut self, spread: u32) -> c_uint {
        0
    }

    fn filter_spread_min(&self) -> u32 {
        0
    }

    fn filter_spread_min1(&mut self, spread: u32) -> c_uint {
        0
    }

    fn trade_mode(&self) -> u32 {
        0
    }

    fn trade_mode1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn calc_mode(&self) -> u32 {
        0
    }

    fn calc_mode1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn exec_mode(&self) -> u32 {
        0
    }

    fn exec_mode1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn gtc_mode(&self) -> u32 {
        0
    }

    fn gtc_mode1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn fill_flags(&self) -> u32 {
        0
    }

    fn fill_flags1(&mut self, flags: u32) -> c_uint {
        0
    }

    fn expir_flags(&self) -> u32 {
        0
    }

    fn expir_flags1(&mut self, flags: u32) -> c_uint {
        0
    }

    fn spread(&self) -> u32 {
        0
    }

    fn spread1(&mut self, spread: u32) -> c_uint {
        0
    }

    fn spread_balance(&self) -> i32 {
        0
    }

    fn spread_balance1(&mut self, spread: i32) -> c_uint {
        0
    }

    fn spread_diff(&self) -> i32 {
        0
    }

    fn spread_diff1(&mut self, diff: i32) -> c_uint {
        0
    }

    fn spread_diff_balance(&self) -> i32 {
        0
    }

    fn spread_diff_balance1(&mut self, spread: i32) -> c_uint {
        0
    }

    fn tick_value(&self) -> f64 {
        0.0
    }

    fn tick_value1(&mut self, value: f64) -> c_uint {
        0
    }

    fn tick_size(&self) -> f64 {
        0.0
    }

    fn tick_size1(&mut self, size: f64) -> c_uint {
        0
    }

    fn contract_size(&self) -> f64 {
        0.0
    }

    fn contract_size1(&mut self, size: f64) -> c_uint {
        0
    }

    fn stops_level(&self) -> i32 {
        0
    }

    fn stops_level1(&mut self, level: i32) -> c_uint {
        0
    }

    fn freeze_level(&self) -> i32 {
        0
    }

    fn freeze_level1(&mut self, level: i32) -> c_uint {
        0
    }

    fn quotes_timeout(&self) -> u32 {
        0
    }

    fn quotes_timeout1(&mut self, timeout: u32) -> c_uint {
        0
    }

    fn volume_min(&self) -> u64 {
        0
    }

    fn volume_min1(&mut self, volume: u64) -> c_uint {
        0
    }

    fn volume_max(&self) -> u64 {
        0
    }

    fn volume_max1(&mut self, volume: u64) -> c_uint {
        0
    }

    fn volume_step(&self) -> u64 {
        0
    }

    fn volume_step1(&mut self, volume: u64) -> c_uint {
        0
    }

    fn volume_limit(&self) -> u64 {
        0
    }

    fn volume_limit1(&mut self, volume: u64) -> c_uint {
        0
    }

    fn margin_flags(&self) -> u32 {
        0
    }

    fn margin_flags1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn margin_initial(&self) -> f64 {
        0.0
    }

    fn margin_initial1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn margin_maintenance(&self) -> f64 {
        0.0
    }

    fn margin_maintenance1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn margin_long(&self) -> f64 {
        0.0
    }

    fn margin_long1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn margin_short(&self) -> f64 {
        0.0
    }

    fn margin_short1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn margin_limit(&self) -> f64 {
        0.0
    }

    fn margin_limit1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn margin_stop(&self) -> f64 {
        0.0
    }

    fn margin_stop1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn margin_stop_limit(&self) -> f64 {
        0.0
    }

    fn margin_stop_limit1(&mut self, margin: f64) -> c_uint {
        0
    }

    fn swap_mode(&self) -> u32 {
        0
    }

    fn swap_mode1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn swap_long(&self) -> f64 {
        0.0
    }

    fn swap_long1(&mut self, swap: f64) -> c_uint {
        0
    }

    fn swap_short(&self) -> f64 {
        0.0
    }

    fn swap_short1(&mut self, swap: f64) -> c_uint {
        0
    }

    fn swap3_day(&self) -> u32 {
        0
    }

    fn swap3_day1(&mut self, day: u32) -> c_uint {
        0
    }

    fn expiration_mode(&self) -> u32 {
        0
    }

    fn expiration_mode1(&mut self, mode: u32) -> c_uint {
        0
    }

    fn expiration_day(&self) -> u32 {
        0
    }

    fn expiration_day1(&mut self, day: u32) -> c_uint {
        0
    }

    fn last_trade(&self) -> f64 {
        0.0
    }

    fn last_trade1(&mut self, price: f64) -> c_uint {
        0
    }

    fn last_bid(&self) -> f64 {
        0.0
    }

    fn last_bid1(&mut self, price: f64) -> c_uint {
        0
    }

    fn last_ask(&self) -> f64 {
        0.0
    }

    fn last_ask1(&mut self, price: f64) -> c_uint {
        0
    }

    fn last_deal(&self) -> f64 {
        0.0
    }

    fn last_deal1(&mut self, price: f64) -> c_uint {
        0
    }

    fn last_time(&self) -> u64 {
        0
    }

    fn last_time1(&mut self, time: u64) -> c_uint {
        0
    }

    fn time_start(&self) -> i64 {
        0
    }

    fn time_start1(&mut self, start: i64) -> u32 {
        0
    }

    fn time_expiration(&self) -> i64 {
        0
    }

    fn time_expiration1(&mut self, expiration: i64) -> u32 {
        0
    }

    fn session_quote_add(&mut self, wday: u32, symbol: &mut IMTConSymbolSession) -> u32 {
        0
    }

    fn session_quote_update(&mut self, wday: u32, pos: u32, session: &IMTConSymbolSession) -> u32 {
        0
    }

    fn session_quote_delete(&mut self, wday: u32, pos: u32) -> u32 {
        0
    }

    fn session_quote_clear(&mut self, wday: u32) -> u32 {
        0
    }

    fn session_quote_shift(&mut self, wday: u32, pos: u32, shift: i32) -> u32 {
        0
    }

    fn session_quote_total(&self, wday: u32) -> u32 {
        0
    }

    fn session_quote_next(&self, wday: u32, pos: u32, session: &mut IMTConSymbolSession) -> u32 {
        0
    }

    fn session_trade_add(&mut self, wday: u32, symbol: &mut IMTConSymbolSession) -> u32 {
        0
    }

    fn session_trade_update(&mut self, wday: u32, pos: u32, session: &IMTConSymbolSession) -> u32 {
        0
    }

    fn session_trade_delete(&mut self, wday: u32, pos: u32) -> u32 {
        0
    }

    fn session_trade_clear(&mut self, wday: u32) -> u32 {
        0
    }

    fn session_trade_shift(&mut self, wday: u32, pos: u32, shift: i32) -> u32 {
        0
    }

    fn session_trade_total(&self, wday: u32) -> u32 {
        0
    }

    fn session_trade_next(&self, wday: u32, pos: u32, session: &mut IMTConSymbolSession) -> u32 {
        0
    }

    fn re_flags(&self) -> u32 {
        0
    }

    fn re_flags1(&mut self, flags: u32) -> u32 {
        0
    }

    fn re_timeout(&self) -> u32 {
        0
    }

    fn re_timeout1(&mut self, timeout: u32) -> u32 {
        0
    }

    fn ie_check_mode(&self) -> u32 {
        0
    }

    fn ie_check_mode1(&mut self, mode: u32) -> u32 {
        0
    }

    fn ie_timeout(&self) -> u32 {
        0
    }

    fn ie_timeout1(&mut self, timeout: u32) -> u32 {
        0
    }

    fn ie_slip_profit(&self) -> u32 {
        0
    }

    fn ie_slip_profit1(&mut self, slippage: u32) -> u32 {
        0
    }

    fn ie_slip_losing(&self) -> u32 {
        0
    }

    fn ie_slip_losing1(&mut self, slippage: u32) -> u32 {
        0
    }

    fn ie_volume_max(&self) -> u64 {
        0
    }

    fn ie_volume_max1(&mut self, volume: u64) -> u32 {
        0
    }

    fn price_settle(&self) -> f64 {
        0.0
    }

    fn price_settle1(&mut self, price: f64) -> u32 {
        0
    }

    fn price_limit_max(&self) -> f64 {
        0.0
    }

    fn price_limit_max1(&mut self, price: f64) -> u32 {
        0
    }

    fn price_limit_min(&self) -> f64 {
        0.0
    }

    fn price_limit_min1(&mut self, price: f64) -> u32 {
        0
    }

    fn trade_flags(&self) -> u64 {
        0
    }

    fn trade_flags1(&mut self, flags: u64) -> u32 {
        0
    }

    fn order_flags(&self) -> u32 {
        0
    }

    fn order_flags1(&mut self, flags: u32) -> u32 {
        0
    }

    fn margin_rate_initial(&self, type_: u32) -> f64 {
        0.0
    }

    fn margin_rate_initial1(&mut self, type_: u32, margin_rate: f64) -> u32 {
        0
    }

    fn margin_rate_maintenance(&self, type_: u32) -> f64 {
        0.0
    }

    fn margin_rate_maintenance1(&mut self, type_: u32, margin_rate: f64) -> u32 {
        0
    }

    fn options_mode(&self) -> u32 {
        0
    }

    fn options_mode1(&mut self, mode: u32) -> u32 {
        0
    }

    fn price_strike(&self) -> f64 {
        0.0
    }

    fn price_strike1(&mut self, price: f64) -> u32 {
        0
    }

    fn margin_rate_liquidity(&self) -> f64 {
        0.0
    }

    fn margin_rate_liquidity1(&mut self, margin_rate: f64) -> u32 {
        0
    }

    fn face_value(&self) -> f64 {
        0.0
    }

    fn face_value1(&mut self, value: f64) -> u32 {
        0
    }

    fn accrued_interest(&self) -> f64 {
        0.0
    }

    fn accrued_interest1(&mut self, interest: f64) -> u32 {
        0
    }

    fn splice_type(&self) -> u32 {
        0
    }

    fn splice_type1(&mut self, type_: u32) -> u32 {
        0
    }

    fn splice_time_type(&self) -> u32 {
        0
    }

    fn splice_time_type1(&mut self, time_type: u32) -> u32 {
        0
    }

    fn splice_time_days(&self) -> u32 {
        0
    }

    fn splice_time_days1(&mut self, days: u32) -> u32 {
        0
    }

    fn margin_hedged(&self) -> f64 {
        0.0
    }

    fn margin_hedged1(&mut self, margin: f64) -> u32 {
        0
    }

    fn margin_rate_currency(&self) -> f64 {
        0.0
    }

    fn margin_rate_currency1(&mut self, margin_rate: f64) -> u32 {
        0
    }

    fn filter_gap(&self) -> u32 {
        0
    }

    fn filter_gap1(&mut self, gap: u32) -> u32 {
        0
    }

    fn filter_gap_ticks(&self) -> u32 {
        0
    }

    fn filter_gap_ticks1(&mut self, ticks: u32) -> u32 {
        0
    }

    fn chart_mode(&self) -> u32 {
        0
    }

    fn chart_mode1(&mut self, mode: u32) -> u32 {
        0
    }

    fn currency_base_digits_set(&mut self, digits: u32) -> u32 {
        0
    }

    fn currency_profit_digits_set(&mut self, digits: u32) -> u32 {
        0
    }

    fn currency_margin_digits_set(&mut self, digits: u32) -> u32 {
        0
    }

    fn ie_flags(&self) -> u32 {
        0
    }

    fn ie_flags1(&mut self, flags: u32) -> u32 {
        0
    }

    fn volume_min_ext(&self) -> u64 {
        0
    }

    fn volume_min_ext1(&mut self, volume: u64) -> u32 {
        0
    }

    fn volume_max_ext(&self) -> u64 {
        0
    }

    fn volume_max_ext1(&mut self, volume: u64) -> u32 {
        0
    }

    fn volume_step_ext(&self) -> u64 {
        0
    }

    fn volume_step_ext1(&mut self, volume: u64) -> u32 {
        0
    }

    fn volume_limit_ext(&self) -> u64 {
        0
    }

    fn volume_limit_ext1(&mut self, volume: u64) -> u32 {
        0
    }

    fn ie_volume_max_ext(&self) -> u64 {
        0
    }

    fn ie_volume_max_ext1(&mut self, volume: u64) -> u32 {
        0
    }

    fn category(&self) -> *const u16 {
        std::ptr::null()
    }

    fn category1(&mut self, category: *const u16) -> u32 {
        0
    }

    fn exchange(&self) -> *const u16 {
        std::ptr::null()
    }

    fn exchange1(&mut self, exchange: *const u16) -> u32 {
        0
    }

    fn cfi(&self) -> *const u16 {
        std::ptr::null()
    }

    fn cfi1(&mut self, cfi: *const u16) -> u32 {
        0
    }

    fn sector(&self) -> u32 {
        0
    }

    fn sector1(&mut self, sector: u32) -> u32 {
        0
    }

    fn industry(&self) -> u32 {
        0
    }

    fn industry1(&mut self, industry: u32) -> u32 {
        0
    }

    fn country(&self) -> *const u16 {
        std::ptr::null()
    }

    fn country1(&mut self, country: *const u16) -> u32 {
        0
    }

    fn subscriptions_delay(&self) -> u32 {
        0
    }

    fn subscriptions_delay1(&mut self, delay: u32) -> u32 {
        0
    }

    fn swap_year_days(&self) -> u32 {
        0
    }

    fn swap_year_days1(&mut self, days: u32) -> u32 {
        0
    }

    fn swap_flags(&self) -> u32 {
        0
    }

    fn swap_flags1(&mut self, flags: u32) -> u32 {
        0
    }

    fn swap_rate_sunday(&self) -> f64 {
        0.0
    }

    fn swap_rate_sunday1(&mut self, rate: f64) -> u32 {
        0
    }

    fn swap_rate_monday(&self) -> f64 {
        0.0
    }

    fn swap_rate_monday1(&mut self, rate: f64) -> u32 {
        0
    }

    fn swap_rate_tuesday(&self) -> f64 {
        0.0
    }

    fn swap_rate_tuesday1(&mut self, rate: f64) -> u32 {
        0
    }

    fn swap_rate_wednesday(&self) -> f64 {
        0.0
    }

    fn swap_rate_wednesday1(&mut self, rate: f64) -> u32 {
        0
    }

    fn swap_rate_thursday(&self) -> f64 {
        0.0
    }

    fn swap_rate_thursday1(&mut self, rate: f64) -> u32 {
        0
    }

    fn swap_rate_friday(&self) -> f64 {
        0.0
    }

    fn swap_rate_friday1(&mut self, rate: f64) -> u32 {
        0
    }

    fn swap_rate_saturday(&self) -> f64 {
        0.0
    }

    fn swap_rate_saturday1(&mut self, rate: f64) -> u32 {
        0
    }
}
