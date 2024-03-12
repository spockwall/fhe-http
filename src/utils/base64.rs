extern crate base64;

use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn encode(input: &str) -> String {
    return STANDARD.encode(input.as_bytes());
}

pub fn decode(input: &str) -> String {
    match STANDARD.decode(input.as_bytes()) {
        Ok(decoded) => {
            let decoded_str = String::from_utf8_lossy(&decoded);
            return decoded_str.to_string();
        }
        Err(err) => {
            return format!("Decoding error: {:?}", err);
        }
    }
}
