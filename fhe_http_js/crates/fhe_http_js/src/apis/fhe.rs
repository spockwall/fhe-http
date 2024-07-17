//! This module provides python api for encryption and decryption
//! Input and output are serialized as Vec<u8> and PyFheType
use fhe_http_core::apis::fhe;
use fhe_http_core::configs::typing::{
    FheType, ProvenFheType, SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams,
};

/// Fhe class for encryption and decryption. This class requires
/// client key for encryption and decryption. Public key is only
/// need when using asymmetric encryption, i.e., proven encryption.

#[neon::export]
pub fn encrypt(val: Vec<u8>, client_key: SerialClientKey, data_type: String) -> Vec<u8> {
    let ty = FheType::from_str(&data_type);
    fhe::encrypt(&val, &client_key, &ty)
}
/// Encrypt with public key and zk parameters.
/// Allowing server to verify the encryption.
#[neon::export]
pub fn proven_encrypt(
    val: Vec<u8>,
    public_key: SerialCompactPublicKey,
    public_zk_params: SerialPublicZkParams,
    data_type: String,
) -> Vec<u8> {
    let ty = ProvenFheType::from_str(&data_type);
    fhe::proven_encrypt(&val, &public_key, &public_zk_params, &ty)
}

#[neon::export]
pub fn decrypt(val: Vec<u8>, client_key: SerialClientKey, data_type: String) -> Vec<u8> {
    let ty = FheType::from_str(&data_type);
    fhe::decrypt(&val, &client_key, &ty)
}
