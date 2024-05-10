use crate::fhe_traits::key_serialize::KeySerialize;
use crate::utils::file_ctl;
use crate::utils::json;
use tfhe::ClientKey;

pub fn create_fhe_header(method: &str) -> String {
    return file_ctl::create_fhe_header(&method);
}

pub fn encrypt_fhe_body(keys: Vec<String>, data: &str, client_key: Vec<u8>) -> String {
    let body = file_ctl::parse_json(data);
    let client_key: ClientKey = KeySerialize::deserialize(&client_key);
    let keys = keys.iter().map(|x| x.as_str()).collect();
    let encrypted_body = json::encrypt_json(&keys, &body, &client_key);
    return serde_json::to_string(&encrypted_body).unwrap();
}

pub fn decrypt_fhe_body(keys: Vec<String>, data: &str, client_key: Vec<u8>) -> String {
    let body: serde_json::Map<String, serde_json::Value> = serde_json::from_str(data).unwrap();
    let client_key: ClientKey = KeySerialize::deserialize(&client_key);
    let keys = keys.iter().map(|x| x.as_str()).collect();
    let decrypted_body = json::decrypt_json(&keys, &body, &client_key);
    return serde_json::to_string(&decrypted_body).unwrap();
}
