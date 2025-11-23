use mlua::{
    Lua,
    HookTriggers,
    VmState,
    Function,
    Value,
    MultiValue,
    Table,
    UserData
};

use std::sync::mpsc::{
    channel,
    Sender,
    Receiver
};

use std::thread::ThreadId;

struct InterruptionGuard<'a> {
    pub handler_ref: &'a LuaHandler,
    pub need_comlete_interruption: bool
}

impl<'a> Drop for InterruptionGuard<'a> {
    fn drop(&mut self) {
        if self.need_comlete_interruption {
            self.handler_ref.complete_interrupt();
        }
    }
}

#[derive(Debug)]
pub struct LuaHandler {
    lua: Lua,
    own_pid: ThreadId,
    int_req: Sender<Sender<()>>,
    int_ret: Sender<()>,
}

impl LuaHandler {
    pub fn new(lua: Lua) -> LuaHandler {
        let (int_req_tx, int_req_rx): (Sender<Sender<()>>, Receiver<Sender<()>>) = channel();
        let (int_ret_tx, int_ret_rx): (Sender<()>, Receiver<()>) = channel();

        lua.set_hook(HookTriggers::EVERY_LINE.every_nth_instruction(50), move |_lua, _debug| {
            if let Ok(int_in) = int_req_rx.try_recv() {
                int_in.send(()).unwrap();
                int_ret_rx.recv().unwrap();
            }

            Ok(VmState::Continue)
        }).unwrap();

        LuaHandler {
            lua,
            own_pid: std::thread::current().id(),
            int_req: int_req_tx,
            int_ret: int_ret_tx,
        }
    }

    pub fn call(&self, lua_impl: Table, name: &str, args: MultiValue) -> Value {
        let _guard = self.request_interrupt();

        Self::perform_call(lua_impl, name, args)
    }

    fn perform_call(lua_impl: Table, name: &str, args: MultiValue) -> Value {
        if let Ok(func) = lua_impl.get::<Function>(name) {
            func.call::<Value>(args).unwrap_or(Value::Nil)
        }
        else {
            println!("FAIL {}", name);
            Value::Nil
        }
    }

    pub fn wrap_value<T: UserData + 'static>(&self, value: T) -> Value {
        let _guard = self.request_interrupt();

        let res = Value::UserData(self.lua.create_userdata(value).unwrap());

        res
    }

    pub fn create_string(&self, data: &[u8]) -> mlua::String {
        let _guard = self.request_interrupt();

        self.lua.create_string(data).unwrap()
    }

    fn request_interrupt(&'_ self) -> InterruptionGuard<'_> {
        if std::thread::current().id() != self.own_pid {
            let (send, recv): (Sender<()>, Receiver<()>) = channel();

            self.int_req.send(send).unwrap();

            recv.recv().unwrap();
        }

        InterruptionGuard { handler_ref: self,
            need_comlete_interruption: std::thread::current().id() != self.own_pid
        }
    }

    fn complete_interrupt(&self) {
        self.int_ret.send(()).unwrap();
    }
}
