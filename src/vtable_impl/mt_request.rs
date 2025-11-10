use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTRequest_Release(this: *mut IMTRequest) {
    (*(*this).impl_ptr).release()
}

unsafe extern "C" fn IMTRequest_Assign(this: *mut IMTRequest, request: *const IMTRequest) -> MTAPIRES {
    (*(*this).impl_ptr).assign(&*request)
}

unsafe extern "C" fn IMTRequest_Clear(this: *mut IMTRequest) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTRequest_Print(this: *const IMTRequest, string: *mut MTAPISTR) -> LPCWSTR {
    (*(*this).impl_ptr).print(&mut *string)
}

unsafe extern "C" fn IMTRequest_ID(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).id()
}

unsafe extern "C" fn IMTRequest_Login(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).login()
}

unsafe extern "C" fn IMTRequest_Login1(this: *mut IMTRequest, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).login1(login)
}

unsafe extern "C" fn IMTRequest_Group(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).group()
}

unsafe extern "C" fn IMTRequest_Symbol(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).symbol()
}

unsafe extern "C" fn IMTRequest_Symbol1(this: *mut IMTRequest, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol1(symbol)
}

unsafe extern "C" fn IMTRequest_Digits(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).digits()
}

unsafe extern "C" fn IMTRequest_Action(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).action()
}

unsafe extern "C" fn IMTRequest_Action1(this: *mut IMTRequest, action: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).action1(action)
}

unsafe extern "C" fn IMTRequest_TimeExpiration(this: *const IMTRequest) -> INT64 {
    (*(*this).impl_ptr).time_expiration()
}

unsafe extern "C" fn IMTRequest_TimeExpiration1(this: *mut IMTRequest, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_expiration1(time)
}

unsafe extern "C" fn IMTRequest_Type(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).type_()
}

unsafe extern "C" fn IMTRequest_Type1(this: *mut IMTRequest, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type1(type_)
}

unsafe extern "C" fn IMTRequest_TypeFill(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).type_fill()
}

unsafe extern "C" fn IMTRequest_TypeFill1(this: *mut IMTRequest, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type_fill1(type_)
}

unsafe extern "C" fn IMTRequest_TypeTime(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).type_time()
}

unsafe extern "C" fn IMTRequest_TypeTime1(this: *mut IMTRequest, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type_time1(type_)
}

unsafe extern "C" fn IMTRequest_Flags(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).flags()
}

unsafe extern "C" fn IMTRequest_Flags1(this: *mut IMTRequest, flags: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).flags1(flags)
}

unsafe extern "C" fn IMTRequest_Volume(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).volume()
}

unsafe extern "C" fn IMTRequest_Volume1(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume1(volume)
}

unsafe extern "C" fn IMTRequest_Order(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).order()
}

unsafe extern "C" fn IMTRequest_Order1(this: *mut IMTRequest, order: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).order1(order)
}

unsafe extern "C" fn IMTRequest_OrderExternalID(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).order_external_id()
}

unsafe extern "C" fn IMTRequest_OrderExternalID1(this: *mut IMTRequest, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).order_external_id1(id)
}

unsafe extern "C" fn IMTRequest_PriceOrder(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).price_order()
}

unsafe extern "C" fn IMTRequest_PriceOrder1(this: *mut IMTRequest, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_order1(price)
}

unsafe extern "C" fn IMTRequest_PriceTrigger(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).price_trigger()
}

unsafe extern "C" fn IMTRequest_PriceTrigger1(this: *mut IMTRequest, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_trigger1(price)
}

unsafe extern "C" fn IMTRequest_PriceSL(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).price_sl()
}

unsafe extern "C" fn IMTRequest_PriceSL1(this: *mut IMTRequest, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_sl1(price)
}

unsafe extern "C" fn IMTRequest_PriceTP(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).price_tp()
}

unsafe extern "C" fn IMTRequest_PriceTP1(this: *mut IMTRequest, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_tp1(price)
}

