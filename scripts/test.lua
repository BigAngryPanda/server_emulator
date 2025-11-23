local settings = require("scripts.settings")

if not settings then
    print("No settings. Exit")
    return
end

local deal     = require("scripts.deal")
local group    = require("scripts.group")
local order    = require("scripts.order")
local position = require("scripts.position")
local request  = require("scripts.request")
local symbol   = require("scripts.symbol")

function input()
    print("Input...")
    _ = io.read()
end

local server_impl = require("scripts.server")

local srv = Server.new(server_impl, settings.path)

srv:init()

-- input()

srv:start()

-- input()

-- server_impl.trade_sink:hook_trade_request_add(request, group, symbol, position, order, deal)

srv:stop()
