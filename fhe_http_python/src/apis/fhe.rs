use fhe_http_core::apis::fhe::{decrypt, encrypt};
use fhe_http_core::configs::typing::{SerialClientKey, SerialFheJsonValue, SerialNorJsonValue};
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

    pub fn encrypt(&self, val: SerialNorJsonValue) -> SerialFheJsonValue {
        encrypt(&val, &self.client_key)
    }

    pub fn decrypt(&self, val: SerialFheJsonValue) -> SerialNorJsonValue {
        decrypt(&val, &self.client_key)
    }
}