unsafe extern "C" fn IMTRequest_PriceDeviation(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).price_deviation()
}

unsafe extern "C" fn IMTRequest_PriceDeviation1(this: *mut IMTRequest, deviation: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).price_deviation1(deviation)
}

unsafe extern "C" fn IMTRequest_PriceDeviationTop(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).price_deviation_top()
}

unsafe extern "C" fn IMTRequest_PriceDeviationBottom(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).price_deviation_bottom()
}

unsafe extern "C" fn IMTRequest_Comment(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).comment()
}

unsafe extern "C" fn IMTRequest_Comment1(this: *mut IMTRequest, comment: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).comment1(comment)
}

unsafe extern "C" fn IMTRequest_ResultRetcode(this: *const IMTRequest) -> MTAPIRES {
    (*(*this).impl_ptr).result_retcode()
}

unsafe extern "C" fn IMTRequest_ResultDealer(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).result_dealer()
}

unsafe extern "C" fn IMTRequest_ResultDeal(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).result_deal()
}

unsafe extern "C" fn IMTRequest_ResultOrder(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).result_order()
}

unsafe extern "C" fn IMTRequest_ResultVolume(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).result_volume()
}

unsafe extern "C" fn IMTRequest_ResultPrice(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_price()
}

unsafe extern "C" fn IMTRequest_ResultDealerBid(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_dealer_bid()
}

unsafe extern "C" fn IMTRequest_ResultDealerAsk(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_dealer_ask()
}

unsafe extern "C" fn IMTRequest_ResultDealerLast(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_dealer_last()
}

unsafe extern "C" fn IMTRequest_ResultMarketBid(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_market_bid()
}

unsafe extern "C" fn IMTRequest_ResultMarketAsk(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_market_ask()
}

unsafe extern "C" fn IMTRequest_ResultMarketLast(this: *const IMTRequest) -> f64 {
    (*(*this).impl_ptr).result_market_last()
}

unsafe extern "C" fn IMTRequest_ResultComment(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).result_comment()
}

unsafe extern "C" fn IMTRequest_ExternalAccount(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).external_account()
}

unsafe extern "C" fn IMTRequest_ExternalAccount1(this: *mut IMTRequest, account: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).external_account1(account)
}

unsafe extern "C" fn IMTRequest_IDClient(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).id_client()
}

unsafe extern "C" fn IMTRequest_IP(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).ip()
}

unsafe extern "C" fn IMTRequest_IP1(this: *mut IMTRequest, ip: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).ip1(ip)
}

unsafe extern "C" fn IMTRequest_SourceLogin(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).source_login()
}

unsafe extern "C" fn IMTRequest_SourceLogin1(this: *mut IMTRequest, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).source_login1(login)
}

unsafe extern "C" fn IMTRequest_Position(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).position()
}

unsafe extern "C" fn IMTRequest_Position1(this: *mut IMTRequest, position: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).position1(position)
}

unsafe extern "C" fn IMTRequest_PositionBy(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).position_by()
}

unsafe extern "C" fn IMTRequest_PositionBy1(this: *mut IMTRequest, position: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).position_by1(position)
}

unsafe extern "C" fn IMTRequest_PositionExternalID(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).position_external_id()
}

unsafe extern "C" fn IMTRequest_PositionExternalID1(this: *mut IMTRequest, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).position_external_id1(id)
}

unsafe extern "C" fn IMTRequest_PositionByExternalID(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).position_by_external_id()
}

unsafe extern "C" fn IMTRequest_PositionByExternalID1(this: *mut IMTRequest, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).position_by_external_id1(id)
}

unsafe extern "C" fn IMTRequest_VolumeExt(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).volume_ext()
}

unsafe extern "C" fn IMTRequest_VolumeExt1(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_ext1(volume)
}

unsafe extern "C" fn IMTRequest_ResultVolumeExt(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).result_volume_ext()
}

unsafe extern "C" fn IMTRequest_DigitsSet(this: *mut IMTRequest, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits_set(digits)
}

