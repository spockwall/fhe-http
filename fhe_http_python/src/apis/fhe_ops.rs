use fhe_http_core::apis::fhe_ops::{fhe_add, fhe_mul, fhe_sub};
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

    pub fn sub(&self, a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
        fhe_sub(&a, &b, &data_type).unwrap()
    }

    pub fn mul(&self, a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
        fhe_mul(&a, &b, &data_type).unwrap()
    }
}
