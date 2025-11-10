use crate::mt5_apiserver;

use mt5_apiserver::*;

#[allow(unused_variables)]
unsafe extern "C" fn IMTServerPlugin_Release(this: *mut IMTServerPlugin) { }

#[allow(unused_variables)]
unsafe extern "C" fn IMTServerPlugin_Start(this: *mut IMTServerPlugin, server: *mut IMTServerAPI) -> MTAPIRES {
    0
}

#[allow(unused_variables)]
unsafe extern "C" fn IMTServerPlugin_Stop(this: *mut IMTServerPlugin) -> MTAPIRES {
    0
}
