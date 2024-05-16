use fhe_http_core::apis::fhe::{decrypt, encrypt};
use fhe_http_core::configs::typing::SerializedClientKey;
use pyo3::prelude::*;

#[pyclass]
pub struct Fhe {
    client_key: SerializedClientKey,
}

#[pymethods]
impl Fhe {
    #[new]
    pub fn new(client_key: SerializedClientKey) -> Self {
        Fhe {
            client_key: client_key.clone(),
        }
    }

    pub fn encrypt(&self, val: Vec<u8>) -> Vec<u8> {
        encrypt(&val, &self.client_key)
    }

    pub fn decrypt(&self, val: Vec<u8>) -> Vec<u8> {
        decrypt(&val, &self.client_key)
    }
}
