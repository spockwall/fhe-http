#[cfg(test)]
mod fhe_tests {
    use fhe_http::configs::fhe_types::FheSupportedType;
    use fhe_http::configs::json::NorJsonValue;
    use fhe_http::fhe::computable::Computable;
    use fhe_http::fhe::decryptable::Decryptable;
    use fhe_http::fhe::encryptable::Encryptable;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder};
    #[test]
    fn encrypt_and_decrypt_json() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        set_server_key(server_key);

        // declare plain_text a, b
        let plain_a = 123;
        let plain_b = 456;
        let encrypted_a = NorJsonValue::Int64(plain_a).encrypt(&client_key).unwrap();
        let encrypted_b = NorJsonValue::Int64(plain_b).encrypt(&client_key).unwrap();

        // test fhe add
        let encrypted_c = encrypted_a.add(&encrypted_b, &FheSupportedType::Int64);
        let decrypted_c = encrypted_c.decrypt(&client_key).unwrap().to_i64().unwrap();
        print!("decrypted_c: {}", decrypted_c);
        assert_eq!(plain_a + plain_b, decrypted_c);

        // test fhe sub
        let encrypted_c = encrypted_a.sub(&encrypted_b, &FheSupportedType::Int64);
        let decrypted_c = encrypted_c.decrypt(&client_key).unwrap().to_i64().unwrap();
        assert_eq!(plain_a - plain_b, decrypted_c);

        // test fhe mul
        let encrypted_c = encrypted_a.mul(&encrypted_b, &FheSupportedType::Int64);
        let decrypted_c = encrypted_c.decrypt(&client_key).unwrap().to_i64().unwrap();
        assert_eq!(plain_a * plain_b, decrypted_c);
    }
}
