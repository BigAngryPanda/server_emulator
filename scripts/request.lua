local request_impl =
{
    m_action = 1,
    m_login  = 1001,
    m_symbol = "EURUSD",
    m_volume_ext = 1,
    m_type = 0,
    m_flags = 0
}

function request_impl.action(self)
    return self.m_action
end

function request_impl.volume_ext(self)
    return self.m_volume_ext
end

function request_impl.login(self)
    return self.m_login
end

function request_impl.symbol(self)
    return self.m_symbol
end

function request_impl.type(self)
    return self.m_type
end

function request_impl.flags(self)
    return self.m_flags
end

return request_impl
