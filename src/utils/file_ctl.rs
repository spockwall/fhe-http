use crate::configs::types::is_supported_type;
use serde_json;
use serde_json::Value;
use std::io::{Read, Result};

pub fn read_from_stream<T: Read>(mut stream: T) -> Result<String> {
    let mut contents: String = String::new();
    stream.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn parse_http_packet(packet: &str) -> (String, String) {
    // split the packet into header and body
    let res: Vec<&str> = packet.split("\r\n\r\n").collect();
    if res.len() == 2 {
        return (res[0].to_string(), res[1].to_string());
    }
    return ("".to_string(), "".to_string());
}

pub fn parse_json(json: &str) -> serde_json::Map<String, Value> {
    let res: Value = serde_json::from_str(&json).unwrap();
    let data = res.as_object().unwrap();
    return data.clone();
}

pub fn create_fhe_transfrom_mapping(
    keys: Vec<&str>,
    types: Vec<&str>,
) -> serde_json::Map<String, Value> {
    // keys: keys to the values to be encrypted
    // types: types to the values to be encrypted

    if keys.len() != types.len() {
        panic!("keys and types should have the same length");
    }

    // check if the types are supported
    for _type in types.iter() {
        if !is_supported_type(_type) {
            panic!("{} is not a supported type", _type);
        }
    }

    // using serde_json::Map to create a json object,
    // keys are from keys and values are from types
    let mut res: serde_json::Map<String, Value> = serde_json::Map::new();
    for i in 0..keys.len() {
        let key = keys[i].to_string();
        let _type = types[i].to_string();
        let value = serde_json::json!({
            "type": _type,
        });
        res.insert(key, value);
    }
    return res;
}
