use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTPosition_Release(this: *mut IMTPosition) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTPosition>());
}

unsafe extern "C" fn IMTPosition_Assign(this: *mut IMTPosition, position: *const IMTPosition) -> MTAPIRES {
    (*(*this).impl_ptr).assign(position)
}

unsafe extern "C" fn IMTPosition_Clear(this: *mut IMTPosition) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTPosition_Print(this: *const IMTPosition, string: *mut MTAPISTR) -> LPCWSTR {
    (*(*this).impl_ptr).print(string)
}

unsafe extern "C" fn IMTPosition_Login(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).login()
}

unsafe extern "C" fn IMTPosition_Symbol1(this: *mut IMTPosition, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol1(symbol)
}

unsafe extern "C" fn IMTPosition_Symbol(this: *const IMTPosition) -> LPCWSTR {
    (*(*this).impl_ptr).symbol()
}

unsafe extern "C" fn IMTPosition_Action1(this: *mut IMTPosition, action: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).action1(action)
}

unsafe extern "C" fn IMTPosition_Action(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).action()
}

unsafe extern "C" fn IMTPosition_Digits1(this: *mut IMTPosition, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits1(digits)
}

unsafe extern "C" fn IMTPosition_Digits(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).digits()
}

unsafe extern "C" fn IMTPosition_DigitsCurrency1(this: *mut IMTPosition, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits_currency1(digits)
}

unsafe extern "C" fn IMTPosition_DigitsCurrency(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).digits_currency()
}

unsafe extern "C" fn IMTPosition_ContractSize1(this: *mut IMTPosition, contract_size: f64) -> MTAPIRES {
    (*(*this).impl_ptr).contract_size1(contract_size)
}

unsafe extern "C" fn IMTPosition_ContractSize(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).contract_size()
}

unsafe extern "C" fn IMTPosition_TimeCreate1(this: *mut IMTPosition, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_create1(time)
}

unsafe extern "C" fn IMTPosition_TimeCreate(this: *const IMTPosition) -> INT64 {
    (*(*this).impl_ptr).time_create()
}

unsafe extern "C" fn IMTPosition_TimeUpdate1(this: *mut IMTPosition, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_update1(time)
}

unsafe extern "C" fn IMTPosition_TimeUpdate(this: *const IMTPosition) -> INT64 {
    (*(*this).impl_ptr).time_update()
}

unsafe extern "C" fn IMTPosition_PriceOpen1(this: *mut IMTPosition, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_open1(price)
}

unsafe extern "C" fn IMTPosition_PriceOpen(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).price_open()
}

unsafe extern "C" fn IMTPosition_PriceCurrent1(this: *mut IMTPosition, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_current1(price)
}

unsafe extern "C" fn IMTPosition_PriceCurrent(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).price_current()
}

unsafe extern "C" fn IMTPosition_PriceSL1(this: *mut IMTPosition, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_sl1(price)
}

unsafe extern "C" fn IMTPosition_PriceSL(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).price_sl()
}

unsafe extern "C" fn IMTPosition_PriceTP1(this: *mut IMTPosition, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_tp1(price)
}

unsafe extern "C" fn IMTPosition_PriceTP(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).price_tp()
}

unsafe extern "C" fn IMTPosition_Volume1(this: *mut IMTPosition, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume1(volume)
}

unsafe extern "C" fn IMTPosition_Volume(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).volume()
}

unsafe extern "C" fn IMTPosition_Profit1(this: *mut IMTPosition, profit: f64) -> MTAPIRES {
    (*(*this).impl_ptr).profit1(profit)
}

unsafe extern "C" fn IMTPosition_Profit(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).profit()
}

unsafe extern "C" fn IMTPosition_Storage1(this: *mut IMTPosition, storage: f64) -> MTAPIRES {
    (*(*this).impl_ptr).storage1(storage)
}

unsafe extern "C" fn IMTPosition_Storage(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).storage()
}

unsafe extern "C" fn IMTPosition_ObsoleteValue1(this: *mut IMTPosition, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).obsolete_value1(value)
}

unsafe extern "C" fn IMTPosition_ObsoleteValue(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).obsolete_value()
}

unsafe extern "C" fn IMTPosition_RateProfit1(this: *mut IMTPosition, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).rate_profit1(rate)
}

unsafe extern "C" fn IMTPosition_RateProfit(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).rate_profit()
}

