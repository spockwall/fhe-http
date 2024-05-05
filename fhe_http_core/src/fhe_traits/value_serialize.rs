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
        match self {
            FheJsonValue::FheInt64(value) => bincode::serialize(&value).unwrap(),
            FheJsonValue::FheUint64(value) => bincode::serialize(&value).unwrap(),
            FheJsonValue::FheString(value) => bincode::serialize(&value).unwrap(),
        }
    }

    fn deserialize(data: &Vec<u8>) -> Self {
        if let Ok(value) = bincode::deserialize::<tfhe::FheInt64>(data) {
            return FheJsonValue::FheInt64(value);
        }

        if let Ok(value) = bincode::deserialize::<tfhe::FheUint64>(data) {
            return FheJsonValue::FheUint64(value);
        }

        if let Ok(value) = bincode::deserialize::<Vec<tfhe::FheUint8>>(data) {
            return FheJsonValue::FheString(value);
        }

        panic!("Failed to deserialize");
    }
}
//
impl NorJsonValueSerialize for NorJsonValue {
    fn serialize(&self) -> Vec<u8> {
        match self {
            NorJsonValue::Int64(value) => bincode::serialize(&value).unwrap(),
            NorJsonValue::Uint64(value) => bincode::serialize(&value).unwrap(),
            NorJsonValue::String(value) => bincode::serialize(&value).unwrap(),
        }
    }

    fn deserialize(data: &Vec<u8>) -> Self {
        if let Ok(value) = bincode::deserialize::<i64>(data) {
            return NorJsonValue::Int64(value);
        }

        if let Ok(value) = bincode::deserialize::<u64>(data) {
            return NorJsonValue::Uint64(value);
        }

        if let Ok(value) = bincode::deserialize::<String>(data) {
            return NorJsonValue::String(value);
        }

        panic!("Failed to deserialize");
    }
}
