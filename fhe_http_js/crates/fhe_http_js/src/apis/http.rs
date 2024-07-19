use crate::configs::typing::JsFheType;
use crate::utils::conversion::{array_to_vec_string, object_to_json_str};
use fhe_http_core::apis::http;
use fhe_http_core::configs::typing::{
    FheType, SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams, SerialServerKey,
};
use neon::prelude::*;

/// Define fhe header in http request
/// 1. method: self-defined
/// 2. fhe-version: "tfhe:<version>"
/// 3. zk-experiment: bool
#[neon::export]
pub fn create_fhe_header(method: String, zk_experiment: Option<bool>) -> String {
    http::create_fhe_header(method.as_str(), zk_experiment)
}

/// Encrypt body of http request with FHE
///
/// Input:
///     keys: List[Tuple[str, PyFheType]]
///         - list of keys in dict whose values to be enrypted with type: PyFheType
///     data: Dict - json data to be encrypted
#[neon::export]
pub fn encrypt_fhe_body<'cx>(
    cx: &mut FunctionContext<'cx>,
    keys: Handle<JsArray>, // Vec<String>
    data_type: JsFheType,
    data: Handle<JsObject>,
    client_key: SerialClientKey,
) -> String {
    let ty = FheType::from_str(&data_type);
    let keys = array_to_vec_string(cx, keys);
    let keys_ty = keys.iter().map(|k| (k.clone(), ty.clone())).collect();
    let data_json_str = object_to_json_str(cx, data).unwrap();
    http::encrypt_fhe_body(&keys_ty, data_json_str.as_str(), &client_key)
}

#[neon::export]
pub fn decrypt_fhe_body<'cx>(
    cx: &mut FunctionContext<'cx>,
    keys: Handle<JsArray>,
    data_type: JsFheType,
    data: Handle<JsObject>,
    client_key: SerialClientKey,
) -> String {
    let ty = FheType::from_str(&data_type);
    let keys = array_to_vec_string(cx, keys);
    let keys_ty = keys.iter().map(|k| (k.clone(), ty.clone())).collect();
    let data_json_str = object_to_json_str(cx, data).unwrap();
    http::decrypt_fhe_body(&keys_ty, data_json_str.as_str(), &client_key)
}

#[neon::export]
pub fn set_server_key_to_json<'cx>(
    cx: &mut FunctionContext<'cx>,
    server_key: SerialServerKey,
    data: Handle<JsObject>,
) -> String {
    let data_json_str = object_to_json_str(cx, data).unwrap();
    http::set_server_key_to_json(&server_key, &data_json_str)
}

#[neon::export]
pub fn set_public_key_to_json<'cx>(
    cx: &mut FunctionContext<'cx>,
    public_key: SerialCompactPublicKey,
    data: Handle<JsObject>,
) -> String {
    let data_json_str = object_to_json_str(cx, data).unwrap();
    http::set_public_key_to_json(&public_key, &data_json_str)
}

#[neon::export]
pub fn set_public_zk_params_to_json<'cx>(
    cx: &mut FunctionContext<'cx>,
    public_zk_params: SerialPublicZkParams,
    data: Handle<JsObject>,
) -> String {
    let data_json_str = object_to_json_str(cx, data).unwrap();
    http::set_public_zk_params_to_json(&public_zk_params, &data_json_str)
}

#[neon::export]
pub fn check_http_packet(packet: String) -> Result<(), String> {
    http::check_http_packet(packet.as_str()).map_err(|e| e.to_string())
}

#[neon::export]
pub fn get_fhe_value_from_json<'cx>(
    cx: &mut FunctionContext<'cx>,
    key: String,
    data: Handle<JsObject>,
) -> Vec<u8> {
    let data_json_str = object_to_json_str(cx, data).unwrap();
    http::get_fhe_value_from_json(key.as_str(), &data_json_str)
}
