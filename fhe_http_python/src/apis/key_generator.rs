use fhe_http_core::fhe_traits::key_serialize::KeySerialize;
use fhe_http_core::tfhe::{generate_keys, set_server_key, Config, ConfigBuilder, ServerKey};
use pyo3::prelude::*;

#[pyclass]
pub struct KeyGenerator {
    client_key: Vec<u8>,
    server_key: Vec<u8>,
    config: Config,
}

#[pymethods]
impl KeyGenerator {
    #[new]
    // check if the new keys generation is needed
    pub fn new(client_key: Option<Vec<u8>>, server_key: Option<Vec<u8>>) -> Self {
        let config: Config = ConfigBuilder::default().build();

        // check if the new keys generation is needed
        let new_keys = client_key.is_none() || server_key.is_none();

        // if new keys are needed, generate new keys
        if new_keys {
            let (client_key, server_key) = generate_keys(config);
            return KeyGenerator {
                client_key: client_key.serialize(),
                server_key: server_key.serialize(),
                config,
            };
        } else {
            return KeyGenerator {
                client_key: client_key.unwrap(),
                server_key: server_key.unwrap(),
                config,
            };
        }
    }
    pub fn generate_new_keys(&self) -> Self {
        let (client_key, server_key) = generate_keys(self.config);
        KeyGenerator {
            client_key: client_key.serialize(),
            server_key: server_key.serialize(),
            config: self.config,
        }
    }
    pub fn set_server_key(&self, server_key: Vec<u8>) {
        let server_key: ServerKey = KeySerialize::deserialize(&server_key);
        set_server_key(server_key);
    }

    pub fn get_client_key(&self) -> Vec<u8> {
        self.client_key.clone()
    }
    pub fn get_server_key(&self) -> Vec<u8> {
        self.server_key.clone()
    }
}
