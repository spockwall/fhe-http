#[cfg(test)]
mod file_ctl_tests {
    use fhe_http_core::configs::json::FheJsonValue;
    use fhe_http_core::fhe_traits::decryptable::Decryptable;
    use fhe_http_core::fhe_traits::encryptable::Encryptable;
    use fhe_http_core::utils::file_ctl::read_from_stream;
    use fhe_http_core::utils::json::{decrypt_json, encrypt_json, parse_json};
    use std::fs::File;
    use tfhe::{generate_keys, set_server_key, ConfigBuilder};
    const FILE_PATH: &str = "examples/json_files/param.json";
    #[test]
    fn encrypt_and_decrypt_json() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, _) = generate_keys(config);
        // read file json file from examples
        let json = read_from_stream(File::open(FILE_PATH).unwrap()).unwrap();
        let mut plain_data = parse_json(&json);
        let keys = vec!["a", "b"];
        let encrypted_data = encrypt_json(&keys, &mut plain_data, &client_key);
        let _ = decrypt_json(&keys, &encrypted_data, &client_key);
    }

    #[test]
    fn operate_on_ciphertext() {
        use fhe_http_core::fhe_traits::value_serialize::FheJsonValueSerialize;
        use fhe_http_core::utils::json::get_encrypted_value_from_json;
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        set_server_key(server_key);
        // read file json file from examples
        let json = read_from_stream(File::open(FILE_PATH).unwrap()).unwrap();
        let plain_data = parse_json(&json);
        let keys = vec!["a", "b"];
        let encrypted_data = encrypt_json(&keys, &plain_data, &client_key);

        let encrypted_a = get_encrypted_value_from_json("a", &encrypted_data);
        let encrypted_b = get_encrypted_value_from_json("b", &encrypted_data);
        //let deserialized_a: FheJsonValue = bincode::deserialize(&encrypted_a).unwrap();

        let deserialized_a = FheJsonValue::deserialize(&encrypted_a);
        let deserialized_b = FheJsonValue::deserialize(&encrypted_b);
        let fhe_a = deserialized_a.to_fhe_i64().unwrap();
        let fhe_b = deserialized_b.to_fhe_i64().unwrap();
        //// Addition
        let encrypted_c = FheJsonValue::FheInt64(fhe_a.clone() + fhe_b.clone());
        let c = encrypted_c.decrypt(&client_key).unwrap();
        print!("c: {}", c.to_i64().unwrap());
        assert_eq!(
            c.to_i64().unwrap(),
            plain_data.get("c").unwrap().as_i64().unwrap()
        );
    }
}
