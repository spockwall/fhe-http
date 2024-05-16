use crate::configs::json::{FheJsonValue, NorJsonValue};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::Encryptable;
use crate::utils::base64;

use bincode;
use serde_json::{Map, Value};
use tfhe::ClientKey;

/// Parse a JSON string into a JSON object
pub fn parse_json(json: &str) -> serde_json::Map<String, Value> {
    let res: Value = match serde_json::from_str(&json) {
        Ok(v) => v,
        Err(_) => panic!("Error parsing JSON"),
    };
    let data = res.as_object().unwrap();
    return data.clone();
}

/// Encrypt a JSON object using the provided keys
/// The keys are used to encrypt the values of the JSON object
/// Encode the plaintext values by base64
/// args:
///   keys: Vec<&str> - The keys to the values to encrypt
///   data: Map<String, Value> - The JSON object to encrypt
///   client_key: &ClientKey - The secret key used for encryption
/// returns:
///   Map<String, Value> - The encrypted JSON object

pub fn encrypt_json(
    keys: &Vec<&str>,
    data: &Map<String, Value>,
    client_key: &ClientKey,
) -> Map<String, Value> {
    // Create new empty Map object that will store the encrypted values
    let mut encrypted_data: Map<String, Value> = serde_json::Map::new();

    for key in keys.iter() {
        if !data.contains_key(*key) {
            panic!("Key not found in data");
        }
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
                    fhe_json_val = NorJsonValue::Int64(n).encrypt(client_key).unwrap();
                } else {
                    let n: u64 = n.as_u64().unwrap();
                    fhe_json_val = NorJsonValue::Uint64(n).encrypt(client_key).unwrap();
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

/// Decrypts a JSON object using the provided keys
/// The encrypted values of the JSON object are encdoed by base64
/// args:
///    keys: Vec<&str> - The keys to the values to decrypt
///    data: Map<String, Value> - The JSON object to decrypt
///    client_key: &ClientKey - The secret key used for decryption
/// returns:
///    Map<String, Value> - The decrypted JSON object
/// panic:
///   If the key is not found in the data
///   If the value type is not supported
pub fn decrypt_json(
    keys: &Vec<&str>,
    data: &Map<String, Value>,
    client_key: &ClientKey,
) -> Map<String, Value> {
    let mut decrypted_data: Map<String, Value> = serde_json::Map::new();
    for &key in keys.iter() {
        let val_str = data.get(key).and_then(Value::as_str).unwrap();
        let encrypted_data = base64::decode_vec_u8(val_str).unwrap();
        let fhe_json_val: FheJsonValue = bincode::deserialize(&encrypted_data).unwrap();
        let decrypted_val: NorJsonValue = fhe_json_val.decrypt(client_key).unwrap();
        match decrypted_val {
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

pub fn get_fhe_value_from_json(key: &str, encrypted_data: &Map<String, Value>) -> Vec<u8> {
    if !encrypted_data.contains_key(key) {
        panic!("Key not found in data");
    }
    let val = encrypted_data.get(key).unwrap();
    base64::decode_vec_u8(val.as_str().unwrap()).unwrap()
}
