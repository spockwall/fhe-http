use crate::configs::typing::PyFheValue;
use fhe_http_core::apis::fhe::{decrypt, encrypt};
use fhe_http_core::configs::typing::{SerialClientKey, SerialFheInt64};
use pyo3::prelude::*;

#[pyclass]
pub struct Fhe {
    client_key: SerialClientKey,
}

#[pymethods]
impl Fhe {
    #[new]
    pub fn new(client_key: SerialClientKey) -> Self {
        Fhe {
            client_key: client_key.clone(),
        }
    }

    pub fn encrypt(&self, val: Vec<u8>, data_type: &PyFheValue) -> SerialFheInt64 {
        encrypt(&val, &self.client_key, data_type.as_str())
    }

    pub fn decrypt(&self, val: Vec<u8>, data_type: &PyFheValue) -> SerialFheInt64 {
        decrypt(&val, &self.client_key, data_type.as_str())
    }
}
