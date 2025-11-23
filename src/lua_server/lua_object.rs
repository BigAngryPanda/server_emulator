use mlua::{
    Table,
    Value,
    MultiValue
};

use crate::lua_server::{
    lua_handler::LuaHandler,
    option_ptr::OptionPtr
};

use crate::conversion::{
    str_to_utf16,
    FromInt64
};

use std::sync::Arc;

pub trait LuaObject {
    fn lua_impl(&self) -> Table;

    fn lua_handler(&self) -> &LuaHandler;

    fn call(&self, name: &str) -> Value {
        self.call_with_args(name, &[])
    }

    fn call_with_args(&self, name: &str, args: &[Value]) -> Value {
        self.lua_handler().call(self.lua_impl(), name, self.prepare_args(args))
    }

    fn call_int<T: FromInt64>(&self, name: &str, default: T) -> T {
        self.call_int_with_args(name, &[], default)
    }

    fn call_int_with_args<T: FromInt64>(&self, name: &str, args: &[Value], default: T) -> T {
        match self.call_with_args(name, args) {
            Value::Integer(result) => {
                FromInt64::from_i64(result)
            }
            _ => {
                default
            }
        }
    }

    fn call_str(&self, name: &str) -> Vec<u16> {
        self.call_str_with_args(name, &[])
    }

    fn call_str_with_args(&self, name: &str, args: &[Value]) -> Vec<u16> {
        match self.call_with_args(name, args) {
            Value::String(lua_str) => {
                str_to_utf16(&lua_str.to_string_lossy())
            }
            _ => {
                Vec::new()
            }
        }
    }

    fn prepare_args(&self, args: &[Value]) -> MultiValue {
        let mut result = MultiValue::new();

        result.push_back(Value::Table(self.lua_impl()));

        for arg in args {
            result.push_back(arg.clone());
        }

        result
    }
}

pub fn as_lua_value<T: ?Sized, U: LuaObject>(arg: *mut T) -> Value {
    let impl_ptr: &mut U = unsafe { &mut *(arg as *mut U) };

    Value::Table(impl_ptr.lua_impl())
}

pub trait LuaConstructible : Sized {
    type MTType;

    fn new(lua: Arc<LuaHandler>, lua_impl: Table) -> Self;
    fn mt_type(self_ptr: *mut Self, mt_obj: &mut Self::MTType);
    fn free_by_ptr(self_ptr: *mut Self::MTType);

    fn alloc(lua: Arc<LuaHandler>, lua_impl: Table) -> *mut Self::MTType {
        let obj: *mut Self = Box::into_raw(Box::new(Self::new(lua, lua_impl)));

        let mt_obj = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<Self::MTType>()) as *mut Self::MTType
        };

        unsafe { Self::mt_type(obj, &mut *mt_obj) };

        mt_obj
    }

    fn alloc_if(lua: Arc<LuaHandler>, lua_impl: Value) -> OptionPtr<Self> {
        match lua_impl {
            Value::Table(table) => {
                OptionPtr::new(Self::alloc(lua, table))
            }
            _ => {
                OptionPtr::null_mut()
            }
        }
    }

    fn free(&mut self) {
        unsafe {
            std::ptr::drop_in_place(self);
            std::alloc::dealloc(self as *mut Self as *mut u8, std::alloc::Layout::new::<Self>());
        };
    }
}
