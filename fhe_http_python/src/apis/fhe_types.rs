use fhe_http_core::configs::typing::NorJsonValue;
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
    pub fn from_i64(&self, value: i64) -> Vec<u8> {
        NorJsonValue::Int64(value).serialize()
    }
    pub fn from_u64(&self, value: u64) -> Vec<u8> {
        NorJsonValue::Uint64(value).serialize()
    }
    pub fn from_string(&self, value: String) -> Vec<u8> {
        NorJsonValue::String(value).serialize()
    }

    pub fn to_i64(&self, value: Vec<u8>) -> i64 {
        let res: NorJsonValue = NorJsonValueSerialize::deserialize(&value);
        match res {
            NorJsonValue::Int64(v) => v,
            _ => panic!("Invalid type"),
        }
    }

    pub fn to_u64(&self, value: Vec<u8>) -> u64 {
        let res: NorJsonValue = NorJsonValueSerialize::deserialize(&value);
        match res {
            NorJsonValue::Uint64(v) => v,
            _ => panic!("Invalid type"),
        }
    }

    pub fn to_string(&self, value: Vec<u8>) -> String {
        let res: NorJsonValue = NorJsonValueSerialize::deserialize(&value);
        match res {
            NorJsonValue::String(v) => v,
            _ => panic!("Invalid type"),
        }
    }
}
