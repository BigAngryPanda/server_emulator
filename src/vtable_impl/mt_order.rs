use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTOrder_Release(this: *mut IMTOrder) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTOrder>());
}

unsafe extern "C" fn IMTOrder_Assign(this: *mut IMTOrder, order: *const IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).assign(order)
}

unsafe extern "C" fn IMTOrder_Clear(this: *mut IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTOrder_Print(this: *const IMTOrder, string: *mut MTAPISTR) -> LPCWSTR {
    (*(*this).impl_ptr).print(string)
}

unsafe extern "C" fn IMTOrder_Order(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).order()
}

unsafe extern "C" fn IMTOrder_ExternalID(this: *const IMTOrder) -> LPCWSTR {
    (*(*this).impl_ptr).external_id()
}

unsafe extern "C" fn IMTOrder_ExternalID1(this: *mut IMTOrder, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).external_id1(id)
}

unsafe extern "C" fn IMTOrder_Login(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).login()
}

unsafe extern "C" fn IMTOrder_Login1(this: *mut IMTOrder, order: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).login1(order)
}

unsafe extern "C" fn IMTOrder_Dealer(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).dealer()
}

unsafe extern "C" fn IMTOrder_Dealer1(this: *mut IMTOrder, dealer: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).dealer1(dealer)
}

unsafe extern "C" fn IMTOrder_Symbol(this: *const IMTOrder) -> LPCWSTR {
    (*(*this).impl_ptr).symbol()
}

unsafe extern "C" fn IMTOrder_Symbol1(this: *mut IMTOrder, symbol: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).symbol1(symbol)
}

unsafe extern "C" fn IMTOrder_Digits(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).digits()
}

unsafe extern "C" fn IMTOrder_Digits1(this: *mut IMTOrder, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits1(digits)
}

unsafe extern "C" fn IMTOrder_DigitsCurrency(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).digits_currency()
}

unsafe extern "C" fn IMTOrder_DigitsCurrency1(this: *mut IMTOrder, digits: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).digits_currency1(digits)
}

unsafe extern "C" fn IMTOrder_ContractSize(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).contract_size()
}

unsafe extern "C" fn IMTOrder_ContractSize1(this: *mut IMTOrder, contract_size: f64) -> MTAPIRES {
    (*(*this).impl_ptr).contract_size1(contract_size)
}

unsafe extern "C" fn IMTOrder_State(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).state()
}

unsafe extern "C" fn IMTOrder_Reason(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).reason()
}

unsafe extern "C" fn IMTOrder_TimeSetup(this: *const IMTOrder) -> INT64 {
    (*(*this).impl_ptr).time_setup()
}

unsafe extern "C" fn IMTOrder_TimeSetup1(this: *mut IMTOrder, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_setup1(time)
}

unsafe extern "C" fn IMTOrder_TimeExpiration(this: *const IMTOrder) -> INT64 {
    (*(*this).impl_ptr).time_expiration()
}

unsafe extern "C" fn IMTOrder_TimeExpiration1(this: *mut IMTOrder, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_expiration1(time)
}

unsafe extern "C" fn IMTOrder_TimeDone(this: *const IMTOrder) -> INT64 {
    (*(*this).impl_ptr).time_done()
}

unsafe extern "C" fn IMTOrder_TimeDone1(this: *mut IMTOrder, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_done1(time)
}

unsafe extern "C" fn IMTOrder_Type(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).type_()
}

unsafe extern "C" fn IMTOrder_Type1(this: *mut IMTOrder, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type1(type_)
}

unsafe extern "C" fn IMTOrder_TypeFill(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).type_fill()
}

unsafe extern "C" fn IMTOrder_TypeFill1(this: *mut IMTOrder, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type_fill1(type_)
}

unsafe extern "C" fn IMTOrder_TypeTime(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).type_time()
}

unsafe extern "C" fn IMTOrder_TypeTime1(this: *mut IMTOrder, type_: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).type_time1(type_)
}

unsafe extern "C" fn IMTOrder_PriceOrder(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).price_order()
}

unsafe extern "C" fn IMTOrder_PriceOrder1(this: *mut IMTOrder, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_order1(price)
}

unsafe extern "C" fn IMTOrder_PriceTrigger(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).price_trigger()
}

unsafe extern "C" fn IMTOrder_PriceTrigger1(this: *mut IMTOrder, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_trigger1(price)
}

unsafe extern "C" fn IMTOrder_PriceCurrent(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).price_current()
}

unsafe extern "C" fn IMTOrder_PriceCurrent1(this: *mut IMTOrder, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_current1(price)
}

unsafe extern "C" fn IMTOrder_PriceSL(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).price_sl()
}

unsafe extern "C" fn IMTOrder_PriceSL1(this: *mut IMTOrder, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_sl1(price)
}

unsafe extern "C" fn IMTOrder_PriceTP(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).price_tp()
}

