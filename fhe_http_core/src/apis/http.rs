use crate::configs::typing::{SerialClientKey, SerialServerKey, StringfiedJson};
use crate::fhe_traits::key_serialize::KeySerialize;
use crate::utils::{http, json};
use tfhe::ClientKey;

pub fn create_fhe_header(method: &str) -> String {
    return http::create_fhe_header(&method);
}

pub fn encrypt_fhe_body(
    keys: Vec<String>,
    data: &StringfiedJson,
    client_key: &SerialClientKey,
) -> String {
    let client_key: ClientKey = KeySerialize::deserialize(client_key);
    let encrypted_body = http::encrypt_fhe_body(keys, data, &client_key);
    return serde_json::to_string(&encrypted_body).unwrap();
}

pub fn decrypt_fhe_body(
    keys: Vec<String>,
    data: &StringfiedJson,
    client_key: &SerialClientKey,
) -> String {
    let client_key: ClientKey = KeySerialize::deserialize(client_key);
    let decrypted_body = http::decrypt_fhe_body(keys, data, &client_key);
    return serde_json::to_string(&decrypted_body).unwrap();
}

pub fn set_server_key_to_json(server_key: &SerialServerKey, data: &StringfiedJson) -> String {
    return http::set_server_key_to_json(server_key, data);
}

pub fn check_http_packet(packet: &str) -> Result<(), &str> {
    return http::check_http_packet(packet);
}

pub fn get_fhe_value_from_json(keys: &str, data: &StringfiedJson) -> Vec<u8> {
    let data = json::parse_json(data);
    return json::get_fhe_value_from_json(keys, &data);
}
