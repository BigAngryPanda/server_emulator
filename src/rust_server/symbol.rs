use crate::mt5_apiserver::{
    IMTConSymbol,
    IMTConSymbol__bindgen_vtable
};

use crate::interfaces::con_symbol::MTConSymbol;

use crate::vtable_impl;

pub struct Symbol {
    dummy: char
}

impl Symbol {
    const MT_CON_SYMBOL_VTABLE: IMTConSymbol__bindgen_vtable = vtable_impl::con_symbol::new();

    fn new() -> Symbol {
        Symbol {
            dummy: 'f'
        }
    }

    pub fn alloc() -> *mut Symbol {
        Box::into_raw(Box::new(Symbol::new()))
    }

    pub fn alloc_mt_con_symbol(self_ptr: *mut dyn MTConSymbol) -> *mut IMTConSymbol {
        let mt_con_symbol = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTConSymbol>()) as *mut IMTConSymbol
        };

        unsafe {
            (*mt_con_symbol).vtable_ = &Self::MT_CON_SYMBOL_VTABLE as *const IMTConSymbol__bindgen_vtable;
            (*mt_con_symbol).impl_ptr = self_ptr;
        }

        mt_con_symbol
    }
}

impl MTConSymbol for Symbol {
    fn release(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut Symbol as *mut u8, std::alloc::Layout::new::<Symbol>())
        };
    }
}