unsafe extern "C" fn IMTOrder_PriceTP1(this: *mut IMTOrder, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).price_tp1(price)
}

unsafe extern "C" fn IMTOrder_VolumeInitial(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).volume_initial()
}

unsafe extern "C" fn IMTOrder_VolumeInitial1(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_initial1(volume)
}

unsafe extern "C" fn IMTOrder_VolumeCurrent(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).volume_current()
}

unsafe extern "C" fn IMTOrder_VolumeCurrent1(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_current1(volume)
}

unsafe extern "C" fn IMTOrder_ExpertID(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).expert_id()
}

unsafe extern "C" fn IMTOrder_ExpertID1(this: *mut IMTOrder, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).expert_id1(id)
}

unsafe extern "C" fn IMTOrder_PositionID(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).position_id()
}

unsafe extern "C" fn IMTOrder_PositionID1(this: *mut IMTOrder, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).position_id1(id)
}

unsafe extern "C" fn IMTOrder_Comment(this: *const IMTOrder) -> LPCWSTR {
    (*(*this).impl_ptr).comment()
}

unsafe extern "C" fn IMTOrder_Comment1(this: *mut IMTOrder, comment: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).comment1(comment)
}

unsafe extern "C" fn IMTOrder_ActivationMode(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).activation_mode()
}

unsafe extern "C" fn IMTOrder_ActivationTime(this: *const IMTOrder) -> INT64 {
    (*(*this).impl_ptr).activation_time()
}

unsafe extern "C" fn IMTOrder_ActivationPrice(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).activation_price()
}

unsafe extern "C" fn IMTOrder_ActivationFlags(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).activation_flags()
}

