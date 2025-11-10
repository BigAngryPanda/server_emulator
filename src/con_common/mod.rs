use server_emulator_macro::{
    log_impl_calls
};

use pyo3::{
    prelude::*,
    types::PyTuple
};

use crate::{
    interfaces::con_common::MT5ConCommon,
    mt5_apiserver::IMTConCommon,
    indexed_storage::IndexedStorage
};

use std::sync::{
    Weak,
    RwLock
};

use std::os::raw::c_uint;

pub struct ConCommon {
    pub id: usize,
    pub common_impl: pyo3::PyObject,
    storage: Weak<RwLock<IndexedStorage<Self>>>
}

impl ConCommon {
    const STR_DATA: &[u16] = &[85, 110, 107, 110, 111, 119, 110, 0]; // "Unknown"

    pub fn new(common_impl: pyo3::PyObject, storage: Weak<RwLock<IndexedStorage<Self>>>) -> ConCommon {
        ConCommon {
            id: 0,
            common_impl,
            storage
        }
    }

    fn call_impl(&self, method: &str) -> *const u16 {
        let data: Vec<u8> = Python::with_gil(|py| -> PyResult<Vec<u8>> {
            match self.common_impl.call_method(py, method, PyTuple::empty(py), None) {
                Ok(val) => {
                    PyResult::Ok(val.extract(py).unwrap_or(Vec::new()))
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
}

#[log_impl_calls]
impl MT5ConCommon for ConCommon {
    fn release(&mut self) {
        Python::with_gil(|py| {
            let _ = self.common_impl.call_method(py, "release", (), None);
        });

        self.storage.upgrade().unwrap().write().unwrap().remove(self.id);
    }

    fn assign(&mut self, param: &IMTConCommon) -> c_uint {
        0
    }

    fn clear(&mut self) -> c_uint {
        0
    }

    fn name1(&mut self, name: &[u16]) -> c_uint {
        0
    }

    fn name(&self) -> *const u16 {
        self.call_impl("name")
    }

    fn name_full(&self) -> *const u16 {
        self.call_impl("name_full")
    }

    fn owner(&self) -> *const u16 {
        self.call_impl("name_full")
    }

    fn owner_id(&self) -> *const u16 {
        self.call_impl("owner_id")
    }

    fn owner_host(&self) -> *const u16 {
        self.call_impl("owner_host")
    }

    fn owner_email(&self) -> *const u16 {
        self.call_impl("owner_email")
    }

    fn product(&self) -> *const u16 {
        self.call_impl("product")
    }

    fn account_url(&self) -> *const u16 {
        self.call_impl("account_url")
    }

    fn account_deposit_url(&self) -> *const u16 {
        self.call_impl("account_deposit_url")
    }

    fn account_withdrawal_url(&self) -> *const u16 {
        self.call_impl("account_withdrawal_url")
    }
}
