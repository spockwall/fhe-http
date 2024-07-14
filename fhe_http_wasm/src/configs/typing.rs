use fhe_http_core::configs::typing::{FheType, ProvenFheType};
use std::ops::Deref;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Clone, Debug)]
pub struct WasmFheType {
    inner: FheType,
}

#[wasm_bindgen]
pub struct WasmProvenFheType {
    inner: ProvenFheType,
}

impl WasmFheType {
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn from_str(s: &str) -> Self {
        WasmFheType {
            inner: FheType::from_str(s),
        }
    }
}

impl WasmProvenFheType {
    pub fn as_str(&self) -> &str {
        self.inner.as_str()
    }

    pub fn from_str(s: &str) -> Self {
        WasmProvenFheType {
            inner: ProvenFheType::from_str(s),
        }
    }
}

impl Into<FheType> for WasmFheType {
    fn into(self) -> FheType {
        self.inner
    }
}

impl Into<ProvenFheType> for WasmProvenFheType {
    fn into(self) -> ProvenFheType {
        self.inner
    }
}

impl Deref for WasmFheType {
    type Target = FheType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Deref for WasmProvenFheType {
    type Target = ProvenFheType;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[wasm_bindgen]
pub fn create_fhe_type(s: &str) -> WasmFheType {
    WasmFheType::from_str(s)
}

#[wasm_bindgen]
pub fn create_proven_fhe_type(s: &str) -> WasmProvenFheType {
    WasmProvenFheType::from_str(s)
}