unsafe extern "C" fn IMTOrder_ApiDataSet(
    this: *mut IMTOrder,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set(app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataSet1(
    this: *mut IMTOrder,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set1(app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataSet2(
    this: *mut IMTOrder,
    app_id: USHORT,
    id: UCHAR,
    value: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set2(app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataGet(
    this: *const IMTOrder,
    app_id: USHORT,
    id: UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get(app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataGet1(
    this: *const IMTOrder,
    app_id: USHORT,
    id: UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get1(app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataGet2(
    this: *const IMTOrder,
    app_id: USHORT,
    id: UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get2(app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataClear(this: *mut IMTOrder, app_id: USHORT) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear(app_id)
}

unsafe extern "C" fn IMTOrder_ApiDataClearAll(this: *mut IMTOrder) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear_all()
}

unsafe extern "C" fn IMTOrder_TimeSetupMsc(this: *const IMTOrder) -> INT64 {
    (*(*this).impl_ptr).time_setup_msc()
}

unsafe extern "C" fn IMTOrder_TimeSetupMsc1(this: *mut IMTOrder, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_setup_msc1(time)
}

unsafe extern "C" fn IMTOrder_TimeDoneMsc(this: *const IMTOrder) -> INT64 {
    (*(*this).impl_ptr).time_done_msc()
}

unsafe extern "C" fn IMTOrder_TimeDoneMsc1(this: *mut IMTOrder, time: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).time_done_msc1(time)
}

unsafe extern "C" fn IMTOrder_ActivationMode1(this: *mut IMTOrder, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).activation_mode1(mode)
}

unsafe extern "C" fn IMTOrder_ActivationTime1(this: *mut IMTOrder, atm: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).activation_time1(atm)
}

unsafe extern "C" fn IMTOrder_ActivationPrice1(this: *mut IMTOrder, price: f64) -> MTAPIRES {
    (*(*this).impl_ptr).activation_price1(price)
}

unsafe extern "C" fn IMTOrder_ActivationFlags1(this: *mut IMTOrder, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).activation_flags1(flags)
}

unsafe extern "C" fn IMTOrder_RateMargin(this: *const IMTOrder) -> f64 {
    (*(*this).impl_ptr).rate_margin()
}

unsafe extern "C" fn IMTOrder_RateMargin1(this: *mut IMTOrder, rate: f64) -> MTAPIRES {
    (*(*this).impl_ptr).rate_margin1(rate)
}

unsafe extern "C" fn IMTOrder_ApiDataUpdate(
    this: *mut IMTOrder,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update(pos, app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataUpdate1(
    this: *mut IMTOrder,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataUpdate2(
    this: *mut IMTOrder,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataNext(
    this: *const IMTOrder,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next(pos, app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataNext1(
    this: *const IMTOrder,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTOrder_ApiDataNext2(
    this: *const IMTOrder,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTOrder_OrderSet(this: *mut IMTOrder, order: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).order_set(order)
}

unsafe extern "C" fn IMTOrder_PositionByID(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).position_by_id()
}

unsafe extern "C" fn IMTOrder_PositionByID1(this: *mut IMTOrder, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).position_by_id1(id)
}

unsafe extern "C" fn IMTOrder_ModificationFlags(this: *const IMTOrder) -> UINT {
    (*(*this).impl_ptr).modification_flags()
}

unsafe extern "C" fn IMTOrder_StateSet(this: *mut IMTOrder, state: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).state_set(state)
}

unsafe extern "C" fn IMTOrder_ReasonSet(this: *mut IMTOrder, reason: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).reason_set(reason)
}

unsafe extern "C" fn IMTOrder_VolumeInitialExt(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).volume_initial_ext()
}

unsafe extern "C" fn IMTOrder_VolumeInitialExt1(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_initial_ext1(volume)
}

unsafe extern "C" fn IMTOrder_VolumeCurrentExt(this: *const IMTOrder) -> UINT64 {
    (*(*this).impl_ptr).volume_current_ext()
}

unsafe extern "C" fn IMTOrder_VolumeCurrentExt1(this: *mut IMTOrder, volume: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).volume_current_ext1(volume)
}

pub const fn new() -> IMTOrder__bindgen_vtable {
    IMTOrder__bindgen_vtable {
        IMTOrder_Release,
        IMTOrder_Assign,
        IMTOrder_Clear,
        IMTOrder_Print,
        IMTOrder_Order,
        IMTOrder_ExternalID,
        IMTOrder_ExternalID1,
        IMTOrder_Login,
        IMTOrder_Login1,
        IMTOrder_Dealer,
        IMTOrder_Dealer1,
        IMTOrder_Symbol,
        IMTOrder_Symbol1,
        IMTOrder_Digits,
        IMTOrder_Digits1,
        IMTOrder_DigitsCurrency,
        IMTOrder_DigitsCurrency1,
        IMTOrder_ContractSize,
        IMTOrder_ContractSize1,
        IMTOrder_State,
        IMTOrder_Reason,
        IMTOrder_TimeSetup,
        IMTOrder_TimeSetup1,
        IMTOrder_TimeExpiration,
        IMTOrder_TimeExpiration1,
        IMTOrder_TimeDone,
        IMTOrder_TimeDone1,
        IMTOrder_Type,
        IMTOrder_Type1,
        IMTOrder_TypeFill,
        IMTOrder_TypeFill1,
        IMTOrder_TypeTime,
        IMTOrder_TypeTime1,
        IMTOrder_PriceOrder,
        IMTOrder_PriceOrder1,
        IMTOrder_PriceTrigger,
        IMTOrder_PriceTrigger1,
        IMTOrder_PriceCurrent,
        IMTOrder_PriceCurrent1,
        IMTOrder_PriceSL,
        IMTOrder_PriceSL1,
        IMTOrder_PriceTP,
        IMTOrder_PriceTP1,
        IMTOrder_VolumeInitial,
        IMTOrder_VolumeInitial1,
        IMTOrder_VolumeCurrent,
        IMTOrder_VolumeCurrent1,
        IMTOrder_ExpertID,
        IMTOrder_ExpertID1,
        IMTOrder_PositionID,
        IMTOrder_PositionID1,
        IMTOrder_Comment,
        IMTOrder_Comment1,
        IMTOrder_ActivationMode,
        IMTOrder_ActivationTime,
        IMTOrder_ActivationPrice,
        IMTOrder_ActivationFlags,
        IMTOrder_ApiDataSet,
        IMTOrder_ApiDataSet1,
        IMTOrder_ApiDataSet2,
        IMTOrder_ApiDataGet,
        IMTOrder_ApiDataGet1,
        IMTOrder_ApiDataGet2,
        IMTOrder_ApiDataClear,
        IMTOrder_ApiDataClearAll,
        IMTOrder_TimeSetupMsc,
        IMTOrder_TimeSetupMsc1,
        IMTOrder_TimeDoneMsc,
        IMTOrder_TimeDoneMsc1,
        IMTOrder_ActivationMode1,
        IMTOrder_ActivationTime1,
        IMTOrder_ActivationPrice1,
        IMTOrder_ActivationFlags1,
        IMTOrder_RateMargin,
        IMTOrder_RateMargin1,
        IMTOrder_ApiDataUpdate,
        IMTOrder_ApiDataUpdate1,
        IMTOrder_ApiDataUpdate2,
        IMTOrder_ApiDataNext,
        IMTOrder_ApiDataNext1,
        IMTOrder_ApiDataNext2,
        IMTOrder_OrderSet,
        IMTOrder_PositionByID,
        IMTOrder_PositionByID1,
        IMTOrder_ModificationFlags,
        IMTOrder_StateSet,
        IMTOrder_ReasonSet,
        IMTOrder_VolumeInitialExt,
        IMTOrder_VolumeInitialExt1,
        IMTOrder_VolumeCurrentExt,
        IMTOrder_VolumeCurrentExt1
    }
}