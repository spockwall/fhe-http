use crate::configs::typing::{FheValue, SerialClientKey};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::Encryptable;
use crate::fhe_traits::serializable::{FheValueSerializable, KeySerializable, ValueSerializable};
use tfhe::{ClientKey, FheInt64, FheUint64};

pub fn encrypt(value: &Vec<u8>, client_key: &SerialClientKey, data_type: &str) -> Vec<u8> {
    let deserialized_key = KeySerializable::try_deserialize(client_key).unwrap();
    let data_type = FheValue::from_str(data_type);
    match data_type {
        FheValue::Int64 => {
            let deserialized_val = i64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .val_encrypt(&deserialized_key)
                .expect("Failed to encrypt i64");
            encrypted.try_serialize().expect("Failed to serialize i64")
        }
        FheValue::Uint64 => {
            let deserialized_val = u64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .val_encrypt(&deserialized_key)
                .expect("Failed to encrypt u64");
            encrypted.try_serialize().expect("Failed to serialize u64")
        }
    }
}

pub fn decrypt(value: &Vec<u8>, client_key: &SerialClientKey, data_type: &str) -> Vec<u8> {
    let deserialized_key: ClientKey = KeySerializable::try_deserialize(client_key).unwrap();
    let data_type = FheValue::from_str(data_type);
    match data_type {
        FheValue::Int64 => {
            let deserialized_val = FheInt64::try_deserialize(value).unwrap();
            let decrypted = deserialized_val.val_decrypt(&deserialized_key);
            decrypted.try_serialize().expect("Failed to serialize i64")
        }
        FheValue::Uint64 => {
            let deserialized_val = FheUint64::try_deserialize(value).unwrap();
            let decrypted = deserialized_val.val_decrypt(&deserialized_key);
            decrypted.try_serialize().expect("Failed to serialize u64")
        }
    }
}
