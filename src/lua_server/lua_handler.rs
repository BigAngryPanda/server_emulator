use mlua::{
    Lua,
    HookTriggers,
    VmState,
    Function,
    Value,
    MultiValue,
    Table
};

use std::sync::mpsc::{
    channel,
    Sender,
    Receiver
};

pub type CallingMsg = (Table, &'static str, MultiValue, Sender<Value>);

pub struct LuaHandler {
    lua: Lua,
    own_pid: u32,
    sender: Sender<CallingMsg>
}

impl LuaHandler {
    pub fn new(lua: Lua) -> LuaHandler {
        let (send, recv): (Sender<CallingMsg>, Receiver<CallingMsg>) = channel();

        lua.set_hook(HookTriggers::EVERY_LINE.every_nth_instruction(5), move |_lua, _debug| {
            if let Ok((lua_impl, name, args, tx)) = recv.try_recv() {
                let res = Self::perform_call(lua_impl, name, args);

                let _ = tx.send(res);
            }

            Ok(VmState::Continue)
        }).unwrap();

        LuaHandler {
            lua,
            own_pid: std::process::id(),
            sender: send
        }
    }

    pub fn call(&self, lua_impl: Table, name: &'static str, args: MultiValue) -> Value {
        if std::process::id() == self.own_pid {
            Self::perform_call(lua_impl, name, args)
        }
        else {
            let (send, recv): (Sender<Value>, Receiver<Value>) = channel();

            let _ = self.sender.send((lua_impl, name, args, send));

            recv.recv().unwrap_or(Value::Nil)
        }
    }

    fn perform_call(lua_impl: Table, name: &'static str, args: MultiValue) -> Value {
        if let Ok(func) = lua_impl.get::<Function>(name) {
            func.call::<Value>(args).unwrap_or(Value::Nil)
        }
        else {
            Value::Nil
        }
    }
}
