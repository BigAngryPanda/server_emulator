use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTDeal_Release(this: *mut IMTDeal) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTDeal>());
}

unsafe extern "C" fn IMTDeal_Assign(this: *mut IMTDeal, deal: *const IMTDeal) -> MTAPIRES {
    (*(*this).impl_ptr).assign(deal)
}

unsafe extern "C" fn IMTDeal_Clear(this: *mut IMTDeal) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTDeal_Print(this: *const IMTDeal, string: *mut MTAPISTR) -> LPCWSTR {
    (*(*this).impl_ptr).print(string)
}

unsafe extern "C" fn IMTDeal_Deal(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).deal()
}

unsafe extern "C" fn IMTDeal_ExternalID1(this: *mut IMTDeal, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).external_id1(id)
}

unsafe extern "C" fn IMTDeal_ExternalID(this: *const IMTDeal) -> LPCWSTR {
    (*(*this).impl_ptr).external_id()
}

unsafe extern "C" fn IMTDeal_Login1(this: *mut IMTDeal, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).login1(login)
}

unsafe extern "C" fn IMTDeal_Login(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).login()
}

unsafe extern "C" fn IMTDeal_Dealer1(this: *mut IMTDeal, dealer: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).dealer1(dealer)
}

unsafe extern "C" fn IMTDeal_Dealer(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).dealer()
}

unsafe extern "C" fn IMTDeal_Order1(this: *mut IMTDeal, order: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).order1(order)
}

unsafe extern "C" fn IMTDeal_Order(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).order()
}

unsafe extern "C" fn IMTDeal_Action1(this: *mut IMTDeal, action: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).action1(action)
}

unsafe extern "C" fn IMTDeal_Action(this: *const IMTDeal) -> UINT {
    (*(*this).impl_ptr).action()
}

unsafe extern "C" fn IMTDeal_Entry1(this: *mut IMTDeal, entry: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).entry1(entry)
}

unsafe extern "C" fn IMTDeal_Entry(this: *const IMTDeal) -> UINT {
    (*(*this).impl_ptr).entry()
}

unsafe extern "C" fn IMTDeal_Digits1(this: *mut IMTDeal, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits1(digits)
}

unsafe extern "C" fn IMTDeal_Digits(this: *const IMTDeal) -> UINT {
    (*(*this).impl_ptr).digits()
}

unsafe extern "C" fn IMTDeal_DigitsCurrency1(this: *mut IMTDeal, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits_currency1(digits)
}

unsafe extern "C" fn IMTDeal_DigitsCurrency(this: *const IMTDeal) -> UINT {
    (*(*this).impl_ptr).digits_currency()
}

unsafe extern "C" fn IMTDeal_ContractSize1(this: *mut IMTDeal, contract_size: f64) -> MTAPIRES {
    (*(*this).impl_ptr).contract_size1(contract_size)
}

unsafe extern "C" fn IMTDeal_ContractSize(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).contract_size()
}

unsafe extern "C" fn IMTDeal_Time1(this: *mut IMTDeal, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time1(time)
}

unsafe extern "C" fn IMTDeal_Time(this: *const IMTDeal) -> INT64 {
    (*(*this).impl_ptr).time()
}

unsafe extern "C" fn IMTDeal_Symbol1(this: *mut IMTDeal, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol1(symbol)
}

unsafe extern "C" fn IMTDeal_Symbol(this: *const IMTDeal) -> LPCWSTR {
    (*(*this).impl_ptr).symbol()
}

unsafe extern "C" fn IMTDeal_Price1(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price1(price)
}

unsafe extern "C" fn IMTDeal_Price(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).price()
}

unsafe extern "C" fn IMTDeal_Volume1(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume1(volume)
}

unsafe extern "C" fn IMTDeal_Volume(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).volume()
}

unsafe extern "C" fn IMTDeal_Profit1(this: *mut IMTDeal, profit: f64) -> MTAPIRES {
    (*(*this).impl_ptr).profit1(profit)
}

unsafe extern "C" fn IMTDeal_Profit(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).profit()
}

