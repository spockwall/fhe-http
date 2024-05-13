use crate::fhe_traits::key_serialize::KeySerialize;
use crate::utils::base64;
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

pub fn set_server_key_in_body(server_key: &Vec<u8>, data: &str) -> String {
    let mut body = file_ctl::parse_json(data);
    body.insert(
        "server_key".to_string(),
        serde_json::Value::String(base64::encode_vec_u8(server_key)),
    );
    return serde_json::to_string(&body).unwrap();
}

pub fn check_http_packet(packet: &str) -> Result<(), &str> {
    let (header, body) = file_ctl::parse_http_packet(packet);
    if header == "" || body == "" {
        return Err("Error parsing HTTP packet");
    }
    // check if fhe-method is present in the header
    let header = file_ctl::parse_json(&header);
    if !header.contains_key("fhe-method") {
        return Err("fhe-method not found in the header");
    }

    // check if server_key is present in the body
    let body = file_ctl::parse_json(&body);
    if !body.contains_key("server_key") {
        return Err("server_key not found in the body");
    }

    return Ok(());
}
