use fhe_http_core::configs::typing::*;
use fhe_http_core::fhe_traits::serializable::ValueSerializable;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Serializer {}

#[wasm_bindgen]
impl Serializer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Serializer {}
    }
    pub fn from_i64(&self, value: i64) -> SerialInt64 {
        value
            .try_serialize()
            .expect(format!("Failed to serialize {}", value).as_str())
    }
    pub fn from_u64(&self, value: u64) -> SerialUint64 {
        value
            .try_serialize()
            .expect(format!("Failed to serialize {}", value).as_str())
    }

    pub fn to_i64(&self, value: SerialInt64) -> i64 {
        i64::try_deserialize(&value).expect("Failed to deserialize i64")
    }

    pub fn to_u64(&self, value: SerialUint64) -> u64 {
        u64::try_deserialize(&value).expect("Failed to deserialize u64")
    }
}
