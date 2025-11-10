-- print("Hello from Lua!")

local server_impl = require("server")


srv = Server.new(server_impl, "")

srv:init()

srv:start()

--print("Input...")
--line = io.read()