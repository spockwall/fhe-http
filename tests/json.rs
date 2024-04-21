#[cfg(test)]
mod file_ctl_tests {
    extern crate fhe_http;
    use fhe_http::utils::file_ctl::{parse_json, read_from_stream};
    use fhe_http::utils::json::{decrypt_json, encrypt_json};
    use std::fs::File;
    use tfhe::{generate_keys, ConfigBuilder};
    const FILE_PATH: &str = "examples/json_files/param.json";
    #[test]
    fn test_encrypt_and_decrypt_json() {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        // read file json file from examples
        let json = read_from_stream(File::open(FILE_PATH).unwrap()).unwrap();
        let mut plain_data = parse_json(&json);
        let keys = vec!["a", "b"];
        let encrypted_data = encrypt_json(&keys, &mut plain_data, &client_key, &server_key);
        let _ = decrypt_json(&keys, &encrypted_data, &client_key);
    }
}
