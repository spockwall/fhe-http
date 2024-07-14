use crate::configs::typing::WasmFheType;
use crate::utils::conversion::js_object_to_json_str;
use fhe_http_core::apis::http;
use fhe_http_core::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams, SerialServerKey,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyType {
    key: String,
    value: WasmFheType,
}

/// Define fhe header in http request
/// 1. method: self-defined
/// 2. fhe-version: "tfhe:<version>"
/// 3. zk-experiment: bool
#[wasm_bindgen]
pub fn create_fhe_header(method: &str, zk_experiment: Option<bool>) -> String {
    http::create_fhe_header(method, zk_experiment)
}
/// Encrypt body of http request with FHE
///
/// Input:
///     keys: Array<KeyType>
///         - Array of Object whose values to be enrypted with type: WasmFheType
///         - Example: [{key: "key1", value: <WasmFheType>}, {key: "key2", value: <WasmFheType>}]
///     data: Object - json data to be encrypted
///     client_key: SerialClientKey - client key to encrypt data
#[wasm_bindgen]
pub fn encrypt_fhe_body(
    keys: Vec<KeyType>,
    data: js_sys::Object,
    client_key: SerialClientKey,
) -> String {
    let keys = keys
        .iter()
        .map(|k| (k.key.clone(), k.value.clone().into()))
        .collect();
    let data_json_str = js_object_to_json_str(&data).unwrap();
    http::encrypt_fhe_body(&keys, &data_json_str, &client_key)
}

/// Decrypt body of http request with FHE
///
/// Input:
///     keys: Array<KeyType>
///         - Array of Object whose values to be decrypted with type: WasmFheType
///         - Example: [{key: "key1", value: <WasmFheType>}, {key: "key2", value: <WasmFheType>}]
///     data: Object - json data to be decrypted
///     client_key: SerialClientKey - client key to decrypte data
#[wasm_bindgen]
pub fn decrypt_fhe_body(
    keys: Vec<KeyType>,
    data: js_sys::Object,
    client_key: SerialClientKey,
) -> String {
    let keys = keys
        .iter()
        .map(|k| (k.key.clone(), k.value.clone().into()))
        .collect();
    let data_json_str = js_object_to_json_str(&data).unwrap();
    http::decrypt_fhe_body(&keys, &data_json_str, &client_key)
}

/// Set server key to json data
/// Input:
///    server_key: SerialServerKey - serialized server key for operation
///    data: Object - json data to be set
/// Output:
///     String - stringified json data with server key inside
#[wasm_bindgen]
pub fn set_server_key_to_json(server_key: SerialServerKey, data: js_sys::Object) -> String {
    let data_json_str = js_object_to_json_str(&data).unwrap();
    http::set_server_key_to_json(&server_key, &data_json_str)
}

/// Set public key to json data
/// Input:
///    public_key: SerialCompactPublicKey - serialized public key for encryption with zk feature
///    data: Object - json data to be set
/// Output:
///    String - stringified json data with public key inside
#[wasm_bindgen]
pub fn set_public_key_to_json(public_key: SerialCompactPublicKey, data: js_sys::Object) -> String {
    let data_json_str = js_object_to_json_str(&data).unwrap();
    http::set_public_key_to_json(&public_key, &data_json_str)
}

/// Set public zk params to json data
/// Input:
///    public_zk_params: SerialPublicZkParams - serialized public zk params for zk verification
///    data: Object - json data to be set
/// Output:
///    String - stringified json data with public zk params inside
#[wasm_bindgen]
pub fn set_public_zk_params_to_json(
    public_zk_params: SerialPublicZkParams,
    data: js_sys::Object,
) -> String {
    let data_json_str = js_object_to_json_str(&data).unwrap();
    http::set_public_zk_params_to_json(&public_zk_params, &data_json_str)
}

/// Get FHE value from json data
/// Input:
///    key: String - key to get FHE value
///    data: Object - json data where FHE value is stored
/// Output:
///    Vec<u8> - FHE value in bytes
#[wasm_bindgen]
pub fn get_fhe_value_from_json(key: &str, data: js_sys::Object) -> Vec<u8> {
    let data_json_str = js_object_to_json_str(&data).unwrap();
    http::get_fhe_value_from_json(key, &data_json_str)
}
