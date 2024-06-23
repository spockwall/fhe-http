use crate::configs::typing::{
    FheValue, ProvenFheValue, SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams,
};
use crate::fhe_traits::decryptable::Decryptable;
use crate::fhe_traits::encryptable::{Encryptable, ProvenEncryptable};
use crate::fhe_traits::serializable::{
    FheValueSerializable, KeySerializable, ProvenFheValueSerializable, ValueSerializable,
    ZkSerializable,
};
use tfhe::zk::CompactPkePublicParams;
use tfhe::{ClientKey, CompactPublicKey, FheInt64, FheUint64};

pub fn encrypt(value: &Vec<u8>, client_key: &SerialClientKey, data_type: &FheValue) -> Vec<u8> {
    let deserialized_key = KeySerializable::try_deserialize(client_key).unwrap();
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

pub fn proven_encrypt(
    value: &Vec<u8>,
    public_key: &SerialCompactPublicKey,
    public_zk_params: &SerialPublicZkParams,
    data_type: &ProvenFheValue,
) -> Vec<u8> {
    let deserialized_public_key = CompactPublicKey::try_deserialize(public_key).unwrap();
    let deserialized_public_zk_params =
        CompactPkePublicParams::try_deserialize(public_zk_params).unwrap();
    match data_type {
        ProvenFheValue::ProvenInt64 => {
            let deserialized_val = i64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .proven_encrypt(&deserialized_public_zk_params, &deserialized_public_key)
                .expect("Failed to encrypt i64");
            encrypted.try_serialize().expect("Failed to serialize i64")
        }
        ProvenFheValue::ProvenUint64 => {
            let deserialized_val = u64::try_deserialize(value).expect("Failed to deserialize");
            let encrypted = deserialized_val
                .proven_encrypt(&deserialized_public_zk_params, &deserialized_public_key)
                .expect("Failed to encrypt u64");
            encrypted.try_serialize().expect("Failed to serialize u64")
        }
    }
}

pub fn decrypt(value: &Vec<u8>, client_key: &SerialClientKey, data_type: &FheValue) -> Vec<u8> {
    let deserialized_key: ClientKey = KeySerializable::try_deserialize(client_key).unwrap();
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
