use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTConSymbol_Release(this: *mut IMTConSymbol) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTConSymbol>());
}

unsafe extern "C" fn IMTConSymbol_Assign(this: *mut IMTConSymbol, symbol: *const IMTConSymbol) -> MTAPIRES {
    (*(*this).impl_ptr).assign(&*symbol)
}

unsafe extern "C" fn IMTConSymbol_Clear(this: *mut IMTConSymbol) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTConSymbol_Symbol1(this: *mut IMTConSymbol, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol1(symbol)
}

unsafe extern "C" fn IMTConSymbol_Symbol(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).symbol()
}

unsafe extern "C" fn IMTConSymbol_Path(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).path()
}

unsafe extern "C" fn IMTConSymbol_Path1(this: *mut IMTConSymbol, path: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).path1(path)
}

unsafe extern "C" fn IMTConSymbol_ISIN(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).isin()
}

unsafe extern "C" fn IMTConSymbol_ISIN1(this: *mut IMTConSymbol, isin: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).isin1(isin)
}

unsafe extern "C" fn IMTConSymbol_Description(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).description()
}

unsafe extern "C" fn IMTConSymbol_Description1(this: *mut IMTConSymbol, descr: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).description1(descr)
}

unsafe extern "C" fn IMTConSymbol_International1(this: *mut IMTConSymbol, intern: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).international1(intern)
}

unsafe extern "C" fn IMTConSymbol_International(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).international()
}

unsafe extern "C" fn IMTConSymbol_Basis(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).basis()
}

unsafe extern "C" fn IMTConSymbol_Basis1(this: *mut IMTConSymbol, basis: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).basis1(basis)
}

unsafe extern "C" fn IMTConSymbol_Source(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).source()
}

unsafe extern "C" fn IMTConSymbol_Source1(this: *mut IMTConSymbol, source: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).source1(source)
}

unsafe extern "C" fn IMTConSymbol_Page(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).page()
}

unsafe extern "C" fn IMTConSymbol_Page1(this: *mut IMTConSymbol, page: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).page1(page)
}

unsafe extern "C" fn IMTConSymbol_CurrencyBase(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).currency_base()
}

unsafe extern "C" fn IMTConSymbol_CurrencyBase1(this: *mut IMTConSymbol, currency: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).currency_base1(currency)
}

unsafe extern "C" fn IMTConSymbol_CurrencyBaseDigits(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).currency_base_digits()
}

unsafe extern "C" fn IMTConSymbol_CurrencyProfit(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).currency_profit()
}

unsafe extern "C" fn IMTConSymbol_CurrencyProfit1(this: *mut IMTConSymbol, currency: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).currency_profit1(currency)
}

unsafe extern "C" fn IMTConSymbol_CurrencyProfitDigits(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).currency_profit_digits()
}

unsafe extern "C" fn IMTConSymbol_CurrencyMargin(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).currency_margin()
}

unsafe extern "C" fn IMTConSymbol_CurrencyMargin1(this: *mut IMTConSymbol, currency: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).currency_margin1(currency)
}

unsafe extern "C" fn IMTConSymbol_CurrencyMarginDigits(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).currency_margin_digits()
}

unsafe extern "C" fn IMTConSymbol_Color(this: *const IMTConSymbol) -> COLORREF {
    (*(*this).impl_ptr).color()
}

unsafe extern "C" fn IMTConSymbol_Color1(this: *mut IMTConSymbol, color: COLORREF) -> MTAPIRES {
    (*(*this).impl_ptr).color1(color)
}

unsafe extern "C" fn IMTConSymbol_ColorBackground(this: *const IMTConSymbol) -> COLORREF {
    (*(*this).impl_ptr).color_background()
}

unsafe extern "C" fn IMTConSymbol_ColorBackground1(this: *mut IMTConSymbol, color: COLORREF) -> MTAPIRES {
    (*(*this).impl_ptr).color_background1(color)
}

unsafe extern "C" fn IMTConSymbol_Digits(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).digits()
}

unsafe extern "C" fn IMTConSymbol_Digits1(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits1(digits)
}

unsafe extern "C" fn IMTConSymbol_Point(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).point()
}

unsafe extern "C" fn IMTConSymbol_Multiply(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).multiply()
}

unsafe extern "C" fn IMTConSymbol_TickFlags(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).tick_flags()
}

unsafe extern "C" fn IMTConSymbol_TickFlags1(this: *mut IMTConSymbol, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).tick_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_TickBookDepth(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).tick_book_depth()
}

unsafe extern "C" fn IMTConSymbol_TickBookDepth1(this: *mut IMTConSymbol, depth: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).tick_book_depth1(depth)
}

unsafe extern "C" fn IMTConSymbol_FilterSoft(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_soft()
}

unsafe extern "C" fn IMTConSymbol_FilterSoft1(this: *mut IMTConSymbol, filter: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_soft1(filter)
}

