use crate::utils::conversion::py_dict_to_json;
use fhe_http_core::apis::http;
use fhe_http_core::configs::typing::{SerialClientKey, SerialServerKey};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pyfunction]
pub fn create_fhe_header(method: &str) -> String {
    http::create_fhe_header(method)
}

#[pyfunction]
pub fn encrypt_fhe_body<'py>(
    keys: Vec<String>,
    data: Bound<'py, PyDict>,
    client_key: SerialClientKey,
) -> String {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::encrypt_fhe_body(keys, &data_json_str, &client_key)
}

#[pyfunction]
pub fn decrypt_fhe_body<'py>(
    keys: Vec<String>,
    data: Bound<'py, PyDict>,
    client_key: SerialClientKey,
) -> String {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::decrypt_fhe_body(keys, &data_json_str, &client_key)
}

#[pyfunction]
pub fn set_server_key_to_json<'py>(
    server_key: SerialServerKey,
    data: Bound<'py, PyDict>,
) -> String {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::set_server_key_to_json(&server_key, &data_json_str)
}

#[pyfunction]
pub fn check_http_packet(packet: &str) -> PyResult<()> {
    http::check_http_packet(packet)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
}

#[pyfunction]
pub fn get_fhe_value_from_json<'py>(key: &str, data: Bound<'py, PyDict>) -> Vec<u8> {
    let data_json_str = py_dict_to_json(data).unwrap();
    http::get_fhe_value_from_json(key, &data_json_str)
}
