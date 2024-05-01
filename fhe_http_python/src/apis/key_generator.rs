use fhe_http_core::fhe_traits::key_serialize::KeySerialize;
use fhe_http_core::tfhe::{generate_keys, set_server_key, Config, ConfigBuilder, ServerKey};
use pyo3::prelude::*;

#[pyclass]
struct KeyGenerator {
    client_key: Vec<u8>,
    server_key: Vec<u8>,
    config: Config,
}

#[pymethods]
impl KeyGenerator {
    #[new]
    fn new() -> Self {
        let config: Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        KeyGenerator {
            client_key: client_key.serialize(),
            server_key: server_key.serialize(),
            config,
        }
    }
    pub fn generate_new_keys(&self) -> KeyGenerator {
        let (client_key, server_key) = generate_keys(self.config);
        KeyGenerator {
            client_key: client_key.serialize(),
            server_key: server_key.serialize(),
            config: self.config,
        }
    }
    pub fn set_server_key(&self, key: Vec<u8>) {
        let server_key: ServerKey = KeySerialize::deserialize(&key);
        set_server_key(server_key);
    }

    pub fn get_client_key(&self) -> Vec<u8> {
        self.client_key.clone()
    }
    pub fn get_server_key(&self) -> Vec<u8> {
        self.server_key.clone()
    }
}

#[pymodule]
fn fhe_http_python(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<KeyGenerator>()?;
    Ok(())
}
