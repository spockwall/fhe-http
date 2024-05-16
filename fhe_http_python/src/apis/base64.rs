use fhe_http_core::apis::base64;
use pyo3::prelude::*;

#[pyfunction]
pub fn encode_fhe_value(value: Vec<u8>) -> String {
    base64::encode_vec_u8(&value)
}

#[pyfunction]
pub fn decode_fhe_value(value: &str) -> Vec<u8> {
    base64::decode_vec_u8(value)
}
