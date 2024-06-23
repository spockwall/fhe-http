use crate::configs::typing::{PyFheValue, PyProvenFheValue};
use fhe_http_core::apis::fhe::{decrypt, encrypt, proven_encrypt};
use fhe_http_core::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams,
};
use pyo3::prelude::*;

#[pyclass]
pub struct Fhe {
    client_key: SerialClientKey,
    public_key: Option<SerialCompactPublicKey>,
}

#[pymethods]
impl Fhe {
    #[new]
    pub fn new(client_key: SerialClientKey, public_key: Option<SerialCompactPublicKey>) -> Self {
        let public_key = match public_key {
            Some(pk) => Some(pk.clone()),
            None => None,
        };
        Fhe {
            client_key: client_key.clone(),
            public_key: public_key.clone(),
        }
    }

    pub fn encrypt(&self, val: Vec<u8>, data_type: PyFheValue) -> Vec<u8> {
        encrypt(&val, &self.client_key, &data_type.inner)
    }

    pub fn proven_encrypt(
        &self,
        val: Vec<u8>,
        data_type: PyProvenFheValue,
        public_zk_params: SerialPublicZkParams,
    ) -> Vec<u8> {
        if let Some(pk) = &self.public_key {
            proven_encrypt(&val, pk, &public_zk_params, &data_type.inner)
        } else {
            panic!("Public key is required for proven encryption");
        }
    }

    pub fn decrypt(&self, val: Vec<u8>, data_type: PyFheValue) -> Vec<u8> {
        decrypt(&val, &self.client_key, &data_type.inner)
    }
}
