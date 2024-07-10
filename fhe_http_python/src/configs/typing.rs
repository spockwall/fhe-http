use fhe_http_core::configs::typing::{FheType, ProvenFheType};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyFheType {
    pub inner: FheType,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyProvenFheType {
    pub inner: ProvenFheType,
}

#[pymethods]
impl PyFheType {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        let value = FheType::from_str(s);
        Ok(PyFheType { inner: value })
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

#[pymethods]
impl PyProvenFheType {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        let value = ProvenFheType::from_str(s);
        Ok(PyProvenFheType { inner: value })
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

#[pyfunction]
pub fn create_fhe_value_type(s: &str) -> PyResult<PyFheType> {
    PyFheType::from_str(s)
}

#[pyfunction]
pub fn create_proven_fhe_value_type(s: &str) -> PyResult<PyProvenFheType> {
    PyProvenFheType::from_str(s)
}
