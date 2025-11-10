use pyo3::prelude::*;

use std::sync::{
    Arc,
    RwLock
};

use crate::{
    indexed_storage::IndexedStorage,
    con_common,
    con_server
};

#[pyclass]
pub struct Storage {
    con_servers: Arc<RwLock<IndexedStorage<con_server::ConServer>>>,
    con_commons: Arc<RwLock<IndexedStorage<con_common::ConCommon>>>,
}

pub type alloc_id = usize;

#[pymethods]
impl Storage {
    pub fn alloc_con_common(&mut self, con_common_impl: PyObject) -> alloc_id {
        let id = self.con_commons.write().unwrap().insert(
            con_common::ConCommon::new(con_common_impl, Arc::downgrade(&self.con_commons)));

        self.con_commons.write().unwrap()[id].id = id;

        id
    }

    pub fn alloc_con_server(&mut self, con_srv_impl: PyObject) -> alloc_id {
        let id = self.con_servers.write().unwrap().insert(
            con_server::ConServer::new(con_srv_impl, Arc::downgrade(&self.con_servers)));

        self.con_servers.write().unwrap()[id].id = id;

        id
    }
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            con_servers: Arc::new(RwLock::new(IndexedStorage::new())),
            con_commons: Arc::new(RwLock::new(IndexedStorage::new())),
        }
    }

    pub fn con_common(&self, idx: usize) -> *mut con_common::ConCommon {
        &mut self.con_commons.write().unwrap()[idx] as *mut con_common::ConCommon
    }

    pub fn con_server(&self, idx: usize) -> *mut con_server::ConServer {
        &mut self.con_servers.write().unwrap()[idx] as *mut con_server::ConServer
    }
}