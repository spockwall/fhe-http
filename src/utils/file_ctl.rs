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