unsafe extern "C" fn IMTConSymbol_FilterSoftTicks(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_soft_ticks()
}

unsafe extern "C" fn IMTConSymbol_FilterSoftTicks1(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_soft_ticks1(ticks)
}

unsafe extern "C" fn IMTConSymbol_FilterHard(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_hard()
}

unsafe extern "C" fn IMTConSymbol_FilterHard1(this: *mut IMTConSymbol, filter: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_hard1(filter)
}

unsafe extern "C" fn IMTConSymbol_FilterHardTicks(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_hard_ticks()
}

unsafe extern "C" fn IMTConSymbol_FilterHardTicks1(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_hard_ticks1(ticks)
}

unsafe extern "C" fn IMTConSymbol_FilterDiscard(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_discard()
}

unsafe extern "C" fn IMTConSymbol_FilterDiscard1(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_discard1(ticks)
}

unsafe extern "C" fn IMTConSymbol_FilterSpreadMax(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_spread_max()
}

unsafe extern "C" fn IMTConSymbol_FilterSpreadMax1(this: *mut IMTConSymbol, spread: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_spread_max1(spread)
}

unsafe extern "C" fn IMTConSymbol_FilterSpreadMin(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_spread_min()
}

unsafe extern "C" fn IMTConSymbol_FilterSpreadMin1(this: *mut IMTConSymbol, spread: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_spread_min1(spread)
}

unsafe extern "C" fn IMTConSymbol_TradeMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).trade_mode()
}

unsafe extern "C" fn IMTConSymbol_TradeMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).trade_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_CalcMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).calc_mode()
}

unsafe extern "C" fn IMTConSymbol_CalcMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).calc_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_ExecMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).exec_mode()
}

unsafe extern "C" fn IMTConSymbol_ExecMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).exec_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_GTCMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).gtc_mode()
}

unsafe extern "C" fn IMTConSymbol_GTCMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).gtc_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_FillFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).fill_flags()
}

unsafe extern "C" fn IMTConSymbol_FillFlags1(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).fill_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_ExpirFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).expir_flags()
}

unsafe extern "C" fn IMTConSymbol_ExpirFlags1(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).expir_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_Spread(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).spread()
}

unsafe extern "C" fn IMTConSymbol_Spread1(this: *mut IMTConSymbol, spread: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).spread1(spread)
}

unsafe extern "C" fn IMTConSymbol_SpreadBalance(this: *const IMTConSymbol) -> INT {
    (*(*this).impl_ptr).spread_balance()
}

unsafe extern "C" fn IMTConSymbol_SpreadBalance1(this: *mut IMTConSymbol, spread: INT) -> MTAPIRES {
    (*(*this).impl_ptr).spread_balance1(spread)
}

unsafe extern "C" fn IMTConSymbol_SpreadDiff(this: *const IMTConSymbol) -> INT {
    (*(*this).impl_ptr).spread_diff()
}

unsafe extern "C" fn IMTConSymbol_SpreadDiff1(this: *mut IMTConSymbol, diff: INT) -> MTAPIRES {
    (*(*this).impl_ptr).spread_diff1(diff)
}

unsafe extern "C" fn IMTConSymbol_SpreadDiffBalance(this: *const IMTConSymbol) -> INT {
    (*(*this).impl_ptr).spread_diff_balance()
}

unsafe extern "C" fn IMTConSymbol_SpreadDiffBalance1(this: *mut IMTConSymbol, spread: INT) -> MTAPIRES {
    (*(*this).impl_ptr).spread_diff_balance1(spread)
}

unsafe extern "C" fn IMTConSymbol_TickValue(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).tick_value()
}

unsafe extern "C" fn IMTConSymbol_TickValue1(this: *mut IMTConSymbol, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).tick_value1(value)
}

unsafe extern "C" fn IMTConSymbol_TickSize(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).tick_size()
}

unsafe extern "C" fn IMTConSymbol_TickSize1(this: *mut IMTConSymbol, size: f64) -> MTAPIRES {
    (*(*this).impl_ptr).tick_size1(size)
}

unsafe extern "C" fn IMTConSymbol_ContractSize(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).contract_size()
}

unsafe extern "C" fn IMTConSymbol_ContractSize1(this: *mut IMTConSymbol, size: f64) -> MTAPIRES {
    (*(*this).impl_ptr).contract_size1(size)
}

unsafe extern "C" fn IMTConSymbol_StopsLevel(this: *const IMTConSymbol) -> INT {
    (*(*this).impl_ptr).stops_level()
}

unsafe extern "C" fn IMTConSymbol_StopsLevel1(this: *mut IMTConSymbol, level: INT) -> MTAPIRES {
    (*(*this).impl_ptr).stops_level1(level)
}

unsafe extern "C" fn IMTConSymbol_FreezeLevel(this: *const IMTConSymbol) -> INT {
    (*(*this).impl_ptr).freeze_level()
}

