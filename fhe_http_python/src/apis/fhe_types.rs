use fhe_http_core::configs::json::NorJsonValue;
use fhe_http_core::fhe_traits::value_serialize::NorJsonValueSerialize;
use pyo3::prelude::*;

#[pyclass]
pub struct FheTypes {}

#[pymethods]
impl FheTypes {
    #[new]
    fn new() -> Self {
        FheTypes {}
    }
    fn from_i64(&self, value: i64) -> Vec<u8> {
        NorJsonValue::Int64(value).serialize()
    }
    fn from_u64(&self, value: u64) -> Vec<u8> {
        NorJsonValue::Uint64(value).serialize()
    }
    fn from_string(&self, value: String) -> Vec<u8> {
        NorJsonValue::String(value).serialize()
    }
}
