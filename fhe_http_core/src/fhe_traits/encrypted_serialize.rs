use crate::configs::json::FheJsonValue;
use bincode;
pub trait EncryptedSerialize {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8]) -> Self;
}

impl EncryptedSerialize for FheJsonValue {
    fn serialize(&self) -> Vec<u8> {
        match self {
            FheJsonValue::FheInt64(value) => bincode::serialize(&value).unwrap(),
            FheJsonValue::FheUint64(value) => bincode::serialize(&value).unwrap(),
            FheJsonValue::FheString(value) => bincode::serialize(&value).unwrap(),
        }
    }

    fn deserialize(data: &[u8]) -> Self {
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
