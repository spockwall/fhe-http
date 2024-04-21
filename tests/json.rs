#[cfg(test)]
mod file_ctl_tests {
    extern crate fhe_http;
    use fhe_http::configs::json::FheJsonValue;
    use fhe_http::utils::base64;
    use fhe_http::utils::fhe::{Decryptable, Encryptable};
    use fhe_http::utils::file_ctl::{parse_json, read_from_stream};
    use fhe_http::utils::json::{decrypt_json, encrypt_json};
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
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        // read file json file from examples
        let json = read_from_stream(File::open(FILE_PATH).unwrap()).unwrap();
        let plain_data = parse_json(&json);
        let keys = vec!["a", "b"];
        let encrypted_data = encrypt_json(&keys, &plain_data, &client_key);

        let a = encrypted_data.get("a").unwrap();
        let b = encrypted_data.get("b").unwrap();
        set_server_key(server_key);
        let decoded_a = base64::decode_vec_u8(a.as_str().unwrap()).unwrap();
        let decoded_b = base64::decode_vec_u8(b.as_str().unwrap()).unwrap();
        let deserialized_a: FheJsonValue = bincode::deserialize(&decoded_a).unwrap();
        let deserialized_b: FheJsonValue = bincode::deserialize(&decoded_b).unwrap();
        let fhe_a = deserialized_a.to_fhe_i64();
        let fhe_b = deserialized_b.to_fhe_i64();
        let encrypted_c = FheJsonValue::FheInt64(fhe_a + fhe_b);
        let decrypted_c = encrypted_c.decrypt(&client_key).unwrap().to_i64();
        assert_eq!(decrypted_c, plain_data.get("c").unwrap().as_i64().unwrap());
    }
}
