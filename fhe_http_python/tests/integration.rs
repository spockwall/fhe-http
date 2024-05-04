#[cfg(test)]
mod file_ctl_tests {
    use fhe_http_python::apis::{fhe, fhe_ops, fhe_types, key_generator};
    #[test]
    fn encrypt_and_decrypt_json() {
        let kg = key_generator::KeyGenerator::new(true);
        let client_key = kg.get_client_key();
        let _server_key = kg.get_server_key();
        kg.set_server_key();
        let types = fhe_types::FheTypes::new();
        let ops = fhe_ops::FheOps::new();
        let fhe = fhe::Fhe::new(client_key);

        let a = types.from_i64(10);
        let b = types.from_i64(20);
        let encrypted_a = fhe.encrypt(a);
        let encrypted_b = fhe.encrypt(b);
        let res = ops.add(encrypted_a, encrypted_b, "Int64".to_string());
        let decrypted_res = fhe.decrypt(res);
        let deserialized_res = types.to_i64(&decrypted_res);
        assert_eq!(30, deserialized_res);
    }
}
