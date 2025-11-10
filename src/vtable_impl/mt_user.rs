use crate::mt5_apiserver::*;

unsafe extern "C" fn IMTUser_Release(this: *mut IMTUser) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTUser>());
}

unsafe extern "C" fn IMTUser_Assign(this: *mut IMTUser, user: *const IMTUser) -> MTAPIRES {
    (*(*this).impl_ptr).assign(&*user)
}

unsafe extern "C" fn IMTUser_Clear(this: *mut IMTUser) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTUser_Login(this: *const IMTUser) -> UINT64 {
    (*(*this).impl_ptr).login()
}

unsafe extern "C" fn IMTUser_Login1(this: *mut IMTUser, login: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).login1(login)
}

unsafe extern "C" fn IMTUser_Group(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).group()
}

unsafe extern "C" fn IMTUser_Group1(this: *mut IMTUser, group: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).group1(group)
}

unsafe extern "C" fn IMTUser_CertSerialNumber(this: *const IMTUser) -> UINT64 {
    (*(*this).impl_ptr).cert_serial_number()
}

unsafe extern "C" fn IMTUser_Rights(this: *const IMTUser) -> UINT64 {
    (*(*this).impl_ptr).rights()
}

unsafe extern "C" fn IMTUser_Rights1(this: *mut IMTUser, rights: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).rights1(rights)
}

unsafe extern "C" fn IMTUser_Registration(this: *const IMTUser) -> INT64 {
    (*(*this).impl_ptr).registration()
}

unsafe extern "C" fn IMTUser_LastAccess(this: *const IMTUser) -> INT64 {
    (*(*this).impl_ptr).last_access()
}

unsafe extern "C" fn IMTUser_LastIP(this: *const IMTUser, ip: *mut MTAPISTR) -> LPCWSTR {
    (*(*this).impl_ptr).last_ip(&mut *ip)
}

unsafe extern "C" fn IMTUser_Name(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).name()
}

unsafe extern "C" fn IMTUser_Name1(this: *mut IMTUser, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).name1(name)
}

unsafe extern "C" fn IMTUser_Company(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).company()
}

unsafe extern "C" fn IMTUser_Company1(this: *mut IMTUser, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).company1(id)
}

unsafe extern "C" fn IMTUser_Account(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).account()
}

unsafe extern "C" fn IMTUser_Account1(this: *mut IMTUser, account: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).account1(account)
}

unsafe extern "C" fn IMTUser_Country(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).country()
}

unsafe extern "C" fn IMTUser_Country1(this: *mut IMTUser, account: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).country1(account)
}

unsafe extern "C" fn IMTUser_Language(this: *const IMTUser) -> UINT {
    (*(*this).impl_ptr).language()
}

unsafe extern "C" fn IMTUser_Language1(this: *mut IMTUser, language: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).language1(language)
}

unsafe extern "C" fn IMTUser_City(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).city()
}

unsafe extern "C" fn IMTUser_City1(this: *mut IMTUser, city: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).city1(city)
}

unsafe extern "C" fn IMTUser_State(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).state()
}

unsafe extern "C" fn IMTUser_State1(this: *mut IMTUser, state: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).state1(state)
}

unsafe extern "C" fn IMTUser_ZIPCode(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).zipcode()
}

unsafe extern "C" fn IMTUser_ZIPCode1(this: *mut IMTUser, code: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).zipcode1(code)
}

unsafe extern "C" fn IMTUser_Address(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).address()
}

unsafe extern "C" fn IMTUser_Address1(this: *mut IMTUser, code: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).address1(code)
}

unsafe extern "C" fn IMTUser_Phone(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).phone()
}

unsafe extern "C" fn IMTUser_Phone1(this: *mut IMTUser, phone: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).phone1(phone)
}

unsafe extern "C" fn IMTUser_EMail(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).email()
}

unsafe extern "C" fn IMTUser_EMail1(this: *mut IMTUser, email: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).email1(email)
}

unsafe extern "C" fn IMTUser_ID(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).id()
}

unsafe extern "C" fn IMTUser_ID1(this: *mut IMTUser, email: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).id1(email)
}

unsafe extern "C" fn IMTUser_Status(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).status()
}

