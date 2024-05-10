use fhe_http_core::apis::http;
use pyo3::prelude::*;

#[pyfunction]
pub fn create_fhe_header(method: &str) -> String {
    http::create_fhe_header(method)
}

#[pyfunction]
pub fn encrypt_fhe_body(keys: Vec<String>, data: &str, client_key: Vec<u8>) -> String {
    http::encrypt_fhe_body(keys, data, client_key)
}

#[pyfunction]
pub fn decrypt_fhe_body(keys: Vec<String>, data: &str, client_key: Vec<u8>) -> String {
    http::decrypt_fhe_body(keys, data, client_key)
}