unsafe extern "C" fn IMTRequest_ApiDataSet(this: *mut IMTRequest, app_id: USHORT, id: UCHAR, value: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set(app_id, id, value)
}

unsafe extern "C" fn IMTRequest_ApiDataSet1(this: *mut IMTRequest, app_id: USHORT, id: UCHAR, value: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set1(app_id, id, value)
}

unsafe extern "C" fn IMTRequest_ApiDataSet2(this: *mut IMTRequest, app_id: USHORT, id: UCHAR, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set2(app_id, id, value)
}

unsafe extern "C" fn IMTRequest_ApiDataGet(this: *const IMTRequest, app_id: USHORT, id: UCHAR, value: *mut INT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get(app_id, id, &mut *value)
}

unsafe extern "C" fn IMTRequest_ApiDataGet1(this: *const IMTRequest, app_id: USHORT, id: UCHAR, value: *mut UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get1(app_id, id, &mut *value)
}

unsafe extern "C" fn IMTRequest_ApiDataGet2(this: *const IMTRequest, app_id: USHORT, id: UCHAR, value: *mut f64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get2(app_id, id, &mut *value)
}

unsafe extern "C" fn IMTRequest_ApiDataUpdate(this: *mut IMTRequest, pos: UINT, app_id: USHORT, id: UCHAR, value: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update(pos, app_id, id, value)
}

unsafe extern "C" fn IMTRequest_ApiDataUpdate1(this: *mut IMTRequest, pos: UINT, app_id: USHORT, id: UCHAR, value: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTRequest_ApiDataUpdate2(this: *mut IMTRequest, pos: UINT, app_id: USHORT, id: UCHAR, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTRequest_ApiDataNext(this: *const IMTRequest, pos: UINT, app_id: *mut USHORT, id: *mut UCHAR, value: *mut INT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next(pos, &mut *app_id, &mut *id, &mut *value)
}

unsafe extern "C" fn IMTRequest_ApiDataNext1(this: *const IMTRequest, pos: UINT, app_id: *mut USHORT, id: *mut UCHAR, value: *mut UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next1(pos, &mut *app_id, &mut *id, &mut *value)
}

unsafe extern "C" fn IMTRequest_ApiDataNext2(this: *const IMTRequest, pos: UINT, app_id: *mut USHORT, id: *mut UCHAR, value: *mut f64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next2(pos, &mut *app_id, &mut *id, &mut *value)
}

unsafe extern "C" fn IMTRequest_ApiDataRaw(this: *const IMTRequest) -> LPVOID {
    (*(*this).impl_ptr).api_data_raw()
}

unsafe extern "C" fn IMTRequest_ApiDataRawMax(this: *const IMTRequest) -> UINT {
    (*(*this).impl_ptr).api_data_raw_max()
}

unsafe extern "C" fn IMTRequest_ApiDataClear(this: *mut IMTRequest, app_id: USHORT) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear(app_id)
}

unsafe extern "C" fn IMTRequest_ApiDataClearAll(this: *mut IMTRequest) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear_all()
}

unsafe extern "C" fn IMTRequest_VolumeCurrent(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).volume_current()
}

unsafe extern "C" fn IMTRequest_VolumeCurrent1(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_current1(volume)
}

unsafe extern "C" fn IMTRequest_VolumeCurrentExt(this: *const IMTRequest) -> UINT64 {
    (*(*this).impl_ptr).volume_current_ext()
}

unsafe extern "C" fn IMTRequest_VolumeCurrentExt1(this: *mut IMTRequest, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_current_ext1(volume)
}

unsafe extern "C" fn IMTRequest_SymbolOriginal(this: *const IMTRequest) -> LPCWSTR {
    (*(*this).impl_ptr).symbol_original()
}

unsafe extern "C" fn IMTRequest_SymbolOriginal1(this: *mut IMTRequest, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol_original1(symbol)
}