unsafe extern "C" fn IMTDeal_Storage1(this: *mut IMTDeal, storage: f64) -> MTAPIRES {
    (*(*this).impl_ptr).storage1(storage)
}

unsafe extern "C" fn IMTDeal_Storage(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).storage()
}

unsafe extern "C" fn IMTDeal_Commission1(this: *mut IMTDeal, comm: f64) -> MTAPIRES {
    (*(*this).impl_ptr).commission1(comm)
}

unsafe extern "C" fn IMTDeal_Commission(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).commission()
}

unsafe extern "C" fn IMTDeal_ObsoleteValue1(this: *mut IMTDeal, agent: f64) -> MTAPIRES {
    (*(*this).impl_ptr).obsolete_value1(agent)
}

unsafe extern "C" fn IMTDeal_ObsoleteValue(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).obsolete_value()
}

unsafe extern "C" fn IMTDeal_RateProfit1(this: *mut IMTDeal, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).rate_profit1(rate)
}

unsafe extern "C" fn IMTDeal_RateProfit(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).rate_profit()
}

unsafe extern "C" fn IMTDeal_RateMargin1(this: *mut IMTDeal, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).rate_margin1(rate)
}

unsafe extern "C" fn IMTDeal_RateMargin(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).rate_margin()
}

unsafe extern "C" fn IMTDeal_ExpertID1(this: *mut IMTDeal, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).expert_id1(id)
}

unsafe extern "C" fn IMTDeal_ExpertID(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).expert_id()
}

unsafe extern "C" fn IMTDeal_PositionID1(this: *mut IMTDeal, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).position_id1(id)
}

unsafe extern "C" fn IMTDeal_PositionID(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).position_id()
}

unsafe extern "C" fn IMTDeal_Comment1(this: *mut IMTDeal, comment: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).comment1(comment)
}

unsafe extern "C" fn IMTDeal_Comment(this: *const IMTDeal) -> LPCWSTR {
    (*(*this).impl_ptr).comment()
}

