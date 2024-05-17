use crate::configs::typing::{FheJsonValue, NorJsonValue};
use tfhe::{prelude::*, ClientKey, Error, FheInt64, FheUint64, FheUint8};

// Define an alias for the error type
type UnspportedError = Box<dyn std::error::Error>;

// Trait that abstracts over the encryption and decryption operations
pub trait Decryptable {
    fn to_fhe_i64(&self) -> Result<FheInt64, UnspportedError>;
    fn to_fhe_u64(&self) -> Result<FheUint64, UnspportedError>;
    fn to_fhe_string(&self) -> Result<Vec<FheUint8>, UnspportedError>;
    fn decrypt(&self, client_key: &ClientKey) -> Result<NorJsonValue, Error>
    where
        Self: Sized;
}

impl Decryptable for FheJsonValue {
    fn to_fhe_i64(&self) -> Result<FheInt64, UnspportedError> {
        match self {
            FheJsonValue::FheInt64(n) => Ok(n.clone()),
            _ => panic!("Unsupported type passed, should be FheInt64"), // Unsupported
        }
    }
    fn to_fhe_u64(&self) -> Result<FheUint64, UnspportedError> {
        match self {
            FheJsonValue::FheUint64(n) => Ok(n.clone()),
            _ => panic!("Unsupported type passed, should be FheUint64"), // Unsupported
        }
    }
    fn to_fhe_string(&self) -> Result<Vec<FheUint8>, UnspportedError> {
        match self {
            FheJsonValue::FheString(s) => Ok(s.clone()),
            _ => panic!("Unsupported type passed, should be Vec<FheUint8>"), // Unsupported
        }
    }
    fn decrypt(&self, client_key: &ClientKey) -> Result<NorJsonValue, Error> {
        match self {
            FheJsonValue::FheInt64(n) => Ok(NorJsonValue::Int64(n.decrypt(client_key))),
            FheJsonValue::FheUint64(n) => Ok(NorJsonValue::Uint64(n.decrypt(client_key))),
            FheJsonValue::FheString(s) => {
                // decrypt chunks is a empty vector of bytes at the begining
                let mut decrypted_chunks: Vec<u8> = Vec::new();
                for chunk in s {
                    let res: u8 = chunk.decrypt(client_key);
                    decrypted_chunks.push(res);
                }

                // turn decrypted_chunks into a string
                let decrypted_string: String = String::from_utf8(decrypted_chunks).unwrap();
                return Ok(NorJsonValue::String(decrypted_string));
            }
        }
    }
}