pub const fn new() -> IMTRequest__bindgen_vtable {
    IMTRequest__bindgen_vtable {
        IMTRequest_Release,
        IMTRequest_Assign,
        IMTRequest_Clear,
        IMTRequest_Print,
        IMTRequest_ID,
        IMTRequest_Login,
        IMTRequest_Login1,
        IMTRequest_Group,
        IMTRequest_Symbol,
        IMTRequest_Symbol1,
        IMTRequest_Digits,
        IMTRequest_Action,
        IMTRequest_Action1,
        IMTRequest_TimeExpiration,
        IMTRequest_TimeExpiration1,
        IMTRequest_Type,
        IMTRequest_Type1,
        IMTRequest_TypeFill,
        IMTRequest_TypeFill1,
        IMTRequest_TypeTime,
        IMTRequest_TypeTime1,
        IMTRequest_Flags,
        IMTRequest_Flags1,
        IMTRequest_Volume,
        IMTRequest_Volume1,
        IMTRequest_Order,
        IMTRequest_Order1,
        IMTRequest_OrderExternalID,
        IMTRequest_OrderExternalID1,
        IMTRequest_PriceOrder,
        IMTRequest_PriceOrder1,
        IMTRequest_PriceTrigger,
        IMTRequest_PriceTrigger1,
        IMTRequest_PriceSL,
        IMTRequest_PriceSL1,
        IMTRequest_PriceTP,
        IMTRequest_PriceTP1,
        IMTRequest_PriceDeviation,
        IMTRequest_PriceDeviation1,
        IMTRequest_PriceDeviationTop,
        IMTRequest_PriceDeviationBottom,
        IMTRequest_Comment,
        IMTRequest_Comment1,
        IMTRequest_ResultRetcode,
        IMTRequest_ResultDealer,
        IMTRequest_ResultDeal,
        IMTRequest_ResultOrder,
        IMTRequest_ResultVolume,
        IMTRequest_ResultPrice,
        IMTRequest_ResultDealerBid,
        IMTRequest_ResultDealerAsk,
        IMTRequest_ResultDealerLast,
        IMTRequest_ResultMarketBid,
        IMTRequest_ResultMarketAsk,
        IMTRequest_ResultMarketLast,
        IMTRequest_ResultComment,
        IMTRequest_ExternalAccount,
        IMTRequest_ExternalAccount1,
        IMTRequest_IDClient,
        IMTRequest_IP,
        IMTRequest_IP1,
        IMTRequest_SourceLogin,
        IMTRequest_SourceLogin1,
        IMTRequest_Position,
        IMTRequest_Position1,
        IMTRequest_PositionBy,
        IMTRequest_PositionBy1,
        IMTRequest_PositionExternalID,
        IMTRequest_PositionExternalID1,
        IMTRequest_PositionByExternalID,
        IMTRequest_PositionByExternalID1,
        IMTRequest_VolumeExt,
        IMTRequest_VolumeExt1,
        IMTRequest_ResultVolumeExt,
        IMTRequest_DigitsSet,
        IMTRequest_ApiDataSet,
        IMTRequest_ApiDataSet1,
        IMTRequest_ApiDataSet2,
        IMTRequest_ApiDataGet,
        IMTRequest_ApiDataGet1,
        IMTRequest_ApiDataGet2,
        IMTRequest_ApiDataUpdate,
        IMTRequest_ApiDataUpdate1,
        IMTRequest_ApiDataUpdate2,
        IMTRequest_ApiDataNext,
        IMTRequest_ApiDataNext1,
        IMTRequest_ApiDataNext2,
        IMTRequest_ApiDataRaw,
        IMTRequest_ApiDataRawMax,
        IMTRequest_ApiDataClear,
        IMTRequest_ApiDataClearAll,
        IMTRequest_VolumeCurrent,
        IMTRequest_VolumeCurrent1,
        IMTRequest_VolumeCurrentExt,
        IMTRequest_VolumeCurrentExt1,
        IMTRequest_SymbolOriginal,
        IMTRequest_SymbolOriginal1,
    }
}
