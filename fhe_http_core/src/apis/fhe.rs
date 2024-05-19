use crate::configs::typing::SerialClientKey;
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::Encryptable;
use crate::fhe_traits::serializable::ValueSerializable;
use crate::fhe_traits::serializable::{FheValueSerializable, KeySerializable};
use tfhe::ClientKey;

pub fn encrypt<T, U>(value: &Vec<u8>, client_key: &SerialClientKey) -> Vec<u8>
where
    T: Encryptable<Output = U> + ValueSerializable,
    U: Decryptable + FheValueSerializable,
{
    let deserialized_key = KeySerializable::deserialize(client_key);
    let deserailized_val: T = ValueSerializable::deserialize(value);
    let encrypted = deserailized_val.val_encrypt(&deserialized_key);
    match encrypted {
        Ok(encrypted) => encrypted.serialize(),
        Err(_) => panic!("Failed to encrypt the value"),
    }
}

pub fn decrypt<T, U>(value: &Vec<u8>, client_key: &SerialClientKey) -> Vec<u8>
where
    T: Decryptable<Output = U> + FheValueSerializable,
    U: Encryptable + ValueSerializable,
{
    let deserialized_key: ClientKey = KeySerializable::deserialize(client_key);
    let deserialized_val: T = FheValueSerializable::deserialize(value);
    let decrypted = deserialized_val.val_decrypt(&deserialized_key);
    return decrypted.serialize();
}
