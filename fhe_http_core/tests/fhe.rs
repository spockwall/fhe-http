#[cfg(test)]
mod fhe_tests {
    use fhe_http_core::configs::json::{FheJsonValue, NorJsonValue};
    use fhe_http_core::fhe_traits::decryptable::Decryptable;
    use fhe_http_core::fhe_traits::encryptable::Encryptable;
    use tfhe::{generate_keys, ClientKey, ConfigBuilder, Error};

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

    #[test]
    fn serialize_and_deserialize() {
        use fhe_http_core::configs::fhe_types::FheSupportedType;
        use fhe_http_core::fhe_traits::computable::Computable;
        use fhe_http_core::fhe_traits::key_serialize::KeySerialize;
        use tfhe::{set_server_key, ServerKey};
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);

        let client_key_serialized = client_key.serialize();
        let server_key_serialized = server_key.serialize();
        let client_key_deserialized: ClientKey = KeySerialize::deserialize(&client_key_serialized);
        let server_key_deserialized: ServerKey = KeySerialize::deserialize(&server_key_serialized);
        set_server_key(server_key_deserialized);
        let a = NorJsonValue::Int64(123)
            .encrypt(&client_key_deserialized)
            .unwrap();
        let b = NorJsonValue::Int64(456)
            .encrypt(&client_key_deserialized)
            .unwrap();

        let c = a
            .add(&b, &FheSupportedType::Int64)
            .decrypt(&client_key_deserialized)
            .unwrap();
        assert_eq!(c.to_i64().unwrap(), 123 + 456);
    }

    #[test]
    fn encrypted_serialize() {
        use fhe_http_core::fhe_traits::encrypted_serialize::EncryptedSerialize;
        use tfhe::{generate_keys, ConfigBuilder};
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        let a = NorJsonValue::Int64(123);
        let encrypted_a = a.encrypt(&client_key).unwrap();
        let serialized_a = encrypted_a.serialize();
        let deserialized_a = FheJsonValue::deserialize(&serialized_a);
        let decrypted_a = deserialized_a.decrypt(&client_key).unwrap();
        let decrypted = decrypted_a.to_i64().unwrap();
        print!("{:?}", decrypted);
        match a {
            NorJsonValue::Int64(n) => assert_eq!(n, decrypted_a.to_i64().unwrap()),
            _ => panic!("Unsupported type"),
        }
    }
}
