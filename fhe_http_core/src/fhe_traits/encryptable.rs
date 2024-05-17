use crate::configs::typing::{FheJsonValue, NorJsonValue};
use tfhe::{prelude::*, ClientKey, Error, FheInt64, FheUint64, FheUint8};

// Define an alias for the error type
type UnspportedError = Box<dyn std::error::Error>;

// Trait that abstracts over the encryption and decryption operations
pub trait Encryptable {
    fn to_i64(&self) -> Result<i64, UnspportedError>;
    fn to_u64(&self) -> Result<u64, UnspportedError>;
    fn to_string(&self) -> Result<String, UnspportedError>;
    fn encrypt(&self, client_key: &ClientKey) -> Result<FheJsonValue, Error>
    where
        Self: Sized;
}

impl Encryptable for NorJsonValue {
    fn to_i64(&self) -> Result<i64, UnspportedError> {
        match self {
            NorJsonValue::Int64(n) => Ok(*n),
            _ => panic!("unsupported type passed"), // Unsupported
        }
    }
    fn to_u64(&self) -> Result<u64, UnspportedError> {
        match self {
            NorJsonValue::Uint64(n) => Ok(*n),
            _ => panic!("unsupported type passed"), // Unsupported
        }
    }
    fn to_string(&self) -> Result<String, UnspportedError> {
        match self {
            NorJsonValue::Int64(n) => Ok(n.to_string()),
            NorJsonValue::Uint64(n) => Ok(n.to_string()),
            NorJsonValue::String(s) => Ok(s.clone()),
        }
    }
    fn encrypt(&self, client_key: &ClientKey) -> Result<FheJsonValue, Error> {
        match self {
            NorJsonValue::Int64(n) => {
                FheInt64::try_encrypt(*n, client_key).map(FheJsonValue::FheInt64)
            }
            NorJsonValue::Uint64(n) => {
                FheUint64::try_encrypt(*n, client_key).map(FheJsonValue::FheUint64)
            }
            NorJsonValue::String(s) => {
                let encrypted_chunks: Result<Vec<FheUint8>, _> = s
                    .bytes()
                    .map(|byte| FheUint8::try_encrypt(byte, client_key))
                    .collect();
                encrypted_chunks.map(FheJsonValue::FheString)
            }
        }
    }
}
