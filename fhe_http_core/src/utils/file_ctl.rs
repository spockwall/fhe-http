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

pub fn create_fhe_header(method: &str) -> String {
    let mut header = serde_json::Map::new();
    header.insert("fhe-method".to_string(), Value::String(method.to_string()));
    return serde_json::to_string(&header).unwrap();
}
