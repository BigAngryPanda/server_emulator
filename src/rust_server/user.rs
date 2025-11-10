use crate::mt5_apiserver::{
    IMTUser,
    IMTUser__bindgen_vtable
};

use crate::interfaces::mt_user::MTUser;

use crate::vtable_impl;

pub struct User {
    dummy: char
}

impl User {
    const MT_USER_VTABLE: IMTUser__bindgen_vtable = vtable_impl::mt_user::new();

    pub fn new() -> User {
        User {
            dummy: 'f'
        }
    }

    pub fn alloc() -> *mut User {
        Box::into_raw(Box::new(User::new()))
    }

    pub fn alloc_mt_user(self_ptr: *mut User) -> *mut IMTUser {
        let mt_user = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTUser>()) as *mut IMTUser
        };

        unsafe {
            (*mt_user).vtable_ = &Self::MT_USER_VTABLE as *const IMTUser__bindgen_vtable;
            (*mt_user).impl_ptr = self_ptr;
        }

        mt_user
    }
}

impl MTUser for User {
    fn release(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut User as *mut u8, std::alloc::Layout::new::<User>())
        };
    }
}