use fhe_http_core::configs::typing::{NorJsonValue, SerialNorJsonValue};
use fhe_http_core::fhe_traits::value_serialize::NorJsonValueSerialize;
use pyo3::prelude::*;

#[pyclass]
pub struct FheTypes {}

#[pymethods]
impl FheTypes {
    #[new]
    pub fn new() -> Self {
        FheTypes {}
    }
    pub fn from_i64(&self, value: i64) -> SerialNorJsonValue {
        NorJsonValue::Int64(value).serialize()
    }
    pub fn from_u64(&self, value: u64) -> SerialNorJsonValue {
        NorJsonValue::Uint64(value).serialize()
    }
    pub fn from_string(&self, value: String) -> SerialNorJsonValue {
        NorJsonValue::String(value).serialize()
    }

    pub fn to_i64(&self, value: SerialNorJsonValue) -> i64 {
        let res: NorJsonValue = NorJsonValueSerialize::deserialize(&value);
        match res {
            NorJsonValue::Int64(v) => v,
            _ => panic!("Invalid type"),
        }
    }

    pub fn to_u64(&self, value: SerialNorJsonValue) -> u64 {
        let res: NorJsonValue = NorJsonValueSerialize::deserialize(&value);
        match res {
            NorJsonValue::Uint64(v) => v,
            _ => panic!("Invalid type"),
        }
    }

    pub fn to_string(&self, value: SerialNorJsonValue) -> String {
        let res: NorJsonValue = NorJsonValueSerialize::deserialize(&value);
        match res {
            NorJsonValue::String(v) => v,
            _ => panic!("Invalid type"),
        }
    }
}
