use fhe_http_core::apis::base64;
use wasm_bindgen::prelude::*;

/// encode vec<u8> to base64
#[wasm_bindgen]
pub fn encode_fhe_value(value: Vec<u8>) -> String {
    base64::encode_vec_u8(&value)
}

/// decode base64 to vec<u8>
#[wasm_bindgen]
pub fn decode_fhe_value(value: &str) -> Vec<u8> {
    base64::decode_vec_u8(value)
}
