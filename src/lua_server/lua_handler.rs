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
    sender: Sender<CallingMsg>
}

impl LuaHandler {
    pub fn new(lua: Lua) -> LuaHandler {
        let (send, recv): (Sender<CallingMsg>, Receiver<CallingMsg>) = channel();

        lua.set_hook(HookTriggers::EVERY_LINE.every_nth_instruction(5), move |_lua, _debug| {
            if let Ok((lua_impl, name, args, tx)) = recv.try_recv() {
                let res = if let Ok(func) = lua_impl.get::<Function>(name) {
                    func.call::<Value>(args).unwrap_or(Value::Nil)
                }
                else {
                    Value::Nil
                };

                let _ = tx.send(res);
            }

            Ok(VmState::Continue)
        }).unwrap();

        LuaHandler {
            lua,
            sender: send
        }
    }

    pub fn call(&self, lua_impl: Table, name: &'static str, args: MultiValue) -> Value {
        let (send, recv): (Sender<Value>, Receiver<Value>) = channel();

        //let _ = self.sender.send((lua_impl, name, args, send));

        //recv.recv().unwrap_or(Value::Nil)

        let res = if let Ok(func) = lua_impl.get::<Function>(name) {
            func.call::<Value>(args).unwrap_or(Value::Nil)
        }
        else {
            Value::Nil
        };

        res
    }
}
