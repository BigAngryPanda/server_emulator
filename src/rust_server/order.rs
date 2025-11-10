use crate::mt5_apiserver::{
    IMTOrder,
    IMTOrder__bindgen_vtable
};

use crate::interfaces::mt_order::MTOrder;

use crate::vtable_impl;

pub struct Order {
    dummy: char
}

impl Order {
    const MT_ORDER_VTABLE: IMTOrder__bindgen_vtable = vtable_impl::mt_order::new();

    fn new() -> Order {
        Order {
            dummy: 'f'
        }
    }

    pub fn alloc() -> *mut Order {
        Box::into_raw(Box::new(Order::new()))
    }

    pub fn alloc_mt_request(self_ptr: *mut dyn MTOrder) -> *mut IMTOrder {
        let mt_request = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTOrder>()) as *mut IMTOrder
        };

        unsafe {
            (*mt_request).vtable_ = &Self::MT_ORDER_VTABLE as *const IMTOrder__bindgen_vtable;
            (*mt_request).impl_ptr = self_ptr;
        }

        mt_request
    }
}

impl MTOrder for Order {
    fn release(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut Order as *mut u8, std::alloc::Layout::new::<Order>())
        };
    }
}
