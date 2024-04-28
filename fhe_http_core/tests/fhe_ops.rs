#[cfg(test)]
mod fhe_tests {
    use fhe_http_core::configs::fhe_types::FheSupportedType;
    use fhe_http_core::configs::json::NorJsonValue;
    use fhe_http_core::fhe_traits::computable::Computable;
    use fhe_http_core::fhe_traits::decryptable::Decryptable;
    use fhe_http_core::fhe_traits::encryptable::Encryptable;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder};

    #[test]
    fn fhe_operations() {
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
