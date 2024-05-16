use crate::configs::json::{FheJsonValue, NorJsonValue};
use bincode;
pub trait NorJsonValueSerialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &Vec<u8>) -> Self;
}

pub trait FheJsonValueSerialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &Vec<u8>) -> Self;
}

impl FheJsonValueSerialize for FheJsonValue {
    fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(value) => return value,
            Err(e) => panic!("Failed to serialize: {}", e),
        }
    }

    fn deserialize(data: &Vec<u8>) -> Self {
        match bincode::deserialize(data) {
            Ok(value) => return value,
            Err(e) => panic!("Failed to deserialize: {}", e),
        }
    }
}

impl NorJsonValueSerialize for NorJsonValue {
    fn serialize(&self) -> Vec<u8> {
        match bincode::serialize(&self) {
            Ok(value) => return value,
            Err(e) => panic!("Failed to serialize: {}", e),
        }
    }

    fn deserialize(data: &Vec<u8>) -> Self {
        match bincode::deserialize(data) {
            Ok(value) => return value,
            Err(e) => panic!("Failed to deserialize: {}", e),
        }
    }
}
