use crate::mt5_apiserver::{
    IMTRequest,
    IMTRequest__bindgen_vtable
};

use crate::interfaces::mt_request::MTRequest;

use crate::vtable_impl;

pub struct Request {
    dummy: char
}

impl Request {
    const MT_REQUEST_VTABLE: IMTRequest__bindgen_vtable = vtable_impl::mt_request::new();

    fn new() -> Request {
        Request {
            dummy: 'f'
        }
    }

    pub fn alloc() -> *mut Request {
        Box::into_raw(Box::new(Request::new()))
    }

    pub fn alloc_mt_request(self_ptr: *mut dyn MTRequest) -> *mut IMTRequest {
        let mt_request = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTRequest>()) as *mut IMTRequest
        };

        unsafe {
            (*mt_request).vtable_ = &Self::MT_REQUEST_VTABLE as *const IMTRequest__bindgen_vtable;
            (*mt_request).impl_ptr = self_ptr;
        }

        mt_request
    }
}

impl MTRequest for Request {
    fn release(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut Request as *mut u8, std::alloc::Layout::new::<Request>())
        };
    }
}