use fhe_http_core::apis::fhe_ops::{fhe_add, fhe_mul, fhe_sub};
use fhe_http_core::fhe_traits::key_serialize::KeySerialize;
use fhe_http_core::tfhe::{set_server_key, CompressedServerKey, ServerKey};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

#[pyclass]
pub struct FheOps {}

#[pymethods]
impl FheOps {
    #[new]
    pub fn new() -> Self {
        FheOps {}
    }
    fn decompress_server_key(&self, server_key: Vec<u8>) -> Vec<u8> {
        let compressed_sks: CompressedServerKey = KeySerialize::deserialize(&server_key.clone());
        let decompressed_sks = compressed_sks.decompress();
        decompressed_sks.serialize().clone()
    }

    pub fn set_server_key(&self, server_key: Vec<u8>) {
        let server_key: ServerKey = KeySerialize::deserialize(&server_key);
        set_server_key(server_key);
    }

    pub fn add(
        &self,
        py: Python<'_>,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: &str,
    ) -> PyResult<PyObject> {
        fhe_add(&a, &b, data_type)
            .map(|result| PyBytes::new_bound(py, &result).into())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
    }

    pub fn sub(
        &self,
        py: Python<'_>,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: String,
    ) -> PyResult<PyObject> {
        fhe_sub(&a, &b, &data_type)
            .map(|result| PyBytes::new_bound(py, &result).into())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
    }

    pub fn mul(
        &self,
        py: Python<'_>,
        a: Vec<u8>,
        b: Vec<u8>,
        data_type: String,
    ) -> PyResult<PyObject> {
        fhe_mul(&a, &b, &data_type)
            .map(|result| PyBytes::new_bound(py, &result).into())
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyException, _>(format!("{}", e)))
    }
}
