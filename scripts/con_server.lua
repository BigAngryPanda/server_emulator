local con_server_impl =
{
    m_address = "localhost"
}

function con_server_impl.address(self)
    return self.m_address
end

return con_server_impl