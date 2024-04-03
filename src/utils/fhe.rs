use tfhe::{prelude::*, ClientKey, FheInt32, FheUint32, FheUint8};

pub fn encrypt_u32(input: u32, client_key: &ClientKey) -> FheUint32 {
    let res = FheUint32::try_encrypt(input, client_key);
    return res.unwrap();
}

pub fn decrypt_u32(input: FheUint32, client_key: &ClientKey) -> u32 {
    let res: u32 = input.decrypt(client_key);
    return res;
}

pub fn encrypt_i32(input: i32, client_key: &ClientKey) -> FheInt32 {
    let res = FheInt32::try_encrypt(input, client_key);
    return res.unwrap();
}

pub fn decrypt_i32(input: FheInt32, client_key: &ClientKey) -> i32 {
    let res: i32 = input.decrypt(client_key);
    return res;
}

pub fn encrypt_string(input: &str, client_key: &ClientKey) -> Vec<FheUint8> {
    // divide input into 8-bit chunks
    let mut encrypted_chunks: Vec<FheUint8> = Vec::new();
    for byte in input.bytes() {
        let res = FheUint8::try_encrypt(byte, client_key);
        let temp = res.unwrap();
        encrypted_chunks.push(temp);
    }

    // turn encrypted_chunks into a string

    return encrypted_chunks;
}

pub fn decrypt_chunks(input: Vec<FheUint8>, client_key: &ClientKey) -> String {
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

#[cfg(test)]
mod fhe_tests {
    use super::*;
    use tfhe::{generate_keys, ConfigBuilder};

    #[test]
    fn test_encrypt_and_decrypt_string() {
        let http_response: &str = "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\nContent-Length: 123\r\n\r\n<html><body>Hello, World!</body></html>";
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let encrypted: Vec<FheUint8> = encrypt_string(http_response, &client_key);
        let decrypted: String = decrypt_chunks(encrypted, &client_key);
        assert_eq!(http_response, decrypted);
    }
    #[test]
    fn test_encrypt_and_decrypt_u32() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<u32> = vec![0, 11, 4294967295, 34234];
        for input in input_vec {
            let encrypted: FheUint32 = encrypt_u32(input, &client_key);
            let decrypted: u32 = decrypt_u32(encrypted, &client_key);
            assert_eq!(input, decrypted);
        }
    }

    #[test]
    fn test_encrypt_and_decrypt_i32() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<i32> = vec![0, 11, -2_147_483_648, 2_147_483_647, 34234];
        for input in input_vec {
            let encrypted: FheInt32 = encrypt_i32(input, &client_key);
            let decrypted: i32 = decrypt_i32(encrypted, &client_key);
            assert_eq!(input, decrypted);
        }
    }
}
