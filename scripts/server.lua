local con_common_impl = require("con_common")
local con_plugin_impl = require("con_plugin")
local con_server_impl = require("con_server")

local server_impl = { }

function server_impl.common_create()
    return con_common_impl
end

function server_impl.common_get(impl)
    return 0
end

function server_impl.plugin_create()
    return con_plugin_impl
end

function server_impl.plugin_current(impl)
    return 0
end

function server_impl.net_server_create()
    return con_server_impl
end

return server_impl