unsafe extern "C" fn IMTConSymbol_FreezeLevel1(this: *mut IMTConSymbol, level: INT) -> MTAPIRES {
    (*(*this).impl_ptr).freeze_level1(level)
}

unsafe extern "C" fn IMTConSymbol_QuotesTimeout(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).quotes_timeout()
}

unsafe extern "C" fn IMTConSymbol_QuotesTimeout1(this: *mut IMTConSymbol, timeout: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).quotes_timeout1(timeout)
}

unsafe extern "C" fn IMTConSymbol_VolumeMin(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_min()
}

unsafe extern "C" fn IMTConSymbol_VolumeMin1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_min1(volume)
}

unsafe extern "C" fn IMTConSymbol_VolumeMax(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_max()
}

unsafe extern "C" fn IMTConSymbol_VolumeMax1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_max1(volume)
}

unsafe extern "C" fn IMTConSymbol_VolumeStep(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_step()
}

unsafe extern "C" fn IMTConSymbol_VolumeStep1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_step1(volume)
}

unsafe extern "C" fn IMTConSymbol_VolumeLimit(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_limit()
}

unsafe extern "C" fn IMTConSymbol_VolumeLimit1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_limit1(volume)
}

unsafe extern "C" fn IMTConSymbol_MarginFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).margin_flags()
}

unsafe extern "C" fn IMTConSymbol_MarginFlags1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).margin_flags1(mode)
}

unsafe extern "C" fn IMTConSymbol_MarginInitial(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_initial()
}

unsafe extern "C" fn IMTConSymbol_MarginInitial1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_initial1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginMaintenance(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_maintenance()
}

unsafe extern "C" fn IMTConSymbol_MarginMaintenance1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_maintenance1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginLong(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_long()
}

unsafe extern "C" fn IMTConSymbol_MarginLong1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_long1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginShort(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_short()
}

unsafe extern "C" fn IMTConSymbol_MarginShort1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_short1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginLimit(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_limit()
}

unsafe extern "C" fn IMTConSymbol_MarginLimit1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_limit1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginStop(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_stop()
}

unsafe extern "C" fn IMTConSymbol_MarginStop1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_stop1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginStopLimit(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_stop_limit()
}

unsafe extern "C" fn IMTConSymbol_MarginStopLimit1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_stop_limit1(margin)
}

unsafe extern "C" fn IMTConSymbol_SwapMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).swap_mode()
}

unsafe extern "C" fn IMTConSymbol_SwapMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).swap_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_SwapLong(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_long()
}

unsafe extern "C" fn IMTConSymbol_SwapLong1(this: *mut IMTConSymbol, swap: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_long1(swap)
}

unsafe extern "C" fn IMTConSymbol_SwapShort(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_short()
}

unsafe extern "C" fn IMTConSymbol_SwapShort1(this: *mut IMTConSymbol, swap: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_short1(swap)
}

unsafe extern "C" fn IMTConSymbol_Swap3Day(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).swap3_day()
}

unsafe extern "C" fn IMTConSymbol_Swap3Day1(this: *mut IMTConSymbol, day: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).swap3_day1(day)
}

unsafe extern "C" fn IMTConSymbol_TimeStart(this: *const IMTConSymbol) -> INT64 {
    (*(*this).impl_ptr).time_start()
}

unsafe extern "C" fn IMTConSymbol_TimeStart1(this: *mut IMTConSymbol, start: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_start1(start)
}

unsafe extern "C" fn IMTConSymbol_TimeExpiration(this: *const IMTConSymbol) -> INT64 {
    (*(*this).impl_ptr).time_expiration()
}

