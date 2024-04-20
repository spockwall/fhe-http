extern crate base64;
use base64::{engine::general_purpose::STANDARD, Engine as _};

pub fn encode_vec_u8(input: &Vec<u8>) -> String {
    return STANDARD.encode(input);
}

pub fn decode_vec_u8(input: &str) -> Result<Vec<u8>, base64::DecodeError> {
    return STANDARD.decode(input);
}
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

#[cfg(test)]
mod tests {
    use super::*;
    use base64::DecodeError;
    #[test]
    fn encode() {
        let input: &str = "Hello, World!";
        let expected: &str = "SGVsbG8sIFdvcmxkIQ==";
        let result: String = STANDARD.encode(input);
        assert_eq!(result, expected);
    }

    #[test]
    fn decode() {
        let input: &str = "SGVsbG8sIFdvcmxkIQ==";
        let expected: &str = "Hello, World!";
        let result: Result<Vec<u8>, DecodeError> = STANDARD.decode(input);
        assert_eq!(result.unwrap(), expected.as_bytes());
    }
}
