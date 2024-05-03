use fhe_http_core::apis::fhe_ops::{fhe_add, fhe_mul, fhe_sub};
use pyo3::prelude::*;

#[pyclass]
pub struct FheOps {}

#[pymethods]
impl FheOps {
    #[new]
    fn new() -> Self {
        FheOps {}
    }
    pub fn add(&self, a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
        fhe_add(&a, &b, &data_type).unwrap()
    }

    pub fn sub(&self, a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
        fhe_sub(&a, &b, &data_type).unwrap()
    }

    pub fn mul(&self, a: Vec<u8>, b: Vec<u8>, data_type: String) -> Vec<u8> {
        fhe_mul(&a, &b, &data_type).unwrap()
    }
}