unsafe extern "C" fn IMTConSymbol_TimeExpiration1(this: *mut IMTConSymbol, expiration: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_expiration1(expiration)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteAdd(this: *mut IMTConSymbol, wday: UINT, symbol: *mut IMTConSymbolSession) -> MTAPIRES {
    (*(*this).impl_ptr).session_quote_add(wday, &mut *symbol)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteUpdate(this: *mut IMTConSymbol, wday: UINT, pos: UINT, session: *const IMTConSymbolSession) -> MTAPIRES {
    (*(*this).impl_ptr).session_quote_update(wday, pos, &*session)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteDelete(this: *mut IMTConSymbol, wday: UINT, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).session_quote_delete(wday, pos)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteClear(this: *mut IMTConSymbol, wday: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).session_quote_clear(wday)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteShift(this: *mut IMTConSymbol, wday: UINT, pos: UINT, shift: ::std::os::raw::c_int) -> MTAPIRES {
    (*(*this).impl_ptr).session_quote_shift(wday, pos, shift)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteTotal(this: *const IMTConSymbol, wday: UINT) -> UINT {
    (*(*this).impl_ptr).session_quote_total(wday)
}

unsafe extern "C" fn IMTConSymbol_SessionQuoteNext(this: *const IMTConSymbol, wday: UINT, pos: UINT, session: *mut IMTConSymbolSession) -> MTAPIRES {
    (*(*this).impl_ptr).session_quote_next(wday, pos, &mut *session)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeAdd(this: *mut IMTConSymbol, wday: UINT, symbol: *mut IMTConSymbolSession) -> MTAPIRES {
    (*(*this).impl_ptr).session_trade_add(wday, &mut *symbol)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeUpdate(this: *mut IMTConSymbol, wday: UINT, pos: UINT, session: *const IMTConSymbolSession) -> MTAPIRES {
    (*(*this).impl_ptr).session_trade_update(wday, pos, &*session)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeDelete(this: *mut IMTConSymbol, wday: UINT, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).session_trade_delete(wday, pos)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeClear(this: *mut IMTConSymbol, wday: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).session_trade_clear(wday)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeShift(this: *mut IMTConSymbol, wday: UINT, pos: UINT, shift: ::std::os::raw::c_int) -> MTAPIRES {
    (*(*this).impl_ptr).session_trade_shift(wday, pos, shift)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeTotal(this: *const IMTConSymbol, wday: UINT) -> UINT {
    (*(*this).impl_ptr).session_trade_total(wday)
}

unsafe extern "C" fn IMTConSymbol_SessionTradeNext(this: *const IMTConSymbol, wday: UINT, pos: UINT, session: *mut IMTConSymbolSession) -> MTAPIRES {
    (*(*this).impl_ptr).session_trade_next(wday, pos, &mut *session)
}

unsafe extern "C" fn IMTConSymbol_REFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).re_flags()
}

unsafe extern "C" fn IMTConSymbol_REFlags1(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).re_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_RETimeout(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).re_timeout()
}

unsafe extern "C" fn IMTConSymbol_RETimeout1(this: *mut IMTConSymbol, timeout: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).re_timeout1(timeout)
}

unsafe extern "C" fn IMTConSymbol_IECheckMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).ie_check_mode()
}

unsafe extern "C" fn IMTConSymbol_IECheckMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).ie_check_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_IETimeout(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).ie_timeout()
}

unsafe extern "C" fn IMTConSymbol_IETimeout1(this: *mut IMTConSymbol, timeout: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).ie_timeout1(timeout)
}

unsafe extern "C" fn IMTConSymbol_IESlipProfit(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).ie_slip_profit()
}

unsafe extern "C" fn IMTConSymbol_IESlipProfit1(this: *mut IMTConSymbol, slippage: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).ie_slip_profit1(slippage)
}

unsafe extern "C" fn IMTConSymbol_IESlipLosing(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).ie_slip_losing()
}

unsafe extern "C" fn IMTConSymbol_IESlipLosing1(this: *mut IMTConSymbol, slippage: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).ie_slip_losing1(slippage)
}

unsafe extern "C" fn IMTConSymbol_IEVolumeMax(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).ie_volume_max()
}

unsafe extern "C" fn IMTConSymbol_IEVolumeMax1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).ie_volume_max1(volume)
}

unsafe extern "C" fn IMTConSymbol_PriceSettle(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).price_settle()
}

unsafe extern "C" fn IMTConSymbol_PriceSettle1(this: *mut IMTConSymbol, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_settle1(price)
}

unsafe extern "C" fn IMTConSymbol_PriceLimitMax(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).price_limit_max()
}

unsafe extern "C" fn IMTConSymbol_PriceLimitMax1(this: *mut IMTConSymbol, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_limit_max1(price)
}

unsafe extern "C" fn IMTConSymbol_PriceLimitMin(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).price_limit_min()
}

unsafe extern "C" fn IMTConSymbol_PriceLimitMin1(this: *mut IMTConSymbol, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_limit_min1(price)
}

unsafe extern "C" fn IMTConSymbol_TradeFlags(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).trade_flags()
}

unsafe extern "C" fn IMTConSymbol_TradeFlags1(this: *mut IMTConSymbol, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).trade_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_OrderFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).order_flags()
}

unsafe extern "C" fn IMTConSymbol_OrderFlags1(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).order_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_MarginRateInitial(this: *const IMTConSymbol, type_: UINT) -> f64 {
    (*(*this).impl_ptr).margin_rate_initial(type_)
}

unsafe extern "C" fn IMTConSymbol_MarginRateInitial1(this: *mut IMTConSymbol, type_: UINT, margin_rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_rate_initial1(type_, margin_rate)
}

unsafe extern "C" fn IMTConSymbol_MarginRateMaintenance(this: *const IMTConSymbol, type_: UINT) -> f64 {
    (*(*this).impl_ptr).margin_rate_maintenance(type_)
}

unsafe extern "C" fn IMTConSymbol_MarginRateMaintenance1(this: *mut IMTConSymbol, type_: UINT, margin_rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_rate_maintenance1(type_, margin_rate)
}

unsafe extern "C" fn IMTConSymbol_OptionsMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).options_mode()
}

