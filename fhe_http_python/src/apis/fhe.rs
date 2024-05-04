use fhe_http_core::apis::fhe::{decrypt, encrypt};
use pyo3::prelude::*;

#[pyclass]
pub struct Fhe {
    client_key: Vec<u8>,
}

#[pymethods]
impl Fhe {
    #[new]
    pub fn new(client_key: Vec<u8>) -> Self {
        Fhe { client_key }
    }

    pub fn encrypt(&self, val: Vec<u8>) -> Vec<u8> {
        encrypt(&val, &self.client_key)
    }

    pub fn decrypt(&self, val: Vec<u8>) -> Vec<u8> {
        decrypt(&val, &self.client_key)
    }
}