unsafe extern "C" fn IMTDeal_ApiDataSet2(this: *mut IMTDeal, app_id: USHORT, id: UCHAR, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set2(app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataSet1(
    this: *mut IMTDeal,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set1(app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataSet(
    this: *mut IMTDeal,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set(app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataGet2(
    this: *const IMTDeal,
    app_id: USHORT,
    id: UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get2(app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataGet1(
    this: *const IMTDeal,
    app_id: USHORT,
    id: UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get1(app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataGet(
    this: *const IMTDeal,
    app_id: USHORT,
    id: UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get(app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataClear(this: *mut IMTDeal, app_id: USHORT) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear(app_id)
}

unsafe extern "C" fn IMTDeal_ApiDataClearAll(this: *mut IMTDeal) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear_all()
}

unsafe extern "C" fn IMTDeal_ProfitRaw1(this: *mut IMTDeal, profit: f64) -> MTAPIRES {
    (*(*this).impl_ptr).profit_raw1(profit)
}

unsafe extern "C" fn IMTDeal_ProfitRaw(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).profit_raw()
}

unsafe extern "C" fn IMTDeal_PricePosition1(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_position1(price)
}

unsafe extern "C" fn IMTDeal_PricePosition(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).price_position()
}

unsafe extern "C" fn IMTDeal_VolumeClosed1(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_closed1(volume)
}

unsafe extern "C" fn IMTDeal_VolumeClosed(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).volume_closed()
}

unsafe extern "C" fn IMTDeal_TickValue1(this: *mut IMTDeal, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).tick_value1(value)
}

unsafe extern "C" fn IMTDeal_TickValue(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).tick_value()
}

unsafe extern "C" fn IMTDeal_TickSize1(this: *mut IMTDeal, size: f64) -> MTAPIRES {
    (*(*this).impl_ptr).tick_size1(size)
}

unsafe extern "C" fn IMTDeal_TickSize(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).tick_size()
}

unsafe extern "C" fn IMTDeal_Flags1(this: *mut IMTDeal, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).flags1(flags)
}

unsafe extern "C" fn IMTDeal_Flags(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).flags()
}

unsafe extern "C" fn IMTDeal_TimeMsc1(this: *mut IMTDeal, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_msc1(time)
}

unsafe extern "C" fn IMTDeal_TimeMsc(this: *const IMTDeal) -> INT64 {
    (*(*this).impl_ptr).time_msc()
}

unsafe extern "C" fn IMTDeal_Reason(this: *const IMTDeal) -> UINT {
    (*(*this).impl_ptr).reason()
}

unsafe extern "C" fn IMTDeal_Gateway(this: *const IMTDeal) -> LPCWSTR {
    (*(*this).impl_ptr).gateway()
}

unsafe extern "C" fn IMTDeal_PriceGateway(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).price_gateway()
}

unsafe extern "C" fn IMTDeal_ApiDataUpdate2(
    this: *mut IMTDeal,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: f64
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataUpdate1(
    this: *mut IMTDeal,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataUpdate(
    this: *mut IMTDeal,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: INT64
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update(pos, app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataNext2(
    this: *const IMTDeal,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut f64
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataNext1(
    this: *const IMTDeal,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTDeal_ApiDataNext(
    this: *const IMTDeal,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next(pos, app_id, id, value)
}

unsafe extern "C" fn IMTDeal_DealSet(this: *mut IMTDeal, deal: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).deal_set(deal)
}

unsafe extern "C" fn IMTDeal_ModificationFlags(this: *const IMTDeal) -> UINT {
    (*(*this).impl_ptr).modification_flags()
}

unsafe extern "C" fn IMTDeal_ReasonSet(this: *mut IMTDeal, reason: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).reason_set(reason)
}

unsafe extern "C" fn IMTDeal_GatewaySet(this: *mut IMTDeal, gateway: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).gateway_set(gateway)
}

unsafe extern "C" fn IMTDeal_PriceGatewaySet(this: *mut IMTDeal, price_gateway: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_gateway_set(price_gateway)
}

unsafe extern "C" fn IMTDeal_PriceSL1(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_sl1(price)
}

unsafe extern "C" fn IMTDeal_PriceSL(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).price_sl()
}

unsafe extern "C" fn IMTDeal_PriceTP1(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_tp1(price)
}

unsafe extern "C" fn IMTDeal_PriceTP(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).price_tp()
}

unsafe extern "C" fn IMTDeal_VolumeExt1(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_ext1(volume)
}

unsafe extern "C" fn IMTDeal_VolumeExt(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).volume_ext()
}

unsafe extern "C" fn IMTDeal_VolumeClosedExt1(this: *mut IMTDeal, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_closed_ext1(volume)
}

unsafe extern "C" fn IMTDeal_VolumeClosedExt(this: *const IMTDeal) -> UINT64 {
    (*(*this).impl_ptr).volume_closed_ext()
}

unsafe extern "C" fn IMTDeal_Fee1(this: *mut IMTDeal, fee: f64) -> MTAPIRES {
    (*(*this).impl_ptr).fee1(fee)
}

unsafe extern "C" fn IMTDeal_Fee(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).fee()
}

unsafe extern "C" fn IMTDeal_Value1(this: *mut IMTDeal, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).value1(value)
}

unsafe extern "C" fn IMTDeal_Value(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).value()
}

unsafe extern "C" fn IMTDeal_MarketBid(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).market_bid()
}

unsafe extern "C" fn IMTDeal_MarketAsk(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).market_ask()
}

unsafe extern "C" fn IMTDeal_MarketLast(this: *const IMTDeal) -> f64 {
    (*(*this).impl_ptr).market_last()
}

unsafe extern "C" fn IMTDeal_MarketBidSet(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).market_bid_set(price)
}

unsafe extern "C" fn IMTDeal_MarketAskSet(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).market_ask_set(price)
}

unsafe extern "C" fn IMTDeal_MarketLastSet(this: *mut IMTDeal, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).market_last_set(price)
}