unsafe extern "C" fn IMTConSymbol_OptionsMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).options_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_PriceStrike(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).price_strike()
}

unsafe extern "C" fn IMTConSymbol_PriceStrike1(this: *mut IMTConSymbol, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_strike1(price)
}

unsafe extern "C" fn IMTConSymbol_MarginRateLiquidity(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_rate_liquidity()
}

unsafe extern "C" fn IMTConSymbol_MarginRateLiquidity1(this: *mut IMTConSymbol, margin_rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_rate_liquidity1(margin_rate)
}

unsafe extern "C" fn IMTConSymbol_FaceValue(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).face_value()
}

unsafe extern "C" fn IMTConSymbol_FaceValue1(this: *mut IMTConSymbol, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).face_value1(value)
}

unsafe extern "C" fn IMTConSymbol_AccruedInterest(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).accrued_interest()
}

unsafe extern "C" fn IMTConSymbol_AccruedInterest1(this: *mut IMTConSymbol, interest: f64) -> MTAPIRES {
    (*(*this).impl_ptr).accrued_interest1(interest)
}

unsafe extern "C" fn IMTConSymbol_SpliceType(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).splice_type()
}

unsafe extern "C" fn IMTConSymbol_SpliceType1(this: *mut IMTConSymbol, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).splice_type1(type_)
}

unsafe extern "C" fn IMTConSymbol_SpliceTimeType(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).splice_time_type()
}

unsafe extern "C" fn IMTConSymbol_SpliceTimeType1(this: *mut IMTConSymbol, time_type: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).splice_time_type1(time_type)
}

unsafe extern "C" fn IMTConSymbol_SpliceTimeDays(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).splice_time_days()
}

unsafe extern "C" fn IMTConSymbol_SpliceTimeDays1(this: *mut IMTConSymbol, days: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).splice_time_days1(days)
}

unsafe extern "C" fn IMTConSymbol_MarginHedged(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_hedged()
}

unsafe extern "C" fn IMTConSymbol_MarginHedged1(this: *mut IMTConSymbol, margin: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_hedged1(margin)
}

unsafe extern "C" fn IMTConSymbol_MarginRateCurrency(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).margin_rate_currency()
}

unsafe extern "C" fn IMTConSymbol_MarginRateCurrency1(this: *mut IMTConSymbol, margin_rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).margin_rate_currency1(margin_rate)
}

unsafe extern "C" fn IMTConSymbol_FilterGap(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_gap()
}

unsafe extern "C" fn IMTConSymbol_FilterGap1(this: *mut IMTConSymbol, gap: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_gap1(gap)
}

unsafe extern "C" fn IMTConSymbol_FilterGapTicks(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).filter_gap_ticks()
}

unsafe extern "C" fn IMTConSymbol_FilterGapTicks1(this: *mut IMTConSymbol, ticks: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).filter_gap_ticks1(ticks)
}

unsafe extern "C" fn IMTConSymbol_ChartMode(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).chart_mode()
}

unsafe extern "C" fn IMTConSymbol_ChartMode1(this: *mut IMTConSymbol, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).chart_mode1(mode)
}

unsafe extern "C" fn IMTConSymbol_CurrencyBaseDigitsSet(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).currency_base_digits_set(digits)
}

unsafe extern "C" fn IMTConSymbol_CurrencyProfitDigitsSet(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).currency_profit_digits_set(digits)
}

unsafe extern "C" fn IMTConSymbol_CurrencyMarginDigitsSet(this: *mut IMTConSymbol, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).currency_margin_digits_set(digits)
}

unsafe extern "C" fn IMTConSymbol_IEFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).ie_flags()
}

unsafe extern "C" fn IMTConSymbol_IEFlags1(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).ie_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_VolumeMinExt(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_min_ext()
}

unsafe extern "C" fn IMTConSymbol_VolumeMinExt1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_min_ext1(volume)
}

unsafe extern "C" fn IMTConSymbol_VolumeMaxExt(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_max_ext()
}

unsafe extern "C" fn IMTConSymbol_VolumeMaxExt1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_max_ext1(volume)
}

unsafe extern "C" fn IMTConSymbol_VolumeStepExt(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_step_ext()
}

unsafe extern "C" fn IMTConSymbol_VolumeStepExt1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_step_ext1(volume)
}

unsafe extern "C" fn IMTConSymbol_VolumeLimitExt(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).volume_limit_ext()
}

unsafe extern "C" fn IMTConSymbol_VolumeLimitExt1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_limit_ext1(volume)
}

unsafe extern "C" fn IMTConSymbol_IEVolumeMaxExt(this: *const IMTConSymbol) -> UINT64 {
    (*(*this).impl_ptr).ie_volume_max_ext()
}

unsafe extern "C" fn IMTConSymbol_IEVolumeMaxExt1(this: *mut IMTConSymbol, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).ie_volume_max_ext1(volume)
}

