use tfhe::ClientKey;

use crate::configs::json::{FheJsonValue, NorJsonValue};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::Encryptable;
use crate::fhe_traits::key_serialize::KeySerialize;
use crate::fhe_traits::value_serialize::FheJsonValueSerialize;

pub fn encrypt<T: Encryptable>(value: &NorJsonValue, client_key: &Vec<u8>) -> Vec<u8> {
    let deserialized_key: ClientKey = KeySerialize::deserialize(client_key);
    let encrypted = value.encrypt(&deserialized_key);
    match encrypted {
        Ok(encrypted) => encrypted.serialize(),
        Err(_) => panic!("Failed to encrypt the value"),
    }
}

pub fn decrypt<T: Decryptable>(value: &Vec<u8>, client_key: &Vec<u8>) -> NorJsonValue {
    let deserialized_key: ClientKey = KeySerialize::deserialize(client_key);
    let deserialized_value: FheJsonValue = FheJsonValueSerialize::deserialize(value);
    let decrypted = deserialized_value.decrypt(&deserialized_key);
    match decrypted {
        Ok(decrypted) => decrypted,
        Err(_) => panic!("Failed to decrypt the value"),
    }
}