pub const fn new() -> IMTDeal__bindgen_vtable {
    IMTDeal__bindgen_vtable {
        IMTDeal_Release,
        IMTDeal_Assign,
        IMTDeal_Clear,
        IMTDeal_Print,
        IMTDeal_Deal,
        IMTDeal_ExternalID1,
        IMTDeal_ExternalID,
        IMTDeal_Login1,
        IMTDeal_Login,
        IMTDeal_Dealer1,
        IMTDeal_Dealer,
        IMTDeal_Order1,
        IMTDeal_Order,
        IMTDeal_Action1,
        IMTDeal_Action,
        IMTDeal_Entry1,
        IMTDeal_Entry,
        IMTDeal_Digits1,
        IMTDeal_Digits,
        IMTDeal_DigitsCurrency1,
        IMTDeal_DigitsCurrency,
        IMTDeal_ContractSize1,
        IMTDeal_ContractSize,
        IMTDeal_Time1,
        IMTDeal_Time,
        IMTDeal_Symbol1,
        IMTDeal_Symbol,
        IMTDeal_Price1,
        IMTDeal_Price,
        IMTDeal_Volume1,
        IMTDeal_Volume,
        IMTDeal_Profit1,
        IMTDeal_Profit,
        IMTDeal_Storage1,
        IMTDeal_Storage,
        IMTDeal_Commission1,
        IMTDeal_Commission,
        IMTDeal_ObsoleteValue1,
        IMTDeal_ObsoleteValue,
        IMTDeal_RateProfit1,
        IMTDeal_RateProfit,
        IMTDeal_RateMargin1,
        IMTDeal_RateMargin,
        IMTDeal_ExpertID1,
        IMTDeal_ExpertID,
        IMTDeal_PositionID1,
        IMTDeal_PositionID,
        IMTDeal_Comment1,
        IMTDeal_Comment,
        IMTDeal_ApiDataSet2,
        IMTDeal_ApiDataSet1,
        IMTDeal_ApiDataSet,
        IMTDeal_ApiDataGet2,
        IMTDeal_ApiDataGet1,
        IMTDeal_ApiDataGet,
        IMTDeal_ApiDataClear,
        IMTDeal_ApiDataClearAll,
        IMTDeal_ProfitRaw1,
        IMTDeal_ProfitRaw,
        IMTDeal_PricePosition1,
        IMTDeal_PricePosition,
        IMTDeal_VolumeClosed1,
        IMTDeal_VolumeClosed,
        IMTDeal_TickValue1,
        IMTDeal_TickValue,
        IMTDeal_TickSize1,
        IMTDeal_TickSize,
        IMTDeal_Flags1,
        IMTDeal_Flags,
        IMTDeal_TimeMsc1,
        IMTDeal_TimeMsc,
        IMTDeal_Reason,
        IMTDeal_Gateway,
        IMTDeal_PriceGateway,
        IMTDeal_ApiDataUpdate2,
        IMTDeal_ApiDataUpdate1,
        IMTDeal_ApiDataUpdate,
        IMTDeal_ApiDataNext2,
        IMTDeal_ApiDataNext1,
        IMTDeal_ApiDataNext,
        IMTDeal_DealSet,
        IMTDeal_ModificationFlags,
        IMTDeal_ReasonSet,
        IMTDeal_GatewaySet,
        IMTDeal_PriceGatewaySet,
        IMTDeal_PriceSL1,
        IMTDeal_PriceSL,
        IMTDeal_PriceTP1,
        IMTDeal_PriceTP,
        IMTDeal_VolumeExt1,
        IMTDeal_VolumeExt,
        IMTDeal_VolumeClosedExt1,
        IMTDeal_VolumeClosedExt,
        IMTDeal_Fee1,
        IMTDeal_Fee,
        IMTDeal_Value1,
        IMTDeal_Value,
        IMTDeal_MarketBid,
        IMTDeal_MarketAsk,
        IMTDeal_MarketLast,
        IMTDeal_MarketBidSet,
        IMTDeal_MarketAskSet,
        IMTDeal_MarketLastSet
    }
}
