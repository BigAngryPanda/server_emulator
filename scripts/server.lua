local con_common_impl = require("scripts.con_common")
local con_plugin_impl = require("scripts.con_plugin")
local con_server_impl = require("scripts.con_server")
local con_symbol_impl = require("scripts.symbol")

local server_impl = {
    trade_sink = { hook_trade_request_add = nil },
    platform_name = "Lua test",
    platform_owner = "Unknown",
    server_version = 1,
    server_build = 1,
    server_type = 1,
    server_id = 1
}

-- [u16; 64usize]
function server_impl.get_platform_name(self)
    return self.platform_name
end

-- [u16; 128usize]
function server_impl.get_platform_owner(self)
    return self.platform_owner
end

function server_impl.get_server_version(self)
    return self.server_version
end

function server_impl.get_server_build(self)
    return self.server_build
end

function server_impl.get_server_type(self)
    return self.server_type
end

function server_impl.get_server_id(self)
    return self.server_id
end

function server_impl.about(self)
    return 0
end

function server_impl.license_check(self, license)
    return 0
end

function server_impl.common_create(self)
    return con_common_impl
end

function server_impl.common_get(self, con_common)
    return 0
end

function server_impl.plugin_create(self)
    return con_plugin_impl
end

function server_impl.plugin_current(self, con_plugin)
    return 0
end

function server_impl.net_server_create(self)
    return con_server_impl
end

function server_impl.net_server_get(self, id, con_server)
    return 0
end

function server_impl.trade_subscribe(self, sink)
    self.trade_sink = sink

    return 0
end

function server_impl.symbol_next(self, pos, symbol)
    return 1
end

function server_impl.time_current_msc()
    return 0
end

function server_impl.symbol_create(self)
    return con_symbol_impl
end

return server_impl
