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

#[pyfunction]
pub fn set_server_key_to_json(server_key: Vec<u8>, data: &str) -> String {
    http::set_server_key_to_json(&server_key, data)
}

#[pyfunction]
pub fn check_http_packet(packet: &str) -> PyResult<()> {
    http::check_http_packet(packet)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
}

#[pyfunction]
pub fn get_fhe_value_from_json(key: &str, data: &str) -> Vec<u8> {
    http::get_fhe_value_from_json(key, data)
}
