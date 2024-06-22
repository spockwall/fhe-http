use fhe_http_core::configs::typing::{FheValue, ProvenFheValue};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyFheValue {
    pub inner: FheValue,
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyProvenFheValue {
    pub inner: ProvenFheValue,
}

#[pymethods]
impl PyFheValue {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        let value = FheValue::from_str(s);
        Ok(PyFheValue { inner: value })
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

#[pymethods]
impl PyProvenFheValue {
    #[staticmethod]
    pub fn from_str(s: &str) -> PyResult<Self> {
        let value = ProvenFheValue::from_str(s);
        Ok(PyProvenFheValue { inner: value })
    }

    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }
}

#[pyfunction]
pub fn create_fhe_value_type(s: &str) -> PyResult<PyFheValue> {
    PyFheValue::from_str(s)
}

#[pyfunction]
pub fn create_proven_fhe_value_type(s: &str) -> PyResult<PyProvenFheValue> {
    PyProvenFheValue::from_str(s)
}
