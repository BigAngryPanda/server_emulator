local con_plugin_impl = {
    plugin_id = 1,
    plugin_name = "lua_impl"
}

function con_plugin_impl.server(self)
    return self.plugin_id
end

function con_plugin_impl.name(self)
    return self.plugin_name
end

return con_plugin_impl