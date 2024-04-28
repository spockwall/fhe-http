use crate::fhe_traits::key_serialize::KeySerialize;
use pyo3::prelude::*;
use tfhe::{generate_keys, set_server_key, ConfigBuilder, ServerKey};

#[pyclass]
struct KeyGenerator {
    client_key: Vec<u8>,
    server_key: Vec<u8>,
}

#[pymethods]
impl KeyGenerator {
    pub fn generate_keys(&self) -> KeyGenerator {
        let config: tfhe::Config = ConfigBuilder::default().build();
        let (client_key, server_key) = generate_keys(config);
        KeyGenerator {
            client_key: client_key.serialize(),
            server_key: server_key.serialize(),
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
fn fhe_http(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<KeyGenerator>()?;
    Ok(())
}
