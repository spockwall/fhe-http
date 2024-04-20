use crate::configs::fhe_types::is_val_supported_type;
use crate::configs::json::{FheJsonValue, NorJsonValue};
use crate::utils::base64;
use crate::utils::fhe::{Decryptable, Encryptable};
use bincode;
use serde_json::{Map, Value};
use tfhe::{ClientKey, ServerKey};
pub fn encrypt_json(
    keys: &Vec<&str>,
    data: &Map<String, Value>,
    client_key: &ClientKey,
    _: &ServerKey,
) -> Map<String, Value> {
    // keys: keys to the values to be encrypted
    // data: a json object containing the data to be encrypted

    // Create new empty Map object that will store the encrypted values
    let mut encrypted_data: Map<String, Value> = serde_json::Map::new();

    // check if the types are supported
    for key in keys.iter() {
        if !data.contains_key(*key) {
            panic!("Key {} not found in data", key);
        }
        // check if the value is supported
        if !is_val_supported_type(&data[*key]) {
            panic!("Type {} is not supported", &data[*key]);
        }
    }
    for key in keys.iter() {
        let val: &Value = &data[*key];
        let fhe_json_val: FheJsonValue;
        match val {
            Value::String(s) => {
                let json_val = NorJsonValue::String(s.clone());
                fhe_json_val = json_val.encrypt(client_key).unwrap();
            }
            Value::Number(n) => {
                if n.is_i64() {
                    let n: i64 = n.as_i64().unwrap();
                    let json_val = NorJsonValue::Int64(n);
                    fhe_json_val = json_val.encrypt(client_key).unwrap();
                } else {
                    let n: u64 = n.as_u64().unwrap();
                    let json_val = NorJsonValue::Uint64(n);
                    fhe_json_val = json_val.encrypt(client_key).unwrap();
                }
            }
            _ => panic!("Unsupported value type"),
        }
        let serial_data = bincode::serialize(&fhe_json_val).unwrap();
        let encrypted_data_str = base64::encode_vec_u8(&serial_data);
        encrypted_data.insert(key.to_string(), encrypted_data_str.into());
    }
    // turn encrypted_data into a string
    return encrypted_data;
}

pub fn decrypt_json(
    keys: &Vec<&str>,
    data: &Map<String, Value>,
    client_key: &ClientKey,
) -> Map<String, Value> {
    // keys: keys to the values to be encrypted
    // data: a json object containing encrypted values
    let mut decrypted_data: Map<String, Value> = serde_json::Map::new();
    for key in keys.iter() {
        if !data.contains_key(*key) {
            panic!("Key {} not found in data", key);
        }
        let val = &data[*key];
        let decrypted: NorJsonValue;
        match val {
            Value::String(s) => {
                let encrypted_data: Vec<u8> = base64::decode_vec_u8(s).unwrap();
                let fhe_json_val: FheJsonValue = bincode::deserialize(&encrypted_data).unwrap();
                decrypted = fhe_json_val.decrypt(client_key).unwrap();
            }
            _ => panic!("Unsupported value type"),
        }
        match decrypted {
            NorJsonValue::Int64(n) => {
                decrypted_data.insert(key.to_string(), n.into());
            }
            NorJsonValue::Uint64(n) => {
                decrypted_data.insert(key.to_string(), n.into());
            }
            NorJsonValue::String(s) => {
                decrypted_data.insert(key.to_string(), s.into());
            }
        }
    }
    return decrypted_data;
}
