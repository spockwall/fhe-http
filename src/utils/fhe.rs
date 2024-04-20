use crate::configs::fhe_types::FheSupportedType;
use std::u32;

use tfhe::{prelude::*, ClientKey, Error, FheInt32, FheInt64, FheUint32, FheUint64, FheUint8};

// Trait that abstracts over the encryption and decryption operations
pub trait FheSupport<T, U>
where
    T: FheSupportedType<T>,
{
    fn encrypt(input: T, client_key: &ClientKey) -> Result<Self, Error>
    where
        Self: Sized;
    // set constraints That T must be FheSupportedTypes

    fn decrypt(input: U, client_key: &ClientKey) -> T;
}

impl FheSupport<i64, FheInt64> for FheInt64 {
    fn encrypt(input: i64, client_key: &ClientKey) -> Result<FheInt64, Error> {
        FheInt64::try_encrypt(input, client_key)
    }

    fn decrypt(input: FheInt64, client_key: &ClientKey) -> i64 {
        input.decrypt(client_key)
    }
}

impl FheSupport<u64, FheUint64> for FheUint64 {
    fn encrypt(input: u64, client_key: &ClientKey) -> Result<FheUint64, Error> {
        FheUint64::try_encrypt(input, client_key)
    }

    fn decrypt(input: FheUint64, client_key: &ClientKey) -> u64 {
        input.decrypt(client_key)
    }
}

impl FheSupport<u32, FheUint32> for FheUint32 {
    fn encrypt(input: u32, client_key: &ClientKey) -> Result<FheUint32, Error> {
        FheUint32::try_encrypt(input, client_key)
    }

    fn decrypt(input: FheUint32, client_key: &ClientKey) -> u32 {
        input.decrypt(client_key)
    }
}

impl FheSupport<i32, FheInt32> for FheInt32 {
    fn encrypt(input: i32, client_key: &ClientKey) -> Result<FheInt32, Error> {
        FheInt32::try_encrypt(input, client_key)
    }

    fn decrypt(input: FheInt32, client_key: &ClientKey) -> i32 {
        input.decrypt(client_key)
    }
}
impl FheSupport<String, Vec<FheUint8>> for Vec<FheUint8> {
    fn encrypt(input: String, client_key: &ClientKey) -> Result<Vec<FheUint8>, Error> {
        // divide input into 8-bit chunks
        let mut encrypted_chunks: Vec<FheUint8> = Vec::new();
        for byte in input.bytes() {
            let res = FheUint8::try_encrypt(byte, client_key);
            let temp = res.unwrap();
            encrypted_chunks.push(temp);
        }

        // turn encrypted_chunks into a string if Ok
        Ok(encrypted_chunks)
    }

    fn decrypt(input: Vec<FheUint8>, client_key: &ClientKey) -> String {
        // decrypt chunks is a empty vector of bytes at the begining
        let mut decrypted_chunks: Vec<u8> = Vec::new();
        for chunk in input {
            let res: u8 = chunk.decrypt(client_key);
            decrypted_chunks.push(res);
        }

        // turn decrypted_chunks into a string
        let decrypted_string: String = String::from_utf8(decrypted_chunks).unwrap();
        return decrypted_string;
    }
}

// wrap the decrypt function for every impl
pub fn fhe_encrypt<T, U>(input: T, client_key: &ClientKey) -> Result<U, Error>
where
    T: FheSupportedType<T>,
    U: FheSupport<T, U>,
{
    U::encrypt(input, client_key)
}

pub fn fhe_decrypt<T, U>(input: U, client_key: &ClientKey) -> T
where
    T: FheSupportedType<T>,
    U: FheSupport<T, U>,
{
    U::decrypt(input, client_key)
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
        let res_copy = http_response.clone();
        let encrypted: Result<Vec<FheUint8>, Error> = fhe_encrypt(res_copy, &client_key);
        match encrypted {
            Ok(cipher) => {
                let decrypted: String = fhe_decrypt(cipher, &client_key);
                assert_eq!(http_response, decrypted);
            }
            Err(_) => panic!("Failed to encrypt the string"),
        }
    }
    #[test]
    fn test_encrypt_and_decrypt_u32() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<u32> = vec![0, 11, 4294967295, 34234];
        for input in input_vec {
            let encrypted: Result<FheUint32, Error> = fhe_encrypt(input, &client_key);
            match encrypted {
                Ok(cipher) => {
                    let decrypted: u32 = fhe_decrypt(cipher, &client_key);
                    assert_eq!(input, decrypted);
                }
                Err(_) => panic!("Failed to encrypt the u32"),
            }
        }
    }
    #[test]
    fn test_encrypt_and_decrypt_i32() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<i32> = vec![0, 11, -2_147_483_648, 2_147_483_647, 34234];
        for input in input_vec {
            let encrypted: Result<FheInt32, Error> = fhe_encrypt(input, &client_key);
            match encrypted {
                Ok(cipher) => {
                    let decrypted: i32 = fhe_decrypt(cipher, &client_key);
                    assert_eq!(input, decrypted);
                }
                Err(_) => panic!("Failed to encrypt the i32"),
            }
        }
    }
}
