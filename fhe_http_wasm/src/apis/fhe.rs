use crate::configs::typing::{WasmFheType, WasmProvenFheType};
use fhe_http_core::apis::fhe;
use fhe_http_core::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialPublicZkParams,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Fhe {
    client_key: SerialClientKey,
    public_key: Option<SerialCompactPublicKey>,
}

#[wasm_bindgen]
impl Fhe {
    #[wasm_bindgen(constructor)]
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

    pub fn encrypt(&self, val: Vec<u8>, data_type: WasmFheType) {
        fhe::encrypt(&val, &self.client_key, &data_type);
    }

    pub fn proven_encrypt(
        &self,
        val: Vec<u8>,
        data_type: WasmProvenFheType,
        public_zk_params: SerialPublicZkParams,
    ) {
        if let Some(pk) = &self.public_key {
            fhe::proven_encrypt(&val, pk, &public_zk_params, &data_type);
        } else {
            panic!("Public key is required for proven encryption");
        }
    }

    pub fn decrypt(&self, val: Vec<u8>, data_type: WasmFheType) {
        fhe::decrypt(&val, &self.client_key, &data_type);
    }
}