unsafe extern "C" fn IMTConSymbol_Category(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).category()
}

unsafe extern "C" fn IMTConSymbol_Category1(this: *mut IMTConSymbol, category: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).category1(category)
}

unsafe extern "C" fn IMTConSymbol_Exchange(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).exchange()
}

unsafe extern "C" fn IMTConSymbol_Exchange1(this: *mut IMTConSymbol, exchange: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).exchange1(exchange)
}

unsafe extern "C" fn IMTConSymbol_CFI(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).cfi()
}

unsafe extern "C" fn IMTConSymbol_CFI1(this: *mut IMTConSymbol, cfi: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).cfi1(cfi)
}

unsafe extern "C" fn IMTConSymbol_Sector(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).sector()
}

unsafe extern "C" fn IMTConSymbol_Sector1(this: *mut IMTConSymbol, sector: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).sector1(sector)
}

unsafe extern "C" fn IMTConSymbol_Industry(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).industry()
}

unsafe extern "C" fn IMTConSymbol_Industry1(this: *mut IMTConSymbol, industry: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).industry1(industry)
}

unsafe extern "C" fn IMTConSymbol_Country(this: *const IMTConSymbol) -> LPCWSTR {
    (*(*this).impl_ptr).country()
}

unsafe extern "C" fn IMTConSymbol_Country1(this: *mut IMTConSymbol, country: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).country1(country)
}

unsafe extern "C" fn IMTConSymbol_SubscriptionsDelay(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).subscriptions_delay()
}

unsafe extern "C" fn IMTConSymbol_SubscriptionsDelay1(this: *mut IMTConSymbol, delay: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).subscriptions_delay1(delay)
}

unsafe extern "C" fn IMTConSymbol_SwapYearDays(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).swap_year_days()
}

unsafe extern "C" fn IMTConSymbol_SwapYearDays1(this: *mut IMTConSymbol, days: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).swap_year_days1(days)
}

unsafe extern "C" fn IMTConSymbol_SwapFlags(this: *const IMTConSymbol) -> UINT {
    (*(*this).impl_ptr).swap_flags()
}

unsafe extern "C" fn IMTConSymbol_SwapFlags1(this: *mut IMTConSymbol, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).swap_flags1(flags)
}

unsafe extern "C" fn IMTConSymbol_SwapRateSunday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_sunday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateSunday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_sunday1(rate)
}

unsafe extern "C" fn IMTConSymbol_SwapRateMonday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_monday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateMonday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_monday1(rate)
}

unsafe extern "C" fn IMTConSymbol_SwapRateTuesday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_tuesday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateTuesday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_tuesday1(rate)
}

unsafe extern "C" fn IMTConSymbol_SwapRateWednesday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_wednesday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateWednesday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_wednesday1(rate)
}

unsafe extern "C" fn IMTConSymbol_SwapRateThursday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_thursday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateThursday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_thursday1(rate)
}

unsafe extern "C" fn IMTConSymbol_SwapRateFriday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_friday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateFriday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_friday1(rate)
}

unsafe extern "C" fn IMTConSymbol_SwapRateSaturday(this: *const IMTConSymbol) -> f64 {
    (*(*this).impl_ptr).swap_rate_saturday()
}

unsafe extern "C" fn IMTConSymbol_SwapRateSaturday1(this: *mut IMTConSymbol, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).swap_rate_saturday1(rate)
}

