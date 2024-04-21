use crate::configs::json::{FheJsonValue, NorJsonValue};
use tfhe::{prelude::*, ClientKey, Error, FheInt64, FheUint64, FheUint8};

// Trait that abstracts over the encryption and decryption operations

pub trait Encryptable {
    fn to_i64(&self) -> i64;
    fn to_u64(&self) -> u64;
    fn to_string(&self) -> String;
    fn encrypt(&self, client_key: &ClientKey) -> Result<FheJsonValue, Error>
    where
        Self: Sized;
}

pub trait Decryptable {
    fn to_fhe_i64(&self) -> FheInt64;
    fn to_fhe_u64(&self) -> FheUint64;
    fn to_fhe_string(&self) -> Vec<FheUint8>;
    fn decrypt(&self, client_key: &ClientKey) -> Result<NorJsonValue, Error>
    where
        Self: Sized;
}

impl Encryptable for NorJsonValue {
    fn to_i64(&self) -> i64 {
        match self {
            NorJsonValue::Int64(n) => *n,
            _ => panic!("unsupported type passed"), // Unsupported
        }
    }
    fn to_u64(&self) -> u64 {
        match self {
            NorJsonValue::Uint64(n) => *n,
            _ => panic!("unsupported type passed"), // Unsupported
        }
    }
    fn to_string(&self) -> String {
        match self {
            NorJsonValue::Int64(n) => n.to_string(),
            NorJsonValue::Uint64(n) => n.to_string(),
            NorJsonValue::String(s) => s.clone(),
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

impl Decryptable for FheJsonValue {
    fn to_fhe_i64(&self) -> FheInt64 {
        match self {
            FheJsonValue::FheInt64(n) => n.clone(),
            _ => panic!("unsupported type passed"), // Unsupported
        }
    }
    fn to_fhe_u64(&self) -> FheUint64 {
        match self {
            FheJsonValue::FheUint64(n) => n.clone(),
            _ => panic!("unsupported type passed"), // Unsupported
        }
    }
    fn to_fhe_string(&self) -> Vec<FheUint8> {
        match self {
            FheJsonValue::FheString(s) => s.clone(),
            _ => panic!("unsupported type passed"), // Unsupported
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

#[cfg(test)]
mod fhe_tests {
    use super::*;
    use tfhe::{generate_keys, ConfigBuilder};

    #[test]
    fn test_encrypt_and_decrypt_string() {
        let http_response= String::from("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: 123\r\n\r\n<html><body>Hello, World!</body></html>");
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let res_copy = NorJsonValue::String(http_response.clone());
        let encrypted: Result<FheJsonValue, Error> = res_copy.encrypt(&client_key);
        match encrypted {
            Ok(cipher) => {
                let decrypted = cipher.decrypt(&client_key).unwrap();
                assert_eq!(http_response, decrypted.to_string());
            }
            Err(_) => panic!("Failed to encrypt the string"),
        }
    }
    #[test]
    fn test_encrypt_and_decrypt_u32() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<u64> = vec![0, 11, 18446744073709551615, 34234];
        for input in input_vec {
            let val = NorJsonValue::Uint64(input);
            let encrypted = val.encrypt(&client_key).unwrap();
            let decrypted = encrypted.decrypt(&client_key).unwrap();
            assert_eq!(input, decrypted.to_u64());
        }
    }
    #[test]
    fn test_encrypt_and_decrypt_i32() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<i64> = vec![
            0,
            1123,
            -9_223_372_036_854_775_808,
            9_223_372_036_854_775_807i64,
        ];
        for input in input_vec {
            let val = NorJsonValue::Int64(input);
            let encrypted = val.encrypt(&client_key).unwrap();
            let decrypted = encrypted.decrypt(&client_key).unwrap();
            assert_eq!(input, decrypted.to_i64());
        }
    }
}
