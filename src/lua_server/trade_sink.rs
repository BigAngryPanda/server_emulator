use crate::mt5_apiserver::IMTTradeSink;

use mlua::{
    UserData,
    UserDataMethods,
    Value
};

use crate::lua_server::{
    request::Request,
    group::Group,
    symbol::Symbol,
    order::Order,
    deal::Deal,
    position::Position
};

use crate::lua_server::{
    lua_handler::LuaHandler,
    lua_object::LuaConstructible,
};

use std::sync::Arc;

#[derive(Debug)]
pub struct TradeSink {
    mt_trade_sink: *mut IMTTradeSink,
    lua: Arc<LuaHandler>
}

impl TradeSink {
    pub fn new(lua: Arc<LuaHandler>, mt_trade_sink: *mut IMTTradeSink) -> TradeSink {
        TradeSink {
            mt_trade_sink,
            lua
        }
    }

    // HookTradeRequestAdd IMTTradeSink_HookTradeRequestAdd

    /*
        this: *mut IMTTradeSink,
        request: *const IMTRequest,
        group: *const IMTConGroup,
        symbol: *const IMTConSymbol,
        position: *const IMTPosition,
        order: *const IMTOrder,
        deal: *mut IMTDeal,
    */
    pub fn hook_trade_request_add(
        &mut self,
        request_impl: Value,
        group_impl: Value,
        symbol_impl: Value,
        position_impl: Value,
        order_impl: Value,
        deal_impl: Value)
    {
        let request_ptr = Request::alloc_if(self.lua.clone(), request_impl);
        let group_ptr = Group::alloc_if(self.lua.clone(), group_impl);
        let symbol_ptr = Symbol::alloc_if(self.lua.clone(), symbol_impl);
        let position_ptr = Position::alloc_if(self.lua.clone(), position_impl);
        let order_ptr = Order::alloc_if(self.lua.clone(), order_impl);
        let deal_ptr = Deal::alloc_if(self.lua.clone(), deal_impl);

        unsafe {
            ((*(*self.mt_trade_sink).vtable_).IMTTradeSink_HookTradeRequestAdd)(
                self.mt_trade_sink,
                request_ptr.get(),
                group_ptr.get(),
                symbol_ptr.get(),
                position_ptr.get(),
                order_ptr.get(),
                deal_ptr.get_mut())
        };
    }
}

impl UserData for TradeSink {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("hook_trade_request_add", |_, this, args: (Value, Value, Value, Value, Value, Value)| {
            Ok(this.hook_trade_request_add(args.0, args.1, args.2, args.3, args.4, args.5))
        });
    }
}