use crate::configs::error::WasmError;
use crate::utils::web::get_storage;
use fhe_http_core::apis::base64;
use fhe_http_core::configs::typing::{
    SerialClientKey, SerialCompactPublicKey, SerialPbsParams, SerialPublicZkParams, SerialServerKey,
};
use fhe_http_core::fhe_traits::serializable::{KeySerializable, ZkSerializable};
use fhe_http_core::tfhe;
use tfhe::zk::CompactPkeCrs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct KeyGenerator {
    client_key: SerialClientKey,
    server_key: SerialServerKey,
    public_key: SerialCompactPublicKey,
    public_zk_params: SerialPublicZkParams,
    config: tfhe::Config,
}

#[wasm_bindgen]
impl KeyGenerator {
    #[wasm_bindgen(constructor)]
    pub fn new(params: Option<SerialPbsParams>) -> Self {
        let config: tfhe::Config = match params {
            Some(p) => {
                let params = tfhe::shortint::ClassicPBSParameters::try_deserialize(&p)
                    .unwrap_or_else(|_| panic!("Invalid PBS params"));
                tfhe::ConfigBuilder::with_custom_parameters(params, None).build()
            }
            None => tfhe::ConfigBuilder::default().build(),
        };
        Self {
            client_key: Vec::new(),
            server_key: Vec::new(),
            public_key: Vec::new(),
            public_zk_params: Vec::new(),
            config,
        }
    }

    pub fn init_keys(&mut self) {
        let res = self.load_keys_from_storage();
        if res.is_err() {
            print!("Damn no key\n");
            self.generate_new_keys();
            let _saving_res = self.save_keys_to_storage();
        }
    }

    pub fn get_client_key(&self) -> SerialClientKey {
        self.client_key.clone()
    }

    pub fn get_server_key(&self) -> SerialServerKey {
        // server key is compressed
        self.server_key.clone()
    }

    pub fn get_public_key(&self) -> SerialCompactPublicKey {
        // compact public key
        self.public_key.clone()
    }

    pub fn get_public_zk_params(&self) -> SerialPublicZkParams {
        self.public_zk_params.clone()
    }

    pub fn generate_new_keys(&mut self) {
        print!("Generating new keys\n");
        let cks = tfhe::ClientKey::generate(self.config);
        let compressed_sks: tfhe::CompressedServerKey = tfhe::CompressedServerKey::new(&cks);
        let public_key = tfhe::CompactPublicKey::new(&cks);
        self.client_key = cks.try_serialize().unwrap();
        self.server_key = compressed_sks.try_serialize().unwrap();
        self.public_key = public_key.try_serialize().unwrap();
    }

    pub fn generate_public_zk_params(
        &mut self,
        max_num_message: usize,
        params: Option<SerialPbsParams>,
    ) {
        println!("Generating new public zk params");

        let params = params
            .map(|p| tfhe::shortint::ClassicPBSParameters::try_deserialize(&p))
            .unwrap_or_else(|| panic!("No PBS params provided to generate public zk params"))
            .unwrap_or_else(|_| panic!("Invalid PBS params"));

        let crs = CompactPkeCrs::from_shortint_params(params, max_num_message)
            .unwrap_or_else(|_| panic!("Error: failed to generate public zk params"));

        let public_zk_params = crs.public_params();
        self.public_zk_params = public_zk_params.try_serialize().unwrap();
    }

    fn save_keys_to_storage(&self) -> Result<(), WasmError> {
        let local_storage = get_storage()?;
        let client_key = base64::encode_vec_u8(&self.client_key);
        let public_key = base64::encode_vec_u8(&self.public_key);
        let server_key = base64::encode_vec_u8(&self.server_key);

        let ck_res = local_storage.set_item("client_key", client_key.as_str());
        let pk_res = local_storage.set_item("public_key", public_key.as_str());
        let sk_res = local_storage.set_item("server_key", server_key.as_str());

        if ck_res.is_err() || pk_res.is_err() || sk_res.is_err() {
            return Err(WasmError::KeySavingError(
                "Error: failed to save keys".to_string(),
            ));
        }
        Ok(())
    }

    fn load_keys_from_storage(&mut self) -> Result<(), WasmError> {
        let local_storage = get_storage()?;

        //self.client_key =
        let client_key_str = local_storage.get_item("client_key").map_or(
            Err(WasmError::KeyLoadingError(
                "Error: client key not found".to_string(),
            )),
            |a| {
                Ok(a.ok_or(WasmError::KeyLoadingError(
                    "Error: client key not found".to_string(),
                ))?)
            },
        )?;

        let public_key_str = local_storage.get_item("public_key").map_or(
            Err(WasmError::KeyLoadingError(
                "Error: public key not found".to_string(),
            )),
            |a| {
                Ok(a.ok_or(WasmError::KeyLoadingError(
                    "Error: public key not found".to_string(),
                ))?)
            },
        )?;

        let server_key_str = local_storage.get_item("server_key").map_or(
            Err(WasmError::KeyLoadingError(
                "Error: server key not found".to_string(),
            )),
            |a| {
                Ok(a.ok_or(WasmError::KeyLoadingError(
                    "Error: server key not found".to_string(),
                ))?)
            },
        )?;

        self.client_key = base64::decode_vec_u8(&client_key_str);
        self.public_key = base64::decode_vec_u8(&public_key_str);
        self.server_key = base64::decode_vec_u8(&server_key_str);

        Ok(())
    }
}
