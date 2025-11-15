use mlua::{
    Table,
    Value,
    MultiValue
};

pub trait LuaObject {
    fn lua_impl(&self) -> Table;
}

pub fn as_lua_arg<T: ?Sized, U: LuaObject>(arg: *mut T) -> MultiValue {
    let impl_ptr: &mut U = unsafe { &mut *(arg as *mut U) };

    let mut result = MultiValue::new();

    result.push_back(Value::Table(impl_ptr.lua_impl()));

    result
}