#[cfg(test)]
mod fhe_tests {
    use fhe_http::configs::json::{FheJsonValue, NorJsonValue};
    use fhe_http::utils::fhe::{Decryptable, Encryptable};
    use tfhe::{generate_keys, ConfigBuilder, Error};

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
                assert_eq!(http_response, decrypted.to_string().unwrap());
            }
            Err(_) => panic!("Failed to encrypt the string"),
        }
    }
    #[test]
    fn encrypt_and_decrypt_u64() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let input_vec: Vec<u64> = vec![0, 11, 18446744073709551615, 34234];
        for input in input_vec {
            let val = NorJsonValue::Uint64(input);
            let encrypted = val.encrypt(&client_key).unwrap();
            let decrypted = encrypted.decrypt(&client_key).unwrap();
            assert_eq!(input, decrypted.to_u64().unwrap());
        }
    }
    #[test]
    fn encrypt_and_decrypt_i64() {
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
            assert_eq!(input, decrypted.to_i64().unwrap());
        }
    }
}
