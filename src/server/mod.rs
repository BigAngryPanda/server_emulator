pub mod storage;

use pyo3::{
    prelude::*,
    types::PyTuple,
};

use crate::mt5_apiserver::*;
use crate::interfaces::server::MT5Server;
use crate::{
    py_call,
    vtable_impl,
};
use storage::{
    Storage,
    alloc_id
};

use server_emulator_macro::{
    log_impl_calls
};

use crate::{
    con_common,
    con_plugin,
    con_server
};

use std::os::raw::{
    c_longlong,
    c_uint,
    c_ulonglong,
};

use std::sync::atomic::{
    AtomicPtr,
    Ordering::Relaxed
};

#[pyclass]
pub struct Server {
    server_impl: PyObject,
    plugin: libloading::Library,
    plugin_vtable: AtomicPtr<IMTServerPlugin>,
    server_api: AtomicPtr<IMTServerAPI>,
    con_plugin: con_plugin::ConPlugin,
    storage: Py<Storage>,
}

impl Server {
    fn plugin_vtable(&self) -> &IMTServerPlugin__bindgen_vtable {
        unsafe { &(*(*self.plugin_vtable.load(Relaxed)).vtable_) }
    }
}

#[pymethods]
impl Server {
    const SERVER_VTABLE: IMTServerAPI__bindgen_vtable = vtable_impl::server::new();
    const CON_COMMON_VTABLE: IMTConCommon__bindgen_vtable = vtable_impl::con_common::new();
    const CON_PLUGIN_VTABLE: IMTConPlugin__bindgen_vtable = vtable_impl::con_plugin::new();
    const CON_SERVER_VTABLE: IMTConServer__bindgen_vtable = vtable_impl::con_server::new();

    #[staticmethod]
    pub fn new(server_impl: PyObject, path: &str, py: Python<'_>) -> Server {
        use std::sync::atomic::AtomicPtr;

        pyo3::prepare_freethreaded_python();

        let plugin = unsafe {
            libloading::Library::new(path).expect("Failed to get library")
        };

        Server {
            server_impl,
            plugin,
            plugin_vtable: AtomicPtr::new(std::ptr::null_mut()),
            server_api: AtomicPtr::new(std::ptr::null_mut()),
            con_plugin: con_plugin::ConPlugin::new(),
            storage: Py::new(py, Storage::new()).unwrap(),
        }
    }

    pub fn init(&mut self) -> u32 {
        let func: libloading::Symbol<unsafe fn(c_uint, *mut *mut IMTServerPlugin) -> u32> = unsafe {
            self.plugin.get(b"MTServerCreate").unwrap()
        };

        let mut data: *mut IMTServerPlugin = std::ptr::null_mut();

        let ret: u32 = unsafe {
            func(0, &mut data as *mut *mut IMTServerPlugin)
        };

        if ret == 0 {
            self.plugin_vtable.store(data, Relaxed);
        }

        ret
    }

    pub fn start(&mut self) {
        let server_api = unsafe {
            std::alloc::alloc(std::alloc::Layout::new::<IMTServerAPI>()) as *mut IMTServerAPI
        };

        unsafe {
            (*server_api).vtable_ = &Self::SERVER_VTABLE as *const IMTServerAPI__bindgen_vtable;
            (*server_api).impl_ptr = self as *mut Server;
        }

        self.server_api.store(server_api, Relaxed);

        unsafe {
            (self.plugin_vtable().IMTServerPlugin_Start)(
                self.plugin_vtable.load(Relaxed),
                self.server_api.load(Relaxed));
        }
    }

    pub fn stop(&mut self) {
        unsafe {
            (self.plugin_vtable().IMTServerPlugin_Stop)(
                self.plugin_vtable.load(Relaxed)
            );
        }
    }

    pub fn storage<'a>(&mut self, py: Python<'a>) -> PyRefMut<'a, Storage> {
        self.storage.bind_borrowed(py).borrow_mut()
    }
}

