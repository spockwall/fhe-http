#[cfg(test)]
mod tests {
    use fhe_http_core::assembly;
    use tfhe::prelude::*;

    #[test]
    fn assembly_execute() {
        // Parse the assembly code from input string
        let asm = assembly::parser::parse_asm_from_str(
            "VAR a r0\nVAR b r1\nMOV 123 r2\nMOV 456 r3\nNEG r3 r4\nADD r2 r4 r5\nADD r0 r5 r6\nOUT r6",
        )
        .unwrap();
        let mut args = std::collections::HashMap::new();

        // Initialize the FHE context
        let config = tfhe::ConfigBuilder::default().build();
        let (client_key, server_keys) = tfhe::generate_keys(config);
        let arg1 = tfhe::FheInt64::try_encrypt(12, &client_key);
        let arg2 = tfhe::FheInt64::try_encrypt(45, &client_key);
        tfhe::set_server_key(server_keys);

        // Create the arguments hashmap
        args.insert("a".to_string(), arg1.unwrap());
        args.insert("b".to_string(), arg2.unwrap());

        // Execute the assembly code
        let res = assembly::executor::execute_asm_i64(&asm, &args, &client_key);
        let encrypted = res.unwrap();
        let decrypted: i64 = encrypted.decrypt(&client_key);
        assert_eq!(decrypted, -321);
    }
}