unsafe extern "C" fn IMTPosition_RateMargin1(this: *mut IMTPosition, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).rate_margin1(rate)
}

unsafe extern "C" fn IMTPosition_RateMargin(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).rate_margin()
}

unsafe extern "C" fn IMTPosition_ExpertID1(this: *mut IMTPosition, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).expert_id1(id)
}

unsafe extern "C" fn IMTPosition_ExpertID(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).expert_id()
}

unsafe extern "C" fn IMTPosition_ExpertPositionID1(this: *mut IMTPosition, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).expert_position_id1(id)
}

unsafe extern "C" fn IMTPosition_ExpertPositionID(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).expert_position_id()
}

unsafe extern "C" fn IMTPosition_Comment1(this: *mut IMTPosition, comment: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).comment1(comment)
}

unsafe extern "C" fn IMTPosition_Comment(this: *const IMTPosition) -> LPCWSTR {
    (*(*this).impl_ptr).comment()
}

unsafe extern "C" fn IMTPosition_ActivationMode1(this: *mut IMTPosition, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).activation_mode1(mode)
}

unsafe extern "C" fn IMTPosition_ActivationMode(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).activation_mode()
}

unsafe extern "C" fn IMTPosition_ActivationTime1(this: *mut IMTPosition, atm: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).activation_time1(atm)
}

unsafe extern "C" fn IMTPosition_ActivationTime(this: *const IMTPosition) -> INT64 {
    (*(*this).impl_ptr).activation_time()
}

unsafe extern "C" fn IMTPosition_ActivationPrice1(this: *mut IMTPosition, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).activation_price1(price)
}

unsafe extern "C" fn IMTPosition_ActivationPrice(this: *const IMTPosition) -> f64 {
    (*(*this).impl_ptr).activation_price()
}

unsafe extern "C" fn IMTPosition_ActivationFlags1(this: *mut IMTPosition, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).activation_flags1(flags)
}

unsafe extern "C" fn IMTPosition_ActivationFlags(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).activation_flags()
}

