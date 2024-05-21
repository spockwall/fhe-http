use crate::utils::base64;

pub fn encode_vec_u8(input: &Vec<u8>) -> String {
    base64::encode_vec_u8(input)
}

pub fn decode_vec_u8(input: &str) -> Vec<u8> {
    base64::decode_vec_u8(input).unwrap()
}
pub fn encode(input: &str) -> String {
    base64::encode(input)
}

pub fn decode(input: &str) -> String {
    base64::decode(input).unwrap()
}
