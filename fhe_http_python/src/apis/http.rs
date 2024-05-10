use fhe_http_core::apis::http;
use pyo3::prelude::*;

#[pyfunction]
pub fn create_fhe_header(method: &str) -> String {
    http::create_fhe_header(method)
}