unsafe extern "C" fn IMTPosition_ApiDataSet2(
    this: *mut IMTPosition,
    app_id: USHORT,
    id: UCHAR,
    value: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set2(app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataSet1(
    this: *mut IMTPosition,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set1(app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataSet(
    this: *mut IMTPosition,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set(app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataGet2(
    this: *const IMTPosition,
    app_id: USHORT,
    id: UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get2(app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataGet1(
    this: *const IMTPosition,
    app_id: USHORT,
    id: UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get1(app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataGet(
    this: *const IMTPosition,
    app_id: USHORT,
    id: UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get(app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataClear(this: *mut IMTPosition, app_id: USHORT) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear(app_id)
}

unsafe extern "C" fn IMTPosition_ApiDataClearAll(this: *mut IMTPosition) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear_all()
}

unsafe extern "C" fn IMTPosition_TimeCreateMsc1(this: *mut IMTPosition, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_create_msc1(time)
}

unsafe extern "C" fn IMTPosition_TimeCreateMsc(this: *const IMTPosition) -> INT64 {
    (*(*this).impl_ptr).time_create_msc()
}

unsafe extern "C" fn IMTPosition_TimeUpdateMsc1(this: *mut IMTPosition, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_update_msc1(time)
}

unsafe extern "C" fn IMTPosition_TimeUpdateMsc(this: *const IMTPosition) -> INT64 {
    (*(*this).impl_ptr).time_update_msc()
}

unsafe extern "C" fn IMTPosition_Dealer1(this: *mut IMTPosition, dealer: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).dealer1(dealer)
}

unsafe extern "C" fn IMTPosition_Dealer(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).dealer()
}

unsafe extern "C" fn IMTPosition_ApiDataUpdate2(
    this: *mut IMTPosition,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataUpdate1(
    this: *mut IMTPosition,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataUpdate(
    this: *mut IMTPosition,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update(pos, app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataNext2(
    this: *const IMTPosition,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataNext1(
    this: *const IMTPosition,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTPosition_ApiDataNext(
    this: *const IMTPosition,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next(pos, app_id, id, value)
}

unsafe extern "C" fn IMTPosition_LoginSet(this: *mut IMTPosition, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).login_set(login)
}

unsafe extern "C" fn IMTPosition_Position(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).position()
}

unsafe extern "C" fn IMTPosition_ExternalID1(this: *mut IMTPosition, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).external_id1(id)
}

unsafe extern "C" fn IMTPosition_ExternalID(this: *const IMTPosition) -> LPCWSTR {
    (*(*this).impl_ptr).external_id()
}

unsafe extern "C" fn IMTPosition_ModificationFlags(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).modification_flags()
}

unsafe extern "C" fn IMTPosition_Reason(this: *const IMTPosition) -> UINT {
    (*(*this).impl_ptr).reason()
}

unsafe extern "C" fn IMTPosition_VolumeExt1(this: *mut IMTPosition, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_ext1(volume)
}

unsafe extern "C" fn IMTPosition_VolumeExt(this: *const IMTPosition) -> UINT64 {
    (*(*this).impl_ptr).volume_ext()
}

unsafe extern "C" fn IMTPosition_ReasonSet(this: *mut IMTPosition, reason: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).reason_set(reason)
}

pub const fn new() -> IMTPosition__bindgen_vtable {
    IMTPosition__bindgen_vtable {
        IMTPosition_Release,
        IMTPosition_Assign,
        IMTPosition_Clear,
        IMTPosition_Print,
        IMTPosition_Login,
        IMTPosition_Symbol1,
        IMTPosition_Symbol,
        IMTPosition_Action1,
        IMTPosition_Action,
        IMTPosition_Digits1,
        IMTPosition_Digits,
        IMTPosition_DigitsCurrency1,
        IMTPosition_DigitsCurrency,
        IMTPosition_ContractSize1,
        IMTPosition_ContractSize,
        IMTPosition_TimeCreate1,
        IMTPosition_TimeCreate,
        IMTPosition_TimeUpdate1,
        IMTPosition_TimeUpdate,
        IMTPosition_PriceOpen1,
        IMTPosition_PriceOpen,
        IMTPosition_PriceCurrent1,
        IMTPosition_PriceCurrent,
        IMTPosition_PriceSL1,
        IMTPosition_PriceSL,
        IMTPosition_PriceTP1,
        IMTPosition_PriceTP,
        IMTPosition_Volume1,
        IMTPosition_Volume,
        IMTPosition_Profit1,
        IMTPosition_Profit,
        IMTPosition_Storage1,
        IMTPosition_Storage,
        IMTPosition_ObsoleteValue1,
        IMTPosition_ObsoleteValue,
        IMTPosition_RateProfit1,
        IMTPosition_RateProfit,
        IMTPosition_RateMargin1,
        IMTPosition_RateMargin,
        IMTPosition_ExpertID1,
        IMTPosition_ExpertID,
        IMTPosition_ExpertPositionID1,
        IMTPosition_ExpertPositionID,
        IMTPosition_Comment1,
        IMTPosition_Comment,
        IMTPosition_ActivationMode1,
        IMTPosition_ActivationMode,
        IMTPosition_ActivationTime1,
        IMTPosition_ActivationTime,
        IMTPosition_ActivationPrice1,
        IMTPosition_ActivationPrice,
        IMTPosition_ActivationFlags1,
        IMTPosition_ActivationFlags,
        IMTPosition_ApiDataSet2,
        IMTPosition_ApiDataSet1,
        IMTPosition_ApiDataSet,
        IMTPosition_ApiDataGet2,
        IMTPosition_ApiDataGet1,
        IMTPosition_ApiDataGet,
        IMTPosition_ApiDataClear,
        IMTPosition_ApiDataClearAll,
        IMTPosition_TimeCreateMsc1,
        IMTPosition_TimeCreateMsc,
        IMTPosition_TimeUpdateMsc1,
        IMTPosition_TimeUpdateMsc,
        IMTPosition_Dealer1,
        IMTPosition_Dealer,
        IMTPosition_ApiDataUpdate2,
        IMTPosition_ApiDataUpdate1,
        IMTPosition_ApiDataUpdate,
        IMTPosition_ApiDataNext2,
        IMTPosition_ApiDataNext1,
        IMTPosition_ApiDataNext,
        IMTPosition_LoginSet,
        IMTPosition_Position,
        IMTPosition_ExternalID1,
        IMTPosition_ExternalID,
        IMTPosition_ModificationFlags,
        IMTPosition_Reason,
        IMTPosition_VolumeExt1,
        IMTPosition_VolumeExt,
        IMTPosition_ReasonSet
    }
}
