use pyo3::{
    pyclass,
    prelude::*
};

use server_emulator_macro::log_impl_calls;

use crate::{
    interfaces::con_server::MT5ConServer,
    indexed_storage::IndexedStorage
};

use std::sync::{
    Weak,
    RwLock
};

#[pyclass]
pub struct ConServer {
    pub id: usize,
    pub server_impl: pyo3::PyObject,
    storage: Weak<RwLock<IndexedStorage<Self>>>
}

#[log_impl_calls]
impl ConServer {
    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    pub fn new(server_impl: pyo3::PyObject, storage: Weak<RwLock<IndexedStorage<Self>>>) -> ConServer {
        ConServer {
            id: 0,
            server_impl,
            storage
        }
    }
}

#[log_impl_calls]
impl MT5ConServer for ConServer {
    fn address(&self) -> *const u16 {
        let data = Python::with_gil(|py| -> PyResult<Vec<u8>> {
            match self.server_impl.call_method(py, "address", (), None) {
                Ok(val) => {
                    PyResult::Ok(val.extract(py).unwrap())
                },
                Err(_) => {
                    PyResult::Ok(Vec::new())
                }
            }
        }).unwrap_or(Vec::new());

        if data.is_empty() {
            Self::STR_DATA.as_ptr()
        }
        else {
            data.as_ptr() as *const u16
        }
    }

    fn release(&mut self) {
        Python::with_gil(|py| {
            let _ = self.server_impl.call_method(py, "release", (), None);
        });

        self.storage.upgrade().unwrap().write().unwrap().remove(self.id);
    }
}