pub const fn new() -> IMTConSymbol__bindgen_vtable {
    IMTConSymbol__bindgen_vtable {
        IMTConSymbol_Release,
        IMTConSymbol_Assign,
        IMTConSymbol_Clear,
        IMTConSymbol_Symbol,
        IMTConSymbol_Symbol1,
        IMTConSymbol_Path,
        IMTConSymbol_Path1,
        IMTConSymbol_ISIN,
        IMTConSymbol_ISIN1,
        IMTConSymbol_Description1,
        IMTConSymbol_Description,
        IMTConSymbol_International1,
        IMTConSymbol_International,
        IMTConSymbol_Basis1,
        IMTConSymbol_Basis,
        IMTConSymbol_Source1,
        IMTConSymbol_Source,
        IMTConSymbol_Page1,
        IMTConSymbol_Page,
        IMTConSymbol_CurrencyBase1,
        IMTConSymbol_CurrencyBase,
        IMTConSymbol_CurrencyBaseDigits,
        IMTConSymbol_CurrencyProfit1,
        IMTConSymbol_CurrencyProfit,
        IMTConSymbol_CurrencyProfitDigits,
        IMTConSymbol_CurrencyMargin,
        IMTConSymbol_CurrencyMargin1,
        IMTConSymbol_CurrencyMarginDigits,
        IMTConSymbol_Color,
        IMTConSymbol_Color1,
        IMTConSymbol_ColorBackground,
        IMTConSymbol_ColorBackground1,
        IMTConSymbol_Digits,
        IMTConSymbol_Digits1,
        IMTConSymbol_Point,
        IMTConSymbol_Multiply,
        IMTConSymbol_TickFlags,
        IMTConSymbol_TickFlags1,
        IMTConSymbol_TickBookDepth,
        IMTConSymbol_TickBookDepth1,
        IMTConSymbol_FilterSoft,
        IMTConSymbol_FilterSoft1,
        IMTConSymbol_FilterSoftTicks,
        IMTConSymbol_FilterSoftTicks1,
        IMTConSymbol_FilterHard,
        IMTConSymbol_FilterHard1,
        IMTConSymbol_FilterHardTicks,
        IMTConSymbol_FilterHardTicks1,
        IMTConSymbol_FilterDiscard,
        IMTConSymbol_FilterDiscard1,
        IMTConSymbol_FilterSpreadMax,
        IMTConSymbol_FilterSpreadMax1,
        IMTConSymbol_FilterSpreadMin,
        IMTConSymbol_FilterSpreadMin1,
        IMTConSymbol_TradeMode,
        IMTConSymbol_TradeMode1,
        IMTConSymbol_CalcMode,
        IMTConSymbol_CalcMode1,
        IMTConSymbol_ExecMode,
        IMTConSymbol_ExecMode1,
        IMTConSymbol_GTCMode,
        IMTConSymbol_GTCMode1,
        IMTConSymbol_FillFlags,
        IMTConSymbol_FillFlags1,
        IMTConSymbol_ExpirFlags,
        IMTConSymbol_ExpirFlags1,
        IMTConSymbol_Spread,
        IMTConSymbol_Spread1,
        IMTConSymbol_SpreadBalance,
        IMTConSymbol_SpreadBalance1,
        IMTConSymbol_SpreadDiff,
        IMTConSymbol_SpreadDiff1,
        IMTConSymbol_SpreadDiffBalance,
        IMTConSymbol_SpreadDiffBalance1,
        IMTConSymbol_TickValue,
        IMTConSymbol_TickValue1,
        IMTConSymbol_TickSize,
        IMTConSymbol_TickSize1,
        IMTConSymbol_ContractSize,
        IMTConSymbol_ContractSize1,
        IMTConSymbol_StopsLevel,
        IMTConSymbol_StopsLevel1,
        IMTConSymbol_FreezeLevel,
        IMTConSymbol_FreezeLevel1,
        IMTConSymbol_QuotesTimeout,
        IMTConSymbol_QuotesTimeout1,
        IMTConSymbol_VolumeMin,
        IMTConSymbol_VolumeMin1,
        IMTConSymbol_VolumeMax,
        IMTConSymbol_VolumeMax1,
        IMTConSymbol_VolumeStep,
        IMTConSymbol_VolumeStep1,
        IMTConSymbol_VolumeLimit,
        IMTConSymbol_VolumeLimit1,
        IMTConSymbol_MarginFlags,
        IMTConSymbol_MarginFlags1,
        IMTConSymbol_MarginInitial,
        IMTConSymbol_MarginInitial1,
        IMTConSymbol_MarginMaintenance,
        IMTConSymbol_MarginMaintenance1,
        IMTConSymbol_MarginLong,
        IMTConSymbol_MarginLong1,
        IMTConSymbol_MarginShort,
        IMTConSymbol_MarginShort1,
        IMTConSymbol_MarginLimit,
        IMTConSymbol_MarginLimit1,
        IMTConSymbol_MarginStop,
        IMTConSymbol_MarginStop1,
        IMTConSymbol_MarginStopLimit,
        IMTConSymbol_MarginStopLimit1,
        IMTConSymbol_SwapMode,
        IMTConSymbol_SwapMode1,
        IMTConSymbol_SwapLong,
        IMTConSymbol_SwapLong1,
        IMTConSymbol_SwapShort,
        IMTConSymbol_SwapShort1,
        IMTConSymbol_Swap3Day,
        IMTConSymbol_Swap3Day1,
        IMTConSymbol_TimeStart,
        IMTConSymbol_TimeStart1,
        IMTConSymbol_TimeExpiration,
        IMTConSymbol_TimeExpiration1,
        IMTConSymbol_SessionQuoteAdd,
        IMTConSymbol_SessionQuoteUpdate,
        IMTConSymbol_SessionQuoteDelete,
        IMTConSymbol_SessionQuoteClear,
        IMTConSymbol_SessionQuoteShift,
        IMTConSymbol_SessionQuoteTotal,
        IMTConSymbol_SessionQuoteNext,
        IMTConSymbol_SessionTradeAdd,
        IMTConSymbol_SessionTradeUpdate,
        IMTConSymbol_SessionTradeDelete,
        IMTConSymbol_SessionTradeClear,
        IMTConSymbol_SessionTradeShift,
        IMTConSymbol_SessionTradeTotal,
        IMTConSymbol_SessionTradeNext,
        IMTConSymbol_REFlags,
        IMTConSymbol_REFlags1,
        IMTConSymbol_RETimeout,
        IMTConSymbol_RETimeout1,
        IMTConSymbol_IECheckMode,
        IMTConSymbol_IECheckMode1,
        IMTConSymbol_IETimeout,
        IMTConSymbol_IETimeout1,
        IMTConSymbol_IESlipProfit,
        IMTConSymbol_IESlipProfit1,
        IMTConSymbol_IESlipLosing,
        IMTConSymbol_IESlipLosing1,
        IMTConSymbol_IEVolumeMax,
        IMTConSymbol_IEVolumeMax1,
        IMTConSymbol_PriceSettle,
        IMTConSymbol_PriceSettle1,
        IMTConSymbol_PriceLimitMax,
        IMTConSymbol_PriceLimitMax1,
        IMTConSymbol_PriceLimitMin,
        IMTConSymbol_PriceLimitMin1,
        IMTConSymbol_TradeFlags,
        IMTConSymbol_TradeFlags1,
        IMTConSymbol_OrderFlags,
        IMTConSymbol_OrderFlags1,
        IMTConSymbol_MarginRateInitial,
        IMTConSymbol_MarginRateInitial1,
        IMTConSymbol_MarginRateMaintenance,
        IMTConSymbol_MarginRateMaintenance1,
        IMTConSymbol_OptionsMode,
        IMTConSymbol_OptionsMode1,
        IMTConSymbol_PriceStrike,
        IMTConSymbol_PriceStrike1,
        IMTConSymbol_MarginRateLiquidity,
        IMTConSymbol_MarginRateLiquidity1,
        IMTConSymbol_FaceValue,
        IMTConSymbol_FaceValue1,
        IMTConSymbol_AccruedInterest,
        IMTConSymbol_AccruedInterest1,
        IMTConSymbol_SpliceType,
        IMTConSymbol_SpliceType1,
        IMTConSymbol_SpliceTimeType,
        IMTConSymbol_SpliceTimeType1,
        IMTConSymbol_SpliceTimeDays,
        IMTConSymbol_SpliceTimeDays1,
        IMTConSymbol_MarginHedged,
        IMTConSymbol_MarginHedged1,
        IMTConSymbol_MarginRateCurrency,
        IMTConSymbol_MarginRateCurrency1,
        IMTConSymbol_FilterGap,
        IMTConSymbol_FilterGap1,
        IMTConSymbol_FilterGapTicks,
        IMTConSymbol_FilterGapTicks1,
        IMTConSymbol_ChartMode,
        IMTConSymbol_ChartMode1,
        IMTConSymbol_CurrencyBaseDigitsSet,
        IMTConSymbol_CurrencyProfitDigitsSet,
        IMTConSymbol_CurrencyMarginDigitsSet,
        IMTConSymbol_IEFlags,
        IMTConSymbol_IEFlags1,
        IMTConSymbol_VolumeMinExt,
        IMTConSymbol_VolumeMinExt1,
        IMTConSymbol_VolumeMaxExt,
        IMTConSymbol_VolumeMaxExt1,
        IMTConSymbol_VolumeStepExt,
        IMTConSymbol_VolumeStepExt1,
        IMTConSymbol_VolumeLimitExt,
        IMTConSymbol_VolumeLimitExt1,
        IMTConSymbol_IEVolumeMaxExt,
        IMTConSymbol_IEVolumeMaxExt1,
        IMTConSymbol_Category,
        IMTConSymbol_Category1,
        IMTConSymbol_Exchange,
        IMTConSymbol_Exchange1,
        IMTConSymbol_CFI,
        IMTConSymbol_CFI1,
        IMTConSymbol_Sector,
        IMTConSymbol_Sector1,
        IMTConSymbol_Industry,
        IMTConSymbol_Industry1,
        IMTConSymbol_Country,
        IMTConSymbol_Country1,
        IMTConSymbol_SubscriptionsDelay,
        IMTConSymbol_SubscriptionsDelay1,
        IMTConSymbol_SwapYearDays,
        IMTConSymbol_SwapYearDays1,
        IMTConSymbol_SwapFlags,
        IMTConSymbol_SwapFlags1,
        IMTConSymbol_SwapRateSunday,
        IMTConSymbol_SwapRateSunday1,
        IMTConSymbol_SwapRateMonday,
        IMTConSymbol_SwapRateMonday1,
        IMTConSymbol_SwapRateTuesday,
        IMTConSymbol_SwapRateTuesday1,
        IMTConSymbol_SwapRateWednesday,
        IMTConSymbol_SwapRateWednesday1,
        IMTConSymbol_SwapRateThursday,
        IMTConSymbol_SwapRateThursday1,
        IMTConSymbol_SwapRateFriday,
        IMTConSymbol_SwapRateFriday1,
        IMTConSymbol_SwapRateSaturday,
        IMTConSymbol_SwapRateSaturday1,
    }
}