#[log_impl_calls]
impl MT5Server for Server {
    fn about(&mut self, info: &mut MTServerInfo) -> c_uint {
        py_call::call_with_no_args(&self.server_impl, "about", 0)
    }

    fn license_check(&mut self, license_name: &u16) -> c_uint {
        py_call::call_with_no_args(&self.server_impl, "license_check", 0)
    }

    fn common_create(&mut self) -> *mut IMTConCommon {
        let result = py_call::call_with_no_args(&self.server_impl, "common_create", None);

        match result {
            Some(idx) => {
                let con_srv = unsafe {
                    std::alloc::alloc(std::alloc::Layout::new::<IMTConCommon>()) as *mut IMTConCommon
                };

                unsafe {
                    (*con_srv).vtable_ = &Self::CON_COMMON_VTABLE as *const IMTConCommon__bindgen_vtable;
                    (*con_srv).impl_ptr = Python::with_gil(|py| -> *mut con_common::ConCommon {
                        self.storage.borrow_mut(py).con_common(idx)
                    });
                }

                con_srv
            },
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn common_get(&mut self, common: &mut IMTConCommon) -> c_uint {
        let con_common = unsafe { &mut *(common.impl_ptr as *mut con_common::ConCommon) };

        Python::with_gil(|py| -> c_uint {
            match self.server_impl.call_method(
                py,
                "common_get",
                PyTuple::new(py, [con_common.common_impl.clone_ref(py)]).unwrap_or(PyTuple::empty(py)),
                None)
            {
                Ok(val) => {
                    val.extract(py).unwrap_or(1)
                },
                Err(_) => {
                    1
                }
            }
        })
    }

    fn plugin_create(&mut self) -> *mut IMTConPlugin {
        unsafe { std::alloc::alloc(std::alloc::Layout::new::<IMTConPlugin>()) as *mut IMTConPlugin }
    }

    fn plugin_current(
        &mut self,
        plugin: &mut IMTConPlugin,
    ) -> c_uint {
        plugin.vtable_ = &Self::CON_PLUGIN_VTABLE as *const IMTConPlugin__bindgen_vtable;
        plugin.impl_ptr = &mut self.con_plugin as *mut con_plugin::ConPlugin;

        0
    }

    fn net_server_create(&mut self) -> *mut IMTConServer {
        let result: Option<alloc_id> =
            py_call::call_with_no_args(&self.server_impl, "net_server_create", None);

        match result {
            Some(idx) => {
                let con_srv = unsafe {
                    std::alloc::alloc(std::alloc::Layout::new::<IMTConServer>()) as *mut IMTConServer
                };

                unsafe {
                    (*con_srv).vtable_ = &Self::CON_SERVER_VTABLE as *const IMTConServer__bindgen_vtable;
                    (*con_srv).impl_ptr = Python::with_gil(|py| -> *mut con_server::ConServer {
                        self.storage.borrow_mut(py).con_server(idx)
                    });
                }

                con_srv
            },
            _ => {
                std::ptr::null_mut()
            }
        }
    }

    fn net_server_get(
        &mut self,
        id: c_ulonglong,
        config: &mut IMTConServer,
    ) -> c_uint {
        let con_srv = unsafe { &mut *(config.impl_ptr as *mut con_server::ConServer) };

        Python::with_gil(|py| -> c_uint {
            match self.server_impl.call_method(py, "net_server_get", (id, con_srv.server_impl.clone_ref(py)), None) {
                Ok(val) => {
                    val.extract(py).unwrap_or(0)
                },
                Err(_) => {
                    0
                }
            }
        })
    }

    fn time_current_msc(&mut self) -> c_longlong {
        py_call::call_with_no_args(&self.server_impl, "time_current_msc", 0)
    }

    fn user_subscribe(&mut self, sink: &mut IMTUserSink) -> c_uint {
        0
    }
}
