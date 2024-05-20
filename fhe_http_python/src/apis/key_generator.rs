use fhe_http_core::apis::base64;
use fhe_http_core::configs::typing::{SerialClientKey, SerialServerKey};
use fhe_http_core::fhe_traits::serializable::KeySerializable;
use fhe_http_core::tfhe::{ClientKey, CompressedServerKey, Config, ConfigBuilder};
use project_root;
use pyo3::prelude::*;
use serde_json;
use std::fs::File;
use std::io::Write;

#[pyclass]
pub struct KeyGenerator {
    client_key: SerialClientKey,
    server_key: SerialServerKey,
    config: Config,
}

#[pymethods]
impl KeyGenerator {
    #[new]
    // check if the new keys generation is needed
    pub fn new() -> Self {
        let config: Config = ConfigBuilder::default().build();
        Self {
            client_key: Vec::new(),
            server_key: Vec::new(),
            config,
        }
    }

    pub fn init_keys(&mut self) -> PyResult<()> {
        let res = self.load_keys();
        if res.is_err() {
            print!("fuck no key\n");
            self.generate_new_keys();
            self.save_keys()?;
        }
        Ok(())
    }

    pub fn generate_new_keys(&mut self) -> () {
        print!("Generating new keys\n");
        let cks = ClientKey::generate(self.config);
        let compressed_sks: CompressedServerKey = CompressedServerKey::new(&cks);
        self.client_key = cks.try_serialize().unwrap();
        self.server_key = compressed_sks.try_serialize().unwrap();
    }

    pub fn get_client_key(&self) -> SerialClientKey {
        self.client_key.clone()
    }

    pub fn get_server_key(&self) -> SerialServerKey {
        // server key is compressed
        self.server_key.clone()
    }

    fn get_enc_path(&self) -> String {
        let file_name: String = String::from("fhe-http-key.enc.json");
        let project_root = project_root::get_project_root();
        match project_root {
            Ok(val) => {
                let mut path = val.to_str().unwrap().to_string();
                path.push_str("/");
                path.push_str(&file_name);
                return path;
            }
            Err(_) => {
                panic!("Error: project root not found\n");
            }
        }
    }

    fn save_keys(&self) -> PyResult<()> {
        let file_path = self.get_enc_path();
        let mut file = File::create(file_path).expect("key.enc create failed");

        // json format
        let json = serde_json::json!({
            "client_key": base64::encode_vec_u8(&self.client_key),
            "server_key": base64::encode_vec_u8(&self.server_key)
        });

        // write the keys to the file
        file.write_all(json.to_string().as_bytes())
            .expect("key.enc write failed");

        print!("Keys saved successfully\n");
        Ok(())
    }

    fn load_keys(&mut self) -> PyResult<()> {
        let file_path = self.get_enc_path();
        // Read the entire file. This combines opening the file and reading it into a buffer.
        let buffer = std::fs::read(&file_path).map_err(|_| {
            PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                "Failed to read key file: {}.",
                file_path
            ))
        })?;

        // Parse buffer directly as JSON. This avoids the explicit UTF-8 conversion.
        let data: serde_json::Value = serde_json::from_slice(&buffer).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse JSON from key file. Error: {e}"
            ))
        })?;

        // Extract keys, ensuring they exist and are strings.
        let client_key = data["client_key"].as_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'client_key' in JSON.")
        })?;

        let server_key = data["server_key"].as_str().ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>("Missing 'server_key' in JSON.")
        })?;

        // decompression and set key
        self.client_key = base64::decode_vec_u8(client_key);
        self.server_key = base64::decode_vec_u8(server_key);

        println!("Keys loaded successfully");
        Ok(())
    }
}
