-- print("Hello from Lua!")

function input()
    print("Input...")
    line = io.read()
end

local server_impl = require("server")

srv = Server.new(server_impl, "C:\\Users\\anon\\Work\\trade_limiter_mt5\\x64\\Debug\\trade_limiter_mt5.dll")

srv:init()

-- input()

srv:start()

input()

srv:stop()