unsafe extern "C" fn IMTUser_Status1(this: *mut IMTUser, id: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).status1(id)
}

unsafe extern "C" fn IMTUser_Comment(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).comment()
}

unsafe extern "C" fn IMTUser_Comment1(this: *mut IMTUser, comment: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).comment1(comment)
}

unsafe extern "C" fn IMTUser_Color(this: *const IMTUser) -> COLORREF {
    (*(*this).impl_ptr).color()
}

unsafe extern "C" fn IMTUser_Color1(this: *mut IMTUser, color: COLORREF) -> MTAPIRES {
    (*(*this).impl_ptr).color1(color)
}

unsafe extern "C" fn IMTUser_PhonePassword(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).phone_password()
}

unsafe extern "C" fn IMTUser_PhonePassword1(this: *mut IMTUser, password: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).phone_password1(password)
}

unsafe extern "C" fn IMTUser_Leverage(this: *const IMTUser) -> UINT {
    (*(*this).impl_ptr).leverage()
}

unsafe extern "C" fn IMTUser_Leverage1(this: *mut IMTUser, leverage: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).leverage1(leverage)
}

unsafe extern "C" fn IMTUser_Agent(this: *const IMTUser) -> UINT64 {
    (*(*this).impl_ptr).agent()
}

unsafe extern "C" fn IMTUser_Agent1(this: *mut IMTUser, agent: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).agent1(agent)
}

unsafe extern "C" fn IMTUser_Balance(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).balance()
}

unsafe extern "C" fn IMTUser_Credit(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).credit()
}

unsafe extern "C" fn IMTUser_InterestRate(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).interest_rate()
}

unsafe extern "C" fn IMTUser_CommissionDaily(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).commission_daily()
}

unsafe extern "C" fn IMTUser_CommissionMonthly(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).commission_monthly()
}

unsafe extern "C" fn IMTUser_CommissionAgentDaily(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).commission_agent_daily()
}

unsafe extern "C" fn IMTUser_CommissionAgentMonthly(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).commission_agent_monthly()
}

unsafe extern "C" fn IMTUser_BalancePrevDay(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).balance_prev_day()
}

unsafe extern "C" fn IMTUser_BalancePrevMonth(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).balance_prev_month()
}

unsafe extern "C" fn IMTUser_EquityPrevDay(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).equity_prev_day()
}

unsafe extern "C" fn IMTUser_EquityPrevMonth(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).equity_prev_month()
}

