use crate::configs::typing::{SerialClientKey, SerialServerKey, StringfiedJson};
use crate::utils::{http, json};

pub fn create_fhe_header(method: &str) -> String {
    return http::create_fhe_header(&method);
}

pub fn encrypt_fhe_body(
    keys: Vec<String>,
    data: &StringfiedJson,
    client_key: &SerialClientKey,
) -> String {
    return http::encrypt_fhe_body(keys, data, client_key);
}

pub fn decrypt_fhe_body(
    keys: Vec<String>,
    data: &StringfiedJson,
    client_key: &SerialClientKey,
) -> String {
    return http::decrypt_fhe_body(keys, data, client_key);
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
