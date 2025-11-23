local con_common_impl =
{
    m_name        = "LuaSrv",
    m_name_full   = "Lua Server",
    m_owner       = "Unknown",
    m_owner_id    = "Unknown",
    m_owner_host  = "localhost",
    m_owner_email = "localhost@email.lua",
    m_product     = "lua_server",
    m_account_url = "localhost",
    m_account_deposit_url = "localhost",
    m_account_withdrawal_url = "localhost"
}

function con_common_impl.name(self)
    return self.m_name
end

function con_common_impl.name_full(self)
    return self.m_name_full
end

function con_common_impl.owner(self)
    return self.m_owner
end

function con_common_impl.owner_id(self)
    return self.m_owner_id
end

function con_common_impl.owner_host(self)
    return self.m_owner_host
end

function con_common_impl.owner_email(self)
    return self.m_owner_email
end

function con_common_impl.product(self)
    return self.m_product
end

function con_common_impl.account_url(self)
    return self.m_account_url
end

function con_common_impl.account_deposit_url(self)
    return self.m_account_deposit_url
end

function con_common_impl.account_withdrawal_url(self)
    return self.m_account_withdrawal_url
end

return con_common_impl