unsafe extern "C" fn IMTUser_ApiDataSet(
    this: *mut IMTUser,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set(app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataSet1(
    this: *mut IMTUser,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set1(app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataSet2(this: *mut IMTUser, app_id: USHORT, id: UCHAR, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_set2(app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataGet(
    this: *const IMTUser,
    app_id: USHORT,
    id: UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get(app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataGet1(
    this: *const IMTUser,
    app_id: USHORT,
    id: UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get1(app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataGet2(
    this: *const IMTUser,
    app_id: USHORT,
    id: UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_get2(app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataClear(this: *mut IMTUser, app_id: USHORT) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear(app_id)
}

unsafe extern "C" fn IMTUser_ApiDataClearAll(this: *mut IMTUser) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_clear_all()
}

unsafe extern "C" fn IMTUser_ExternalAccountAdd(this: *mut IMTUser, gateway_id: UINT64, account: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).external_account_add(gateway_id, account)
}

unsafe extern "C" fn IMTUser_ExternalAccountUpdate(
    this: *mut IMTUser,
    pos: UINT,
    gateway_id: UINT64,
    account: LPCWSTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).external_account_update(pos, gateway_id, account)
}

unsafe extern "C" fn IMTUser_ExternalAccountDelete(this: *mut IMTUser, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).external_account_delete(pos)
}

unsafe extern "C" fn IMTUser_ExternalAccountClear(this: *mut IMTUser) -> MTAPIRES {
    (*(*this).impl_ptr).external_account_clear()
}

unsafe extern "C" fn IMTUser_ExternalAccountTotal(this: *const IMTUser) -> UINT {
    (*(*this).impl_ptr).external_account_total()
}

unsafe extern "C" fn IMTUser_ExternalAccountNext(
    this: *const IMTUser,
    pos: UINT,
    gateway_id: *mut UINT64,
    account: *mut MTAPISTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).external_account_next(pos, gateway_id, &mut *account)
}

unsafe extern "C" fn IMTUser_ExternalAccountGet(
    this: *const IMTUser,
    gateway_id: UINT64,
    account: *mut MTAPISTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).external_account_get(gateway_id, &mut *account)
}

unsafe extern "C" fn IMTUser_LastPassChange(this: *const IMTUser) -> INT64 {
    (*(*this).impl_ptr).last_pass_change()
}

unsafe extern "C" fn IMTUser_MQID(this: *const IMTUser, mqid: *mut MTAPISTR) -> LPCWSTR {
    (*(*this).impl_ptr).mqid(&mut *mqid)
}

unsafe extern "C" fn IMTUser_ApiDataUpdate(
    this: *mut IMTUser,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update(pos, app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataUpdate1(
    this: *mut IMTUser,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataUpdate2(
    this: *mut IMTUser,
    pos: UINT,
    app_id: USHORT,
    id: UCHAR,
    value: f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_update2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataNext(
    this: *const IMTUser,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut INT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next(pos, app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataNext1(
    this: *const IMTUser,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut UINT64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next1(pos, app_id, id, value)
}

unsafe extern "C" fn IMTUser_ApiDataNext2(
    this: *const IMTUser,
    pos: UINT,
    app_id: *mut USHORT,
    id: *mut UCHAR,
    value: *mut f64,
) -> MTAPIRES {
    (*(*this).impl_ptr).api_data_next2(pos, app_id, id, value)
}

unsafe extern "C" fn IMTUser_PasswordHash(
    this: *const IMTUser,
    type_: UINT,
    password_hash: *mut MTAPISTR,
) -> MTAPIRES {
    (*(*this).impl_ptr).password_hash(type_, &mut *password_hash)
}

unsafe extern "C" fn IMTUser_LeadCampaign(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).lead_campaign()
}

unsafe extern "C" fn IMTUser_LeadCampaign1(this: *mut IMTUser, lead_campaign: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).lead_campaign1(lead_campaign)
}

unsafe extern "C" fn IMTUser_LeadSource(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).lead_source()
}

unsafe extern "C" fn IMTUser_LeadSource1(this: *mut IMTUser, lead_source: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).lead_source1(lead_source)
}

unsafe extern "C" fn IMTUser_ClientID(this: *const IMTUser) -> UINT64 {
    (*(*this).impl_ptr).client_id()
}

unsafe extern "C" fn IMTUser_ClientID1(this: *mut IMTUser, id: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).client_id1(id)
}

unsafe extern "C" fn IMTUser_FirstName(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).first_name()
}

unsafe extern "C" fn IMTUser_FirstName1(this: *mut IMTUser, first_name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).first_name1(first_name)
}

unsafe extern "C" fn IMTUser_LastName(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).last_name()
}

unsafe extern "C" fn IMTUser_LastName1(this: *mut IMTUser, last_name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).last_name1(last_name)
}

unsafe extern "C" fn IMTUser_MiddleName(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).middle_name()
}

unsafe extern "C" fn IMTUser_MiddleName1(this: *mut IMTUser, middle_name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).middle_name1(middle_name)
}

unsafe extern "C" fn IMTUser_RegistrationSet(this: *mut IMTUser, datetime: INT64) -> MTAPIRES {
    (*(*this).impl_ptr).registration_set(datetime)
}

unsafe extern "C" fn IMTUser_OTPSecret(this: *const IMTUser) -> LPCWSTR {
    (*(*this).impl_ptr).otpsecret()
}

unsafe extern "C" fn IMTUser_OTPSecret1(this: *mut IMTUser, otp_secret: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).otpsecret1(otp_secret)
}

unsafe extern "C" fn IMTUser_LimitOrders(this: *const IMTUser) -> UINT {
    (*(*this).impl_ptr).limit_orders()
}

unsafe extern "C" fn IMTUser_LimitOrders1(this: *mut IMTUser, id: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).limit_orders1(id)
}

