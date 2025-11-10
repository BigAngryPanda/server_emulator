use crate::{
    mt5_apiserver,
    conversion
};

use mt5_apiserver::*;

use std::os::raw::c_int;

unsafe extern "C" fn IMTConPlugin_Release(this: *mut IMTConPlugin) {
    (*(*this).impl_ptr).release();

    std::alloc::dealloc(this as *mut u8, std::alloc::Layout::new::<IMTConPlugin>());
}

unsafe extern "C" fn IMTConPlugin_Assign(this: *mut IMTConPlugin, param: *const IMTConPlugin) -> MTAPIRES {
    (*(*this).impl_ptr).assign(&*param)
}

unsafe extern "C" fn IMTConPlugin_Clear(this: *mut IMTConPlugin) -> MTAPIRES {
    (*(*this).impl_ptr).clear()
}

unsafe extern "C" fn IMTConPlugin_Name1(this: *mut IMTConPlugin, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).name1(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTConPlugin_Name(this: *const IMTConPlugin) -> LPCWSTR {
    (*(*this).impl_ptr).name()
}

unsafe extern "C" fn IMTConPlugin_Server1(this: *mut IMTConPlugin, server: UINT64) -> MTAPIRES {
    (*(*this).impl_ptr).server1(server)
}

unsafe extern "C" fn IMTConPlugin_Server(this: *const IMTConPlugin) -> UINT64 {
    (*(*this).impl_ptr).server()
}

unsafe extern "C" fn IMTConPlugin_Module1(this: *mut IMTConPlugin, name: LPCWSTR) -> MTAPIRES {
    (*(*this).impl_ptr).module1(conversion::to_utf16_str(name))
}

unsafe extern "C" fn IMTConPlugin_Module(this: *const IMTConPlugin) -> LPCWSTR {
    (*(*this).impl_ptr).module()
}

unsafe extern "C" fn IMTConPlugin_Mode1(this: *mut IMTConPlugin, mode: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).mode1(mode)
}

unsafe extern "C" fn IMTConPlugin_Mode(this: *const IMTConPlugin) -> UINT {
    (*(*this).impl_ptr).mode()
}

unsafe extern "C" fn IMTConPlugin_ParameterAdd(this: *mut IMTConPlugin, param: *mut IMTConParam) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_add(&mut *param)
}

unsafe extern "C" fn IMTConPlugin_ParameterUpdate(
    this: *mut IMTConPlugin,
    pos: UINT,
    param: *const IMTConParam,
) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_update(pos, &*param)
}

unsafe extern "C" fn IMTConPlugin_ParameterDelete(this: *mut IMTConPlugin, pos: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_delete(pos)
}

unsafe extern "C" fn IMTConPlugin_ParameterClear(this: *mut IMTConPlugin) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_clear()
}

unsafe extern "C" fn IMTConPlugin_ParameterShift(
    this: *mut IMTConPlugin,
    pos: UINT,
    shift: c_int,
) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_shift(pos, shift)
}

unsafe extern "C" fn IMTConPlugin_ParameterTotal(this: *const IMTConPlugin) -> UINT {
    (*(*this).impl_ptr).parameter_total()
}

unsafe extern "C" fn IMTConPlugin_ParameterNext(
    this: *const IMTConPlugin,
    pos: UINT,
    param: *mut IMTConParam,
) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_next(pos, &mut *param)
}

unsafe extern "C" fn IMTConPlugin_ParameterGet(
    this: *const IMTConPlugin,
    name: LPCWSTR,
    param: *mut IMTConParam,
) -> MTAPIRES {
    (*(*this).impl_ptr).parameter_get(conversion::to_utf16_str(name), &mut *param)
}

unsafe extern "C" fn IMTConPlugin_Flags1(this: *mut IMTConPlugin, flags: UINT) -> MTAPIRES {
    (*(*this).impl_ptr).flags1(flags)
}

unsafe extern "C" fn IMTConPlugin_Flags(this: *const IMTConPlugin) -> UINT {
    (*(*this).impl_ptr).flags()
}

pub const fn new() -> IMTConPlugin__bindgen_vtable {
    IMTConPlugin__bindgen_vtable {
        IMTConPlugin_Release,
        IMTConPlugin_Assign,
        IMTConPlugin_Clear,
        IMTConPlugin_Name1,
        IMTConPlugin_Name,
        IMTConPlugin_Server1,
        IMTConPlugin_Server,
        IMTConPlugin_Module1,
        IMTConPlugin_Module,
        IMTConPlugin_Mode1,
        IMTConPlugin_Mode,
        IMTConPlugin_ParameterAdd,
        IMTConPlugin_ParameterUpdate,
        IMTConPlugin_ParameterDelete,
        IMTConPlugin_ParameterClear,
        IMTConPlugin_ParameterShift,
        IMTConPlugin_ParameterTotal,
        IMTConPlugin_ParameterNext,
        IMTConPlugin_ParameterGet,
        IMTConPlugin_Flags1,
        IMTConPlugin_Flags,
    }
}
