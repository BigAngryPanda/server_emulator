use crate::lua_server::lua_object::LuaConstructible;

pub struct OptionPtr<T : LuaConstructible> {
    ptr: *mut T::MTType
}

impl<T: LuaConstructible> OptionPtr<T>  {
    pub fn new(ptr: *mut T::MTType) -> OptionPtr<T> {
        OptionPtr { ptr }
    }

    pub fn null_mut() -> OptionPtr<T> {
        OptionPtr::new(std::ptr::null_mut())
    }

    pub fn get(&self) -> *const T::MTType {
        self.ptr
    }

    pub fn get_mut(&self) -> *mut T::MTType {
        self.ptr
    }
}

impl<T: LuaConstructible> Drop for OptionPtr<T> {
    fn drop(&mut self) {
        if self.ptr != std::ptr::null_mut() {
            T::free_by_ptr(self.ptr);
        }
    }
}