unsafe extern "C" fn IMTUser_LimitPositionsValue(this: *const IMTUser) -> f64 {
    (*(*this).impl_ptr).limit_positions_value()
}

unsafe extern "C" fn IMTUser_LimitPositionsValue1(this: *mut IMTUser, value: f64) -> MTAPIRES {
    (*(*this).impl_ptr).limit_positions_value1(value)
}

pub const fn new() -> IMTUser__bindgen_vtable {
    IMTUser__bindgen_vtable {
        IMTUser_Release,
        IMTUser_Assign,
        IMTUser_Clear,
        IMTUser_Login,
        IMTUser_Login1,
        IMTUser_Group,
        IMTUser_Group1,
        IMTUser_CertSerialNumber,
        IMTUser_Rights,
        IMTUser_Rights1,
        IMTUser_Registration,
        IMTUser_LastAccess,
        IMTUser_LastIP,
        IMTUser_Name,
        IMTUser_Name1,
        IMTUser_Company,
        IMTUser_Company1,
        IMTUser_Account,
        IMTUser_Account1,
        IMTUser_Country,
        IMTUser_Country1,
        IMTUser_Language,
        IMTUser_Language1,
        IMTUser_City,
        IMTUser_City1,
        IMTUser_State,
        IMTUser_State1,
        IMTUser_ZIPCode,
        IMTUser_ZIPCode1,
        IMTUser_Address,
        IMTUser_Address1,
        IMTUser_Phone,
        IMTUser_Phone1,
        IMTUser_EMail,
        IMTUser_EMail1,
        IMTUser_ID,
        IMTUser_ID1,
        IMTUser_Status,
        IMTUser_Status1,
        IMTUser_Comment,
        IMTUser_Comment1,
        IMTUser_Color,
        IMTUser_Color1,
        IMTUser_PhonePassword,
        IMTUser_PhonePassword1,
        IMTUser_Leverage,
        IMTUser_Leverage1,
        IMTUser_Agent,
        IMTUser_Agent1,
        IMTUser_Balance,
        IMTUser_Credit,
        IMTUser_InterestRate,
        IMTUser_CommissionDaily,
        IMTUser_CommissionMonthly,
        IMTUser_CommissionAgentDaily,
        IMTUser_CommissionAgentMonthly,
        IMTUser_BalancePrevDay,
        IMTUser_BalancePrevMonth,
        IMTUser_EquityPrevDay,
        IMTUser_EquityPrevMonth,
        IMTUser_ApiDataSet,
        IMTUser_ApiDataSet1,
        IMTUser_ApiDataSet2,
        IMTUser_ApiDataGet,
        IMTUser_ApiDataGet1,
        IMTUser_ApiDataGet2,
        IMTUser_ApiDataClear,
        IMTUser_ApiDataClearAll,
        IMTUser_ExternalAccountAdd,
        IMTUser_ExternalAccountUpdate,
        IMTUser_ExternalAccountDelete,
        IMTUser_ExternalAccountClear,
        IMTUser_ExternalAccountTotal,
        IMTUser_ExternalAccountNext,
        IMTUser_ExternalAccountGet,
        IMTUser_LastPassChange,
        IMTUser_MQID,
        IMTUser_ApiDataUpdate,
        IMTUser_ApiDataUpdate1,
        IMTUser_ApiDataUpdate2,
        IMTUser_ApiDataNext,
        IMTUser_ApiDataNext1,
        IMTUser_ApiDataNext2,
        IMTUser_PasswordHash,
        IMTUser_LeadCampaign,
        IMTUser_LeadCampaign1,
        IMTUser_LeadSource,
        IMTUser_LeadSource1,
        IMTUser_ClientID,
        IMTUser_ClientID1,
        IMTUser_FirstName,
        IMTUser_FirstName1,
        IMTUser_LastName,
        IMTUser_LastName1,
        IMTUser_MiddleName,
        IMTUser_MiddleName1,
        IMTUser_RegistrationSet,
        IMTUser_OTPSecret,
        IMTUser_OTPSecret1,
        IMTUser_LimitOrders,
        IMTUser_LimitOrders1,
        IMTUser_LimitPositionsValue,
        IMTUser_LimitPositionsValue1
    }
}
