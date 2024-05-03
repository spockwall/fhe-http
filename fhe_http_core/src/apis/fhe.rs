use crate::configs::json::{FheJsonValue, NorJsonValue};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::Encryptable;
use crate::fhe_traits::key_serialize::KeySerialize;
use crate::fhe_traits::value_serialize::{FheJsonValueSerialize, NorJsonValueSerialize};
use tfhe::ClientKey;

pub fn encrypt<T: Encryptable>(value: &Vec<u8>, client_key: &Vec<u8>) -> Vec<u8> {
    let deserialized_key: ClientKey = KeySerialize::deserialize(client_key);
    let deserailized_val: NorJsonValue = NorJsonValueSerialize::deserialize(value);
    let encrypted = deserailized_val.encrypt(&deserialized_key);
    match encrypted {
        Ok(encrypted) => encrypted.serialize(),
        Err(_) => panic!("Failed to encrypt the value"),
    }
}

pub fn decrypt<T: Decryptable>(value: &Vec<u8>, client_key: &Vec<u8>) -> Vec<u8> {
    let deserialized_key: ClientKey = KeySerialize::deserialize(client_key);
    let deserialized_val: FheJsonValue = FheJsonValueSerialize::deserialize(value);
    let decrypted = deserialized_val.decrypt(&deserialized_key);
    match decrypted {
        Ok(decrypted) => decrypted.serialize(),
        Err(_) => panic!("Failed to decrypt the value"),
    }
}
